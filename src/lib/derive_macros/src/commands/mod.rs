use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, FnArg, ItemFn, LitStr, Pat, Result as SynResult, Type,
};

#[derive(Clone, Debug)]
struct Arg {
    name: String,
    required: bool,
    ty: String,
}

struct CommandAttr {
    name: String,
    permission: Option<String>,
}

impl Parse for CommandAttr {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let name = input.parse::<LitStr>()?.value();
        
        let mut permission = None;
        if input.peek(syn::Token![,]) {
            input.parse::<syn::Token![,]>()?;
            if input.peek(syn::Ident) {
                let ident = input.parse::<syn::Ident>()?;
                if ident == "permission" {
                    input.parse::<syn::Token![=]>()?;
                    permission = Some(input.parse::<LitStr>()?.value());
                }
            }
        }
        
        Ok(CommandAttr { name, permission })
    }
}

pub fn command(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = input_fn.clone().sig.ident;

    let command_attr = parse_macro_input!(attr as CommandAttr);

    let mut args = Vec::new();
    let mut bevy_args = Vec::<(Box<Pat>, Type)>::new();
    let mut has_sender_arg = false;
    let mut sender_arg_before_cmd_args = false;
    let mut sender_arg_index: Option<usize> = None;
    let mut first_arg_index: Option<usize> = None;

    for (idx, fn_arg) in input_fn.sig.inputs.iter_mut().enumerate() {
        let FnArg::Typed(fn_arg) = fn_arg else {
            return TokenStream::from(quote! {
                compiler_error!("command handler cannot have receiver");
            });
        };

        if fn_arg.attrs.is_empty() {
            bevy_args.push((fn_arg.pat.clone(), *fn_arg.ty.clone()));
        }

        let mut is_arg_attr = false;
        let mut is_sender_attr = false;
        let mut sender_arg_mismatched_ty = false;

        fn_arg.attrs.retain(|arg| {
            let is_arg = arg.path().is_ident("arg");
            let is_sender = arg.path().is_ident("sender");

            if is_arg {
                is_arg_attr = true;

                let required = match *fn_arg.ty {
                    Type::Path(ref path) => {
                        !path.path.segments.iter().any(|seg| seg.ident == "Option")
                    }
                    _ => true,
                };

                args.push(Arg {
                    name: fn_arg.pat.to_token_stream().to_string(),
                    required,
                    ty: fn_arg.ty.to_token_stream().to_string(),
                });
            }

            if is_sender {
                is_sender_attr = true;

                match *fn_arg.ty {
                    Type::Path(ref path) => {
                        if path.path.segments.iter().next_back().unwrap().ident != "Sender" {
                            sender_arg_mismatched_ty = true;
                            return false;
                        }
                    }
                    _ => {
                        sender_arg_mismatched_ty = true;
                        return false;
                    }
                }

                has_sender_arg = true;
            }

            !is_arg && !is_sender
        });

        if sender_arg_mismatched_ty {
            return TokenStream::from(quote! {
                compile_error!("invalid type for sender arg - should be Sender");
            });
        }

        if is_sender_attr && sender_arg_index.is_none() {
            sender_arg_index = Some(idx);
        }

        if is_arg_attr && first_arg_index.is_none() {
            first_arg_index = Some(idx);
        }
    }

    if let (Some(sender_idx), Some(arg_idx)) = (sender_arg_index, first_arg_index) {
        if sender_idx < arg_idx {
            sender_arg_before_cmd_args = true;
        }
    }

    if bevy_args.iter().any(|(_, ty)| {
        if let Type::Reference(refr) = ty {
            if let Type::Path(path) = *refr.clone().elem {
                println!("path reference: {:?}", path.path.segments.clone());
                let is_bevy = path.path.segments.iter().any(|seg| {
                    println!("{}", &seg.ident.to_string());
                    &seg.ident.to_string() == "bevy_ecs"
                });
                println!("is bevy? {is_bevy}");
                println!("{}", path.path.is_ident("World"));

                path.path.is_ident("World") && (is_bevy || path.path.segments.len() == 1)
            } else {
                false
            }
        } else {
            false
        }
    }) {
        return TokenStream::from(quote! {
            compile_error!("commands cannot accept bevy world arguments due to bevy restrictions")
        });
    }

    let system_name = format_ident!("__{}_handler", fn_name);
    let system_args = bevy_args
        .clone()
        .iter()
        .map(|(pat, ty)| {
            // Preserve the full pattern including `mut`
            quote! { #pat: #ty }
        })
        .collect::<Vec<proc_macro2::TokenStream>>();
    let system_types_state: Vec<_> = bevy_args
        .iter()
        .map(|(_, ty)| {
            // For SystemState, we need to use 'static, 'static for all SystemParams
            quote! { <#ty as bevy_ecs::system::SystemParam>::Item<'static, 'static> }
        })
        .collect();
    let system_arg_pats = bevy_args
        .clone()
        .iter()
        .map(|(pat, _)| match pat.as_ref() {
            syn::Pat::Ident(pat_ident) => {
                let ident = &pat_ident.ident;
                // Just use the identifier, not the full pattern (no mut in the call)
                quote!(#ident)
            }
            _ => quote!(#pat),
        })
        .collect::<Vec<proc_macro2::TokenStream>>();

    let arg_extractors = args
        .clone()
        .iter()
        .map(|arg| {
            let arg_name = format_ident!("__arg_{}", &arg.name);
            let name = &arg.name;
            let ty = syn::parse_str::<Type>(&arg.ty).expect("invalid arg type");

            quote! {
                let #arg_name: #ty = match __ctx.arg::<#ty>(#name) {
                    Ok(a) => a,
                    Err(err) => {
                        // TODO: Send error message via ChatAPI
                        continue;
                    }
                };
            }
        })
        .collect::<Vec<proc_macro2::TokenStream>>();
    
    let arg_names = args
        .iter()
        .map(|arg| format_ident!("__arg_{}", &arg.name))
        .collect::<Vec<_>>();

    let sender_param = if has_sender_arg {
        quote! { sender.clone(), }
    } else {
        quote! {}
    };

    let ctor_fn_name = format_ident!("__{}_register", fn_name);
    let command_name = command_attr.name;
    let command_permission = match command_attr.permission {
        Some(perm) => quote! { Some(#perm) },
        None => quote! { None },
    };

    let command_args = args
        .iter()
        .map(|arg| {
            let name = arg.name.clone();
            let required = arg.required;
            let ty = format_ident!("{}", &arg.ty);

            quote! {
                ferrumc_commands::arg::CommandArgumentNode {
                    name: #name.to_string(),
                    required: #required,
                    primitive: <#ty as ferrumc_commands::arg::CommandArgument>::primitive(),
                    suggester: <#ty as ferrumc_commands::arg::CommandArgument>::suggest,
                },
            }
        })
        .collect::<Vec<proc_macro2::TokenStream>>();

    let call = if has_sender_arg && sender_arg_before_cmd_args {
        quote! {
            #fn_name(#sender_param #(#arg_names,)* #(#system_arg_pats),*);
        }
    } else if has_sender_arg {
        quote! {
            #fn_name(#(#arg_names,)* #sender_param #(#system_arg_pats),*);
        }
    } else {
        quote! {
            #fn_name(#(#arg_names,)* #(#system_arg_pats),*);
        }
    };

    TokenStream::from(quote! {
        #[allow(non_snake_case)]
        #[allow(dead_code)]
        #[doc(hidden)]
        #input_fn

        #[allow(non_snake_case)]
        #[allow(unused_variables)]
        #[doc(hidden)]
        fn #system_name(world: &mut bevy_ecs::world::World) {
            use std::sync::Mutex;
            static EVENT_STATE: std::sync::OnceLock<Mutex<bevy_ecs::system::SystemState<
                bevy_ecs::event::EventReader<'static, 'static, ferrumc_commands::events::ResolvedCommandDispatchEvent>
            >>> = std::sync::OnceLock::new();
            
            let event_state_lock = EVENT_STATE.get_or_init(|| {
                Mutex::new(bevy_ecs::system::SystemState::new(world))
            });
            let mut event_state = event_state_lock.lock().expect("Failed to lock event state");
            
            // Collect matching events (EventReader auto-drains as we iterate)
            let to_run: Vec<_> = {
                let mut events = event_state.get_mut(world);
                events
                    .read()
                    .filter(|e| e.command.name == #command_name)
                    .map(|e| (e.command.clone(), e.input.clone(), e.sender.clone()))
                    .collect()
            };
            event_state.apply(world);
            
            // Early exit if no events
            if to_run.is_empty() {
                return;
            }

            // Handle each event
            for (__command, __input, sender) in to_run {
                // Parse arguments with CommandContext
                let __command_input = ferrumc_commands::CommandInput::of(__input);
                let mut __ctx = ferrumc_commands::CommandContext {
                    input: __command_input,
                    command: __command.clone(),
                    sender: sender.clone(),
                    world,
                };

                // Extract command arguments
                #(#arg_extractors)*

                drop(__ctx); // Release &mut World borrow

                // Fetch Bevy system parameters and call handler
                let mut params_state: bevy_ecs::system::SystemState<(#(#system_types_state,)*)> = 
                    bevy_ecs::system::SystemState::new(world);
                {
                    let (#(#system_arg_pats,)*) = params_state.get_mut(world);
                    #call
                }
                params_state.apply(world);
            }
        }

        #[ctor::ctor]
        #[doc(hidden)]
        fn #ctor_fn_name() {
            ferrumc_commands::infrastructure::add_system(#system_name);

            ferrumc_commands::infrastructure::register_command(std::sync::Arc::new(ferrumc_commands::Command {
                name: #command_name,
                args: vec![#(#command_args)*],
                permission: #command_permission,
            }));
        }
    })
}
