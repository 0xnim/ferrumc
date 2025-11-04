1 language

- [Deutsch](https://de.minecraft.wiki/w/Minecraft-Server-Protokoll/Pakete "Minecraft-Server-Protokoll/Pakete – Deutsch")

[![This article is licensed under a Creative Commons Attribution-ShareAlike 3.0 license. Derivative works must be licensed using the same or a compatible license.](https://minecraft.wiki/images/Cc-by-sa.svg?fc1a9)](https://creativecommons.org/licenses/by-sa/3.0/ "This article is licensed under a Creative Commons Attribution-ShareAlike 3.0 license. Derivative works must be licensed using the same or a compatible license.")

# _Java Edition_ protocol/Packets  Share article feedback

From Minecraft Wiki

< [Java Edition protocol](https://minecraft.wiki/w/Java_Edition_protocol "Java Edition protocol")

[Jump to navigation](https://minecraft.wiki/w/Java_Edition_protocol/Packets#mw-head) [Jump to search](https://minecraft.wiki/w/Java_Edition_protocol/Packets#searchInput)

This article is about the protocol for a stable release of _[Java Edition](https://minecraft.wiki/w/Java_Edition "Java Edition")_. For the protocol used in development versions of _[Java Edition](https://minecraft.wiki/w/Java_Edition "Java Edition")_, see [Java Edition protocol/Development version](https://minecraft.wiki/w/Java_Edition_protocol/Development_version "Java Edition protocol/Development version"). For the protocol used in _[Bedrock Edition](https://minecraft.wiki/w/Bedrock_Edition "Bedrock Edition")_, see [Bedrock Edition protocol](https://minecraft.wiki/w/Bedrock_Edition_protocol "Bedrock Edition protocol"). For the protocol used in old _[Pocket Edition](https://minecraft.wiki/w/Pocket_Edition "Pocket Edition")_ versions, see [Pocket Edition protocol](https://minecraft.wiki/w/Pocket_Edition_protocol "Pocket Edition protocol").

See also: [Protocol FAQ](https://minecraft.wiki/w/Java_Edition_protocol/FAQ "Java Edition protocol/FAQ")

[![](https://minecraft.wiki/images/Information_icon.svg?15c1c)](https://minecraft.wiki/w/File:Information_icon.svg)

This feature is exclusive to [_Java Edition_](https://minecraft.wiki/w/Java_Edition "Java Edition").

[![](https://minecraft.wiki/images/Information_icon.svg?15c1c)](https://minecraft.wiki/w/File:Information_icon.svg)

While you may use the contents of this page without restriction to create servers, clients, bots, etc; keep in mind that the contents of this page are distributed under the terms of [CC BY-SA 3.0 Unported](https://creativecommons.org/licenses/by-sa/3.0/). Reproductions and derivative works must be distributed accordingly.

This article presents a dissection of the current _[Java Edition](https://minecraft.wiki/w/Java_Edition "Java Edition")_ **protocol** for [1.21.10, protocol 773](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Protocol_version_numbers "Minecraft Wiki:Projects/wiki.vg merge/Protocol version numbers").

The changes between versions may be viewed at [Protocol History](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Protocol_History "Minecraft Wiki:Projects/wiki.vg merge/Protocol History").

## Contents

- [1Definitions](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Definitions)
  - [1.1Data types](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Data_types)
  - [1.2Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Identifier)
  - [1.3VarInt and VarLong](https://minecraft.wiki/w/Java_Edition_protocol/Packets#VarInt_and_VarLong)
  - [1.4Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Position)
  - [1.5Fixed-point numbers](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Fixed-point_numbers)
  - [1.6Arrays](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Arrays)
    - [1.6.1Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Array)
    - [1.6.2Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Prefixed_Array)
  - [1.7Bit sets](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Bit_sets)
    - [1.7.1BitSet](https://minecraft.wiki/w/Java_Edition_protocol/Packets#BitSet)
    - [1.7.2Fixed BitSet](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Fixed_BitSet)
  - [1.8Registry references](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Registry_references)
    - [1.8.1ID or X](https://minecraft.wiki/w/Java_Edition_protocol/Packets#ID_or_X)
    - [1.8.2ID Set](https://minecraft.wiki/w/Java_Edition_protocol/Packets#ID_Set)
  - [1.9Registry data](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Registry_data)
    - [1.9.1Sound Event](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Sound_Event)
    - [1.9.2Chat Type](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Chat_Type)
  - [1.10Teleport Flags](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Teleport_Flags)
  - [1.11Chunk Data](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Chunk_Data)
  - [1.12Light Data](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Light_Data)
  - [1.13Game Profile](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Game_Profile)
    - [1.13.1Debug Subscription Event](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Debug_Subscription_Event)
    - [1.13.2Debug Subscription Update](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Debug_Subscription_Update)
    - [1.13.3Debug Subscription Data](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Debug_Subscription_Data)
    - [1.13.4Debug Path Node](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Debug_Path_Node)
    - [1.13.5Debug Structure Info](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Debug_Structure_Info)
  - [1.14Other definitions](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Other_definitions)
- [2Packet format](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Packet_format)
  - [2.1Without compression](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Without_compression)
  - [2.2With compression](https://minecraft.wiki/w/Java_Edition_protocol/Packets#With_compression)
- [3Handshaking](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Handshaking)
  - [3.1Clientbound](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Clientbound)
  - [3.2Serverbound](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Serverbound)
    - [3.2.1Handshake](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Handshake)
    - [3.2.2Legacy Server List Ping](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Legacy_Server_List_Ping)
- [4Status](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Status)
  - [4.1Clientbound](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Clientbound_2)
    - [4.1.1Status Response](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Status_Response)
    - [4.1.2Pong Response (status)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Pong_Response_(status))
  - [4.2Serverbound](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Serverbound_2)
    - [4.2.1Status Request](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Status_Request)
    - [4.2.2Ping Request (status)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Ping_Request_(status))
- [5Login](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Login)
  - [5.1Clientbound](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Clientbound_3)
    - [5.1.1Disconnect (login)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Disconnect_(login))
    - [5.1.2Encryption Request](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Encryption_Request)
    - [5.1.3Login Success](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Login_Success)
    - [5.1.4Set Compression](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Compression)
    - [5.1.5Login Plugin Request](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Login_Plugin_Request)
    - [5.1.6Cookie Request (login)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Cookie_Request_(login))
  - [5.2Serverbound](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Serverbound_3)
    - [5.2.1Login Start](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Login_Start)
    - [5.2.2Encryption Response](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Encryption_Response)
    - [5.2.3Login Plugin Response](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Login_Plugin_Response)
    - [5.2.4Login Acknowledged](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Login_Acknowledged)
    - [5.2.5Cookie Response (login)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Cookie_Response_(login))
- [6Configuration](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Configuration)
  - [6.1Clientbound](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Clientbound_4)
    - [6.1.1Cookie Request (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Cookie_Request_(configuration))
    - [6.1.2Clientbound Plugin Message (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Clientbound_Plugin_Message_(configuration))
    - [6.1.3Disconnect (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Disconnect_(configuration))
    - [6.1.4Finish Configuration](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Finish_Configuration)
    - [6.1.5Clientbound Keep Alive (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Clientbound_Keep_Alive_(configuration))
    - [6.1.6Ping (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Ping_(configuration))
    - [6.1.7Reset Chat](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Reset_Chat)
    - [6.1.8Registry Data](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Registry_Data_2)
    - [6.1.9Remove Resource Pack (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Remove_Resource_Pack_(configuration))
    - [6.1.10Add Resource Pack (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Add_Resource_Pack_(configuration))
    - [6.1.11Store Cookie (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Store_Cookie_(configuration))
    - [6.1.12Transfer (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Transfer_(configuration))
    - [6.1.13Feature Flags](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Feature_Flags)
    - [6.1.14Update Tags (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Update_Tags_(configuration))
    - [6.1.15Clientbound Known Packs](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Clientbound_Known_Packs)
    - [6.1.16Custom Report Details (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Custom_Report_Details_(configuration))
    - [6.1.17Server Links (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Server_Links_(configuration))
    - [6.1.18Clear Dialog (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Clear_Dialog_(configuration))
    - [6.1.19Show Dialog (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Show_Dialog_(configuration))
  - [6.2Serverbound](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Serverbound_4)
    - [6.2.1Client Information (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Client_Information_(configuration))
    - [6.2.2Cookie Response (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Cookie_Response_(configuration))
    - [6.2.3Serverbound Plugin Message (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Serverbound_Plugin_Message_(configuration))
    - [6.2.4Acknowledge Finish Configuration](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Acknowledge_Finish_Configuration)
    - [6.2.5Serverbound Keep Alive (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Serverbound_Keep_Alive_(configuration))
    - [6.2.6Pong (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Pong_(configuration))
    - [6.2.7Resource Pack Response (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Resource_Pack_Response_(configuration))
    - [6.2.8Serverbound Known Packs](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Serverbound_Known_Packs)
    - [6.2.9Custom Click Action (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Custom_Click_Action_(configuration))
- [7Play](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Play)
  - [7.1Clientbound](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Clientbound_5)
    - [7.1.1Bundle Delimiter](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Bundle_Delimiter)
    - [7.1.2Spawn Entity](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Spawn_Entity)
    - [7.1.3Entity Animation](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Entity_Animation)
    - [7.1.4Award Statistics](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Award_Statistics)
    - [7.1.5Acknowledge Block Change](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Acknowledge_Block_Change)
    - [7.1.6Set Block Destroy Stage](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Block_Destroy_Stage)
    - [7.1.7Block Entity Data](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Block_Entity_Data)
    - [7.1.8Block Action](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Block_Action)
    - [7.1.9Block Update](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Block_Update)
    - [7.1.10Boss Bar](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Boss_Bar)
    - [7.1.11Change Difficulty](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Change_Difficulty)
    - [7.1.12Chunk Batch Finished](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Chunk_Batch_Finished)
    - [7.1.13Chunk Batch Start](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Chunk_Batch_Start)
    - [7.1.14Chunk Biomes](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Chunk_Biomes)
    - [7.1.15Clear Titles](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Clear_Titles)
    - [7.1.16Command Suggestions Response](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Command_Suggestions_Response)
    - [7.1.17Commands](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Commands)
    - [7.1.18Close Container](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Close_Container)
    - [7.1.19Set Container Content](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Container_Content)
    - [7.1.20Set Container Property](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Container_Property)
    - [7.1.21Set Container Slot](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Container_Slot)
    - [7.1.22Cookie Request (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Cookie_Request_(play))
    - [7.1.23Set Cooldown](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Cooldown)
    - [7.1.24Chat Suggestions](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Chat_Suggestions)
    - [7.1.25Clientbound Plugin Message (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Clientbound_Plugin_Message_(play))
    - [7.1.26Damage Event](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Damage_Event)
    - [7.1.27Debug Sample](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Debug_Sample)
    - [7.1.28Delete Message](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Delete_Message)
    - [7.1.29Disconnect (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Disconnect_(play))
    - [7.1.30Disguised Chat Message](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Disguised_Chat_Message)
    - [7.1.31Entity Event](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Entity_Event)
    - [7.1.32Teleport Entity](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Teleport_Entity)
    - [7.1.33Explosion](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Explosion)
    - [7.1.34Unload Chunk](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Unload_Chunk)
    - [7.1.35Game Event](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Game_Event)
    - [7.1.36Open Horse Screen](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Open_Horse_Screen)
    - [7.1.37Hurt Animation](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Hurt_Animation)
    - [7.1.38Initialize World Border](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Initialize_World_Border)
    - [7.1.39Clientbound Keep Alive (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Clientbound_Keep_Alive_(play))
    - [7.1.40Chunk Data and Update Light](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Chunk_Data_and_Update_Light)
    - [7.1.41World Event](https://minecraft.wiki/w/Java_Edition_protocol/Packets#World_Event)
    - [7.1.42Particle](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Particle)
    - [7.1.43Update Light](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Update_Light)
    - [7.1.44Login (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Login_(play))
    - [7.1.45Map Data](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Map_Data)
    - [7.1.46Merchant Offers](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Merchant_Offers)
    - [7.1.47Update Entity Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Update_Entity_Position)
    - [7.1.48Update Entity Position and Rotation](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Update_Entity_Position_and_Rotation)
    - [7.1.49Move Minecart Along Track](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Move_Minecart_Along_Track)
    - [7.1.50Update Entity Rotation](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Update_Entity_Rotation)
    - [7.1.51Move Vehicle (clientbound)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Move_Vehicle_(clientbound))
    - [7.1.52Open Book](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Open_Book)
    - [7.1.53Open Screen](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Open_Screen)
    - [7.1.54Open Sign Editor](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Open_Sign_Editor)
    - [7.1.55Ping (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Ping_(play))
    - [7.1.56Ping Response (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Ping_Response_(play))
    - [7.1.57Place Ghost Recipe](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Place_Ghost_Recipe)
    - [7.1.58Player Abilities (clientbound)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Player_Abilities_(clientbound))
    - [7.1.59Player Chat Message](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Player_Chat_Message)
    - [7.1.60End Combat](https://minecraft.wiki/w/Java_Edition_protocol/Packets#End_Combat)
    - [7.1.61Enter Combat](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Enter_Combat)
    - [7.1.62Combat Death](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Combat_Death)
    - [7.1.63Player Info Remove](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Player_Info_Remove)
    - [7.1.64Player Info Update](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Player_Info_Update)
    - [7.1.65Look At](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Look_At)
    - [7.1.66Synchronize Player Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Synchronize_Player_Position)
    - [7.1.67Player Rotation](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Player_Rotation)
    - [7.1.68Recipe Book Add](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Recipe_Book_Add)
    - [7.1.69Recipe Book Remove](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Recipe_Book_Remove)
    - [7.1.70Recipe Book Settings](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Recipe_Book_Settings)
    - [7.1.71Remove Entities](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Remove_Entities)
    - [7.1.72Remove Entity Effect](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Remove_Entity_Effect)
    - [7.1.73Reset Score](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Reset_Score)
    - [7.1.74Remove Resource Pack (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Remove_Resource_Pack_(play))
    - [7.1.75Add Resource Pack (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Add_Resource_Pack_(play))
    - [7.1.76Respawn](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Respawn)
    - [7.1.77Set Head Rotation](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Head_Rotation)
    - [7.1.78Update Section Blocks](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Update_Section_Blocks)
    - [7.1.79Select Advancements Tab](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Select_Advancements_Tab)
    - [7.1.80Server Data](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Server_Data)
    - [7.1.81Set Action Bar Text](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Action_Bar_Text)
    - [7.1.82Set Border Center](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Border_Center)
    - [7.1.83Set Border Lerp Size](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Border_Lerp_Size)
    - [7.1.84Set Border Size](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Border_Size)
    - [7.1.85Set Border Warning Delay](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Border_Warning_Delay)
    - [7.1.86Set Border Warning Distance](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Border_Warning_Distance)
    - [7.1.87Set Camera](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Camera)
    - [7.1.88Set Center Chunk](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Center_Chunk)
    - [7.1.89Set Render Distance](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Render_Distance)
    - [7.1.90Set Cursor Item](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Cursor_Item)
    - [7.1.91Set Default Spawn Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Default_Spawn_Position)
    - [7.1.92Display Objective](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Display_Objective)
    - [7.1.93Set Entity Metadata](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Entity_Metadata)
    - [7.1.94Link Entities](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Link_Entities)
    - [7.1.95Set Entity Velocity](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Entity_Velocity)
    - [7.1.96Set Equipment](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Equipment)
    - [7.1.97Set Experience](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Experience)
    - [7.1.98Set Health](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Health)
    - [7.1.99Set Held Item (clientbound)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Held_Item_(clientbound))
    - [7.1.100Update Objectives](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Update_Objectives)
    - [7.1.101Set Passengers](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Passengers)
    - [7.1.102Set Player Inventory Slot](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Player_Inventory_Slot)
    - [7.1.103Update Teams](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Update_Teams)
    - [7.1.104Update Score](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Update_Score)
    - [7.1.105Set Simulation Distance](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Simulation_Distance)
    - [7.1.106Set Subtitle Text](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Subtitle_Text)
    - [7.1.107Update Time](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Update_Time)
    - [7.1.108Set Title Text](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Title_Text)
    - [7.1.109Set Title Animation Times](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Title_Animation_Times)
    - [7.1.110Entity Sound Effect](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Entity_Sound_Effect)
    - [7.1.111Sound Effect](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Sound_Effect)
    - [7.1.112Start Configuration](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Start_Configuration)
    - [7.1.113Stop Sound](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Stop_Sound)
    - [7.1.114Store Cookie (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Store_Cookie_(play))
    - [7.1.115System Chat Message](https://minecraft.wiki/w/Java_Edition_protocol/Packets#System_Chat_Message)
    - [7.1.116Set Tab List Header And Footer](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Tab_List_Header_And_Footer)
    - [7.1.117Tag Query Response](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Tag_Query_Response)
    - [7.1.118Pickup Item](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Pickup_Item)
    - [7.1.119Synchronize Vehicle Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Synchronize_Vehicle_Position)
    - [7.1.120Test Instance Block Status](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Test_Instance_Block_Status)
    - [7.1.121Set Ticking State](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Ticking_State)
    - [7.1.122Step Tick](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Step_Tick)
    - [7.1.123Transfer (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Transfer_(play))
    - [7.1.124Update Advancements](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Update_Advancements)
    - [7.1.125Update Attributes](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Update_Attributes)
    - [7.1.126Entity Effect](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Entity_Effect)
    - [7.1.127Update Recipes](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Update_Recipes)
    - [7.1.128Update Tags (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Update_Tags_(play))
    - [7.1.129Projectile Power](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Projectile_Power)
    - [7.1.130Custom Report Details](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Custom_Report_Details)
    - [7.1.131Server Links](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Server_Links)
    - [7.1.132Waypoint](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Waypoint)
    - [7.1.133Clear Dialog (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Clear_Dialog_(play))
    - [7.1.134Show Dialog (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Show_Dialog_(play))
  - [7.2Serverbound](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Serverbound_5)
    - [7.2.1Confirm Teleportation](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Confirm_Teleportation)
    - [7.2.2Query Block Entity Tag](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Query_Block_Entity_Tag)
    - [7.2.3Bundle Item Selected](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Bundle_Item_Selected)
    - [7.2.4Change Difficulty](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Change_Difficulty_2)
    - [7.2.5Change Game Mode](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Change_Game_Mode)
    - [7.2.6Acknowledge Message](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Acknowledge_Message)
    - [7.2.7Chat Command](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Chat_Command)
    - [7.2.8Signed Chat Command](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Signed_Chat_Command)
    - [7.2.9Chat Message](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Chat_Message)
    - [7.2.10Player Session](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Player_Session)
    - [7.2.11Chunk Batch Received](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Chunk_Batch_Received)
    - [7.2.12Client Status](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Client_Status)
    - [7.2.13Client Tick End](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Client_Tick_End)
    - [7.2.14Client Information (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Client_Information_(play))
    - [7.2.15Command Suggestions Request](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Command_Suggestions_Request)
    - [7.2.16Acknowledge Configuration](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Acknowledge_Configuration)
    - [7.2.17Click Container Button](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Click_Container_Button)
    - [7.2.18Click Container](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Click_Container)
    - [7.2.19Close Container](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Close_Container_2)
    - [7.2.20Change Container Slot State](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Change_Container_Slot_State)
    - [7.2.21Cookie Response (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Cookie_Response_(play))
    - [7.2.22Serverbound Plugin Message (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Serverbound_Plugin_Message_(play))
    - [7.2.23Debug Sample Subscription](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Debug_Sample_Subscription)
    - [7.2.24Edit Book](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Edit_Book)
    - [7.2.25Query Entity Tag](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Query_Entity_Tag)
    - [7.2.26Interact](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Interact)
    - [7.2.27Jigsaw Generate](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Jigsaw_Generate)
    - [7.2.28Serverbound Keep Alive (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Serverbound_Keep_Alive_(play))
    - [7.2.29Lock Difficulty](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Lock_Difficulty)
    - [7.2.30Set Player Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Player_Position)
    - [7.2.31Set Player Position and Rotation](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Player_Position_and_Rotation)
    - [7.2.32Set Player Rotation](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Player_Rotation)
    - [7.2.33Set Player Movement Flags](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Player_Movement_Flags)
    - [7.2.34Move Vehicle (serverbound)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Move_Vehicle_(serverbound))
    - [7.2.35Paddle Boat](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Paddle_Boat)
    - [7.2.36Pick Item From Block](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Pick_Item_From_Block)
    - [7.2.37Pick Item From Entity](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Pick_Item_From_Entity)
    - [7.2.38Ping Request (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Ping_Request_(play))
    - [7.2.39Place Recipe](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Place_Recipe)
    - [7.2.40Player Abilities (serverbound)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Player_Abilities_(serverbound))
    - [7.2.41Player Action](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Player_Action)
    - [7.2.42Player Command](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Player_Command)
    - [7.2.43Player Input](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Player_Input)
    - [7.2.44Player Loaded](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Player_Loaded)
    - [7.2.45Pong (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Pong_(play))
    - [7.2.46Change Recipe Book Settings](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Change_Recipe_Book_Settings)
    - [7.2.47Set Seen Recipe](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Seen_Recipe)
    - [7.2.48Rename Item](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Rename_Item)
    - [7.2.49Resource Pack Response (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Resource_Pack_Response_(play))
    - [7.2.50Seen Advancements](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Seen_Advancements)
    - [7.2.51Select Trade](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Select_Trade)
    - [7.2.52Set Beacon Effect](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Beacon_Effect)
    - [7.2.53Set Held Item (serverbound)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Held_Item_(serverbound))
    - [7.2.54Program Command Block](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Program_Command_Block)
    - [7.2.55Program Command Block Minecart](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Program_Command_Block_Minecart)
    - [7.2.56Set Creative Mode Slot](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Creative_Mode_Slot)
    - [7.2.57Program Jigsaw Block](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Program_Jigsaw_Block)
    - [7.2.58Program Structure Block](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Program_Structure_Block)
    - [7.2.59Set Test Block](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Test_Block)
    - [7.2.60Update Sign](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Update_Sign)
    - [7.2.61Swing Arm](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Swing_Arm)
    - [7.2.62Teleport To Entity](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Teleport_To_Entity)
    - [7.2.63Test Instance Block Action](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Test_Instance_Block_Action)
    - [7.2.64Use Item On](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Use_Item_On)
    - [7.2.65Use Item](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Use_Item)
    - [7.2.66Custom Click Action (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Custom_Click_Action_(play))
- [8Navigation](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Navigation)

## Definitions\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=1&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Definitions") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=1&editintro=Template:License_wiki.vg "Edit section's source code: Definitions")\]

The Minecraft server accepts connections from TCP clients and communicates with them using _packets_. A packet is a sequence of bytes sent over the TCP connection. The meaning of a packet depends both on its packet ID and the current state of the connection. The initial state of each connection is [Handshaking](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Handshaking), and state is switched using the packets [Handshake](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Handshake) and [Login Success](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Login_Success).

### Data types\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=2&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Data types") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=2&editintro=Template:License_wiki.vg "Edit section's source code: Data types")\]

All data sent over the network (except for VarInt and VarLong) is [big-endian](https://en.wikipedia.org/wiki/Endianness#Big-endian "wikipedia:Endianness"), that is the bytes are sent from most significant byte to least significant byte. The majority of everyday computers are little-endian, therefore it may be necessary to change the endianness before sending data over the network.

| Name | Size (bytes) | Encodes | Notes |
| --- | --- | --- | --- |
| [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | 1 | Either false or true | True is encoded as `0x01`, false as `0x00`. |
| [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | 1 | An integer between -128 and 127 | Signed 8-bit integer, [two's complement](https://en.wikipedia.org/wiki/Two%27s_complement "wikipedia:Two's complement") |
| [Unsigned Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Unsigned_Byte) | 1 | An integer between 0 and 255 | Unsigned 8-bit integer |
| [Short](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Short) | 2 | An integer between -32768 and 32767 | Signed 16-bit integer, two's complement |
| [Unsigned Short](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Unsigned_Short) | 2 | An integer between 0 and 65535 | Unsigned 16-bit integer |
| [Int](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Int) | 4 | An integer between -2147483648 and 2147483647 | Signed 32-bit integer, two's complement |
| [Long](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Long) | 8 | An integer between -9223372036854775808 and 9223372036854775807 | Signed 64-bit integer, two's complement |
| [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | 4 | A [single-precision 32-bit IEEE 754 floating point number](https://en.wikipedia.org/wiki/Single-precision_floating-point_format "wikipedia:Single-precision floating-point format") |  |
| [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) | 8 | A [double-precision 64-bit IEEE 754 floating point number](https://en.wikipedia.org/wiki/Double-precision_floating-point_format "wikipedia:Double-precision floating-point format") |  |
| [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (n) | ≥ 1 <br>≤ (n×3) + 3 | A sequence of [Unicode](https://en.wikipedia.org/wiki/Unicode "wikipedia:Unicode") [scalar values](http://unicode.org/glossary/#unicode_scalar_value) | [UTF-8](https://en.wikipedia.org/wiki/UTF-8 "wikipedia:UTF-8") string prefixed with its size in bytes as a VarInt. Maximum length of `n` characters, which varies by context. The encoding used on the wire is regular UTF-8, _not_ [Java's "slight modification"](https://docs.oracle.com/en/java/javase/18/docs/api/java.base/java/io/DataInput.html#modified-utf-8). However, the length of the string for purposes of the length limit is its number of [UTF-16](https://en.wikipedia.org/wiki/UTF-16 "wikipedia:UTF-16") code units, that is, scalar values > U+FFFF are counted as two. Up to `n × 3` bytes can be used to encode a UTF-8 string comprising `n` code units when converted to UTF-16, and both of those limits are checked. Maximum `n` value is 32767. The + 3 is due to the max size of a valid length VarInt. |
| [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) | Varies | See [Text component format](https://minecraft.wiki/w/Text_component_format "Text component format") | Encoded as a [NBT Tag](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/NBT "Minecraft Wiki:Projects/wiki.vg merge/NBT"), with the type of tag used depending on the case:<br>- As a [String Tag](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/NBT#Specification:string_tag "Minecraft Wiki:Projects/wiki.vg merge/NBT"): For components only containing text (no styling, no events etc.).<br>- As a [Compound Tag](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/NBT#Specification:compound_tag "Minecraft Wiki:Projects/wiki.vg merge/NBT"): Every other case. |
| [JSON Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:JSON_Text_Component) | ≥ 1 <br>≤ (262144×3) + 3 | See [Text component format](https://minecraft.wiki/w/Text_component_format "Text component format") | The maximum permitted length when decoding is 262144, but the vanilla server since 1.20.3 refuses to encode longer than 32767. This may be a bug. |
| [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) | ≥ 1 <br>≤ (32767×3) + 3 | See [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Identifier) below | Encoded as a String with max length of 32767. |
| [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | ≥ 1 <br>≤ 5 | An integer between -2147483648 and 2147483647 | Variable-length data encoding a two's complement signed 32-bit integer; more info in [their section](https://minecraft.wiki/w/Java_Edition_protocol/Packets#VarInt_and_VarLong) |
| [VarLong](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarLong) | ≥ 1 <br>≤ 10 | An integer between -9223372036854775808 and 9223372036854775807 | Variable-length data encoding a two's complement signed 64-bit integer; more info in [their section](https://minecraft.wiki/w/Java_Edition_protocol/Packets#VarInt_and_VarLong) |
| [Entity Metadata](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Entity_Metadata) | Varies | Miscellaneous information about an entity | See [Entity metadata#Entity Metadata Format](https://minecraft.wiki/w/Java_Edition_protocol/Entity_metadata#Entity_Metadata_Format "Java Edition protocol/Entity metadata") |
| [Slot](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Slot) | Varies | An item stack in an inventory or container | See [Slot Data](https://minecraft.wiki/w/Java_Edition_protocol/Slot_Data "Java Edition protocol/Slot Data") |
| [Hashed Slot](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Hashed_Slot) | Varies | Similar to Slot, but with the data component values being sent as a hash instead of their actual contents | See [Slot Data#Hashed Format](https://minecraft.wiki/w/Java_Edition_protocol/Slot_Data#Hashed_Format "Java Edition protocol/Slot Data") |
| [NBT](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:NBT) | Varies | Depends on context | See [NBT](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/NBT "Minecraft Wiki:Projects/wiki.vg merge/NBT") |
| [Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Position) | 8 | An integer/block position: x (-33554432 to 33554431), z (-33554432 to 33554431), y (-2048 to 2047) | x as a 26-bit integer, followed by z as a 26-bit integer, followed by y as a 12-bit integer (all signed, two's complement). See also [the section below](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Position). |
| [Angle](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Angle) | 1 | A rotation angle in steps of 1/256 of a full turn | Whether or not this is signed does not matter, since the resulting angles are the same. |
| [UUID](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:UUID) | 16 | A [UUID](https://en.wikipedia.org/wiki/Universally_unique_identifier "wikipedia:Universally unique identifier") | Encoded as an unsigned 128-bit integer (or two unsigned 64-bit integers: the most significant 64 bits and then the least significant 64 bits) |
| [BitSet](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:BitSet) | Varies | See [#BitSet](https://minecraft.wiki/w/Java_Edition_protocol/Packets#BitSet) below | A length-prefixed bit set. |
| [Fixed BitSet](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Fixed_BitSet) (n) | ceil(n / 8) | See [#Fixed BitSet](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Fixed_BitSet) below | A bit set with a fixed length of n bits. |
| [Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Optional) X | 0 or size of X | A field of type X, or nothing | Whether or not the field is present must be known from the context. |
| [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) X | size of [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) \+ (is present [?](https://en.wikipedia.org/wiki/Ternary_conditional_operator "wikipedia:Ternary conditional operator") Size of X : 0) | A boolean and if present, a field of type X | The boolean is true if the field is present. |
| [Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Array) of X | length times size of X | Zero or more fields of type X | The length must be known from the context. |
| [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of X | size of [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) \+ size of X \* length | See [#Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Prefixed_Array) below | A length-prefixed array. |
| X [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | size of X | A specific value from a given list | The list of possible values and how each is encoded as an X must be known from the context. An invalid value sent by either side will usually result in the client being disconnected with an error or even crashing. |
| [EnumSet](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:EnumSet) (n) | ceil(n / 8) | [Fixed BitSet](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Fixed_BitSet) (n) | A bitset associated to an enum where each bit corresponds to an enum variant. The number of enum variants n must be known from the context. |
| [Byte Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte_Array) | Varies | Depends on context | This is just a sequence of zero or more bytes, its meaning should be explained somewhere else, e.g. in the packet description. The length must also be known from the context. |
| [ID or](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:ID_or) X | size of [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) \+ (size of X or 0) | See [#ID or X](https://minecraft.wiki/w/Java_Edition_protocol/Packets#ID_or_X) below | Either a registry ID or an inline data definition of type X. |
| [ID Set](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:ID_Set) | Varies | See [#ID Set](https://minecraft.wiki/w/Java_Edition_protocol/Packets#ID_Set) below | Set of registry IDs specified either inline or as a reference to a tag. |
| [Sound Event](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Sound_Event) | Varies | See [#Sound Event](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Sound_Event) below | Parameters for a sound event. |
| [Chat Type](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Chat_Type) | Varies | See [#Chat Type](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Chat_Type) below | Parameters for a direct chat type. |
| [Teleport Flags](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Teleport_Flags) | 4 | See [#Teleport Flags](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Teleport_Flags) below | Bit field specifying how a teleportation is to be applied on each axis. |
| [Recipe Display](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Recipe_Display) | Varies | See [Recipes#Recipe Display structure](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Recipes#Recipe_Display_structure "Minecraft Wiki:Projects/wiki.vg merge/Recipes") | Description of a recipe for use for use by the client. |
| [Slot Display](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Slot_Display) | Varies | See [Recipes#Slot Display structure](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Recipes#Slot_Display_structure "Minecraft Wiki:Projects/wiki.vg merge/Recipes") | Description of a recipe ingredient slot for use for use by the client. |
| [Chunk Data](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Chunk_Data) | Varies | See [#Chunk Data](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Chunk_Data) below |  |
| [Light Data](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Light_Data) | Varies | See [#Light Data](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Light_Data) below |  |
| X [or](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:or) Y | size of [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) \+ (isX ? size of X : size of Y) | A boolean and X or Y | The boolean is true if X is encoded and false if Y is encoded. |
| [Game Profile](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Game_Profile) | Varies | See [#Game Profile](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Game_Profile) below | A Minecraft player profile. |
| [Debug Subscription Event](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Debug_Subscription_Event) | Varies | See [#Debug Subscription Event](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Debug_Subscription_Event) below |  |
| [Debug Subscription Update](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Debug_Subscription_Update) | Varies | See [#Debug Subscription Update](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Debug_Subscription_Update) below |  |

### Identifier

Identifiers are a namespaced location, in the form of `minecraft:thing`. If the namespace is not provided, it defaults to `minecraft` (i.e. `thing` is `minecraft:thing`). Custom content should always be in its own namespace, not the default one. Both the namespace and value can use all lowercase alphanumeric characters (a-z and 0-9), dot (`.`), dash (`-`), and underscore (`_`). In addition, values can use slash (`/`). The naming convention is `lower_case_with_underscores`. [More information](https://minecraft.net/en-us/article/minecraft-snapshot-17w43a).
For ease of determining whether a namespace or value is valid, here are regular expressions for each:

- Namespace: `[a-z0-9.-_]`
- Value: `[a-z0-9.-_/]`

### VarInt and VarLong

Variable-length format such that smaller numbers use fewer bytes. These are very similar to [Protocol Buffer Varints](http://developers.google.com/protocol-buffers/docs/encoding#varints): the 7 least significant bits are used to encode the value and the most significant bit indicates whether there's another byte after it for the next part of the number. The least significant group is written first, followed by each of the more significant groups; thus, VarInts are effectively little endian (however, groups are 7 bits, not 8).

VarInts are never longer than 5 bytes, and VarLongs are never longer than 10 bytes. Within these limits, unnecessarily long encodings (e.g. `81 00` to encode 1) are allowed.

Pseudocode to read and write VarInts and VarLongs:

```
private static final int SEGMENT_BITS = 0x7F;
private static final int CONTINUE_BIT = 0x80;
```

```
public int readVarInt() {
    int value = 0;
    int position = 0;
    byte currentByte;

    while (true) {
        currentByte = readByte();
        value |= (currentByte & SEGMENT_BITS) << position;

        if ((currentByte & CONTINUE_BIT) == 0) break;

        position += 7;

        if (position >= 32) throw new RuntimeException("VarInt is too big");
    }

    return value;
}
```

```
public long readVarLong() {
    long value = 0;
    int position = 0;
    byte currentByte;

    while (true) {
        currentByte = readByte();
        value |= (long) (currentByte & SEGMENT_BITS) << position;

        if ((currentByte & CONTINUE_BIT) == 0) break;

        position += 7;

        if (position >= 64) throw new RuntimeException("VarLong is too big");
    }

    return value;
}
```

```
public void writeVarInt(int value) {
    while (true) {
        if ((value & ~SEGMENT_BITS) == 0) {
            writeByte(value);
            return;
        }

        writeByte((value & SEGMENT_BITS) | CONTINUE_BIT);

        // Note: >>> means that the sign bit is shifted with the rest of the number rather than being left alone
        value >>>= 7;
    }
}
```

```
public void writeVarLong(long value) {
    while (true) {
        if ((value & ~((long) SEGMENT_BITS)) == 0) {
            writeByte(value);
            return;
        }

        writeByte((value & SEGMENT_BITS) | CONTINUE_BIT);

        // Note: >>> means that the sign bit is shifted with the rest of the number rather than being left alone
        value >>>= 7;
    }
}
```

[![](https://minecraft.wiki/images/thumb/Warning_pixelated_art.png/32px-Warning_pixelated_art.png?d0083)](https://minecraft.wiki/w/File:Warning_pixelated_art.png)

Note Minecraft's VarInts are identical to [LEB128](https://en.wikipedia.org/wiki/LEB128) with the slight change of throwing a exception if it goes over a set amount of bytes.

[![](https://minecraft.wiki/images/thumb/Warning_pixelated_art.png/32px-Warning_pixelated_art.png?d0083)](https://minecraft.wiki/w/File:Warning_pixelated_art.png)

Note that Minecraft's VarInts are not encoded using Protocol Buffers; it's just similar. If you try to use Protocol Buffers Varints with Minecraft's VarInts, you'll get incorrect results in some cases. The major differences:

- Minecraft's VarInts are all signed, but do not use the ZigZag encoding. Protocol buffers have 3 types of Varints: `uint32` (normal encoding, unsigned), `sint32` (ZigZag encoding, signed), and `int32` (normal encoding, signed). Minecraft's are the `int32` variety. Because Minecraft uses the normal encoding instead of ZigZag encoding, negative values always use the maximum number of bytes.
- Minecraft's VarInts are never longer than 5 bytes and its VarLongs will never be longer than 10 bytes, while Protocol Buffer Varints will always use 10 bytes when encoding negative numbers, even if it's an `int32`.

Sample VarInts:

| Value | Hex bytes | Decimal bytes |
| --- | --- | --- |
| 0 | 0x00 | 0 |
| 1 | 0x01 | 1 |
| 2 | 0x02 | 2 |
| 127 | 0x7f | 127 |
| 128 | 0x80 0x01 | 128 1 |
| 255 | 0xff 0x01 | 255 1 |
| 25565 | 0xdd 0xc7 0x01 | 221 199 1 |
| 2097151 | 0xff 0xff 0x7f | 255 255 127 |
| 2147483647 | 0xff 0xff 0xff 0xff 0x07 | 255 255 255 255 7 |
| -1 | 0xff 0xff 0xff 0xff 0x0f | 255 255 255 255 15 |
| -2147483648 | 0x80 0x80 0x80 0x80 0x08 | 128 128 128 128 8 |

Sample VarLongs:

| Value | Hex bytes | Decimal bytes |
| --- | --- | --- |
| 0 | 0x00 | 0 |
| 1 | 0x01 | 1 |
| 2 | 0x02 | 2 |
| 127 | 0x7f | 127 |
| 128 | 0x80 0x01 | 128 1 |
| 255 | 0xff 0x01 | 255 1 |
| 2147483647 | 0xff 0xff 0xff 0xff 0x07 | 255 255 255 255 7 |
| 9223372036854775807 | 0xff 0xff 0xff 0xff 0xff 0xff 0xff 0xff 0x7f | 255 255 255 255 255 255 255 255 127 |
| -1 | 0xff 0xff 0xff 0xff 0xff 0xff 0xff 0xff 0xff 0x01 | 255 255 255 255 255 255 255 255 255 1 |
| -2147483648 | 0x80 0x80 0x80 0x80 0xf8 0xff 0xff 0xff 0xff 0x01 | 128 128 128 128 248 255 255 255 255 1 |
| -9223372036854775808 | 0x80 0x80 0x80 0x80 0x80 0x80 0x80 0x80 0x80 0x01 | 128 128 128 128 128 128 128 128 128 1 |

### Position

**Note:** What you are seeing here is the latest version of the [Data types](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Data_types "Minecraft Wiki:Projects/wiki.vg merge/Data types") article, but the position type was [different before 1.14](https://minecraft.wiki/w/Java_Edition_protocol/Data_types?oldid=2767033#Position).

64-bit value split into three **signed** integer parts:

- x: 26 MSBs
- z: 26 middle bits
- y: 12 LSBs

For example, a 64-bit position can be broken down as follows:

Example value (big endian): `01000110000001110110001100 10110000010101101101001000 001100111111`

- The red value is the X coordinate, which is `18357644` in this example.

- The blue value is the Z coordinate, which is `-20882616` in this example.

- The green value is the Y coordinate, which is `831` in this example.


Encoded as follows:

```
((x & 0x3FFFFFF) << 38) | ((z & 0x3FFFFFF) << 12) | (y & 0xFFF)
```

And decoded as:

```
val = read_long();
x = val >> 38;
y = val << 52 >> 52;
z = val << 26 >> 38;
```

Note: The above assumes that the right shift operator sign extends the value (this is called an [arithmetic shift](https://en.wikipedia.org/wiki/Arithmetic_shift)), so that the signedness of the coordinates is preserved. In many languages, this requires the integer type of `val` to be signed. In the absence of such an operator, the following may be useful:

```
if x >= 1 << 25 { x -= 1 << 26 }
if y >= 1 << 11 { y -= 1 << 12 }
if z >= 1 << 25 { z -= 1 << 26 }
```

### Fixed-point numbers

Some fields may be stored as [fixed-point numbers](https://en.wikipedia.org/wiki/Fixed-point_arithmetic), where a certain number of bits represent the signed integer part (number to the left of the decimal point) and the rest represent the fractional part (to the right). Floating point numbers (float and double), in contrast, keep the number itself (mantissa) in one chunk, while the location of the decimal point (exponent) is stored beside it. Essentially, while fixed-point numbers have lower range than floating point numbers, their fractional precision is greater for higher values.

Prior to version 1.9 a fixed-point format with 5 fraction bits and 27 integer bits was used to send entity positions to the client. Some uses of fixed point remain in modern versions, but they differ from that format.

Most programming languages lack support for fractional integers directly, but you can represent them as integers. The following C or Java-like pseudocode converts a double to a fixed-point integer with n fraction bits:

```
 x_fixed = (int)(x_double * (1 << n));
```

And back again:

```
 x_double = (double)x_fixed / (1 << n);
```

### Arrays

The types [Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Array) and [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) represent a collection of X in a specified order.

#### Array

Represents a list where the length is not encoded. The length must be known from the context. If the array is empty nothing will be encoded.

A [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) Array with the values \["Hello", "World!"\] has the following data when encoded:

| Field Name | Field Type | Value |
| --- | --- | --- |
| First element | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) | Hello |
| Second element | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) | World! |

#### Prefixed Array

Represents an array prefixed by its length. If the array is empty the length will still be encoded.

| Field Name | Field Type |
| --- | --- |
| Length | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |
| Data | [Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Array) of X |

### Bit sets

The types [BitSet](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:BitSet) and [Fixed BitSet](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Fixed_BitSet) represent packed lists of bits. The vanilla implementation uses Java's [`BitSet`](https://docs.oracle.com/javase/8/docs/api/java/util/BitSet.html) class.

#### BitSet

Bit sets of type BitSet are prefixed by their length in longs.

| Field Name | Field Type | Meaning |
| --- | --- | --- |
| Length | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Number of longs in the following array. May be 0 (if no bits are set). |
| Data | [Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Array) of [Long](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Long) | A packed representation of the bit set as created by [`BitSet.toLongArray`](https://docs.oracle.com/javase/8/docs/api/java/util/BitSet.html#toLongArray--). |

The ith bit is set when `(Data[i / 64] & (1 << (i % 64))) != 0`, where i starts at 0.

#### Fixed BitSet

Bit sets of type Fixed BitSet (n) have a fixed length of n bits, encoded as `ceil(n / 8)` bytes. Note that this is different from BitSet, which uses longs.

| Field Name | Field Type | Meaning |
| --- | --- | --- |
| Data | [Byte Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte_Array) (n) | A packed representation of the bit set as created by [`BitSet.toByteArray`](https://docs.oracle.com/javase/8/docs/api/java/util/BitSet.html#toByteArray--), padded with zeroes at the end to fit the specified length. |

The ith bit is set when `(Data[i / 8] & (1 << (i % 8))) != 0`, where i starts at 0. This encoding is _not_ equivalent to the long array in BitSet.

### Registry references

#### ID or X

Represents a data record of type X, either inline, or by reference to a registry implied by context.

| Field Name | Field Type | Meaning |
| --- | --- | --- |
| ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | 0 if value of type X is given inline; otherwise registry ID + 1. |
| Value | [Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Optional) X | Only present if ID is 0. |

#### ID Set

Represents a set of IDs in a certain registry (implied by context), either directly (enumerated IDs) or indirectly (tag name).

| Field Name | Field Type | Meaning |
| --- | --- | --- |
| Type | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Value used to determine the data that follows. It can be either:<br>- 0 - Represents a named set of IDs defined by a tag.<br>- Anything else - Represents an ad-hoc set of IDs enumerated inline. |
| Tag Name | [Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Optional) [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) | The registry tag defining the ID set. Only present if Type is 0. |
| IDs | [Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Optional) [Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Array) of [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | An array of registry IDs. Only present if Type is not 0.<br>The size of the array is equal to `Type - 1`. |

### Registry data

These types are commonly used in conjuction with [ID or](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:ID_or) X to specify custom data inline.

#### Sound Event

Describes a sound that can be played.

| Name | Type | Description |
| --- | --- | --- |
| Sound Name | [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) |  |
| Has Fixed Range | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | Whether this sound has a fixed range, as opposed to a variable volume based on distance. |
| Fixed Range | [Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Optional) [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | The maximum range of the sound. Only present if Has Fixed Range is true. |

#### Chat Type

Describes a direct chat type that a message can be sent with.

| Name | Type | Description |
| --- | --- | --- |
| Chat | (See below) |  |
| Narration | (See below) |  |

The chat type decorations look like:

| Name | Type | Description |
| --- | --- | --- |
| Translation Key | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) |  |
| Parameters | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | 0: sender, 1: target, 2: content |
| Style | [NBT](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:NBT) |  |

### Teleport Flags

A bit field represented as an [Int](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Int), specifying how a teleportation is to be applied on each axis.

In the lower 8 bits of the bit field, a set bit means the teleportation on the corresponding axis is relative, and an unset bit that it is absolute.

| Hex Mask | Field |
| --- | --- |
| 0x0001 | Relative X |
| 0x0002 | Relative Y |
| 0x0004 | Relative Z |
| 0x0008 | Relative Yaw |
| 0x0010 | Relative Pitch |
| 0x0020 | Relative Velocity X |
| 0x0040 | Relative Velocity Y |
| 0x0080 | Relative Velocity Z |
| 0x0100 | Rotate velocity according to the change in rotation, _before_ applying the velocity change in this packet. Combining this with absolute rotation works as expected—the difference in rotation is still used. |

### Chunk Data

| Field Name | Field Type | Notes |
| --- | --- | --- |
| Heightmaps | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of Heightmap | See [Chunk Format#Heightmap structure](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Chunk_Format#Heightmap_structure "Minecraft Wiki:Projects/wiki.vg merge/Chunk Format") |
| Data | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | See [Chunk Format#Data structure](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Chunk_Format#Data_structure "Minecraft Wiki:Projects/wiki.vg merge/Chunk Format") |
| Block Entities | Packed XZ | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) | [Unsigned Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Unsigned_Byte) | The packed section coordinates are relative to the chunk they are in. Values 0-15 are valid. <br>```<br>packed_xz = ((blockX & 15) << 4) | (blockZ & 15) // encode<br>x = packed_xz >> 4, z = packed_xz & 15 // decode<br>``` |
| Y | [Short](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Short) | The height relative to the world |
| Type | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | The type of block entity |
| Data | [NBT](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/NBT "Minecraft Wiki:Projects/wiki.vg merge/NBT") | The block entity's data, without the X, Y, and Z values |

### Light Data

| Field Name | Field Type | Notes |
| --- | --- | --- |
| Sky Light Mask | [BitSet](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:BitSet) | BitSet containing bits for each section in the world + 2. Each set bit indicates that the corresponding 16×16×16 chunk section has data in the Sky Light array below. The least significant bit is for blocks 16 blocks to 1 block below the min world height (one section below the world), while the most significant bit covers blocks 1 to 16 blocks above the max world height (one section above the world). |
| Block Light Mask | [BitSet](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:BitSet) | BitSet containing bits for each section in the world + 2. Each set bit indicates that the corresponding 16×16×16 chunk section has data in the Block Light array below. The order of bits is the same as in Sky Light Mask. |
| Empty Sky Light Mask | [BitSet](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:BitSet) | BitSet containing bits for each section in the world + 2. Each set bit indicates that the corresponding 16×16×16 chunk section has all zeros for its Sky Light data. The order of bits is the same as in Sky Light Mask. |
| Empty Block Light Mask | [BitSet](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:BitSet) | BitSet containing bits for each section in the world + 2. Each set bit indicates that the corresponding 16×16×16 chunk section has all zeros for its Block Light data. The order of bits is the same as in Sky Light Mask. |
| Sky Light arrays | Sky Light array | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) (2048) of [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | The length of any inner array is always 2048; There is 1 array for each bit set to true in the sky light mask, starting with the lowest value. Half a byte per light value. |
| Block Light arrays | Block Light array | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) (2048) of [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | The length of any inner array is always 2048; There is 1 array for each bit set to true in the block light mask, starting with the lowest value. Half a byte per light value. |

### Game Profile

Describes a Minecraft player profile.

| Name | Type | Description |
| --- | --- | --- |
| UUID | [UUID](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:UUID) |  |
| Username | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (16) |  |
| Properties | Name | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) (16) | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (64) |  |
| Value | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (32767) |  |
| Signature | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (1024) |  |

The Properties field looks like the response of [querying a player's skin and cape](https://minecraft.wiki/w/Mojang_API#Query_player's_skin_and_cape "Mojang API") from Mojang's official API, with the difference being the usage of the protocol format instead of JSON. That is, each player will usually have one property with Name being “textures” and Value being a JSON string encoded using Base64. An empty properties array is also acceptable, and will cause clients to display the player with one of the default skins depending on their UUID. For more information, refer to the aforementioned Mojang API page.

#### Debug Subscription Event

| Name | Type | Description |
| --- | --- | --- |
| Debug subscription type | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | ID of the debug subscription type for the next field. |
| Data | [Debug Subscription Data](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Debug_Subscription_Data) | Value of the debug subscription. |

#### Debug Subscription Update

| Name | Type | Description |
| --- | --- | --- |
| Subscription type | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | ID of the debug subscription type for the next field. |
| Data | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) [Debug Subscription Data](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Debug_Subscription_Data) | Value of the debug subscription. |

#### Debug Subscription Data

| Subscription Type | Field Name | Field Type | Meaning |
| --- | --- | --- | --- |
| 0: Dedicated server tick time | _no fields_ |
| 1: Bee | Hive position | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) [Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Position) |  |
| Flower position | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) [Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Position) |  |
| Travel ticks | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |
| Blacklisted hives | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of [Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Position) |  |
| 2: Villager brain | Name | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) |  |
| Profession | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) |  |
| XP | [Int](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Int) |  |
| Health | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) |  |
| Max health | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) |  |
| Inventory | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) |  |
| Wants golem | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) |  |
| Anger level | [Int](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Int) |  |
| Activities | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) |  |
| Behaviors | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) |  |
| Memories | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) |  |
| Gossips | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) |  |
| POIs | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of [Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Position) |  |
| Potential POIs | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of [Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Position) |  |
| 3: Breeze | Attack target | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |
| Jump target | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) [Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Position) |  |
| 4: Goal selector | Priority | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |
| Is running | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) |  |
| Name | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (255) |  |
| 5: Entity path | Reached | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) |  |
| Next block index | [Int](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Int) |  |
| Block position | [Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Position) |  |
| Nodes | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of [Debug Path Node](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Debug_Path_Node) |  |
| Target nodes | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of [Debug Path Node](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Debug_Path_Node) |  |
| Open set | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of [Debug Path Node](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Debug_Path_Node) |  |
| Closed set | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of [Debug Path Node](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Debug_Path_Node) |  |
| Max node distance | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) |  |
| 6: Entity block intersection | ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | IN\_BLOCK = 0, IN\_FLUID = 1, IN\_AIR = 2 |
| 7: Bee hive | Type | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | ID in the `minecraft:block` registry. |
| Occupant count | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |
| Honey level | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |
| Sedated | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) |  |
| 8: POI | Position | [Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Position) |  |
| Type | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | ID in the `minecraft:point_of_interest_type` registry. |
| Free ticket count | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |
| 9: Redstone wire orientation | ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |
| 10: Village section | _no fields_ |
| 11: Raid | Positions | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of [Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Position) |  |
| 12: Structure | Structures | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of [Debug Structure Info](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Debug_Structure_Info) |  |
| 13: Game event listener | Listener radius | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |
| 14: Neighbor update | Position | [Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Position) |  |
| 15: Game event | Event | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | ID in the `minecraft:game_event` registry. |
| X | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) |
| Y | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) |
| Z | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) |

#### Debug Path Node

| Field name | Field type | Meaning |
| --- | --- | --- |
| X | [Int](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Int) |  |
| Y | [Int](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Int) |  |
| Z | [Int](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Int) |  |
| Walked distance | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) |  |
| Cost malus | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) |  |
| Closed | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) |  |
| Type | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | BLOCKED = 0, OPEN = 1, WALKABLE = 2, WALKABLE\_DOOR = 3, TRAPDOOR = 4, POWDER\_SNOW = 5, DANGER\_POWDER\_SNOW = 6, FENCE = 7, LAVA = 8, WATER = 9, WATER\_BORDER = 10, RAIL = 11, UNPASSABLE\_RAIL = 12, DANGER\_FIRE = 13, DAMAGE\_FIRE = 14, DANGER\_OTHER = 15, DAMAGE\_OTHER = 16, DOOR\_OPEN = 17, DOOR\_WOOD\_CLOSED = 18, DOOR\_IRON\_CLOSED = 19, BREACH = 20, LEAVES = 21, STICKY\_HONEY = 22, COCOA = 23, DAMAGE\_CAUTIOUS = 24, DANGER\_TRAPDOOR = 25 |
| F | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) |  |

#### Debug Structure Info

| Field name | Field type | Meaning |
| --- | --- | --- |
| Bounding box min | [Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Position) |  |
| Bounding box max | [Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Position) |  |
| Pieces | Piece bounding box min | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) | [Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Position) |  |
| Piece bounding box max | [Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Position) |  |
| Is start | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) |  |

### Other definitions\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=3&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Other definitions") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=3&editintro=Template:License_wiki.vg "Edit section's source code: Other definitions")\]

| Term | Definition |
| --- | --- |
| Player | When used in the singular, Player always refers to the client connected to the server. |
| Entity | Entity refers to any item, player, mob, minecart or boat etc. See [the Minecraft Wiki article](https://minecraft.wiki/w/Entity "Entity") for a full list. |
| EID | An EID — or Entity ID — is a 4-byte sequence used to identify a specific entity. An entity's EID is unique on the entire server. |
| XYZ | In this document, the axis names are the same as those shown in the debug screen (F3). Y points upwards, X points east, and Z points south. |
| Meter | The meter is Minecraft's base unit of length, equal to the length of a vertex of a solid block. The term “block” may be used to mean “meter” or “cubic meter”. |
| Registry | A table describing static, gameplay-related objects of some kind, such as the types of entities, block states or biomes. The entries of a registry are typically associated with textual or numeric identifiers, or both.<br>Minecraft has a unified registry system used to implement most of the registries, including blocks, items, entities, biomes and dimensions. These "ordinary" registries associate entries with both namespaced textual identifiers (see [#Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Identifier)), and signed (positive) 32-bit numeric identifiers. There is also a registry of registries listing all of the registries in the registry system. Some other registries, most notably the [block state registry](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Chunk_Format#Block_state_registry "Minecraft Wiki:Projects/wiki.vg merge/Chunk Format"), are however implemented in a more ad-hoc fashion.<br>Some registries, such as biomes and dimensions, can be customized at runtime by the server (see [Registry Data](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Registry_Data "Minecraft Wiki:Projects/wiki.vg merge/Registry Data")), while others, such as blocks, items and entities, are hardcoded. The contents of the hardcoded registries can be extracted via the built-in [Data Generators](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Data_Generators "Minecraft Wiki:Projects/wiki.vg merge/Data Generators") system. |
| Block state | Each block in Minecraft has 0 or more properties, which in turn may have any number of possible values. These represent, for example, the orientations of blocks, poweredness states of redstone components, and so on. Each of the possible permutations of property values for a block is a distinct block state. The block state registry assigns a numeric identifier to every block state of every block.<br>A current list of properties and state ID ranges is found on [burger](https://pokechu22.github.io/Burger/1.21.html).<br>Alternatively, the vanilla server now includes an option to export the current block state ID mapping by running `java -DbundlerMainClass=net.minecraft.data.Main -jar minecraft_server.jar --reports`. See [Data Generators](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Data_Generators "Minecraft Wiki:Projects/wiki.vg merge/Data Generators") for more information. |
| Vanilla | The official implementation of Minecraft as developed and released by Mojang. |
| Sequence | The action number counter for local block changes, incremented by one when clicking a block with a hand, right-clicking an item, or starting or finishing digging a block. Counter handles latency to avoid applying outdated block changes to the local world. It is also used to revert ghost blocks created when placing blocks, using buckets, or breaking blocks. |

## Packet format\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=4&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Packet format") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=4&editintro=Template:License_wiki.vg "Edit section's source code: Packet format")\]

Packets cannot be larger than 221 − 1 or 2097151 bytes (the maximum that can be sent in a 3-byte [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt)). Moreover, the length field must not be longer than 3 bytes, even if the encoded value is within the limit. Unnecessarily long encodings at 3 bytes or below are still allowed. For compressed packets, this applies to the Packet Length field, i.e. the compressed length.

### Without compression\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=5&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Without compression") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=5&editintro=Template:License_wiki.vg "Edit section's source code: Without compression")\]

| Field Name | Field Type | Notes |
| --- | --- | --- |
| Length | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Length of Packet ID + Data |
| Packet ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Corresponds to `protocol_id` from [the server's packet report](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Data_Generators#Packets_report "Minecraft Wiki:Projects/wiki.vg merge/Data Generators") |
| Data | [Byte Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte_Array) | Depends on the connection state and packet ID, see the sections below |

### With compression\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=6&veaction=edit&editintro=Template:License_wiki.vg "Edit section: With compression") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=6&editintro=Template:License_wiki.vg "Edit section's source code: With compression")\]

Once a [Set Compression](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Compression) packet (with a non-negative threshold) is sent, [zlib](https://en.wikipedia.org/wiki/Zlib "wikipedia:Zlib") compression is enabled for all following packets. The format of a packet changes slightly to include the size of the uncompressed packet.

| Present? | Compressed? | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- |
| always | No | Packet Length | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Length of (Data Length) + length of compressed (Packet ID + Data) |
| if size >= threshold | No | Data Length | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Length of uncompressed (Packet ID + Data) |
| Yes | Packet ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | zlib compressed packet ID (see the sections below) |
| Data | [Byte Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte_Array) | zlib compressed packet data (see the sections below) |
| if size < threshold | No | Data Length | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | 0 to indicate uncompressed |
| Packet ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | packet ID (see the sections below) |
| Data | [Byte Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte_Array) | packet data (see the sections below) |

For serverbound packets, the uncompressed length of (Packet ID + Data) must not be greater than 223 or 8388608 bytes. Note that a length equal to 223 is permitted, which differs from the compressed length limit. The vanilla client, on the other hand, has no limit for the uncompressed length of incoming compressed packets.

If the size of the buffer containing the packet data and ID (as a [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt)) is smaller than the threshold specified in the packet [Set Compression](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Compression). It will be sent as uncompressed. This is done by setting the data length to 0. (Comparable to sending a non-compressed format with an extra 0 between the length and packet data).

If it's larger than or equal to the threshold, then it follows the regular compressed protocol format.

The vanilla server (but not client) rejects compressed packets smaller than the threshold. Uncompressed packets exceeding the threshold, however, are accepted.

Compression can be disabled by sending the packet [Set Compression](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Compression) with a negative Threshold, or not sending the Set Compression packet at all.

## Handshaking\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=7&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Handshaking") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=7&editintro=Template:License_wiki.vg "Edit section's source code: Handshaking")\]

### Clientbound\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=8&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Clientbound") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=8&editintro=Template:License_wiki.vg "Edit section's source code: Clientbound")\]

There are no clientbound packets in the Handshaking state, since the protocol immediately switches to a different state after the client sends the first packet.

### Serverbound\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=9&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Serverbound") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=9&editintro=Template:License_wiki.vg "Edit section's source code: Serverbound")\]

#### Handshake\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=10&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Handshake") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=10&editintro=Template:License_wiki.vg "Edit section's source code: Handshake")\]

This packet causes the server to switch into the target state. It should be sent right after opening the TCP connection to prevent the server from disconnecting.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x00`<br>_resource:_<br>`intention` | Handshaking | Server | Protocol Version | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | See [protocol version numbers](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Protocol_version_numbers "Minecraft Wiki:Projects/wiki.vg merge/Protocol version numbers") (currently 772 in Minecraft 1.21.8). |
| Server Address | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (255) | Hostname or IP, e.g. localhost or 127.0.0.1, that was used to connect. The vanilla server does not use this information. This is the name obtained after [SRV record](https://en.wikipedia.org/wiki/SRV_record "w:SRV record") resolution, except in 1.17 (and no older or newer version) and during server list ping ( [MC-278651](https://bugs.mojang.com/browse/MC-278651)), where it is the host portion of the address specified by the user directly. In 1.17.1 and later if a literal IP address is specified by the user, [reverse DNS lookup](https://en.wikipedia.org/wiki/Reverse_DNS_lookup "w:Reverse DNS lookup") is attempted, and the result is used as the value of this field if successful. |
| Server Port | [Unsigned Short](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Unsigned_Short) | Default is 25565. The vanilla server does not use this information. |
| Intent | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | 1 for [Status](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Status), 2 for [Login](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Login), 3 for [Transfer](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Login). |

#### Legacy Server List Ping\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=11&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Legacy Server List Ping") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=11&editintro=Template:License_wiki.vg "Edit section's source code: Legacy Server List Ping")\]

[![](https://minecraft.wiki/images/thumb/Warning_pixelated_art.png/32px-Warning_pixelated_art.png?d0083)](https://minecraft.wiki/w/File:Warning_pixelated_art.png)

This packet uses a nonstandard format. It is never length-prefixed, and the packet ID is an [Unsigned Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Unsigned_Byte) instead of a [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt).

While not technically part of the current protocol, (legacy) clients may send this packet to initiate [Server List Ping](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Server_List_Ping "Minecraft Wiki:Projects/wiki.vg merge/Server List Ping"), and modern servers should handle it correctly.
The format of this packet is a remnant of the pre-Netty age, before the switch to Netty in 1.7 brought the standard format that is recognized now. This packet merely exists to inform legacy clients that they can't join our modern server.

Modern clients (tested with 1.21.5 + 1.21.4) also send this packet when the server does not send any response within a 30 seconds time window or when the connection is immediately closed.

[![](https://minecraft.wiki/images/thumb/Warning_pixelated_art.png/32px-Warning_pixelated_art.png?d0083)](https://minecraft.wiki/w/File:Warning_pixelated_art.png)

The client does not close the connection with the legacy packet on its own!
It only gets closed when the Minecraft client is closed.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| 0xFE | Handshaking | Server | Payload | [Unsigned Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Unsigned_Byte) | always 1 (`0x01`). |

See [Server List Ping#1.6](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Server_List_Ping#1.6 "Minecraft Wiki:Projects/wiki.vg merge/Server List Ping") for the details of the protocol that follows this packet.

## Status\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=12&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Status") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=12&editintro=Template:License_wiki.vg "Edit section's source code: Status")\]

Main page: [Minecraft Wiki:Projects/wiki.vg merge/Server List Ping](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Server_List_Ping "Minecraft Wiki:Projects/wiki.vg merge/Server List Ping")

### Clientbound\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=13&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Clientbound") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=13&editintro=Template:License_wiki.vg "Edit section's source code: Clientbound")\]

#### Status Response\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=14&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Status Response") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=14&editintro=Template:License_wiki.vg "Edit section's source code: Status Response")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x00`<br>_resource:_<br>`status_response` | Status | Client | JSON Response | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (32767) | See [Server List Ping#Status Response](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Server_List_Ping#Status_Response "Minecraft Wiki:Projects/wiki.vg merge/Server List Ping"); as with all strings, this is prefixed by its length as a [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt). |

#### Pong Response (status)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=15&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Pong Response (status)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=15&editintro=Template:License_wiki.vg "Edit section's source code: Pong Response (status)")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x01`<br>_resource:_<br>`pong_response` | Status | Client | Timestamp | [Long](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Long) | Should match the one sent by the client. |

### Serverbound\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=16&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Serverbound") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=16&editintro=Template:License_wiki.vg "Edit section's source code: Serverbound")\]

#### Status Request\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=17&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Status Request") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=17&editintro=Template:License_wiki.vg "Edit section's source code: Status Request")\]

The status can only be requested once, immediately after the handshake, before any ping. The server won't respond otherwise.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x00`<br>_resource:_<br>`status_request` | Status | Server | _no fields_ |

#### Ping Request (status)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=18&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Ping Request (status)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=18&editintro=Template:License_wiki.vg "Edit section's source code: Ping Request (status)")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x01`<br>_resource:_<br>`ping_request` | Status | Server | Timestamp | [Long](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Long) | May be any number, but vanilla clients will always use the timestamp in milliseconds. |

## Login\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=19&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Login") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=19&editintro=Template:License_wiki.vg "Edit section's source code: Login")\]

The login process is as follows:

01. C→S: [Handshake](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Handshake) with intent set to 2 (login)
02. C→S: [Login Start](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Login_Start)
03. S→C: [Encryption Request](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Encryption_Request)
04. Client auth (if enabled)
05. C→S: [Encryption Response](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Encryption_Response)
06. Server auth (if enabled)
07. Both enable encryption
08. S→C: [Set Compression](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Compression) (optional)
09. S→C: [Login Success](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Login_Success)
10. C→S: [Login Acknowledged](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Login_Acknowledged)

Set Compression, if present, must be sent before Login Success. Note that anything sent after Set Compression must use the [Post Compression packet format](https://minecraft.wiki/w/Java_Edition_protocol/Packets#With_compression).

Three modes of operation are possible depending on how the packets are sent:

- Online-mode with encryption
- Offline-mode with encryption
- Offline-mode without encryption

For online-mode servers (the ones with authentication enabled), encryption is always mandatory, and the entire process described above needs to be followed.

For offline-mode servers (the ones with authentication disabled), encryption is optional, and part of the process can be skipped. In that case, [Login Start](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Login_Start) is directly followed by [Login Success](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Login_Success). The vanilla server only uses UUID v3 for offline player UUIDs, deriving it from the string `OfflinePlayer:<player's name>`. For example, Notch’s offline UUID would be chosen from the string `OfflinePlayer:Notch`. This is not a requirement however, the UUID can be set to anything.

As of 1.21, the vanilla server never uses encryption in offline mode.

See [protocol encryption](https://minecraft.wiki/w/Protocol_encryption "Protocol encryption") for details.

### Clientbound\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=20&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Clientbound") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=20&editintro=Template:License_wiki.vg "Edit section's source code: Clientbound")\]

#### Disconnect (login)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=21&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Disconnect (login)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=21&editintro=Template:License_wiki.vg "Edit section's source code: Disconnect (login)")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x00`<br>_resource:_<br>`login_disconnect` | Login | Client | Reason | [JSON Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:JSON_Text_Component) | The reason why the player was disconnected. |

#### Encryption Request\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=22&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Encryption Request") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=22&editintro=Template:License_wiki.vg "Edit section's source code: Encryption Request")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x01`<br>_resource:_<br>`hello` | Login | Client | Server ID | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (20) | Always empty when sent by the vanilla server. |
| Public Key | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | The server's public key, in bytes. |
| Verify Token | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | A sequence of random bytes generated by the server. |
| Should authenticate | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | Whether the client should attempt to [authenticate through mojang servers](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Protocol_Encryption#Authentication "Minecraft Wiki:Projects/wiki.vg merge/Protocol Encryption"). |

See [protocol encryption](https://minecraft.wiki/w/Protocol_encryption "Protocol encryption") for details.

#### Login Success\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=23&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Login Success") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=23&editintro=Template:License_wiki.vg "Edit section's source code: Login Success")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x02`<br>_resource:_<br>`login_finished` | Login | Client | Profile | [Game Profile](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Game_Profile) |  |

#### Set Compression\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=24&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Set Compression") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=24&editintro=Template:License_wiki.vg "Edit section's source code: Set Compression")\]

Enables compression. If compression is enabled, all following packets are encoded in the [compressed packet format](https://minecraft.wiki/w/Java_Edition_protocol/Packets#With_compression). Negative values will disable compression, meaning the packet format should remain in the [uncompressed packet format](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Without_compression). However, this packet is entirely optional, and if not sent, compression will also not be enabled (the vanilla server does not send the packet when compression is disabled).

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x03`<br>_resource:_<br>`login_compression` | Login | Client | Threshold | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Maximum size of a packet before it is compressed. |

#### Login Plugin Request\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=25&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Login Plugin Request") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=25&editintro=Template:License_wiki.vg "Edit section's source code: Login Plugin Request")\]

Used to implement a custom handshaking flow together with [Login Plugin Response](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Login_Plugin_Response).

Unlike plugin messages in "play" mode, these messages follow a lock-step request/response scheme, where the client is expected to respond to a request indicating whether it understood. The vanilla client always responds that it hasn't understood and sends an empty payload.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x04`<br>_resource:_<br>`custom_query` | Login | Client | Message ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Generated by the server - should be unique to the connection. |
| Channel | [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) | Name of the [plugin channel](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Plugin_channels "Minecraft Wiki:Projects/wiki.vg merge/Plugin channels") used to send the data. |
| Data | [Byte Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte_Array) (1048576) | Any data, depending on the channel. The length of this array must be inferred from the packet length. |

#### Cookie Request (login)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=26&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Cookie Request (login)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=26&editintro=Template:License_wiki.vg "Edit section's source code: Cookie Request (login)")\]

Requests a cookie that was previously stored.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x05`<br>_resource:_<br>`cookie_request` | Login | Client | Key | [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) | The identifier of the cookie. |

### Serverbound\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=27&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Serverbound") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=27&editintro=Template:License_wiki.vg "Edit section's source code: Serverbound")\]

#### Login Start\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=28&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Login Start") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=28&editintro=Template:License_wiki.vg "Edit section's source code: Login Start")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x00`<br>_resource:_<br>`hello` | Login | Server | Name | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (16) | Player's Username. |
| Player UUID | [UUID](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:UUID) | The [UUID](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:UUID) of the player logging in. Unused by the vanilla server. |

#### Encryption Response\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=29&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Encryption Response") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=29&editintro=Template:License_wiki.vg "Edit section's source code: Encryption Response")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x01`<br>_resource:_<br>`key` | Login | Server | Shared Secret | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | Shared Secret value, encrypted with the server's public key. |
| Verify Token | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | Verify Token value, encrypted with the same public key as the shared secret. |

See [protocol encryption](https://minecraft.wiki/w/Protocol_encryption "Protocol encryption") for details.

#### Login Plugin Response\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=30&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Login Plugin Response") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=30&editintro=Template:License_wiki.vg "Edit section's source code: Login Plugin Response")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x02`<br>_resource:_<br>`custom_query_answer` | Login | Server | Message ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Should match ID from server. |
| Data | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) [Byte Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte_Array) (1048576) | Any data, depending on the channel. The length of this array must be inferred from the packet length. Only present if the client understood the request. |

#### Login Acknowledged\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=31&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Login Acknowledged") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=31&editintro=Template:License_wiki.vg "Edit section's source code: Login Acknowledged")\]

Acknowledgement to the [Login Success](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Login_Success) packet sent by the server.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x03`<br>_resource:_<br>`login_acknowledged` | Login | Server | _no fields_ |

This packet switches the connection state to [configuration](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Configuration).

#### Cookie Response (login)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=32&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Cookie Response (login)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=32&editintro=Template:License_wiki.vg "Edit section's source code: Cookie Response (login)")\]

Response to a [Cookie Request (login)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Cookie_Request_(login)) from the server. The vanilla server only accepts responses of up to 5 kiB in size.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x04`<br>_resource:_<br>`cookie_response` | Login | Server | Key | [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) | The identifier of the cookie. |
| Payload | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) (5120) of [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | The data of the cookie. |

## Configuration\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=33&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Configuration") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=33&editintro=Template:License_wiki.vg "Edit section's source code: Configuration")\]

### Clientbound\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=34&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Clientbound") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=34&editintro=Template:License_wiki.vg "Edit section's source code: Clientbound")\]

#### Cookie Request (configuration)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=35&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Cookie Request (configuration)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=35&editintro=Template:License_wiki.vg "Edit section's source code: Cookie Request (configuration)")\]

Requests a cookie that was previously stored.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x00`<br>_resource:_<br>`cookie_request` | Configuration | Client | Key | [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) | The identifier of the cookie. |

#### Clientbound Plugin Message (configuration)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=36&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Clientbound Plugin Message (configuration)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=36&editintro=Template:License_wiki.vg "Edit section's source code: Clientbound Plugin Message (configuration)")\]

Main page: [Minecraft Wiki:Projects/wiki.vg merge/Plugin channels](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Plugin_channels "Minecraft Wiki:Projects/wiki.vg merge/Plugin channels")

Mods and plugins can use this to send their data. Minecraft itself uses several [plugin channels](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Plugin_channels "Minecraft Wiki:Projects/wiki.vg merge/Plugin channels"). These internal channels are in the `minecraft` namespace.

More information on how it works on [Dinnerbone's blog](https://web.archive.org/web/20220831140929/https://dinnerbone.com/blog/2012/01/13/minecraft-plugin-channels-messaging/). More documentation about internal and popular registered channels are [here](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Plugin_channels "Minecraft Wiki:Projects/wiki.vg merge/Plugin channels").

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x01`<br>_resource:_<br>`custom_payload` | Configuration | Client | Channel | [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) | Name of the [plugin channel](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Plugin_channels "Minecraft Wiki:Projects/wiki.vg merge/Plugin channels") used to send the data. |
| Data | [Byte Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte_Array) (1048576) | Any data. The length of this array must be inferred from the packet length. |

In vanilla clients, the maximum data length is 1048576 bytes.

#### Disconnect (configuration)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=37&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Disconnect (configuration)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=37&editintro=Template:License_wiki.vg "Edit section's source code: Disconnect (configuration)")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x02`<br>_resource:_<br>`disconnect` | Configuration | Client | Reason | [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) | The reason why the player was disconnected. |

#### Finish Configuration\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=38&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Finish Configuration") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=38&editintro=Template:License_wiki.vg "Edit section's source code: Finish Configuration")\]

Sent by the server to notify the client that the configuration process has finished. The client answers with [Acknowledge Finish Configuration](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Acknowledge_Finish_Configuration) whenever it is ready to continue.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x03`<br>_resource:_<br>`finish_configuration` | Configuration | Client | _no fields_ |

This packet switches the connection state to [play](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Play).

#### Clientbound Keep Alive (configuration)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=39&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Clientbound Keep Alive (configuration)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=39&editintro=Template:License_wiki.vg "Edit section's source code: Clientbound Keep Alive (configuration)")\]

The server will frequently send out a keep-alive, each containing a random ID. The client must respond with the same payload (see [Serverbound Keep Alive](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Serverbound_Keep_Alive_(configuration))). If the client does not respond to a Keep Alive packet within 15 seconds after it was sent, the server kicks the client. Vice versa, if the server does not send any keep-alives for 20 seconds, the client will disconnect and yield a "Timed out" exception.

The vanilla server uses a system-dependent time in milliseconds to generate the keep alive ID value.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x04`<br>_resource:_<br>`keep_alive` | Configuration | Client | Keep Alive ID | [Long](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Long) |  |

#### Ping (configuration)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=40&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Ping (configuration)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=40&editintro=Template:License_wiki.vg "Edit section's source code: Ping (configuration)")\]

Packet is not used by the vanilla server. When sent to the client, the client responds with a [Pong](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Pong_(configuration)) packet with the same ID.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x05`<br>_resource:_<br>`ping` | Configuration | Client | ID | [Int](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Int) |  |

#### Reset Chat\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=41&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Reset Chat") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=41&editintro=Template:License_wiki.vg "Edit section's source code: Reset Chat")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x06`<br>_resource:_<br>`reset_chat` | Configuration | Client | _no fields_ |

#### Registry Data\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=42&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Registry Data") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=42&editintro=Template:License_wiki.vg "Edit section's source code: Registry Data")\]

Represents certain registries that are sent from the server and are applied on the client.

See [Registry Data](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Registry_Data "Minecraft Wiki:Projects/wiki.vg merge/Registry Data") for details.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x07`<br>_resource:_<br>`registry_data` | Configuration | Client | Registry ID | [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) |  |
| Entries | Entry ID | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) | [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) |  |
| Data | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) [NBT](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:NBT) | Entry data. |

#### Remove Resource Pack (configuration)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=43&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Remove Resource Pack (configuration)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=43&editintro=Template:License_wiki.vg "Edit section's source code: Remove Resource Pack (configuration)")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x08`<br>_resource:_<br>`resource_pack_pop` | Configuration | Client | UUID | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) [UUID](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:UUID) | The [UUID](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:UUID) of the resource pack to be removed. If not present, every resource pack will be removed. |

#### Add Resource Pack (configuration)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=44&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Add Resource Pack (configuration)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=44&editintro=Template:License_wiki.vg "Edit section's source code: Add Resource Pack (configuration)")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x09`<br>_resource:_<br>`resource_pack_push` | Configuration | Client | UUID | [UUID](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:UUID) | The unique identifier of the resource pack. |
| URL | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (32767) | The URL to the resource pack. |
| Hash | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (40) | A 40 character hexadecimal, case-insensitive [SHA-1](https://en.wikipedia.org/wiki/SHA-1 "wikipedia:SHA-1") hash of the resource pack file.<br>If it's not a 40-character hexadecimal string, the client will not use it for hash verification and likely waste bandwidth. |
| Forced | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | The vanilla client will be forced to use the resource pack from the server. If they decline, they will be kicked from the server. |
| Prompt Message | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) | This is shown in the prompt making the client accept or decline the resource pack (only if present). |

#### Store Cookie (configuration)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=45&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Store Cookie (configuration)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=45&editintro=Template:License_wiki.vg "Edit section's source code: Store Cookie (configuration)")\]

Stores some arbitrary data on the client, which persists between server transfers. The vanilla client only accepts cookies of up to 5 kiB in size.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x0A`<br>_resource:_<br>`store_cookie` | Configuration | Client | Key | [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) | The identifier of the cookie. |
| Payload | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) (5120) of [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | The data of the cookie. |

#### Transfer (configuration)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=46&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Transfer (configuration)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=46&editintro=Template:License_wiki.vg "Edit section's source code: Transfer (configuration)")\]

Notifies the client that it should transfer to the given server. Cookies previously stored are preserved between server transfers.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x0B`<br>_resource:_<br>`transfer` | Configuration | Client | Host | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (32767) | The hostname or IP of the server. |
| Port | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | The port of the server. |

#### Feature Flags\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=47&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Feature Flags") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=47&editintro=Template:License_wiki.vg "Edit section's source code: Feature Flags")\]

Used to enable and disable features, generally experimental ones, on the client.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x0C`<br>_resource:_<br>`update_enabled_features` | Configuration | Client | Feature Flags | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) |  |

There is one special feature flag, which is in most versions:

- minecraft:vanilla - enables vanilla features

For the other feature flags, which may change between versions, see [Experiments#Java\_Edition](https://minecraft.wiki/w/Experiments#Java_Edition "Experiments").

#### Update Tags (configuration)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=48&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Update Tags (configuration)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=48&editintro=Template:License_wiki.vg "Edit section's source code: Update Tags (configuration)")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x0D`<br>_resource:_<br>`update_tags` | Configuration | Client | Array of tags | Registry | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) | [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) | Registry identifier (Vanilla expects tags for the registries `minecraft:block`, `minecraft:item`, `minecraft:fluid`, `minecraft:entity_type`, and `minecraft:game_event`) |
| Array of Tag | (See below) |  |

Tag arrays look like:

| Field Name | Field Type | Notes |
| --- | --- | --- |
| Tags | Tag name | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) | [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) |  |
| Entries | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Numeric IDs of the given type (block, item, etc.). This list replaces the previous list of IDs for the given tag. If some preexisting tags are left unmentioned, a warning is printed. |

See [Tag](https://minecraft.wiki/w/Tag "Tag") on the Minecraft Wiki for more information, including a list of vanilla tags.

#### Clientbound Known Packs\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=49&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Clientbound Known Packs") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=49&editintro=Template:License_wiki.vg "Edit section's source code: Clientbound Known Packs")\]

Informs the client of which data packs are present on the server.
The client is expected to respond with its own [Serverbound Known Packs](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Serverbound_Known_Packs) packet.
The vanilla server does not continue with Configuration until it receives a response.

The vanilla client requires the `minecraft:core` pack with version `1.21.8` for a normal login sequence. This packet must be sent before the Registry Data packets.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x0E`<br>_resource:_<br>`select_known_packs` | Configuration | Client | Known Packs | Namespace | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (32767) |  |
| ID | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (32767) |  |
| Version | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (32767) |  |

#### Custom Report Details (configuration)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=50&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Custom Report Details (configuration)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=50&editintro=Template:License_wiki.vg "Edit section's source code: Custom Report Details (configuration)")\]

Contains a list of key-value text entries that are included in any crash or disconnection report generated during connection to the server.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x0F`<br>_resource:_<br>`custom_report_details` | Configuration | Client | Details | Title | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) (32) | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (128) |  |
| Description | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (4096) |  |

#### Server Links (configuration)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=51&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Server Links (configuration)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=51&editintro=Template:License_wiki.vg "Edit section's source code: Server Links (configuration)")\]

This packet contains a list of links that the vanilla client will display in the menu available from the pause menu. Link labels can be built-in or custom (i.e., any text).

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x10`<br>_resource:_<br>`server_links` | Configuration | Client | Links | Label | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) [or](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:or) [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) | Enums are used for built-in labels (see below), and text components for custom labels. |
| URL | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) | Valid URL. |

| ID | Name | Notes |
| --- | --- | --- |
| 0 | Bug Report | Displayed on connection error screen; included as a comment in the disconnection report. |
| 1 | Community Guidelines |  |
| 2 | Support |  |
| 3 | Status |  |
| 4 | Feedback |  |
| 5 | Community |  |
| 6 | Website |  |
| 7 | Forums |  |
| 8 | News |  |
| 9 | Announcements |  |

#### Clear Dialog (configuration)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=52&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Clear Dialog (configuration)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=52&editintro=Template:License_wiki.vg "Edit section's source code: Clear Dialog (configuration)")\]

If we're currently in a dialog screen, then this removes the current screen and switches back to the previous one.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x11`<br>_resource:_<br>`clear_dialog` | Configuration | Client | _no fields_ |

#### Show Dialog (configuration)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=53&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Show Dialog (configuration)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=53&editintro=Template:License_wiki.vg "Edit section's source code: Show Dialog (configuration)")\]

Show a custom dialog screen to the client.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x12`<br>_resource:_<br>`show_dialog` | Configuration | Client | Dialog | [NBT](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:NBT) | Inline definition as described at [Registry\_data#Dialog](https://minecraft.wiki/w/Java_Edition_protocol/Registry_data#Dialog "Java Edition protocol/Registry data"). |

### Serverbound\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=54&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Serverbound") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=54&editintro=Template:License_wiki.vg "Edit section's source code: Serverbound")\]

#### Client Information (configuration)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=55&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Client Information (configuration)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=55&editintro=Template:License_wiki.vg "Edit section's source code: Client Information (configuration)")\]

Sent when the player connects, or when settings are changed.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x00`<br>_resource:_<br>`client_information` | Configuration | Server | Locale | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (16) | e.g. `en_GB`. |
| View Distance | [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | Client-side render distance, in chunks. |
| Chat Mode | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | 0: enabled, 1: commands only, 2: hidden. See [Chat#Client chat mode](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Chat#Client_chat_mode "Minecraft Wiki:Projects/wiki.vg merge/Chat") for more information. |
| Chat Colors | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | “Colors” multiplayer setting. The vanilla server stores this value but does nothing with it (see [MC-64867](https://bugs.mojang.com/browse/MC-64867 "mojira:MC-64867")). Some third-party servers disable all coloring in chat and system messages when it is false. |
| Displayed Skin Parts | [Unsigned Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Unsigned_Byte) | Bit mask, see below. |
| Main Hand | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | 0: Left, 1: Right. |
| Enable text filtering | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | Enables filtering of text on signs and written book titles. The vanilla client sets this according to the `profanityFilterPreferences.profanityFilterOn` account attribute indicated by the [`/player/attributes` Mojang API endpoint](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Mojang_API#Player_Attributes "Minecraft Wiki:Projects/wiki.vg merge/Mojang API"). In offline mode, it is always false. |
| Allow server listings | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | Servers usually list online players; this option should let you not show up in that list. |
| Particle Status | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | 0: all, 1: decreased, 2: minimal |

_Displayed Skin Parts_ flags:

- Bit 0 (0x01): Cape enabled
- Bit 1 (0x02): Jacket enabled
- Bit 2 (0x04): Left Sleeve enabled
- Bit 3 (0x08): Right Sleeve enabled
- Bit 4 (0x10): Left Pants Leg enabled
- Bit 5 (0x20): Right Pants Leg enabled
- Bit 6 (0x40): Hat enabled

The most significant bit (bit 7, 0x80) appears to be unused.

#### Cookie Response (configuration)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=56&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Cookie Response (configuration)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=56&editintro=Template:License_wiki.vg "Edit section's source code: Cookie Response (configuration)")\]

Response to a [Cookie Request (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Cookie_Request_(configuration)) from the server. The vanilla server only accepts responses of up to 5 kiB in size.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x01`<br>_resource:_<br>`cookie_response` | Configuration | Server | Key | [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) | The identifier of the cookie. |
| Payload | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) (5120) of [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | The data of the cookie. |

#### Serverbound Plugin Message (configuration)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=57&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Serverbound Plugin Message (configuration)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=57&editintro=Template:License_wiki.vg "Edit section's source code: Serverbound Plugin Message (configuration)")\]

Main page: [Minecraft Wiki:Projects/wiki.vg merge/Plugin channels](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Plugin_channels "Minecraft Wiki:Projects/wiki.vg merge/Plugin channels")

Mods and plugins can use this to send their data. Minecraft itself uses some [plugin channels](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Plugin_channels "Minecraft Wiki:Projects/wiki.vg merge/Plugin channels"). These internal channels are in the `minecraft` namespace.

More documentation on this: [https://dinnerbone.com/blog/2012/01/13/minecraft-plugin-channels-messaging/](https://dinnerbone.com/blog/2012/01/13/minecraft-plugin-channels-messaging/)

Note that the length of Data is known only from the packet length, since the packet has no length field of any kind.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x02`<br>_resource:_<br>`custom_payload` | Configuration | Server | Channel | [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) | Name of the [plugin channel](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Plugin_channels "Minecraft Wiki:Projects/wiki.vg merge/Plugin channels") used to send the data. |
| Data | [Byte Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte_Array) (32767) | Any data, depending on the channel. `minecraft:` channels are documented [here](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Plugin_channels "Minecraft Wiki:Projects/wiki.vg merge/Plugin channels"). The length of this array must be inferred from the packet length. |

In the vanilla server, the maximum data length is 32767 bytes.

#### Acknowledge Finish Configuration\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=58&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Acknowledge Finish Configuration") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=58&editintro=Template:License_wiki.vg "Edit section's source code: Acknowledge Finish Configuration")\]

Sent by the client to notify the server that the configuration process has finished. It is sent in response to the server's [Finish Configuration](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Finish_Configuration).

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x03`<br>_resource:_<br>`finish_configuration` | Configuration | Server | _no fields_ |

This packet switches the connection state to [play](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Play).

#### Serverbound Keep Alive (configuration)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=59&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Serverbound Keep Alive (configuration)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=59&editintro=Template:License_wiki.vg "Edit section's source code: Serverbound Keep Alive (configuration)")\]

The server will frequently send out a keep-alive (see [Clientbound Keep Alive](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Clientbound_Keep_Alive_(configuration))), each containing a random ID. The client must respond with the same packet.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x04`<br>_resource:_<br>`keep_alive` | Configuration | Server | Keep Alive ID | [Long](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Long) |  |

#### Pong (configuration)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=60&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Pong (configuration)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=60&editintro=Template:License_wiki.vg "Edit section's source code: Pong (configuration)")\]

Response to the clientbound packet ( [Ping](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Ping_(configuration))) with the same id.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x05`<br>_resource:_<br>`pong` | Configuration | Server | ID | [Int](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Int) |  |

#### Resource Pack Response (configuration)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=61&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Resource Pack Response (configuration)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=61&editintro=Template:License_wiki.vg "Edit section's source code: Resource Pack Response (configuration)")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x06`<br>_resource:_<br>`resource_pack` | Configuration | Server | UUID | [UUID](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:UUID) | The unique identifier of the resource pack received in the [Add Resource Pack (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Add_Resource_Pack_(configuration)) request. |
| Result | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | Result ID (see below). |

Result can be one of the following values:

| ID | Result |
| --- | --- |
| 0 | Successfully downloaded |
| 1 | Declined |
| 2 | Failed to download |
| 3 | Accepted |
| 4 | Downloaded |
| 5 | Invalid URL |
| 6 | Failed to reload |
| 7 | Discarded |

#### Serverbound Known Packs\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=62&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Serverbound Known Packs") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=62&editintro=Template:License_wiki.vg "Edit section's source code: Serverbound Known Packs")\]

Informs the server of which data packs are present on the client. The client sends this in response to [Clientbound Known Packs](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Clientbound_Known_Packs).

If the client specifies a pack in this packet, the server should omit its contained data from the [Registry Data](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Registry_Data_2) packet.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x07`<br>_resource:_<br>`select_known_packs` | Configuration | Server | Known Packs | Namespace | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) |  |
| ID | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) |  |
| Version | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) |  |

#### Custom Click Action (configuration)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=63&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Custom Click Action (configuration)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=63&editintro=Template:License_wiki.vg "Edit section's source code: Custom Click Action (configuration)")\]

Sent when the client clicks a [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) with the `minecraft:custom` click action. This is meant as an alternative to running a command, but it will not have any effect on vanilla servers.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x08`<br>_resource:_<br>`custom_click_action` | Configuration | Server | ID | [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) | The identifier for the click action. |
| Payload | [NBT](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:NBT) | The data to send with the click action. May be a TAG\_END (0). |

## Play\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=64&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Play") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=64&editintro=Template:License_wiki.vg "Edit section's source code: Play")\]

### Clientbound\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=65&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Clientbound") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=65&editintro=Template:License_wiki.vg "Edit section's source code: Clientbound")\]

#### Bundle Delimiter\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=66&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Bundle Delimiter") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=66&editintro=Template:License_wiki.vg "Edit section's source code: Bundle Delimiter")\]

The delimiter for a bundle of packets. When received, the client should store every subsequent packet it receives and wait until another delimiter is received. Once that happens, the client is guaranteed to process every packet in the bundle on the same tick, and the client should stop storing packets.

As of 1.20.6, the vanilla server only uses this to ensure [Spawn Entity](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Spawn_Entity) and associated packets used to configure the entity happen on the same tick. Each entity gets a separate bundle.

The vanilla client doesn't allow more than 4096 packets in the same bundle.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x00`<br>_resource:_<br>`bundle_delimiter` | Play | Client | _no fields_ |

#### Spawn Entity\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=67&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Spawn Entity") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=67&editintro=Template:License_wiki.vg "Edit section's source code: Spawn Entity")\]

Sent by the server to create an entity on the client, normally upon the entity spawning within or entering the player's view range.

The local player entity is automatically created by the client, and must not be created explicitly using this packet. Doing so on the vanilla client will have strange consequences.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x01`<br>_resource:_<br>`add_entity` | Play | Client | Entity ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | A unique integer ID mostly used in the protocol to identify the entity. If an entity with the same ID already exists on the client, it is automatically deleted and replaced by the new entity. On the vanilla server entity IDs are globally unique across all dimensions and never reused while the server is running, but not preserved across server restarts. |
| Entity UUID | [UUID](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:UUID) | A unique identifier that is mostly used in persistence and places where the uniqueness matters more. It is possible to create multiple entities with the same UUID on the vanilla client, but a warning will be logged, and functionality dependent on UUIDs may ignore the entity or otherwise misbehave. |
| Type | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | ID in the `minecraft:entity_type` registry (see "type" field in [Entity metadata#Entities](https://minecraft.wiki/w/Java_Edition_protocol/Entity_metadata#Entities "Java Edition protocol/Entity metadata")). |
| X | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) |  |
| Y | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) |  |
| Z | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) |  |
| Pitch | [Angle](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Angle) |  |
| Yaw | [Angle](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Angle) |  |
| Head Yaw | [Angle](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Angle) | Only used by living entities, where the head of the entity may differ from the general body rotation. |
| Data | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Meaning dependent on the value of the Type field, see [Object Data](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Object_Data "Minecraft Wiki:Projects/wiki.vg merge/Object Data") for details. |
| Velocity X | [Short](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Short) | Same units as [Set Entity Velocity](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Entity_Velocity). |
| Velocity Y | [Short](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Short) |
| Velocity Z | [Short](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Short) |

[![](https://minecraft.wiki/images/thumb/Warning_pixelated_art.png/32px-Warning_pixelated_art.png?d0083)](https://minecraft.wiki/w/File:Warning_pixelated_art.png)

The points listed below should be considered when this packet is used to spawn a player entity.

When in [online mode](https://minecraft.wiki/w/Server.properties#online-mode "Server.properties"), the UUIDs must be valid and have valid skin blobs.
In offline mode, the vanilla server uses [UUID v3](https://en.wikipedia.org/wiki/Universally_unique_identifier#Versions_3_and_5_.28namespace_name-based.29 "wikipedia:Universally unique identifier") and chooses the player's UUID by using the String `OfflinePlayer:<player name>`, encoding it in UTF-8 (and case-sensitive), then processes it with `UUID.nameUUIDFromBytes`.

For NPCs UUID v2 should be used. Note:

```
<+Grum> i will never confirm this as a feature you know that :)
```

In an example UUID, `xxxxxxxx-xxxx-Yxxx-xxxx-xxxxxxxxxxxx`, the UUID version is specified by `Y`. So, for UUID v3, `Y` will always be `3`, and for UUID v2, `Y` will always be `2`.

#### Entity Animation\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=68&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Entity Animation") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=68&editintro=Template:License_wiki.vg "Edit section's source code: Entity Animation")\]

Sent whenever an entity should change animation.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x02`<br>_resource:_<br>`animate` | Play | Client | Entity ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Player ID. |
| Animation | [Unsigned Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Unsigned_Byte) | Animation ID (see below). |

Animation can be one of the following values:

| ID | Animation |
| --- | --- |
| 0 | Swing main arm |
| 2 | Leave bed |
| 3 | Swing offhand |
| 4 | Critical effect |
| 5 | Magic critical effect |

#### Award Statistics\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=69&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Award Statistics") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=69&editintro=Template:License_wiki.vg "Edit section's source code: Award Statistics")\]

Sent as a response to [Client Status](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Client_Status) (id 1). Will only send the changed values if previously requested.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x03`<br>_resource:_<br>`award_stats` | Play | Client | Statistics | Category ID | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | ID in the `minecraft:stat_type` registry; see below. |
| Statistic ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | See below. |
| Value | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | The amount to set it to. |

Categories (defined in the `minecraft:stat_type` registry).

| Name | ID | Registry |
| --- | --- | --- |
| `minecraft:mined` | 0 | `minecraft:block` |
| `minecraft:crafted` | 1 | `minecraft:item` |
| `minecraft:used` | 2 | `minecraft:item` |
| `minecraft:broken` | 3 | `minecraft:item` |
| `minecraft:picked_up` | 4 | `minecraft:item` |
| `minecraft:dropped` | 5 | `minecraft:item` |
| `minecraft:killed` | 6 | `minecraft:entity_type` |
| `minecraft:killed_by` | 7 | `minecraft:entity_type` |
| `minecraft:custom` | 8 | `minecraft:custom_stat` |

Blocks, Items, and Entities use block (not block state), item, and entity ids.

Custom uses IDs in the `minecraft:custom_stat` registry:

| Name | ID | Unit |
| --- | --- | --- |
| `minecraft:leave_game` | 0 | None |
| `minecraft:play_time` | 1 | Time |
| `minecraft:total_world_time` | 2 | Time |
| `minecraft:time_since_death` | 3 | Time |
| `minecraft:time_since_rest` | 4 | Time |
| `minecraft:sneak_time` | 5 | Time |
| `minecraft:walk_one_cm` | 6 | Distance |
| `minecraft:crouch_one_cm` | 7 | Distance |
| `minecraft:sprint_one_cm` | 8 | Distance |
| `minecraft:walk_on_water_one_cm` | 9 | Distance |
| `minecraft:fall_one_cm` | 10 | Distance |
| `minecraft:climb_one_cm` | 11 | Distance |
| `minecraft:fly_one_cm` | 12 | Distance |
| `minecraft:walk_under_water_one_cm` | 13 | Distance |
| `minecraft:minecart_one_cm` | 14 | Distance |
| `minecraft:boat_one_cm` | 15 | Distance |
| `minecraft:pig_one_cm` | 16 | Distance |
| `minecraft:happy_ghast_one_cm` | 17 | Distance |
| `minecraft:horse_one_cm` | 18 | Distance |
| `minecraft:aviate_one_cm` | 19 | Distance |
| `minecraft:swim_one_cm` | 20 | Distance |
| `minecraft:strider_one_cm` | 21 | Distance |
| `minecraft:jump` | 22 | None |
| `minecraft:drop` | 23 | None |
| `minecraft:damage_dealt` | 24 | Damage |
| `minecraft:damage_dealt_absorbed` | 25 | Damage |
| `minecraft:damage_dealt_resisted` | 26 | Damage |
| `minecraft:damage_taken` | 27 | Damage |
| `minecraft:damage_blocked_by_shield` | 28 | Damage |
| `minecraft:damage_absorbed` | 29 | Damage |
| `minecraft:damage_resisted` | 30 | Damage |
| `minecraft:deaths` | 31 | None |
| `minecraft:mob_kills` | 32 | None |
| `minecraft:animals_bred` | 33 | None |
| `minecraft:player_kills` | 34 | None |
| `minecraft:fish_caught` | 35 | None |
| `minecraft:talked_to_villager` | 36 | None |
| `minecraft:traded_with_villager` | 37 | None |
| `minecraft:eat_cake_slice` | 38 | None |
| `minecraft:fill_cauldron` | 39 | None |
| `minecraft:use_cauldron` | 40 | None |
| `minecraft:clean_armor` | 41 | None |
| `minecraft:clean_banner` | 42 | None |
| `minecraft:clean_shulker_box` | 43 | None |
| `minecraft:interact_with_brewingstand` | 44 | None |
| `minecraft:interact_with_beacon` | 45 | None |
| `minecraft:inspect_dropper` | 46 | None |
| `minecraft:inspect_hopper` | 47 | None |
| `minecraft:inspect_dispenser` | 48 | None |
| `minecraft:play_noteblock` | 49 | None |
| `minecraft:tune_noteblock` | 50 | None |
| `minecraft:pot_flower` | 51 | None |
| `minecraft:trigger_trapped_chest` | 52 | None |
| `minecraft:open_enderchest` | 53 | None |
| `minecraft:enchant_item` | 54 | None |
| `minecraft:play_record` | 55 | None |
| `minecraft:interact_with_furnace` | 56 | None |
| `minecraft:interact_with_crafting_table` | 57 | None |
| `minecraft:open_chest` | 58 | None |
| `minecraft:sleep_in_bed` | 59 | None |
| `minecraft:open_shulker_box` | 60 | None |
| `minecraft:open_barrel` | 61 | None |
| `minecraft:interact_with_blast_furnace` | 62 | None |
| `minecraft:interact_with_smoker` | 63 | None |
| `minecraft:interact_with_lectern` | 64 | None |
| `minecraft:interact_with_campfire` | 65 | None |
| `minecraft:interact_with_cartography_table` | 66 | None |
| `minecraft:interact_with_loom` | 67 | None |
| `minecraft:interact_with_stonecutter` | 68 | None |
| `minecraft:bell_ring` | 69 | None |
| `minecraft:raid_trigger` | 70 | None |
| `minecraft:raid_win` | 71 | None |
| `minecraft:interact_with_anvil` | 72 | None |
| `minecraft:interact_with_grindstone` | 73 | None |
| `minecraft:target_hit` | 74 | None |
| `minecraft:interact_with_smithing_table` | 75 | None |

Units:

- None: just a normal number (formatted with 0 decimal places)
- Damage: value is 10 times the normal amount
- Distance: a distance in centimeters (hundredths of blocks)
- Time: a time span in ticks

#### Acknowledge Block Change\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=70&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Acknowledge Block Change") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=70&editintro=Template:License_wiki.vg "Edit section's source code: Acknowledge Block Change")\]

Acknowledges a user-initiated block change. After receiving this packet, the client will display the block state sent by the server instead of the one predicted by the client.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x04`<br>_resource:_<br>`block_changed_ack` | Play | Client | Sequence ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Represents the sequence to acknowledge; this is used for properly syncing block changes to the client after interactions. |

#### Set Block Destroy Stage\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=71&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Set Block Destroy Stage") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=71&editintro=Template:License_wiki.vg "Edit section's source code: Set Block Destroy Stage")\]

0–9 are the displayable destroy stages and each other number means that there is no animation on this coordinate.

Block break animations can still be applied on air; the animation will remain visible, although there is no block being broken. However, if this is applied to a transparent block, odd graphical effects may happen, including water losing its transparency. (An effect similar to this can be seen in normal gameplay when breaking ice blocks)

If you need to display several break animations at the same time, you have to give each of them a unique Entity ID. The entity ID does not need to correspond to an actual entity on the client. It is valid to use a randomly generated number.

When removing the break animation, you must use the ID of the entity that set it.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x05`<br>_resource:_<br>`block_destruction` | Play | Client | Entity ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | The ID of the entity breaking the block. |
| Location | [Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Position) | Block Position. |
| Destroy Stage | [Unsigned Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Unsigned_Byte) | 0–9 to set it, any other value to remove it. |

#### Block Entity Data\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=72&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Block Entity Data") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=72&editintro=Template:License_wiki.vg "Edit section's source code: Block Entity Data")\]

Sets the block entity associated with the block at the given location.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x06`<br>_resource:_<br>`block_entity_data` | Play | Client | Location | [Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Position) |  |
| Type | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | ID in the `minecraft:block_entity_type` registry |
| NBT Data | [NBT](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:NBT) | Data to set. |

#### Block Action\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=73&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Block Action") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=73&editintro=Template:License_wiki.vg "Edit section's source code: Block Action")\]

This packet is used for a number of actions and animations performed by blocks, usually non-persistent. The client ignores the provided block type and instead uses the block state in their world.

See [Block Actions](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Block_Actions "Minecraft Wiki:Projects/wiki.vg merge/Block Actions") for a list of values.

[![](https://minecraft.wiki/images/thumb/Warning_pixelated_art.png/32px-Warning_pixelated_art.png?d0083)](https://minecraft.wiki/w/File:Warning_pixelated_art.png)

This packet uses a block ID from the `minecraft:block` registry, not a block state.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x07`<br>_resource:_<br>`block_event` | Play | Client | Location | [Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Position) | Block coordinates. |
| Action ID (Byte 1) | [Unsigned Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Unsigned_Byte) | Varies depending on block — see [Block Actions](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Block_Actions "Minecraft Wiki:Projects/wiki.vg merge/Block Actions"). |
| Action Parameter (Byte 2) | [Unsigned Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Unsigned_Byte) | Varies depending on block — see [Block Actions](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Block_Actions "Minecraft Wiki:Projects/wiki.vg merge/Block Actions"). |
| Block Type | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | ID in the `minecraft:block` registry. This value is unused by the vanilla client, as it will infer the type of block based on the given position. |

#### Block Update\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=74&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Block Update") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=74&editintro=Template:License_wiki.vg "Edit section's source code: Block Update")\]

Fired whenever a block is changed within the render distance.

[![](https://minecraft.wiki/images/thumb/Warning_pixelated_art.png/32px-Warning_pixelated_art.png?d0083)](https://minecraft.wiki/w/File:Warning_pixelated_art.png)

Changing a block in a chunk that is not loaded is not a stable action. The vanilla client currently uses a _shared_ empty chunk, which is modified for all block changes in unloaded chunks; while in 1.9 this chunk never renders in older versions the changed block will appear in all copies of the empty chunk. Servers should avoid sending block changes in unloaded chunks and clients should ignore such packets.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x08`<br>_resource:_<br>`block_update` | Play | Client | Location | [Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Position) | Block Coordinates. |
| Block ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | The new block state ID for the block as given in the [block state registry](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Chunk_Format#Block_state_registry "Minecraft Wiki:Projects/wiki.vg merge/Chunk Format"). |

#### Boss Bar\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=75&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Boss Bar") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=75&editintro=Template:License_wiki.vg "Edit section's source code: Boss Bar")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x09`<br>_resource:_<br>`boss_event` | Play | Client | UUID | [UUID](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:UUID) | Unique ID for this bar. |
| Action | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | Determines the layout of the remaining packet. |
| Action | Field Name |  |  |
| 0: add | Title | [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) |  |
| Health | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | From 0 to 1. Values greater than 1 do not crash a vanilla client, and start [rendering part of a second health bar](https://i.johni0702.de/nA.png) at around 1.5. |
| Color | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | Color ID (see below). |
| Division | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | Type of division (see below). |
| Flags | [Unsigned Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Unsigned_Byte) | Bit mask. 0x01: should darken sky, 0x02: is dragon bar (used to play end music), 0x04: create fog (previously was also controlled by 0x02). |
| 1: remove | _no fields_ | _no fields_ | Removes this boss bar. |
| 2: update health | Health | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | _as above_ |
| 3: update title | Title | [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) |  |
| 4: update style | Color | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | Color ID (see below). |
| Dividers | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | _as above_ |
| 5: update flags | Flags | [Unsigned Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Unsigned_Byte) | _as above_ |

| ID | Color |
| --- | --- |
| 0 | Pink |
| 1 | Blue |
| 2 | Red |
| 3 | Green |
| 4 | Yellow |
| 5 | Purple |
| 6 | White |

| ID | Type of division |
| --- | --- |
| 0 | No division |
| 1 | 6 notches |
| 2 | 10 notches |
| 3 | 12 notches |
| 4 | 20 notches |

#### Change Difficulty\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=76&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Change Difficulty") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=76&editintro=Template:License_wiki.vg "Edit section's source code: Change Difficulty")\]

Changes the difficulty setting in the client's option menu

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x0A`<br>_resource:_<br>`change_difficulty` | Play | Client | Difficulty | [Unsigned Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Unsigned_Byte) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | 0: peaceful, 1: easy, 2: normal, 3: hard. |
| Difficulty locked? | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) |  |

#### Chunk Batch Finished\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=77&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Chunk Batch Finished") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=77&editintro=Template:License_wiki.vg "Edit section's source code: Chunk Batch Finished")\]

Marks the end of a chunk batch. The vanilla client marks the time it receives this packet and calculates the elapsed duration since the [beginning of the chunk batch](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Chunk_Batch_Start). The server uses this duration and the batch size received in this packet to estimate the number of milliseconds elapsed per chunk received. This value is then used to calculate the desired number of chunks per tick through the formula `25 / millisPerChunk`, which is reported to the server through [Chunk Batch Received](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Chunk_Batch_Received). This likely uses `25` instead of the normal tick duration of `50` so chunk processing will only use half of the client's and network's bandwidth.

The vanilla client uses the samples from the latest 15 batches to estimate the milliseconds per chunk number.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x0B`<br>_resource:_<br>`chunk_batch_finished` | Play | Client | Batch size | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Number of chunks. |

#### Chunk Batch Start\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=78&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Chunk Batch Start") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=78&editintro=Template:License_wiki.vg "Edit section's source code: Chunk Batch Start")\]

Marks the start of a chunk batch. The vanilla client marks and stores the time it receives this packet.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x0C`<br>_resource:_<br>`chunk_batch_start` | Play | Client | _no fields_ |

#### Chunk Biomes\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=79&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Chunk Biomes") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=79&editintro=Template:License_wiki.vg "Edit section's source code: Chunk Biomes")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x0D`<br>_resource:_<br>`chunks_biomes` | Play | Client | Chunk biome data | Chunk Z | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) | [Int](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Int) | Chunk coordinate (block coordinate divided by 16, rounded down) |
| Chunk X | [Int](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Int) | Chunk coordinate (block coordinate divided by 16, rounded down) |
| Data | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | Chunk [data structure](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Chunk_Format#Data_structure "Minecraft Wiki:Projects/wiki.vg merge/Chunk Format"), with [sections](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Chunk_Format#Chunk_Section "Minecraft Wiki:Projects/wiki.vg merge/Chunk Format") containing only the `Biomes` field |

Note: The order of X and Z is inverted, because the client reads them as one big-endian [Long](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Long), with Z being the upper 32 bits.

#### Clear Titles\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=80&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Clear Titles") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=80&editintro=Template:License_wiki.vg "Edit section's source code: Clear Titles")\]

Clear the client's current title information, with the option to also reset it.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x0E`<br>_resource:_<br>`clear_titles` | Play | Client | Reset | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) |  |

#### Command Suggestions Response\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=81&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Command Suggestions Response") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=81&editintro=Template:License_wiki.vg "Edit section's source code: Command Suggestions Response")\]

The server responds with a list of auto-completions of the last word sent to it. In the case of regular chat, this is a player username. Command names and parameters are also supported. The client sorts these alphabetically before listing them.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x0F`<br>_resource:_<br>`command_suggestions` | Play | Client | ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Transaction ID. |
| Start | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Start of the text to replace. |
| Length | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Length of the text to replace. |
| Matches | Match | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (32767) | One eligible value to insert, note that each command is sent separately instead of in a single string, hence the need for Count. |
| Tooltip | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) | Tooltip to display. |

#### Commands\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=82&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Commands") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=82&editintro=Template:License_wiki.vg "Edit section's source code: Commands")\]

Lists all of the commands on the server, and how they are parsed.

This is a directed graph, with one root node. Each redirect or child node must refer only to nodes that have already been declared.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x10`<br>_resource:_<br>`commands` | Play | Client | Nodes | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of [Node](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Command_Data#Node_Format "Minecraft Wiki:Projects/wiki.vg merge/Command Data") | An array of nodes. |
| Root index | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Index of the `root` node in the previous array. |

For more information on this packet, see the [Command Data](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Command_Data "Minecraft Wiki:Projects/wiki.vg merge/Command Data") article.

#### Close Container\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=83&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Close Container") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=83&editintro=Template:License_wiki.vg "Edit section's source code: Close Container")\]

This packet is sent from the server to the client when a window is forcibly closed, such as when a chest is destroyed while it's open. The vanilla client disregards the provided window ID and closes any active window.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x11`<br>_resource:_<br>`container_close` | Play | Client | Window ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | This is the ID of the window that was closed. 0 for inventory. |

#### Set Container Content\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=84&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Set Container Content") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=84&editintro=Template:License_wiki.vg "Edit section's source code: Set Container Content")\]

[![](https://minecraft.wiki/images/thumb/Inventory-slots.png/300px-Inventory-slots.png?e5b77)](https://minecraft.wiki/w/File:Inventory-slots.png) The inventory slots

Replaces the contents of a container window. Sent by the server upon initialization of a container window or the player's inventory, and in response to state ID mismatches (see [#Click Container](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Click_Container)).

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x12`<br>_resource:_<br>`container_set_content` | Play | Client | Window ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | The ID of window which items are being sent for. 0 for player inventory. The client ignores any packets targeting a Window ID other than the current one. However, an exception is made for the player inventory, which may be targeted at any time. (The vanilla server does not appear to utilize this special case.) |
| State ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | A server-managed sequence number used to avoid desynchronization; see [#Click Container](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Click_Container). |
| Slot Data | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of [Slot](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Slot) |
| Carried Item | [Slot](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Slot) | Item being dragged with the mouse. |

See [inventory windows](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Inventory#Windows "Minecraft Wiki:Projects/wiki.vg merge/Inventory") for further information about how slots are indexed.
Use [Open Screen](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Open_Screen) to open the container on the client.

#### Set Container Property\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=85&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Set Container Property") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=85&editintro=Template:License_wiki.vg "Edit section's source code: Set Container Property")\]

This packet is used to inform the client that part of a GUI window should be updated.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x13`<br>_resource:_<br>`container_set_data` | Play | Client | Window ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |
| Property | [Short](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Short) | The property to be updated, see below. |
| Value | [Short](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Short) | The new value for the property, see below. |

The meaning of the Property field depends on the type of the window. The following table shows the known combinations of window type and property, and how the value is to be interpreted.

| Window type | Property | Value |
| --- | --- | --- |
| Furnace | 0: Fire icon (fuel left) | counting from fuel burn time down to 0 (in-game ticks) |
| 1: Maximum fuel burn time | fuel burn time or 0 (in-game ticks) |
| 2: Progress arrow | counting from 0 to maximum progress (in-game ticks) |
| 3: Maximum progress | always 200 on the vanilla server |
| Enchantment Table | 0: Level requirement for top enchantment slot | The enchantment's xp level requirement |
| 1: Level requirement for middle enchantment slot |
| 2: Level requirement for bottom enchantment slot |
| 3: The enchantment seed | Used for drawing the enchantment names (in [SGA](https://en.wikipedia.org/wiki/Standard_Galactic_Alphabet "wikipedia:Standard Galactic Alphabet")) clientside. The same seed _is_ used to calculate enchantments, but some of the data isn't sent to the client to prevent easily guessing the entire list (the seed value here is the regular seed bitwise and `0xFFFFFFF0`). |
| 4: Enchantment ID shown on mouse hover over top enchantment slot | The enchantment ID (set to -1 to hide it), see below for values |
| 5: Enchantment ID shown on mouse hover over middle enchantment slot |
| 6: Enchantment ID shown on mouse hover over bottom enchantment slot |
| 7: Enchantment level shown on mouse hover over the top slot | The enchantment level (1 = I, 2 = II, 6 = VI, etc.), or -1 if no enchant |
| 8: Enchantment level shown on mouse hover over the middle slot |
| 9: Enchantment level shown on mouse hover over the bottom slot |
| Beacon | 0: Power level | 0-4, controls what effect buttons are enabled |
| 1: First potion effect | [Potion effect ID](https://minecraft.wiki/w/Data_values#Status_effects "Data values") for the first effect, or -1 if no effect |
| 2: Second potion effect | [Potion effect ID](https://minecraft.wiki/w/Data_values#Status_effects "Data values") for the second effect, or -1 if no effect |
| Anvil | 0: Repair cost | The repair's cost in XP levels |
| Brewing Stand | 0: Brew time | 0 – 400, with 400 making the arrow empty, and 0 making the arrow full |
| 1: Fuel time | 0 - 20, with 0 making the arrow empty, and 20 making the arrow full |
| Stonecutter | 0: Selected recipe | The index of the selected recipe. -1 means none is selected. |
| Loom | 0: Selected pattern | The index of the selected pattern. 0 means none is selected, 0 is also the internal ID of the "base" pattern. |
| Lectern | 0: Page number | The current page number, starting from 0. |
| Smithing Table | 0: Has recipe error | True if greater than zero. |

For an enchanting table, the following numerical IDs are used:

| Numerical ID | Enchantment ID | Enchantment Name |
| --- | --- | --- |
| 0 | minecraft:protection | Protection |
| 1 | minecraft:fire\_protection | Fire Protection |
| 2 | minecraft:feather\_falling | Feather Falling |
| 3 | minecraft:blast\_protection | Blast Protection |
| 4 | minecraft:projectile\_protection | Projectile Protection |
| 5 | minecraft:respiration | Respiration |
| 6 | minecraft:aqua\_affinity | Aqua Affinity |
| 7 | minecraft:thorns | Thorns |
| 8 | minecraft:depth\_strider | Depth Strider |
| 9 | minecraft:frost\_walker | Frost Walker |
| 10 | minecraft:binding\_curse | Curse of Binding |
| 11 | minecraft:soul\_speed | Soul Speed |
| 12 | minecraft:swift\_sneak | Swift Sneak |
| 13 | minecraft:sharpness | Sharpness |
| 14 | minecraft:smite | Smite |
| 15 | minecraft:bane\_of\_arthropods | Bane of Arthropods |
| 16 | minecraft:knockback | Knockback |
| 17 | minecraft:fire\_aspect | Fire Aspect |
| 18 | minecraft:looting | Looting |
| 19 | minecraft:sweeping\_edge | Sweeping Edge |
| 20 | minecraft:efficiency | Efficiency |
| 21 | minecraft:silk\_touch | Silk Touch |
| 22 | minecraft:unbreaking | Unbreaking |
| 23 | minecraft:fortune | Fortune |
| 24 | minecraft:power | Power |
| 25 | minecraft:punch | Punch |
| 26 | minecraft:flame | Flame |
| 27 | minecraft:infinity | Infinity |
| 28 | minecraft:luck\_of\_the\_sea | Luck of the Sea |
| 29 | minecraft:lure | Lure |
| 30 | minecraft:loyalty | Loyalty |
| 31 | minecraft:impaling | Impaling |
| 32 | minecraft:riptide | Riptide |
| 33 | minecraft:channeling | Channeling |
| 34 | minecraft:multishot | Multishot |
| 35 | minecraft:quick\_charge | Quick Charge |
| 36 | minecraft:piercing | Piercing |
| 37 | minecraft:density | Density |
| 38 | minecraft:breach | Breach |
| 39 | minecraft:wind\_burst | Wind Burst |
| 40 | minecraft:mending | Mending |
| 41 | minecraft:vanishing\_curse | Curse of Vanishing |

#### Set Container Slot\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=86&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Set Container Slot") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=86&editintro=Template:License_wiki.vg "Edit section's source code: Set Container Slot")\]

Sent by the server when an item in a slot (in a window) is added/removed.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x14`<br>_resource:_<br>`container_set_slot` | Play | Client | Window ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | The window that is being updated. 0 for player inventory. The client ignores any packets targeting a Window ID other than the current one; see below for exceptions. |
| State ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | A server-managed sequence number used to avoid desynchronization; see [#Click Container](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Click_Container). |
| Slot | [Short](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Short) | The slot that should be updated. |
| Slot Data | [Slot](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Slot) |  |

If Window ID is 0, the hotbar and offhand slots (slots 36 through 45) may be updated even when a different container window is open. (The vanilla server does not appear to utilize this special case.) Updates are also restricted to those slots when the player is looking at a creative inventory tab other than the survival inventory. (The vanilla server does _not_ handle this restriction in any way, leading to [MC-242392](https://bugs.mojang.com/browse/MC-242392).)

When a container window is open, the server never sends updates targeting Window ID 0—all of the [window types](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Inventory "Minecraft Wiki:Projects/wiki.vg merge/Inventory") include slots for the player inventory. The client must automatically apply changes targeting the inventory portion of a container window to the main inventory; the server does not resend them for ID 0 when the window is closed. However, since the armor and offhand slots are only present on ID 0, updates to those slots occurring while a window is open must be deferred by the server until the window's closure.

#### Cookie Request (play)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=87&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Cookie Request (play)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=87&editintro=Template:License_wiki.vg "Edit section's source code: Cookie Request (play)")\]

Requests a cookie that was previously stored.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x15`<br>_resource:_<br>`cookie_request` | Play | Client | Key | [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) | The identifier of the cookie. |

#### Set Cooldown\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=88&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Set Cooldown") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=88&editintro=Template:License_wiki.vg "Edit section's source code: Set Cooldown")\]

Applies a cooldown period to all items with the given type. Used by the vanilla server with enderpearls. This packet should be sent when the cooldown starts and also when the cooldown ends (to compensate for lag), although the client will end the cooldown automatically. Can be applied to any item, note that interactions still get sent to the server with the item, but the client does not play the animation nor attempt to predict results (i.e, block placing).

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x16`<br>_resource:_<br>`cooldown` | Play | Client | Cooldown Group | [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) | Identifier of the item (minecraft:stone) or the cooldown group ("use\_cooldown" item component) |
| Cooldown Ticks | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Number of ticks to apply a cooldown for, or 0 to clear the cooldown. |

#### Chat Suggestions\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=89&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Chat Suggestions") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=89&editintro=Template:License_wiki.vg "Edit section's source code: Chat Suggestions")\]

Unused by the vanilla server. Likely provided for custom servers to send chat message completions to clients.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x17`<br>_resource:_<br>`custom_chat_completions` | Play | Client | Action | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | 0: Add, 1: Remove, 2: Set |
| Entries | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (32767) |  |

#### Clientbound Plugin Message (play)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=90&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Clientbound Plugin Message (play)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=90&editintro=Template:License_wiki.vg "Edit section's source code: Clientbound Plugin Message (play)")\]

Main page: [Minecraft Wiki:Projects/wiki.vg merge/Plugin channels](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Plugin_channels "Minecraft Wiki:Projects/wiki.vg merge/Plugin channels")

Mods and plugins can use this to send their data. Minecraft itself uses several [plugin channels](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Plugin_channels "Minecraft Wiki:Projects/wiki.vg merge/Plugin channels"). These internal channels are in the `minecraft` namespace.

More information on how it works on [Dinnerbone's blog](https://dinnerbone.com/blog/2012/01/13/minecraft-plugin-channels-messaging/). More documentation about internal and popular registered channels is [here](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Plugin_channels "Minecraft Wiki:Projects/wiki.vg merge/Plugin channels").

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x18`<br>_resource:_<br>`custom_payload` | Play | Client | Channel | [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) | Name of the [plugin channel](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Plugin_channels "Minecraft Wiki:Projects/wiki.vg merge/Plugin channels") used to send the data. |
| Data | [Byte Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte_Array) (1048576) | Any data. The length of this array must be inferred from the packet length. |

In vanilla clients, the maximum data length is 1048576 bytes.

#### Damage Event\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=91&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Damage Event") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=91&editintro=Template:License_wiki.vg "Edit section's source code: Damage Event")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x19`<br>_resource:_<br>`damage_event` | Play | Client |
| Entity ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | The ID of the entity taking damage |
| Source Type ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | The type of damage in the `minecraft:damage_type` registry, defined by the [Registry Data](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Registry_Data_2) packet. |
| Source Cause ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | The ID + 1 of the entity responsible for the damage, if present. If not present, the value is 0 |
| Source Direct ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | The ID + 1 of the entity that directly dealt the damage, if present. If not present, the value is 0. If this field is present:<br>- and damage was dealt indirectly, such as by the use of a projectile, this field will contain the ID of such projectile;<br>- and damage was dealt directly, such as by manually attacking, this field will contain the same value as Source Cause ID. |
| Source Position | X | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) | The vanilla server sends the Source Position when the damage was dealt by the /damage command and a position was specified |
| Y | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) |
| Z | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) |

#### Debug Sample\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=92&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Debug Sample") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=92&editintro=Template:License_wiki.vg "Edit section's source code: Debug Sample")\]

Sample data that is sent periodically after the client has subscribed with [Debug Sample Subscription](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Debug_Sample_Subscription).

The vanilla server only sends debug samples to players who are server operators.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x1A`<br>_resource:_<br>`debug_sample` | Play | Client | Sample | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of [Long](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Long) | Array of type-dependent samples. |
| Sample Type | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | See below. |

Types:

| Id | Name | Description |
| --- | --- | --- |
| 0 | Tick time | Four different tick-related metrics, each one represented by one long on the array.<br>They are measured in nano-seconds, and are as follows:<br>- 0: Full tick time: Aggregate of the three times below;<br>- 1: Server tick time: Main server tick logic;<br>- 2: Tasks time: Tasks scheduled to execute after the main logic;<br>- 3: Idle time: Time idling to complete the full 50ms tick cycle.<br>Note that the vanilla client calculates the timings used for min/max/average display by subtracting the idle time from the full tick time. This can cause the displayed values to go negative if the idle time is (nonsensically) greater than the full tick time. |

#### Delete Message\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=93&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Delete Message") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=93&editintro=Template:License_wiki.vg "Edit section's source code: Delete Message")\]

Removes a message from the client's chat. This only works for messages with signatures; system messages cannot be deleted with this packet.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x1B`<br>_resource:_<br>`delete_chat` | Play | Client | Message ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | The message ID + 1, used for validating message signature. The next field is present only when value of this field is equal to 0. |
| Signature | [Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Optional) [Byte Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte_Array) (256) | The previous message's signature. Always 256 bytes and not length-prefixed. |

#### Disconnect (play)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=94&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Disconnect (play)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=94&editintro=Template:License_wiki.vg "Edit section's source code: Disconnect (play)")\]

Sent by the server before it disconnects a client. The client assumes that the server has already closed the connection by the time the packet arrives.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x1C`<br>_resource:_<br>`disconnect` | Play | Client | Reason | [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) | Displayed to the client when the connection terminates. |

#### Disguised Chat Message\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=95&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Disguised Chat Message") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=95&editintro=Template:License_wiki.vg "Edit section's source code: Disguised Chat Message")\]

Main page: [Minecraft\_Wiki:Projects/wiki.vg\_merge/Chat](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Chat "Minecraft Wiki:Projects/wiki.vg merge/Chat")

Sends the client a chat message, but without any message signing information.

The vanilla server uses this packet when the console is communicating with players through commands, such as `/say`, `/tell`, `/me`, among others.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x1D`<br>_resource:_<br>`disguised_chat` | Play | Client | Message | [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) | This is used as the `content` parameter when formatting the message on the client. |
| Chat Type | [ID or](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:ID_or) [Chat Type](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Chat_Type) | Either the type of chat in the `minecraft:chat_type` registry, defined by the [Registry Data](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Registry_Data_2) packet, or an inline definition. |
| Sender Name | [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) | The name of the one sending the message, usually the sender's display name.<br>This is used as the `sender` parameter when formatting the message on the client. |
| Target Name | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) | The name of the one receiving the message, usually the receiver's display name.<br>This is used as the `target` parameter when formatting the message on the client. |

#### Entity Event\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=96&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Entity Event") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=96&editintro=Template:License_wiki.vg "Edit section's source code: Entity Event")\]

Entity statuses generally trigger an animation for an entity. The available statuses vary by the entity's type (and are available to subclasses of that type as well).

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x1E`<br>_resource:_<br>`entity_event` | Play | Client | Entity ID | [Int](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Int) |  |
| Entity Status | [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | See [Entity statuses](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Entity_statuses "Minecraft Wiki:Projects/wiki.vg merge/Entity statuses") for a list of which statuses are valid for each type of entity. |

#### Teleport Entity\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=97&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Teleport Entity") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=97&editintro=Template:License_wiki.vg "Edit section's source code: Teleport Entity")\]

[![](https://minecraft.wiki/images/thumb/Warning_pixelated_art.png/32px-Warning_pixelated_art.png?d0083)](https://minecraft.wiki/w/File:Warning_pixelated_art.png)

The Mojang-specified name of this packet was changed in 1.21.2 from `teleport_entity` to `entity_position_sync`. There is a new `teleport_entity`, which this document more appropriately calls [Synchronize Vehicle Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Synchronize_Vehicle_Position). That packet has a different function and will lead to confusing results if used in place of this one.

This packet is sent by the server when an entity moves more than 8 blocks.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x1F`<br>_resource:_<br>`entity_position_sync` | Play | Client | Entity ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |
| X | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) |  |
| Y | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) |  |
| Z | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) |  |
| Velocity X | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) |  |
| Velocity Y | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) |  |
| Velocity Z | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) |  |
| Yaw | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | Rotation on the X axis, in degrees. |
| Pitch | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | Rotation on the Y axis, in degrees. |
| On Ground | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) |  |

#### Explosion\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=98&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Explosion") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=98&editintro=Template:License_wiki.vg "Edit section's source code: Explosion")\]

Sent when an explosion occurs (creepers, TNT, and ghast fireballs).

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x20`<br>_resource:_<br>`explode` | Play | Client | X | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) |  |
| Y | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) |  |
| Z | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) |  |
| Player Delta Velocity | X | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) | Velocity difference of the player being pushed by the explosion. |
| Y | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) |
| Z | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) |
| Explosion Particle ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | ID in the `minecraft:particle_type` registry. |
| Explosion Particle Data | Varies | Particle data as specified in [Particles](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Particles "Minecraft Wiki:Projects/wiki.vg merge/Particles"). |
| Explosion Sound | [ID or](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:ID_or) [Sound Event](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Sound_Event) | ID in the `minecraft:sound_event` registry, or an inline definition. |

#### Unload Chunk\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=99&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Unload Chunk") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=99&editintro=Template:License_wiki.vg "Edit section's source code: Unload Chunk")\]

Tells the client to unload a chunk column.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x21`<br>_resource:_<br>`forget_level_chunk` | Play | Client | Chunk Z | [Int](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Int) | Block coordinate divided by 16, rounded down. |
| Chunk X | [Int](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Int) | Block coordinate divided by 16, rounded down. |

Note: The order is inverted, because the client reads this packet as one big-endian [Long](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Long), with Z being the upper 32 bits.

It is legal to send this packet even if the given chunk is not currently loaded.

#### Game Event\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=100&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Game Event") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=100&editintro=Template:License_wiki.vg "Edit section's source code: Game Event")\]

Used for a wide variety of game events, such as weather, respawn availability (from [bed](https://minecraft.wiki/w/Bed "Bed") and [respawn anchor](https://minecraft.wiki/w/Respawn_anchor "Respawn anchor")), game mode, some game rules, and [demo](https://minecraft.wiki/w/Demo "Demo") messages.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x22`<br>_resource:_<br>`game_event` | Play | Client | Event | [Unsigned Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Unsigned_Byte) | See below. |
| Value | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | Depends on Event. |

_Events_:

| Event | Effect | Value |
| --- | --- | --- |
| 0 | No respawn block available | Note: Displays message 'block.minecraft.spawn.not\_valid' (You have no home bed or charged respawn anchor, or it was obstructed) to the player. |
| 1 | Begin raining |  |
| 2 | End raining |  |
| 3 | Change game mode | 0: Survival, 1: Creative, 2: Adventure, 3: Spectator. |
| 4 | Win game | 0: Just respawn player.<br>1: Roll the credits and respawn player.<br>Note that 1 is only sent by vanilla server when player has not yet achieved advancement "The end?", else 0 is sent. |
| 5 | Demo event | 0: Show welcome to demo screen.<br>101: Tell movement controls.<br>102: Tell jump control.<br>103: Tell inventory control.<br>104: Tell that the demo is over and print a message about how to take a screenshot. |
| 6 | Arrow hit player | Note: Sent when any player is struck by an arrow. |
| 7 | Rain level change | Note: Seems to change both sky color and lighting.<br>Rain level ranging from 0 to 1. |
| 8 | Thunder level change | Note: Seems to change both sky color and lighting (same as Rain level change, but doesn't start rain). It also requires rain to render by vanilla client.<br>Thunder level ranging from 0 to 1. |
| 9 | Play pufferfish sting sound |
| 10 | Play elder guardian mob appearance (effect and sound) |  |
| 11 | Enable respawn screen | 0: Enable respawn screen.<br>1: Immediately respawn (sent when the `doImmediateRespawn` gamerule changes). |
| 12 | Limited crafting | 0: Disable limited crafting.<br>1: Enable limited crafting (sent when the `doLimitedCrafting` gamerule changes). |
| 13 | Start waiting for level chunks | Instructs the client to begin the waiting process for the level chunks.<br>Sent by the server after the level is cleared on the client and is being re-sent (either during the first, or subsequent reconfigurations). |

#### Open Horse Screen\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=101&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Open Horse Screen") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=101&editintro=Template:License_wiki.vg "Edit section's source code: Open Horse Screen")\]

This packet is used exclusively for opening the horse GUI. [Open Screen](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Open_Screen) is used for all other GUIs. The client will not open the inventory if the Entity ID does not point to a horse-like animal.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x23`<br>_resource:_<br>`horse_screen_open` | Play | Client | Window ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Same as the field of [Open Screen](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Open_Screen). |
| Inventory columns count | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | How many columns of horse inventory slots exist in the GUI, 3 slots per column. |
| Entity ID | [Int](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Int) | The "owner" entity of the GUI. The client should close the GUI if the owner entity dies or is cleared. |

#### Hurt Animation\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=102&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Hurt Animation") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=102&editintro=Template:License_wiki.vg "Edit section's source code: Hurt Animation")\]

Plays a bobbing animation for the entity receiving damage.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x24`<br>_resource:_<br>`hurt_animation` | Play | Client | Entity ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | The ID of the entity taking damage |
| Yaw | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | The direction the damage is coming from in relation to the entity |

#### Initialize World Border\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=103&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Initialize World Border") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=103&editintro=Template:License_wiki.vg "Edit section's source code: Initialize World Border")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x25`<br>_resource:_<br>`initialize_border` | Play | Client | X | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) |  |
| Z | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) |  |
| Old Diameter | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) | Current length of a single side of the world border, in meters. |
| New Diameter | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) | Target length of a single side of the world border, in meters. |
| Speed | [VarLong](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarLong) | Number of real-time _milli_ seconds until New Diameter is reached. It appears that vanilla server does not sync world border speed to game ticks, so it gets out of sync with server lag. If the world border is not moving, this is set to 0. |
| Portal Teleport Boundary | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Resulting coordinates from a portal teleport are limited to ±value. Usually 29999984. |
| Warning Blocks | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | In meters. |
| Warning Time | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | In seconds as set by `/worldborder warning time`. |

The vanilla client determines how solid to display the warning by comparing to whichever is higher, the warning distance or whichever is lower, the distance from the current diameter to the target diameter or the place the border will be after warningTime seconds. In pseudocode:

```
distance = max(min(resizeSpeed * 1000 * warningTime, abs(targetDiameter - currentDiameter)), warningDistance);
if (playerDistance < distance) {
    warning = 1.0 - playerDistance / distance;
} else {
    warning = 0.0;
}
```

#### Clientbound Keep Alive (play)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=104&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Clientbound Keep Alive (play)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=104&editintro=Template:License_wiki.vg "Edit section's source code: Clientbound Keep Alive (play)")\]

The server will frequently send out a keep-alive, each containing a random ID. The client must respond with the same payload (see [Serverbound Keep Alive](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Serverbound_Keep_Alive_(play))). If the client does not respond to a Keep Alive packet within 15 seconds after it was sent, the server kicks the client. Vice versa, if the server does not send any keep-alives for 20 seconds, the client will disconnect and yield a "Timed out" exception.

The vanilla server uses a system-dependent time in milliseconds to generate the keep alive ID value.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x26`<br>_resource:_<br>`keep_alive` | Play | Client | Keep Alive ID | [Long](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Long) |  |

#### Chunk Data and Update Light\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=105&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Chunk Data and Update Light") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=105&editintro=Template:License_wiki.vg "Edit section's source code: Chunk Data and Update Light")\]

Main page: [Minecraft Wiki:Projects/wiki.vg merge/Chunk Format](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Chunk_Format "Minecraft Wiki:Projects/wiki.vg merge/Chunk Format")

See also: [§ Unload Chunk](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Unload_Chunk)

Sent when a chunk comes into the client's view distance, specifying its terrain, lighting and block entities.

The chunk must be within the view area previously specified with [Set Center Chunk](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Center_Chunk); see that packet for details.

It is not strictly necessary to send all block entities in this packet; it is still legal to send them with [Block Entity Data](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Block_Entity_Data) later.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x27`<br>_resource:_<br>`level_chunk_with_light` | Play | Client | Chunk X | [Int](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Int) | Chunk coordinate (block coordinate divided by 16, rounded down) |
| Chunk Z | [Int](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Int) | Chunk coordinate (block coordinate divided by 16, rounded down) |
| Data | [Chunk Data](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Chunk_Data) |  |
| Light | [Light Data](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Light_Data) |  |

Unlike the [Update Light](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Update_Light) packet, which uses the same format, setting the bit corresponding to a section to 0 in both of the block light or sky light masks does not appear to be useful, and the results in testing have been highly inconsistent.

#### World Event\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=106&veaction=edit&editintro=Template:License_wiki.vg "Edit section: World Event") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=106&editintro=Template:License_wiki.vg "Edit section's source code: World Event")\]

Sent when a client is to play a sound or particle effect.

By default, the Minecraft client adjusts the volume of sound effects based on distance. The final boolean field is used to disable this, and instead, the effect is played from 2 blocks away in the correct direction. Currently, this is only used for effect 1023 (wither spawn), effect 1028 (enderdragon death), and effect 1038 (end portal opening); it is ignored on other effects.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x28`<br>_resource:_<br>`level_event` | Play | Client | Event | [Int](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Int) | The event, see below. |
| Location | [Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Position) | The location of the event. |
| Data | [Int](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Int) | Extra data for certain events, see below. |
| Disable Relative Volume | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | See above. |

Events:

| ID | Name | Data |
| --- | --- | --- |
| Sound |
| 1000 | Dispenser dispenses |  |
| 1001 | Dispenser fails to dispense |  |
| 1002 | Dispenser shoots |  |
| 1004 | Firework shot |  |
| 1009 | Fire extinguished |  |
| 1010 | Play record | An ID in the `minecraft:item` registry, corresponding to a [record item](https://minecraft.wiki/w/Music_Disc "Music Disc"). If the ID doesn't correspond to a record, the packet is ignored. Any record already being played at the given location is overwritten. See [Data Generators](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Data_Generators "Minecraft Wiki:Projects/wiki.vg merge/Data Generators") for information on item IDs. |
| 1011 | Stop record |  |
| 1015 | Ghast warns |  |
| 1016 | Ghast shoots |  |
| 1017 | Ender dragon shoots |  |
| 1018 | Blaze shoots |  |
| 1019 | Zombie attacks wooden door |  |
| 1020 | Zombie attacks iron door |  |
| 1021 | Zombie breaks wooden door |  |
| 1022 | Wither breaks block |  |
| 1023 | Wither spawned |  |
| 1024 | Wither shoots |  |
| 1025 | Bat takes off |  |
| 1026 | Zombie infects |  |
| 1027 | Zombie villager converted |  |
| 1028 | Ender dragon dies |  |
| 1029 | Anvil destroyed |  |
| 1030 | Anvil used |  |
| 1031 | Anvil lands |  |
| 1032 | Portal travel |  |
| 1033 | Chorus flower grows |  |
| 1034 | Chorus flower dies |  |
| 1035 | Brewing stand brews |  |
| 1038 | End portal created |  |
| 1039 | Phantom bites |  |
| 1040 | Zombie converts to drowned |  |
| 1041 | Husk converts to zombie by drowning |  |
| 1042 | Grindstone used |  |
| 1043 | Book page turned |  |
| 1044 | Smithing table used |  |
| 1045 | Pointed dripstone landing |  |
| 1046 | Lava dripping on cauldron from dripstone |  |
| 1047 | Water dripping on cauldron from dripstone |  |
| 1048 | Skeleton converts to stray |  |
| 1049 | Crafter successfully crafts item |  |
| 1050 | Crafter fails to craft item |  |
| Particle |
| 1500 | Composter composts |  |
| 1501 | Lava converts block (either water to stone, or removes existing blocks such as torches) |  |
| 1502 | Redstone torch burns out |  |
| 1503 | Ender eye placed in end portal frame |  |
| 1504 | Fluid drips from dripstone |  |
| 1505 | Bone meal particles and sound | How many particles to spawn. |
| 2000 | Dispenser activation smoke | Direction, see below. |
| 2001 | Block break + block break sound | Block state ID (see [Chunk Format#Block state registry](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Chunk_Format#Block_state_registry "Minecraft Wiki:Projects/wiki.vg merge/Chunk Format")). |
| 2002 | Splash potion. Particle effect + glass break sound. | RGB color as an integer (e.g. 8364543 for #7FA1FF). |
| 2003 | Eye of ender entity break animation — particles and sound |  |
| 2004 | Spawner spawns mob: smoke + flames |  |
| 2006 | Dragon breath |  |
| 2007 | Instant splash potion. Particle effect + glass break sound. | RGB color as an integer (e.g. 8364543 for #7FA1FF). |
| 2008 | Ender dragon destroys block |  |
| 2009 | Wet sponge vaporizes |  |
| 2010 | Crafter activation smoke | Direction, see below. |
| 2011 | Bee fertilizes plant | How many particles to spawn. |
| 2012 | Turtle egg placed | How many particles to spawn. |
| 2013 | Smash attack (mace) | How many particles to spawn. |
| 3000 | End gateway spawns |  |
| 3001 | Ender dragon resurrected |  |
| 3002 | Electric spark |  |
| 3003 | Copper apply wax |  |
| 3004 | Copper remove wax |  |
| 3005 | Copper scrape oxidation |  |
| 3006 | Sculk charge |  |
| 3007 | Sculk shrieker shriek |  |
| 3008 | Block finished brushing | Block state ID (see [Chunk Format#Block state registry](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Chunk_Format#Block_state_registry "Minecraft Wiki:Projects/wiki.vg merge/Chunk Format")) |
| 3009 | Sniffer egg cracks | If 1, 3-6, if any other number, 1-3 particles will be spawned. |
| 3011 | Trial spawner spawns mob (at spawner) |  |
| 3012 | Trial spawner spawns mob (at spawn location) |  |
| 3013 | Trial spawner detects player | Number of players nearby |
| 3014 | Trial spawner ejects item |  |
| 3015 | Vault activates |  |
| 3016 | Vault deactivates |  |
| 3017 | Vault ejects item |  |
| 3018 | Cobweb weaved |  |
| 3019 | Ominous trial spawner detects player | Number of players nearby |
| 3020 | Trial spawner turns ominous | If 0, the sound will be played at 0.3 volume. Otherwise, it is played at full volume. |
| 3021 | Ominous item spawner spawns item |  |

Smoke directions:

| ID | Direction |
| --- | --- |
| 0 | Down |
| 1 | Up |
| 2 | North |
| 3 | South |
| 4 | West |
| 5 | East |

#### Particle\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=107&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Particle") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=107&editintro=Template:License_wiki.vg "Edit section's source code: Particle")\]

Displays the named particle

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x29`<br>_resource:_<br>`level_particles` | Play | Client | Long Distance | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | If true, particle distance increases from 256 to 65536. |
| Always Visible | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | Whether this particle should always be visible. |
| X | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) | X position of the particle. |
| Y | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) | Y position of the particle. |
| Z | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) | Z position of the particle. |
| Offset X | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | This is added to the X position after being multiplied by `random.nextGaussian()`. |
| Offset Y | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | This is added to the Y position after being multiplied by `random.nextGaussian()`. |
| Offset Z | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | This is added to the Z position after being multiplied by `random.nextGaussian()`. |
| Max Speed | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) |  |
| Particle Count | [Int](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Int) | The number of particles to create. |
| Particle ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | ID in the `minecraft:particle_type` registry. |
| Data | Varies | Particle data as specified in [Particles](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Particles "Minecraft Wiki:Projects/wiki.vg merge/Particles"). |

#### Update Light\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=108&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Update Light") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=108&editintro=Template:License_wiki.vg "Edit section's source code: Update Light")\]

Updates light levels for a chunk. See [Light](https://minecraft.wiki/w/Light "Light") for information on how lighting works in Minecraft.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x2A`<br>_resource:_<br>`light_update` | Play | Client | Chunk X | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Chunk coordinate (block coordinate divided by 16, rounded down) |
| Chunk Z | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Chunk coordinate (block coordinate divided by 16, rounded down) |
| Data | [Light Data](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Light_Data) |  |

A bit will never be set in both the block light mask and the empty block light mask, though it may be present in neither of them (if the block light does not need to be updated for the corresponding chunk section). The same applies to the sky light mask and the empty sky light mask.

#### Login (play)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=109&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Login (play)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=109&editintro=Template:License_wiki.vg "Edit section's source code: Login (play)")\]

See [protocol encryption](https://minecraft.wiki/w/Protocol_encryption "Protocol encryption") for information on logging in.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x2B`<br>_resource:_<br>`login` | Play | Client | Entity ID | [Int](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Int) | The player's Entity ID (EID). |
| Is hardcore | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) |  |
| Dimension Names | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) | Identifiers for all dimensions on the server. |
| Max Players | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Was once used by the client to draw the player list, but now it is ignored. |
| View Distance | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Render distance (2-32). |
| Simulation Distance | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | The distance that the client will process specific things, such as entities. |
| Reduced Debug Info | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | If true, a vanilla client shows reduced information on the [debug screen](https://minecraft.wiki/w/Debug_screen "Debug screen"). For servers in development, this should almost always be false. |
| Enable respawn screen | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | Set to false when the doImmediateRespawn gamerule is true. |
| Do limited crafting | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | Whether players can only craft recipes they have already unlocked. Currently unused by the client. |
| Dimension Type | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | The ID of the type of dimension in the `minecraft:dimension_type` registry, defined by the Registry Data packet. |
| Dimension Name | [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) | Name of the dimension being spawned into. |
| Hashed seed | [Long](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Long) | First 8 bytes of the SHA-256 hash of the world's seed. Used client-side for biome noise |
| Game mode | [Unsigned Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Unsigned_Byte) | 0: Survival, 1: Creative, 2: Adventure, 3: Spectator. |
| Previous Game mode | [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | -1: Undefined (null), 0: Survival, 1: Creative, 2: Adventure, 3: Spectator. The previous game mode. Vanilla client uses this for the debug (F3 + N & F3 + F4) game mode switch. (More information needed) |
| Is Debug | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | True if the world is a [debug mode](https://minecraft.wiki/w/Debug_mode "Debug mode") world; debug mode worlds cannot be modified and have predefined blocks. |
| Is Flat | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | True if the world is a [superflat](https://minecraft.wiki/w/Superflat "Superflat") world; flat worlds have different void fog and a horizon at y=0 instead of y=63. |
| Has death location | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | If true, then the next two fields are present. |
| Death dimension name | [Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Optional) [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) | Name of the dimension the player died in. |
| Death location | [Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Optional) [Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Position) | The location that the player died at. |
| Portal cooldown | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | The number of ticks until the player can use the last used portal again. Looks like it's an attempt to fix MC-180. |
| Sea level | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |
| Enforces Secure Chat | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) |  |

#### Map Data\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=110&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Map Data") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=110&editintro=Template:License_wiki.vg "Edit section's source code: Map Data")\]

Updates a rectangular area on a [map](https://minecraft.wiki/w/Map "Map") item.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x2C`<br>_resource:_<br>`map_item_data` | Play | Client | Map ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Map ID of the map being modified |
| Scale | [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | From 0 for a fully zoomed-in map (1 block per pixel) to 4 for a fully zoomed-out map (16 blocks per pixel) |
| Locked | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | True if the map has been locked in a cartography table |
| Icons | Type | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | See below |
| X | [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | Map coordinates: -128 for furthest left, +127 for furthest right |
| Z | [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | Map coordinates: -128 for highest, +127 for lowest |
| Direction | [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | 0-15 |
| Display Name | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) |  |
| Color Patch | Columns | [Unsigned Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Unsigned_Byte) | Number of columns updated |
| Rows | [Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Optional) [Unsigned Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Unsigned_Byte) | Only if Columns is more than 0; number of rows updated |
| X | [Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Optional) [Unsigned Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Unsigned_Byte) | Only if Columns is more than 0; x offset of the westernmost column |
| Z | [Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Optional) [Unsigned Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Unsigned_Byte) | Only if Columns is more than 0; z offset of the northernmost row |
| Data | [Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Optional) [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of [Unsigned Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Unsigned_Byte) | Only if Columns is more than 0; see [Map item format](https://minecraft.wiki/w/Map_item_format "Map item format") |

For icons, a direction of 0 is a vertical icon and increments by 22.5° (360/16).

Types are based off of rows and columns in `map_icons.png`:

| Icon type | Result |
| --- | --- |
| 0 | White arrow (players) |
| 1 | Green arrow (item frames) |
| 2 | Red arrow |
| 3 | Blue arrow |
| 4 | White cross |
| 5 | Red pointer |
| 6 | White circle (off-map players) |
| 7 | Small white circle (far-off-map players) |
| 8 | Mansion |
| 9 | Monument |
| 10 | White Banner |
| 11 | Orange Banner |
| 12 | Magenta Banner |
| 13 | Light Blue Banner |
| 14 | Yellow Banner |
| 15 | Lime Banner |
| 16 | Pink Banner |
| 17 | Gray Banner |
| 18 | Light Gray Banner |
| 19 | Cyan Banner |
| 20 | Purple Banner |
| 21 | Blue Banner |
| 22 | Brown Banner |
| 23 | Green Banner |
| 24 | Red Banner |
| 25 | Black Banner |
| 26 | Treasure marker |
| 27 | Desert Village |
| 28 | Plains Village |
| 29 | Savanna Village |
| 30 | Snowy Village |
| 31 | Taiga Village |
| 32 | Jungle Temple |
| 33 | Swamp Hut |
| 34 | Trial Chambers |

#### Merchant Offers\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=111&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Merchant Offers") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=111&editintro=Template:License_wiki.vg "Edit section's source code: Merchant Offers")\]

The list of trades a villager NPC is offering.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x2D`<br>_resource:_<br>`merchant_offers` | Play | Client | Window ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | The ID of the window that is open; this is an int rather than a byte. |
| Trades | Input item 1 | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) | Trade Item | See below. The first item the player has to supply for this villager trade. The count of the item stack is the default "price" of this trade. |
| Output item | [Slot](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Slot) | The item the player will receive from this villager trade. |
| Input item 2 | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) Trade Item | The second item the player has to supply for this villager trade. |
| Trade disabled | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | True if the trade is disabled; false if the trade is enabled. |
| Number of trade uses | [Int](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Int) | Number of times the trade has been used so far. If equal to the maximum number of trades, the client will display a red X. |
| Maximum number of trade uses | [Int](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Int) | Number of times this trade can be used before it's exhausted. |
| XP | [Int](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Int) | Amount of XP the villager will earn each time the trade is used. |
| Special Price | [Int](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Int) | Can be zero or negative. The number is added to the price when an item is discounted due to player reputation or other effects. |
| Price Multiplier | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | Can be low (0.05) or high (0.2). Determines how much demand, player reputation, and temporary effects will adjust the price. |
| Demand | [Int](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Int) | If positive, causes the price to increase. Negative values seem to be treated the same as zero. |
| Villager level | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Appears on the trade GUI; meaning comes from the translation key `merchant.level.` \+ level.<br>1: Novice, 2: Apprentice, 3: Journeyman, 4: Expert, 5: Master. |
| Experience | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Total experience for this villager (always 0 for the wandering trader). |
| Is regular villager | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | True if this is a regular villager; false for the wandering trader. When false, hides the villager level and some other GUI elements. |
| Can restock | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | True for regular villagers and false for the wandering trader. If true, the "Villagers restock up to two times per day." message is displayed when hovering over disabled trades. |

Trade Item:

| Field Name | Field Type | Meaning |
| --- | --- | --- |
| Item ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | The [item ID](https://minecraft.wiki/w/Java_Edition_data_values#Blocks "Java Edition data values"). Item IDs are distinct from block IDs; see [Data Generators](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Data_Generators "Minecraft Wiki:Projects/wiki.vg merge/Data Generators") for more information. |
| Item Count | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | The item count. |
| Components | Component type | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | The type of component. See [Structured components](https://minecraft.wiki/w/Java_Edition_protocol/Slot_Data#Structured_components "Java Edition protocol/Slot Data") for more detail. |
| Component data | Varies | The component-dependent data. See [Structured components](https://minecraft.wiki/w/Java_Edition_protocol/Slot_Data#Structured_components "Java Edition protocol/Slot Data") for more detail. |

Modifiers can increase or decrease the number of items for the first input slot. The second input slot and the output slot never change the number of items. The number of items may never be less than 1, and never more than the stack size. If special price and demand are both zero, only the default price is displayed. If either is non-zero, then the adjusted price is displayed next to the crossed-out default price. The adjusted prices is calculated as follows:

Adjusted price = default price + floor(default price x multiplier x demand) + special price

[![](https://minecraft.wiki/images/thumb/Merchant-slots.png/300px-Merchant-slots.png?6b2ed)](https://minecraft.wiki/w/File:Merchant-slots.png) The merchant UI, for reference

#### Update Entity Position\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=112&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Update Entity Position") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=112&editintro=Template:License_wiki.vg "Edit section's source code: Update Entity Position")\]

This packet is sent by the server when an entity moves a small distance. The change in position is represented as a [fixed-point number](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Fixed-point_numbers) with 12 fraction bits and 4 integer bits. As such, the maximum movement distance along each axis is 8 blocks in the negative direction, or 7.999755859375 blocks in the positive direction. If the movement exceeds these limits, [Teleport Entity](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Teleport_Entity) should be sent instead.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x2E`<br>_resource:_<br>`move_entity_pos` | Play | Client | Entity ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |
| Delta X | [Short](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Short) | Change in X position as `currentX * 4096 - prevX * 4096`. |
| Delta Y | [Short](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Short) | Change in Y position as `currentY * 4096 - prevY * 4096`. |
| Delta Z | [Short](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Short) | Change in Z position as `currentZ * 4096 - prevZ * 4096`. |
| On Ground | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) |  |

#### Update Entity Position and Rotation\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=113&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Update Entity Position and Rotation") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=113&editintro=Template:License_wiki.vg "Edit section's source code: Update Entity Position and Rotation")\]

This packet is sent by the server when an entity rotates and moves. See [#Update Entity Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Update_Entity_Position) for how the position is encoded.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x2F`<br>_resource:_<br>`move_entity_pos_rot` | Play | Client | Entity ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |
| Delta X | [Short](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Short) | Change in X position as `currentX * 4096 - prevX * 4096`. |
| Delta Y | [Short](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Short) | Change in Y position as `currentY * 4096 - prevY * 4096`. |
| Delta Z | [Short](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Short) | Change in Z position as `currentZ * 4096 - prevZ * 4096`. |
| Yaw | [Angle](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Angle) | New angle, not a delta. |
| Pitch | [Angle](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Angle) | New angle, not a delta. |
| On Ground | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) |  |

#### Move Minecart Along Track\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=114&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Move Minecart Along Track") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=114&editintro=Template:License_wiki.vg "Edit section's source code: Move Minecart Along Track")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x30`<br>_resource:_<br>`move_minecart_along_track` | Play | Client | Entity ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |
| Steps | X | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) |  |
| Y | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) |  |
| Z | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) |  |
| Velocity X | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) |  |
| Velocity Y | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) |  |
| Velocity Z | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) |  |
| Yaw | [Angle](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Angle) |  |
| Pitch | [Angle](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Angle) |  |
| Weight | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) |

#### Update Entity Rotation\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=115&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Update Entity Rotation") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=115&editintro=Template:License_wiki.vg "Edit section's source code: Update Entity Rotation")\]

This packet is sent by the server when an entity rotates.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x31`<br>_resource:_<br>`move_entity_rot` | Play | Client | Entity ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |
| Yaw | [Angle](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Angle) | New angle, not a delta. |
| Pitch | [Angle](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Angle) | New angle, not a delta. |
| On Ground | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) |  |

#### Move Vehicle (clientbound)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=116&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Move Vehicle (clientbound)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=116&editintro=Template:License_wiki.vg "Edit section's source code: Move Vehicle (clientbound)")\]

If the player is riding a client-side-controlled vehicle, teleports the vehicle to the specified position. Sent by the vanilla server in response to serverbound [Move Vehicle](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Move_Vehicle_(serverbound)) packets that fail the movement speed check. Note that all fields use absolute positioning and do not allow for relative positioning.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x32`<br>_resource:_<br>`move_vehicle` | Play | Client | X | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) | Absolute position (X coordinate). |
| Y | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) | Absolute position (Y coordinate). |
| Z | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) | Absolute position (Z coordinate). |
| Yaw | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | Absolute rotation on the vertical axis, in degrees. |
| Pitch | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | Absolute rotation on the horizontal axis, in degrees. |

#### Open Book\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=117&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Open Book") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=117&editintro=Template:License_wiki.vg "Edit section's source code: Open Book")\]

Sent when a player right-clicks with a signed book. This tells the client to open the book GUI.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x33`<br>_resource:_<br>`open_book` | Play | Client | Hand | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | 0: Main hand, 1: Off hand . |

#### Open Screen\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=118&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Open Screen") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=118&editintro=Template:License_wiki.vg "Edit section's source code: Open Screen")\]

This is sent to the client when it should open an inventory, such as a chest, workbench, furnace, or other container. Resending this packet with the already existing window ID, will update the window title and window type without closing the window.

This message is not sent to clients opening their own inventory, nor do clients inform the server in any way when doing so. From the server's perspective, the inventory is always "open" whenever no other windows are.

For horses, use [Open Horse Screen](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Open_Horse_Screen).

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x34`<br>_resource:_<br>`open_screen` | Play | Client | Window ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | An identifier for the window to be displayed. The vanilla server implementation is a counter, starting at 1. There can only be one window at a time; this is only used to ignore outdated packets targeting already-closed windows. Note also that the Window ID field in most other packets is only a single byte, and indeed, the vanilla server wraps around after 100. |
| Window Type | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | The window type to use for display. Contained in the `minecraft:menu` registry; see [Inventory](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Inventory "Minecraft Wiki:Projects/wiki.vg merge/Inventory") for the different values. |
| Window Title | [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) | The title of the window. |

#### Open Sign Editor\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=119&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Open Sign Editor") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=119&editintro=Template:License_wiki.vg "Edit section's source code: Open Sign Editor")\]

Sent when the client has placed a sign and is allowed to send [Update Sign](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Update_Sign). There must already be a sign at the given location (which the client does not do automatically) - send a [Block Update](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Block_Update) first.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x35`<br>_resource:_<br>`open_sign_editor` | Play | Client | Location | [Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Position) |  |
| Is Front Text | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | Whether the opened editor is for the front or on the back of the sign |

#### Ping (play)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=120&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Ping (play)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=120&editintro=Template:License_wiki.vg "Edit section's source code: Ping (play)")\]

Packet is not used by the vanilla server. When sent to the client, client responds with a [Pong](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Pong_(play)) packet with the same ID.

Unlike [Keep Alive](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Clientbound_Keep_Alive_(play)) this packet is handled synchronously with game logic on the vanilla client, and can thus be used to reliably detect which serverbound packets were sent after the ping and all preceding clientbound packets were received and handled.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x36`<br>_resource:_<br>`ping` | Play | Client | ID | [Int](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Int) |  |

#### Ping Response (play)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=121&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Ping Response (play)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=121&editintro=Template:License_wiki.vg "Edit section's source code: Ping Response (play)")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x37`<br>_resource:_<br>`pong_response` | Play | Client | Payload | [Long](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Long) | Should be the same as sent by the client. |

#### Place Ghost Recipe\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=122&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Place Ghost Recipe") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=122&editintro=Template:License_wiki.vg "Edit section's source code: Place Ghost Recipe")\]

Response to the serverbound packet ( [Place Recipe](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Place_Recipe)), with the same recipe ID. Appears to be used to notify the UI.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x38`<br>_resource:_<br>`place_ghost_recipe` | Play | Client | Window ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |
| Recipe Display | [Recipe Display](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Recipe_Display) |  |

#### Player Abilities (clientbound)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=123&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Player Abilities (clientbound)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=123&editintro=Template:License_wiki.vg "Edit section's source code: Player Abilities (clientbound)")\]

The latter 2 floats are used to indicate the flying speed and field of view respectively, while the first byte is used to determine the value of 4 booleans.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x39`<br>_resource:_<br>`player_abilities` | Play | Client | Flags | [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | Bit field, see below. |
| Flying Speed | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | 0.05 by default. |
| Field of View Modifier | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | Modifies the field of view, like a speed potion. A vanilla server will use the same value as the movement speed sent in the [Update Attributes](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Update_Attributes) packet, which defaults to 0.1 for players. |

About the flags:

| Field | Bit |
| --- | --- |
| Invulnerable | 0x01 |
| Flying | 0x02 |
| Allow Flying | 0x04 |
| Creative Mode (Instant Break) | 0x08 |

If Flying is set but Allow Flying is unset, the player is unable to stop flying.

#### Player Chat Message\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=124&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Player Chat Message") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=124&editintro=Template:License_wiki.vg "Edit section's source code: Player Chat Message")\]

Main page: [Minecraft\_Wiki:Projects/wiki.vg\_merge/Chat](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Chat "Minecraft Wiki:Projects/wiki.vg merge/Chat")

Sends the client a chat message from a player.

Currently, a lot is unknown about this packet, blank descriptions are for those that are unknown

| Packet ID | State | Bound To | Sector | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x3A`<br>_resource:_<br>`player_chat` | Play | Client | Header | Global Index | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | A counter that starts at zero and gets increased by one for each chat message sent to the client. Each client has its own counter. |
| Sender | [UUID](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:UUID) | Used by the vanilla client for the disableChat launch option. Setting both longs to 0 will always display the message regardless of the setting. |
| Index | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |
| Message Signature bytes | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) [Byte Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte_Array) (256) | Cryptography, the signature consists of the Sender UUID, Session UUID from the [Player Session](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Player_Session) packet, Index, Salt, Timestamp in epoch seconds, the length of the original chat content, the original content itself, the length of Previous Messages, and all of the Previous message signatures. These values are hashed with [SHA-256](https://en.wikipedia.org/wiki/SHA-2) and signed using the [RSA](https://en.wikipedia.org/wiki/RSA_(cryptosystem)) cryptosystem. Modifying any of these values in the packet will cause this signature to fail. This buffer is always 256 bytes long and it is not length-prefixed. |
| Body | Message | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (256) | Raw (optionally) signed sent message content.<br>This is used as the `content` parameter when formatting the message on the client. |
| Timestamp | [Long](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Long) | Represents the time the message was signed as milliseconds since the [epoch](https://en.wikipedia.org/wiki/Unix_time), used to check if the message was received within 2 minutes of it being sent. |
| Salt | [Long](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Long) | Cryptography, used for validating the message signature. |
| [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) (20) | Message ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | The message ID + 1, used for validating message signature. The next field is present only when value of this field is equal to 0. |
| Signature | [Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Optional) [Byte Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte_Array) (256) | The previous message's signature. Contains the same type of data as `Message Signature bytes` (256 bytes) above. Not length-prefixed. |
| Other | Unsigned Content | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) |  |
| Filter Type | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | If the message has been filtered |
| Filter Type Bits | [Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Optional) [BitSet](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:BitSet) | Only present if the Filter Type is Partially Filtered. Specifies the indices at which characters in the original message string should be replaced with the `#` symbol (i.e., filtered) by the vanilla client |
| Chat Formatting | Chat Type | [ID or](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:ID_or) [Chat Type](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Chat_Type) | Either the type of chat in the `minecraft:chat_type` registry, defined by the [Registry Data](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Registry_Data_2) packet, or an inline definition. |
| Sender Name | [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) | The name of the one sending the message, usually the sender's display name.<br>This is used as the `sender` parameter when formatting the message on the client. |
| Target Name | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) | The name of the one receiving the message, usually the receiver's display name.<br>This is used as the `target` parameter when formatting the message on the client. |

[![](https://minecraft.wiki/images/thumb/MinecraftChat.drawio4.png/300px-MinecraftChat.drawio4.png?ccc6e)](https://minecraft.wiki/w/File:MinecraftChat.drawio4.png) Player Chat Handling Logic

Filter Types:

The filter type mask should NOT be specified unless partially filtered is selected

| ID | Name | Description |
| --- | --- | --- |
| 0 | PASS\_THROUGH | Message is not filtered at all |
| 1 | FULLY\_FILTERED | Message is fully filtered |
| 2 | PARTIALLY\_FILTERED | Only some characters in the message are filtered |

#### End Combat\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=125&veaction=edit&editintro=Template:License_wiki.vg "Edit section: End Combat") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=125&editintro=Template:License_wiki.vg "Edit section's source code: End Combat")\]

Unused by the vanilla client. This data was once used for twitch.tv metadata circa 1.8.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x3B`<br>_resource:_<br>`player_combat_end` | Play | Client | Duration | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Length of the combat in ticks. |

#### Enter Combat\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=126&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Enter Combat") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=126&editintro=Template:License_wiki.vg "Edit section's source code: Enter Combat")\]

Unused by the vanilla client. This data was once used for twitch.tv metadata circa 1.8.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x3C`<br>_resource:_<br>`player_combat_enter` | Play | Client | _no fields_ |

#### Combat Death\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=127&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Combat Death") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=127&editintro=Template:License_wiki.vg "Edit section's source code: Combat Death")\]

Used to send a respawn screen.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x3D`<br>_resource:_<br>`player_combat_kill` | Play | Client | Player ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Entity ID of the player that died (should match the client's entity ID). |
| Message | [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) | The death message. |

#### Player Info Remove\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=128&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Player Info Remove") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=128&editintro=Template:License_wiki.vg "Edit section's source code: Player Info Remove")\]

Used by the server to remove players from the player list.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x3E`<br>_resource:_<br>`player_info_remove` | Play | Client | UUIDs | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of [UUID](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:UUID) | UUIDs of players to remove. |

#### Player Info Update\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=129&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Player Info Update") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=129&editintro=Template:License_wiki.vg "Edit section's source code: Player Info Update")\]

Sent by the server to update the user list (<tab> in the client).

[![](https://minecraft.wiki/images/thumb/Warning_pixelated_art.png/32px-Warning_pixelated_art.png?d0083)](https://minecraft.wiki/w/File:Warning_pixelated_art.png)

The EnumSet type is only used here and it is currently undocumented

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x3F`<br>_resource:_<br>`player_info_update` | Play | Client | Actions | [EnumSet](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:EnumSet) | Determines what actions are present. |
| Players | UUID | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) | [UUID](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:UUID) | The player UUID |
| Player Actions | [Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Array) of [Player Actions](https://minecraft.wiki/w/Java_Edition_protocol/Packets#player-info:player-actions) | The length of this array is determined by the number of [Player Actions](https://minecraft.wiki/w/Java_Edition_protocol/Packets#player-info:player-actions) that give a non-zero value when applying its mask to the actions flag. For example, given the decimal number 5, binary 00000101. The masks 0x01 and 0x04 would return a non-zero value, meaning the Player Actions array would include two actions: Add Player and Update Game Mode. |

| Action | Mask | Field Name | Type | Notes |
| --- | --- | --- | --- | --- |
| Add Player | 0x01 | Name | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (16) |
| Property | Name | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) (16) | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (64) |  |
| Value | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (32767) |  |
| Signature | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (1024) |  |
| Initialize Chat | 0x02 | Data | Chat session ID | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) | [UUID](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:UUID) |  |
| Public key expiry time | [Long](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Long) | Key expiry time, as a UNIX timestamp in milliseconds. Only sent if Has Signature Data is true. |
| Encoded public key | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) (512) of [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | The player's public key, in bytes. Only sent if Has Signature Data is true. |
| Public key signature | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) (4096) of [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | The public key's digital signature. Only sent if Has Signature Data is true. |
| Update Game Mode | 0x04 | Game Mode | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |
| Update Listed | 0x08 | Listed | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | Whether the player should be listed on the player list. |
| Update Latency | 0x10 | Ping | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Measured in milliseconds. |
| Update Display Name | 0x20 | Display Name | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) | Only sent if Has Display Name is true. |
| Update List Priority | 0x40 | Priority | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | See below. |
| Update Hat | 0x80 | Visible | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | Whether the player's hat skin layer is shown. |

Player Actions

The properties included in this packet are the same as in [Login Success](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Login_Success), for the current player.

Ping values correspond with icons in the following way:

- A ping that is negative (i.e., not known to the server yet) will result in the no connection icon.
- A ping under 150 milliseconds will result in 5 bars
- A ping under 300 milliseconds will result in 4 bars
- A ping under 600 milliseconds will result in 3 bars
- A ping under 1000 milliseconds (1 second) will result in 2 bars
- A ping greater than or equal to 1 second will result in 1 bar.

The order of players in the player list is determined as follows:

- Players with higher priorities are sorted before those with lower priorities.
- Among players of equal priorities, spectators are sorted after non-spectators.
- Within each of those groups, players are sorted into teams. The teams are ordered case-sensitively by team name in ascending order. Players with no team are listed first.
- The players of each team (and non-team) are sorted case-insensitively by name in ascending order.

#### Look At\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=130&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Look At") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=130&editintro=Template:License_wiki.vg "Edit section's source code: Look At")\]

Used to rotate the client player to face the given location or entity (for `/teleport [<targets>] <x> <y> <z> facing`).

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x40`<br>_resource:_<br>`player_look_at` | Play | Client |
| Feet/eyes | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | Values are feet=0, eyes=1. If set to eyes, aims using the head position; otherwise, aims using the feet position. |
| Target x | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) | x coordinate of the point to face towards. |
| Target y | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) | y coordinate of the point to face towards. |
| Target z | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) | z coordinate of the point to face towards. |
| Is entity | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | If true, additional information about an entity is provided. |
| Entity ID | [Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Optional) [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Only if is entity is true — the entity to face towards. |
| Entity feet/eyes | [Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Optional) [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | Whether to look at the entity's eyes or feet. Same values and meanings as before, just for the entity's head/feet. |

If the entity given by entity ID cannot be found, this packet should be treated as if is entity was false.

#### Synchronize Player Position\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=131&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Synchronize Player Position") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=131&editintro=Template:License_wiki.vg "Edit section's source code: Synchronize Player Position")\]

Teleports the client, e.g., during login, when using an ender pearl, in response to invalid move packets, etc.

Due to latency, the server may receive outdated movement packets sent before the client was aware of the teleport. To account for this, the server ignores all movement packets from the client until a [Confirm Teleportation](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Confirm_Teleportation) packet with an ID matching the one sent in the teleport packet is received. The vanilla client will also send a [Set Player Position and Rotation](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Player_Position_and_Rotation) packet after the [Confirm Teleportation](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Confirm_Teleportation) packet with the position and rotation received from this packet, and horizontal collision and on ground set to false.

Yaw is measured in degrees and does not follow classical trigonometry rules. The unit circle of yaw on the XZ-plane starts at (0, 1) and turns counterclockwise, with 90 at (-1, 0), 180 at (0, -1) and 270 at (1, 0). Additionally, yaw is not clamped to between 0 and 360 degrees; any number is valid, including negative numbers and numbers greater than 360 (see [MC-90097](https://bugs.mojang.com/browse/MC-90097)).

Pitch is measured in degrees, where 0 is looking straight ahead, -90 is looking straight up, and 90 is looking straight down.

If the player is riding a vehicle, this packet has no effect, but both the [Confirm Teleportation](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Confirm_Teleportation) and [Set Player Position and Rotation](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Player_Position_and_Rotation) packets are still sent.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x41`<br>_resource:_<br>`player_position` | Play | Client | Teleport ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Client should confirm this packet with [Confirm Teleportation](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Confirm_Teleportation) containing the same Teleport ID. |
| X | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) | Absolute or relative position, depending on Flags. |
| Y | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) | Absolute or relative position, depending on Flags. |
| Z | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) | Absolute or relative position, depending on Flags. |
| Velocity X | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) |  |
| Velocity Y | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) |  |
| Velocity Z | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) |  |
| Yaw | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | Absolute or relative rotation on the X axis, in degrees. |
| Pitch | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | Absolute or relative rotation on the Y axis, in degrees. |
| Flags | [Teleport Flags](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Teleport_Flags) |  |

#### Player Rotation\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=132&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Player Rotation") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=132&editintro=Template:License_wiki.vg "Edit section's source code: Player Rotation")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x42`<br>_resource:_<br>`player_rotation` | Play | Client | Yaw | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | Rotation on the X axis, in degrees. |
| Pitch | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | Rotation on the Y axis, in degrees. |

#### Recipe Book Add\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=133&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Recipe Book Add") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=133&editintro=Template:License_wiki.vg "Edit section's source code: Recipe Book Add")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x43`<br>_resource:_<br>`recipe_book_add` | Play | Client | Recipes | Recipe ID | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | ID to assign to the recipe. |
| Display | [Recipe Display](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Recipe_Display) |  |
| Group ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |
| Category ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | ID in the `minecraft:recipe_book_category` registry. |
| Ingredients | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of [ID Set](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:ID_Set) | IDs in the `minecraft:item` registry, or an inline definition. |
| Flags | [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | 0x01: show notification; 0x02: highlight as new |
| Replace | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | Replace or Add to known recipes |

#### Recipe Book Remove\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=134&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Recipe Book Remove") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=134&editintro=Template:License_wiki.vg "Edit section's source code: Recipe Book Remove")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x44`<br>_resource:_<br>`recipe_book_remove` | Play | Client | Recipes | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | IDs of recipes to remove. |

#### Recipe Book Settings\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=135&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Recipe Book Settings") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=135&editintro=Template:License_wiki.vg "Edit section's source code: Recipe Book Settings")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x45`<br>_resource:_<br>`recipe_book_settings` | Play | Client | Crafting Recipe Book Open | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | If true, then the crafting recipe book will be open when the player opens its inventory. |
| Crafting Recipe Book Filter Active | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | If true, then the filtering option is active when the player opens its inventory. |
| Smelting Recipe Book Open | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | If true, then the smelting recipe book will be open when the player opens its inventory. |
| Smelting Recipe Book Filter Active | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | If true, then the filtering option is active when the player opens its inventory. |
| Blast Furnace Recipe Book Open | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | If true, then the blast furnace recipe book will be open when the player opens its inventory. |
| Blast Furnace Recipe Book Filter Active | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | If true, then the filtering option is active when the player opens its inventory. |
| Smoker Recipe Book Open | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | If true, then the smoker recipe book will be open when the player opens its inventory. |
| Smoker Recipe Book Filter Active | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | If true, then the filtering option is active when the player opens its inventory. |

#### Remove Entities\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=136&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Remove Entities") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=136&editintro=Template:License_wiki.vg "Edit section's source code: Remove Entities")\]

Sent by the server when an entity is to be destroyed on the client.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x46`<br>_resource:_<br>`remove_entities` | Play | Client | Entity IDs | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | The list of entities to destroy. |

#### Remove Entity Effect\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=137&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Remove Entity Effect") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=137&editintro=Template:License_wiki.vg "Edit section's source code: Remove Entity Effect")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x47`<br>_resource:_<br>`remove_mob_effect` | Play | Client | Entity ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |
| Effect ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | See [this table](https://minecraft.wiki/w/Status_effect#Effect_list "Status effect"). |

#### Reset Score\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=138&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Reset Score") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=138&editintro=Template:License_wiki.vg "Edit section's source code: Reset Score")\]

This is sent to the client when it should remove a scoreboard item.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x48`<br>_resource:_<br>`reset_score` | Play | Client | Entity Name | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (32767) | The entity whose score this is. For players, this is their username; for other entities, it is their UUID. |
| Objective Name | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (32767) | The name of the objective the score belongs to. |

#### Remove Resource Pack (play)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=139&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Remove Resource Pack (play)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=139&editintro=Template:License_wiki.vg "Edit section's source code: Remove Resource Pack (play)")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x49`<br>_resource:_<br>`resource_pack_pop` | Play | Client | UUID | [Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Optional) [UUID](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:UUID) | The UUID of the resource pack to be removed. |

#### Add Resource Pack (play)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=140&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Add Resource Pack (play)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=140&editintro=Template:License_wiki.vg "Edit section's source code: Add Resource Pack (play)")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x4A`<br>_resource:_<br>`resource_pack_push` | Play | Client | UUID | [UUID](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:UUID) | The unique identifier of the resource pack. |
| URL | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (32767) | The URL to the resource pack. |
| Hash | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (40) | A 40 character hexadecimal, case-insensitive [SHA-1](https://en.wikipedia.org/wiki/SHA-1 "wikipedia:SHA-1") hash of the resource pack file.<br>If it's not a 40-character hexadecimal string, the client will not use it for hash verification and likely waste bandwidth. |
| Forced | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | The vanilla client will be forced to use the resource pack from the server. If they decline, they will be kicked from the server. |
| Prompt Message | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) | This is shown in the prompt making the client accept or decline the resource pack. |

#### Respawn\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=141&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Respawn") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=141&editintro=Template:License_wiki.vg "Edit section's source code: Respawn")\]

[![](https://minecraft.wiki/images/thumb/Info.png/16px-Info.png?24414)](https://minecraft.wiki/w/File:Info.png)

This section is missing information about: Although the number of portal cooldown ticks is included in this packet, the whole portal usage process is still dictated entirely by the server. What kind of effect does this value have on the client, if any?

Please expand the section to include this information. Further details may exist on the [talk page](https://minecraft.wiki/w/Talk:Java_Edition_protocol/Packets).

To change the player's dimension (overworld/nether/end), send them a respawn packet with the appropriate dimension, followed by prechunks/chunks for the new dimension, and finally a position and look packet. You do not need to unload chunks; the client will do it automatically.

The background of the loading screen is determined based on the Dimension Name specified in this packet and the one specified in the previous Login or Respawn packet. If either the current or the previous dimension is `minecraft:nether`, the Nether portal background is used. Otherwise, if the current or the previous dimension is `minecraft:the_end`, the End portal background is used. If the player is dead (health is 0), the default background is always used.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x4B`<br>_resource:_<br>`respawn` | Play | Client | Dimension Type | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | The ID of type of dimension in the `minecraft:dimension_type` registry, defined by the [Registry Data](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Registry_Data_2) packet. |
| Dimension Name | [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) | Name of the dimension being spawned into. |
| Hashed seed | [Long](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Long) | First 8 bytes of the SHA-256 hash of the world's seed. Used client-side for biome noise |
| Game mode | [Unsigned Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Unsigned_Byte) | 0: Survival, 1: Creative, 2: Adventure, 3: Spectator. |
| Previous Game mode | [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | -1: Undefined (null), 0: Survival, 1: Creative, 2: Adventure, 3: Spectator. The previous game mode. Vanilla client uses this for the debug (F3 + N & F3 + F4) game mode switch. (More information needed) |
| Is Debug | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | True if the world is a [debug mode](https://minecraft.wiki/w/Debug_mode "Debug mode") world; debug mode worlds cannot be modified and have predefined blocks. |
| Is Flat | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | True if the world is a [superflat](https://minecraft.wiki/w/Superflat "Superflat") world; flat worlds have different void fog and a horizon at y=0 instead of y=63. |
| Has death location | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | If true, then the next two fields are present. |
| Death dimension Name | [Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Optional) [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) | Name of the dimension the player died in. |
| Death location | [Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Optional) [Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Position) | The location that the player died at. |
| Portal cooldown | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | The number of ticks until the player can use the portal again. |
| Sea level | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |
| Data kept | [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | Bit mask. 0x01: Keep attributes, 0x02: Keep metadata. Tells which data should be kept on the client side once the player has respawned.<br>In the vanilla implementation, this is context-dependent:<br>- normal respawns (after death) keep no data;<br>- exiting the end poem/credits keeps the attributes;<br>- other dimension changes (portals or teleports) keep all data. |

[![](https://minecraft.wiki/images/thumb/Warning_pixelated_art.png/32px-Warning_pixelated_art.png?d0083)](https://minecraft.wiki/w/File:Warning_pixelated_art.png)

Avoid changing a player's dimension to the same dimension they were already in unless they are dead. If you change the dimension to one they are already in, weird bugs can occur, such as the player being unable to attack other players in the new world (until they die and respawn).
Before 1.16, if you must respawn a player in the same dimension without killing them, send two respawn packets, one to a different world and then another to the world you want. You do not need to complete the first respawn; it only matters that you send two packets.

#### Set Head Rotation\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=142&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Set Head Rotation") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=142&editintro=Template:License_wiki.vg "Edit section's source code: Set Head Rotation")\]

Changes the direction an entity's head is facing.

While sending the Entity Look packet changes the vertical rotation of the head, sending this packet appears to be necessary to rotate the head horizontally.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x4C`<br>_resource:_<br>`rotate_head` | Play | Client | Entity ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |
| Head Yaw | [Angle](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Angle) | New angle, not a delta. |

#### Update Section Blocks\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=143&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Update Section Blocks") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=143&editintro=Template:License_wiki.vg "Edit section's source code: Update Section Blocks")\]

Fired whenever 2 or more blocks are changed within the same chunk on the same tick.

[![](https://minecraft.wiki/images/thumb/Warning_pixelated_art.png/32px-Warning_pixelated_art.png?d0083)](https://minecraft.wiki/w/File:Warning_pixelated_art.png)

Changing blocks in chunks not loaded by the client is unsafe (see note on [Block Update](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Block_Update)).

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x4D`<br>_resource:_<br>`section_blocks_update` | Play | Client | Chunk section position | [Long](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Long) | Chunk section coordinate (encoded chunk x and z with each 22 bits, and section y with 20 bits, from left to right). |
| Blocks | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of [VarLong](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarLong) | Each entry is composed of the block state id, shifted left by 12, and the relative block position in the chunk section (4 bits for x, z, and y, from left to right). |

Chunk section position is encoded:

```
((sectionX & 0x3FFFFF) << 42) | (sectionY & 0xFFFFF) | ((sectionZ & 0x3FFFFF) << 20);
```

and decoded:

```
sectionX = long >> 42;
sectionY = long << 44 >> 44;
sectionZ = long << 22 >> 42;
```

Blocks are encoded:

```
blockStateId << 12 | (blockLocalX << 8 | blockLocalZ << 4 | blockLocalY)
//Uses the local position of the given block position relative to its respective chunk section
```

and decoded:

```
blockStateId = long >> 12;
blockLocalX = (long >> 8) & 0xF;
blockLocalY = long & 0xF;
blockLocalZ = (long >> 4) & 0xF;
```

#### Select Advancements Tab\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=144&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Select Advancements Tab") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=144&editintro=Template:License_wiki.vg "Edit section's source code: Select Advancements Tab")\]

Sent by the server to indicate that the client should switch advancement tab. Sent either when the client switches tab in the GUI or when an advancement is made in another tab.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x4E`<br>_resource:_<br>`select_advancements_tab` | Play | Client | Identifier | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) | See below. |

The [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) must be one of the following if no custom data pack is loaded:

| Identifier |
| --- |
| minecraft:story/root |
| minecraft:nether/root |
| minecraft:end/root |
| minecraft:adventure/root |
| minecraft:husbandry/root |

If no or an invalid identifier is sent, the client will switch to the first tab in the GUI.

#### Server Data\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=145&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Server Data") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=145&editintro=Template:License_wiki.vg "Edit section's source code: Server Data")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x4F`<br>_resource:_<br>`server_data` | Play | Client | MOTD | [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) |  |
| Icon | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | Icon bytes in the PNG format. |

#### Set Action Bar Text\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=146&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Set Action Bar Text") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=146&editintro=Template:License_wiki.vg "Edit section's source code: Set Action Bar Text")\]

Displays a message above the hotbar. Equivalent to [System Chat Message](https://minecraft.wiki/w/Java_Edition_protocol/Packets#System_Chat_Message) with Overlay set to true, except that [chat message blocking](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Chat#Social_Interactions_(blocking) "Minecraft Wiki:Projects/wiki.vg merge/Chat") isn't performed. Used by the vanilla server only to implement the `/title` command.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x50`<br>_resource:_<br>`set_action_bar_text` | Play | Client | Action bar text | [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) |

#### Set Border Center\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=147&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Set Border Center") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=147&editintro=Template:License_wiki.vg "Edit section's source code: Set Border Center")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x51`<br>_resource:_<br>`set_border_center` | Play | Client | X | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) |  |
| Z | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) |  |

#### Set Border Lerp Size\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=148&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Set Border Lerp Size") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=148&editintro=Template:License_wiki.vg "Edit section's source code: Set Border Lerp Size")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x52`<br>_resource:_<br>`set_border_lerp_size` | Play | Client | Old Diameter | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) | Current length of a single side of the world border, in meters. |
| New Diameter | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) | Target length of a single side of the world border, in meters. |
| Speed | [VarLong](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarLong) | Number of real-time _milli_ seconds until New Diameter is reached. It appears that vanilla server does not sync world border speed to game ticks, so it gets out of sync with server lag. If the world border is not moving, this is set to 0. |

#### Set Border Size\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=149&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Set Border Size") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=149&editintro=Template:License_wiki.vg "Edit section's source code: Set Border Size")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x53`<br>_resource:_<br>`set_border_size` | Play | Client | Diameter | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) | Length of a single side of the world border, in meters. |

#### Set Border Warning Delay\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=150&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Set Border Warning Delay") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=150&editintro=Template:License_wiki.vg "Edit section's source code: Set Border Warning Delay")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x54`<br>_resource:_<br>`set_border_warning_delay` | Play | Client | Warning Time | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | In seconds as set by `/worldborder warning time`. |

#### Set Border Warning Distance\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=151&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Set Border Warning Distance") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=151&editintro=Template:License_wiki.vg "Edit section's source code: Set Border Warning Distance")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x55`<br>_resource:_<br>`set_border_warning_distance` | Play | Client | Warning Blocks | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | In meters. |

#### Set Camera\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=152&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Set Camera") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=152&editintro=Template:License_wiki.vg "Edit section's source code: Set Camera")\]

Sets the entity that the player renders from. This is normally used when the player left-clicks an entity while in spectator mode.

The player's camera will move with the entity and look where it is looking. The entity is often another player, but can be any type of entity. The player is unable to move this entity (move packets will act as if they are coming from the other entity).

If the given entity is not loaded by the player, this packet is ignored. To return control to the player, send this packet with their entity ID.

The vanilla server resets this (sends it back to the default entity) whenever the spectated entity is killed or the player sneaks, but only if they were spectating an entity. It also sends this packet whenever the player switches out of spectator mode (even if they weren't spectating an entity).

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x56`<br>_resource:_<br>`set_camera` | Play | Client | Camera ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | ID of the entity to set the client's camera to. |

The vanilla client also loads certain shaders for given entities:

- Creeper → `shaders/post/creeper.json`
- Spider (and cave spider) → `shaders/post/spider.json`
- Enderman → `shaders/post/invert.json`
- Anything else → the current shader is unloaded

#### Set Center Chunk\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=153&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Set Center Chunk") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=153&editintro=Template:License_wiki.vg "Edit section's source code: Set Center Chunk")\]

Sets the center position of the client's chunk loading area. The area is square-shaped, spanning 2 × server view distance + 7 chunks on both axes (width, not radius!). Since the area's width is always an odd number, there is no ambiguity as to which chunk is the center.

The vanilla client never renders or simulates chunks located outside the loading area, but keeps them in memory (unless explicitly unloaded by the server while still in range), and only automatically unloads a chunk when another chunk is loaded at coordinates congruent to the old chunk's coordinates modulo (2 × server view distance + 7). This means that a chunk may reappear after leaving and later re-entering the loading area through successive uses of this packet, unless it is replaced in the meantime by a different chunk in the same "slot".

The vanilla client ignores attempts to load or unload chunks located outside the loading area. This applies even to unloads targeting chunks that are still loaded, but currently located outside the loading area (per the previous paragraph).

The vanilla server does not rely on any specific behavior for chunks leaving the loading area, and custom clients need not replicate the above exactly. A client may instead choose to immediately unload any chunks outside the loading area, to use a different modulus, or to ignore the loading area completely and keep chunks loaded regardless of their location until the server requests to unload them. Servers aiming for maximal interoperability should always explicitly unload any loaded chunks before they go outside the loading area.

The center chunk is normally the chunk the player is in, but apart from the implications on chunk loading, the (vanilla) client takes no issue with this not being the case. Indeed, as long as chunks are sent only within the default loading area centered on the world origin, it is not necessary to send this packet at all. This may be useful for servers with small bounded worlds, such as minigames, since it ensures chunks never need to be resent after the client has joined, saving on bandwidth.

The vanilla server sends this packet whenever the player moves across a chunk border horizontally, and also (according to testing) for any integer change in the vertical axis, even if it doesn't go across a chunk section border.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x57`<br>_resource:_<br>`set_chunk_cache_center` | Play | Client | Chunk X | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Chunk X coordinate of the loading area center. |
| Chunk Z | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Chunk Z coordinate of the loading area center. |

#### Set Render Distance\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=154&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Set Render Distance") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=154&editintro=Template:License_wiki.vg "Edit section's source code: Set Render Distance")\]

Sent by the integrated singleplayer server when changing render distance. This packet is sent by the server when the client reappears in the overworld after leaving the end.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x58`<br>_resource:_<br>`set_chunk_cache_radius` | Play | Client | View Distance | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Render distance (2-32). |

#### Set Cursor Item\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=155&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Set Cursor Item") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=155&editintro=Template:License_wiki.vg "Edit section's source code: Set Cursor Item")\]

Replaces or sets the inventory item that's being dragged with the mouse.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x59`<br>_resource:_<br>`set_cursor_item` | Play | Client | Carried item | [Slot](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Slot) |  |

#### Set Default Spawn Position\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=156&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Set Default Spawn Position") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=156&editintro=Template:License_wiki.vg "Edit section's source code: Set Default Spawn Position")\]

Sent by the server after login to specify the coordinates of the spawn point (the point at which players spawn at, and which the compass points to). It can be sent at any time to update the point compasses point at.

Before receiving this packet, the client uses the default position 8, 64, 8, and angle 0.0.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x5A`<br>_resource:_<br>`set_default_spawn_position` | Play | Client | Location | [Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Position) | Spawn location. |
| Angle | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | The angle at which to respawn. |

#### Display Objective\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=157&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Display Objective") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=157&editintro=Template:License_wiki.vg "Edit section's source code: Display Objective")\]

This is sent to the client when it should display a scoreboard.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x5B`<br>_resource:_<br>`set_display_objective` | Play | Client | Position | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | The position of the scoreboard. 0: list, 1: sidebar, 2: below name, 3 - 18: team-specific sidebar, indexed as 3 + team color. |
| Score Name | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (32767) | The unique name for the scoreboard to be displayed. |

#### Set Entity Metadata\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=158&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Set Entity Metadata") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=158&editintro=Template:License_wiki.vg "Edit section's source code: Set Entity Metadata")\]

Updates one or more [metadata](https://minecraft.wiki/w/Java_Edition_protocol/Entity_metadata#Entity_Metadata_Format "Java Edition protocol/Entity metadata") properties for an existing entity. Any properties not included in the Metadata field are left unchanged.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x5C`<br>_resource:_<br>`set_entity_data` | Play | Client | Entity ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |
| Metadata | [Entity Metadata](https://minecraft.wiki/w/Java_Edition_protocol/Entity_metadata#Entity_Metadata_Format "Java Edition protocol/Entity metadata") |  |

#### Link Entities\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=159&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Link Entities") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=159&editintro=Template:License_wiki.vg "Edit section's source code: Link Entities")\]

This packet is sent when an entity has been [leashed](https://minecraft.wiki/w/Lead "Lead") to another entity.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x5D`<br>_resource:_<br>`set_entity_link` | Play | Client | Attached Entity ID | [Int](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Int) | Attached entity's EID. |
| Holding Entity ID | [Int](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Int) | ID of the entity holding the lead. Set to -1 to detach. |

#### Set Entity Velocity\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=160&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Set Entity Velocity") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=160&editintro=Template:License_wiki.vg "Edit section's source code: Set Entity Velocity")\]

Velocity is in units of 1/8000 of a block per server tick (50ms); for example, -1343 would move (-1343 / 8000) = −0.167875 blocks per tick (or −3.3575 blocks per second).

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x5E`<br>_resource:_<br>`set_entity_motion` | Play | Client | Entity ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |
| Velocity X | [Short](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Short) | Velocity on the X axis. |
| Velocity Y | [Short](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Short) | Velocity on the Y axis. |
| Velocity Z | [Short](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Short) | Velocity on the Z axis. |

#### Set Equipment\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=161&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Set Equipment") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=161&editintro=Template:License_wiki.vg "Edit section's source code: Set Equipment")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x5F`<br>_resource:_<br>`set_equipment` | Play | Client | Entity ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Entity's ID. |
| Equipment | Slot | [Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Array) | [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | The length of the array is not known beforehand and has to be determined by reading all entries, as the most significant bit of the slot indicates if there is a next entry. | Equipment slot (see below). Also has the top bit set if another entry follows, and otherwise unset if this is the last item in the array. |
| Item | [Slot](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Slot) |  |

Equipment slot can be one of the following:

| ID | Equipment slot |
| --- | --- |
| 0 | Main hand |
| 1 | Off hand |
| 2 | Boots |
| 3 | Leggings |
| 4 | Chestplate |
| 5 | Helmet |
| 6 | Body |
| 7 | Saddle |

#### Set Experience\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=162&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Set Experience") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=162&editintro=Template:License_wiki.vg "Edit section's source code: Set Experience")\]

Sent by the server when the client should change experience levels.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x60`<br>_resource:_<br>`set_experience` | Play | Client | Experience bar | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | Between 0 and 1. |
| Level | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |
| Total Experience | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | See [Experience#Leveling up](https://minecraft.wiki/w/Experience#Leveling_up "Experience") on the Minecraft Wiki for Total Experience to Level conversion. |

#### Set Health\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=163&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Set Health") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=163&editintro=Template:License_wiki.vg "Edit section's source code: Set Health")\]

Sent by the server to set the health of the player it is sent to.

Food [saturation](https://minecraft.wiki/w/Food#Hunger_and_saturation "Food") acts as a food “overcharge”. Food values will not decrease while the saturation is over zero. New players logging in or respawning automatically get a saturation of 5.0. Eating food increases the saturation as well as the food bar.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x61`<br>_resource:_<br>`set_health` | Play | Client | Health | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | 0 or less = dead, 20 = full HP. |
| Food | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | 0–20. |
| Food Saturation | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | Seems to vary from 0.0 to 5.0 in integer increments. |

#### Set Held Item (clientbound)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=164&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Set Held Item (clientbound)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=164&editintro=Template:License_wiki.vg "Edit section's source code: Set Held Item (clientbound)")\]

Sent to change the player's slot selection.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x62`<br>_resource:_<br>`set_held_slot` | Play | Client | Slot | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | The slot which the player has selected (0–8). |

#### Update Objectives\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=165&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Update Objectives") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=165&editintro=Template:License_wiki.vg "Edit section's source code: Update Objectives")\]

This is sent to the client when it should create a new [scoreboard](https://minecraft.wiki/w/Scoreboard "Scoreboard") objective or remove one.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x63`<br>_resource:_<br>`set_objective` | Play | Client | Objective Name | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (32767) | A unique name for the objective. |
| Mode | [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | 0 to create the scoreboard. 1 to remove the scoreboard. 2 to update the display text. |
| Objective Value | [Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Optional) [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) | Only if mode is 0 or 2.The text to be displayed for the score. |
| Type | [Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Optional) [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | Only if mode is 0 or 2. 0 = "integer", 1 = "hearts". |
| Has Number Format | [Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Optional) [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | Only if mode is 0 or 2. Whether this objective has a set number format for the scores. |
| Number Format | [Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Optional) [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | Only if mode is 0 or 2 and the previous boolean is true. Determines how the score number should be formatted. |
| Number Format | Field Name |  |  |
| 0: blank | _no fields_ | Show nothing. |
| 1: styled | Styling | [Compound Tag](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/NBT#Specification:compound_tag "Minecraft Wiki:Projects/wiki.vg merge/NBT") | The styling to be used when formatting the score number. Contains the [text component styling fields](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Text_formatting#Styling_fields "Minecraft Wiki:Projects/wiki.vg merge/Text formatting"). |
| 2: fixed | Content | [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) | The text to be used as placeholder. |

#### Set Passengers\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=166&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Set Passengers") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=166&editintro=Template:License_wiki.vg "Edit section's source code: Set Passengers")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x64`<br>_resource:_<br>`set_passengers` | Play | Client | Entity ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Vehicle's EID. |
| Passengers | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | EIDs of entity's passengers. |

#### Set Player Inventory Slot\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=167&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Set Player Inventory Slot") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=167&editintro=Template:License_wiki.vg "Edit section's source code: Set Player Inventory Slot")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x65`<br>_resource:_<br>`set_player_inventory` | Play | Client | Slot | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |
| Slot Data | [Slot](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Slot) |  |

#### Update Teams\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=168&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Update Teams") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=168&editintro=Template:License_wiki.vg "Edit section's source code: Update Teams")\]

Creates and updates teams.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x66`<br>_resource:_<br>`set_player_team` | Play | Client | Team Name | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (32767) | A unique name for the team. (Shared with scoreboard). |
| Method | [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | Determines the layout of the remaining packet. |
| 0: create team | Team Display Name | [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) |  |
| Friendly Flags | [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | Bit mask. 0x01: Allow friendly fire, 0x02: can see invisible players on the same team. |
| Name Tag Visibility | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | 0 = ALWAYS, 1 = NEVER, 2 = HIDE\_FOR\_OTHER\_TEAMS, 3 = HIDE\_FOR\_OWN\_TEAMS |
| Collision Rule | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | 0 = ALWAYS, 1 = NEVER, 2 = PUSH\_OTHER\_TEAMS, 3 = PUSH\_OWN\_TEAM |
| Team Color | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | Used to color the names of players on the team; see below. |
| Team Prefix | [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) | Displayed before the names of players that are part of this team. |
| Team Suffix | [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) | Displayed after the names of players that are part of this team. |
| Entities | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (32767) | Identifiers for the entities in this team. For players, this is their username; for other entities, it is their UUID. |
| 1: remove team | _no fields_ | _no fields_ |  |
| 2: update team info | Team Display Name | [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) |  |
| Friendly Flags | [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | Bit mask. 0x01: Allow friendly fire, 0x02: can see invisible entities on the same team. |
| Name Tag Visibility | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | 0 = ALWAYS, 1 = NEVER, 2 = HIDE\_FOR\_OTHER\_TEAMS, 3 = HIDE\_FOR\_OWN\_TEAMS |
| Collision Rule | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | 0 = ALWAYS, 1 = NEVER, 2 = PUSH\_OTHER\_TEAMS, 3 = PUSH\_OWN\_TEAM |
| Team Color | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | Used to color the names of players on the team; see below. |
| Team Prefix | [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) | Displayed before the names of players that are part of this team. |
| Team Suffix | [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) | Displayed after the names of players that are part of this team. |
| 3: add entities to team | Entities | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (32767) | Identifiers for the added entities. For players, this is their username; for other entities, it is their UUID. |
| 4: remove entities from team | Entities | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (32767) | Identifiers for the removed entities. For players, this is their username; for other entities, it is their UUID. |

Team Color: The color of a team defines how the names of the team members are visualized; any formatting code can be used. The following table lists all the possible values.

| ID | Formatting |
| --- | --- |
| 0-15 | Color formatting, same values as in [Text formatting#Colors](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Text_formatting#Colors "Minecraft Wiki:Projects/wiki.vg merge/Text formatting"). |
| 16 | Obfuscated |
| 17 | Bold |
| 18 | Strikethrough |
| 19 | Underlined |
| 20 | Italic |
| 21 | Reset |

#### Update Score\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=169&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Update Score") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=169&editintro=Template:License_wiki.vg "Edit section's source code: Update Score")\]

This is sent to the client when it should update a scoreboard item.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x67`<br>_resource:_<br>`set_score` | Play | Client | Entity Name | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (32767) | The entity whose score this is. For players, this is their username; for other entities, it is their UUID. |
| Objective Name | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (32767) | The name of the objective the score belongs to. |
| Value | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | The score to be displayed next to the entry. |
| Display Name | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) | The custom display name. |
| Number Format | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | Determines how the score number should be formatted. |
| Number Format | Field Name |  |  |
| 0: blank | _no fields_ | Show nothing. |
| 1: styled | Styling | [Compound Tag](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/NBT#Specification:compound_tag "Minecraft Wiki:Projects/wiki.vg merge/NBT") | The styling to be used when formatting the score number. Contains the [text component styling fields](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Text_formatting#Styling_fields "Minecraft Wiki:Projects/wiki.vg merge/Text formatting"). |
| 2: fixed | Content | [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) | The text to be used as placeholder. |

#### Set Simulation Distance\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=170&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Set Simulation Distance") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=170&editintro=Template:License_wiki.vg "Edit section's source code: Set Simulation Distance")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x68`<br>_resource:_<br>`set_simulation_distance` | Play | Client | Simulation Distance | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | The distance that the client will process specific things, such as entities. |

#### Set Subtitle Text\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=171&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Set Subtitle Text") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=171&editintro=Template:License_wiki.vg "Edit section's source code: Set Subtitle Text")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x69`<br>_resource:_<br>`set_subtitle_text` | Play | Client | Subtitle Text | [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) |  |

#### Update Time\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=172&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Update Time") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=172&editintro=Template:License_wiki.vg "Edit section's source code: Update Time")\]

Time is based on ticks, where 20 ticks happen every second. There are 24000 ticks in a day, making Minecraft days exactly 20 minutes long.

The time of day is based on the timestamp modulo 24000. 0 is sunrise, 6000 is noon, 12000 is sunset, and 18000 is midnight.

The default SMP server increments the time by `20` every second.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x6A`<br>_resource:_<br>`set_time` | Play | Client | World Age | [Long](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Long) | In ticks; not changed by server commands. |
| Time of day | [Long](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Long) | The world (or region) time, in ticks. |
| Time of day increasing | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | If true, the client should automatically advance the time of day according to its ticking rate. |

#### Set Title Text\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=173&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Set Title Text") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=173&editintro=Template:License_wiki.vg "Edit section's source code: Set Title Text")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x6B`<br>_resource:_<br>`set_title_text` | Play | Client | Title Text | [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) |  |

#### Set Title Animation Times\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=174&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Set Title Animation Times") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=174&editintro=Template:License_wiki.vg "Edit section's source code: Set Title Animation Times")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x6C`<br>_resource:_<br>`set_titles_animation` | Play | Client | Fade In | [Int](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Int) | Ticks to spend fading in. |
| Stay | [Int](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Int) | Ticks to keep the title displayed. |
| Fade Out | [Int](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Int) | Ticks to spend fading out, not when to start fading out. |

#### Entity Sound Effect\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=175&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Entity Sound Effect") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=175&editintro=Template:License_wiki.vg "Edit section's source code: Entity Sound Effect")\]

Plays a sound effect from an entity, either by hardcoded ID or Identifier. Sound IDs and names can be found [here](https://pokechu22.github.io/Burger/1.21.html#sounds).

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x6D`<br>_resource:_<br>`sound_entity` | Play | Client | Sound Event | [ID or](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:ID_or) [Sound Event](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Sound_Event) | ID in the `minecraft:sound_event` registry, or an inline definition. |
| Sound Category | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | The category that this sound will be played from ( [current categories](https://gist.github.com/konwboj/7c0c380d3923443e9d55)). |
| Entity ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |
| Volume | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | 1.0 is 100%, capped between 0.0 and 1.0 by vanilla clients. |
| Pitch | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | Float between 0.5 and 2.0 by vanilla clients. |
| Seed | [Long](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Long) | Seed used to pick sound variant. |

#### Sound Effect\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=176&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Sound Effect") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=176&editintro=Template:License_wiki.vg "Edit section's source code: Sound Effect")\]

Plays a sound effect at the given location, either by hardcoded ID or Identifier. Sound IDs and names can be found [here](https://pokechu22.github.io/Burger/1.21.html#sounds).

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x6E`<br>_resource:_<br>`sound` | Play | Client | Sound Event | [ID or](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:ID_or) [Sound Event](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Sound_Event) | ID in the `minecraft:sound_event` registry, or an inline definition. |
| Sound Category | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | The category that this sound will be played from ( [current categories](https://gist.github.com/konwboj/7c0c380d3923443e9d55)). |
| Effect Position X | [Int](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Int) | Effect X multiplied by 8 ( [fixed-point number](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Data_types#Fixed-point_numbers "Minecraft Wiki:Projects/wiki.vg merge/Data types") with only 3 bits dedicated to the fractional part). |
| Effect Position Y | [Int](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Int) | Effect Y multiplied by 8 ( [fixed-point number](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Data_types#Fixed-point_numbers "Minecraft Wiki:Projects/wiki.vg merge/Data types") with only 3 bits dedicated to the fractional part). |
| Effect Position Z | [Int](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Int) | Effect Z multiplied by 8 ( [fixed-point number](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Data_types#Fixed-point_numbers "Minecraft Wiki:Projects/wiki.vg merge/Data types") with only 3 bits dedicated to the fractional part). |
| Volume | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | 1.0 is 100%, capped between 0.0 and 1.0 by vanilla clients. |
| Pitch | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | Float between 0.5 and 2.0 by vanilla clients. |
| Seed | [Long](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Long) | Seed used to pick sound variant. |

#### Start Configuration\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=177&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Start Configuration") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=177&editintro=Template:License_wiki.vg "Edit section's source code: Start Configuration")\]

Sent during gameplay in order to redo the configuration process. The client must respond with [Acknowledge Configuration](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Acknowledge_Configuration) for the process to start.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x6F`<br>_resource:_<br>`start_configuration` | Play | Client | _no fields_ |

This packet switches the connection state to [configuration](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Configuration).

#### Stop Sound\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=178&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Stop Sound") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=178&editintro=Template:License_wiki.vg "Edit section's source code: Stop Sound")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x70`<br>_resource:_<br>`stop_sound` | Play | Client | Flags | [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | Controls which fields are present. |
| Source | [Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Optional) [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | Only if flags is 3 or 1 (bit mask 0x1). See below. If not present, then sounds from all sources are cleared. |
| Sound | [Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Optional) [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) | Only if flags is 2 or 3 (bit mask 0x2). A sound effect name, see [Custom Sound Effect](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Custom_Sound_Effect). If not present, then all sounds are cleared. |

Categories:

| Name | Value |
| --- | --- |
| master | 0 |
| music | 1 |
| record | 2 |
| weather | 3 |
| block | 4 |
| hostile | 5 |
| neutral | 6 |
| player | 7 |
| ambient | 8 |
| voice | 9 |

#### Store Cookie (play)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=179&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Store Cookie (play)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=179&editintro=Template:License_wiki.vg "Edit section's source code: Store Cookie (play)")\]

Stores some arbitrary data on the client, which persists between server transfers. The vanilla client only accepts cookies of up to 5 kiB in size.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x71`<br>_resource:_<br>`store_cookie` | Play | Client | Key | [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) | The identifier of the cookie. |
| Payload | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) (5120) of [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | The data of the cookie. |

#### System Chat Message\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=180&veaction=edit&editintro=Template:License_wiki.vg "Edit section: System Chat Message") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=180&editintro=Template:License_wiki.vg "Edit section's source code: System Chat Message")\]

Main page: [Minecraft\_Wiki:Projects/wiki.vg\_merge/Chat](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Chat "Minecraft Wiki:Projects/wiki.vg merge/Chat")

Sends the client a raw system message.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x72`<br>_resource:_<br>`system_chat` | Play | Client | Content | [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) | Limited to 262144 bytes. |
| Overlay | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | Whether the message is an actionbar or chat message. See also [#Set Action Bar Text](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Action_Bar_Text). |

#### Set Tab List Header And Footer\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=181&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Set Tab List Header And Footer") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=181&editintro=Template:License_wiki.vg "Edit section's source code: Set Tab List Header And Footer")\]

This packet may be used by custom servers to display additional information above/below the player list. It is never sent by the vanilla server.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x73`<br>_resource:_<br>`tab_list` | Play | Client | Header | [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) | To remove the header, send an empty text component: `{"text":""}`. |
| Footer | [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) | To remove the footer, send an empty text component: `{"text":""}`. |

#### Tag Query Response\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=182&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Tag Query Response") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=182&editintro=Template:License_wiki.vg "Edit section's source code: Tag Query Response")\]

Sent in response to [Query Block Entity Tag](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Query_Block_Entity_Tag) or [Query Entity Tag](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Query_Entity_Tag).

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x74`<br>_resource:_<br>`tag_query` | Play | Client | Transaction ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Can be compared to the one sent in the original query packet. |
| NBT | [NBT](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:NBT) | The NBT of the block or entity. May be a TAG\_END (0), in which case no NBT is present. |

#### Pickup Item\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=183&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Pickup Item") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=183&editintro=Template:License_wiki.vg "Edit section's source code: Pickup Item")\]

Sent by the server when someone picks up an item lying on the ground — its sole purpose appears to be the animation of the item flying towards you. It doesn't destroy the entity in the client memory, and it doesn't add it to your inventory. The server only checks for items to be picked up after each [Set Player Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Player_Position) (and [Set Player Position And Rotation](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Player_Position_And_Rotation)) packet sent by the client. The collector entity can be any entity; it does not have to be a player. The collected entity can also be any entity, but the vanilla server only uses this for items, experience orbs, and the different varieties of arrows.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x75`<br>_resource:_<br>`take_item_entity` | Play | Client | Collected Entity ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |
| Collector Entity ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |
| Pickup Item Count | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Seems to be 1 for XP orbs, otherwise the number of items in the stack. |

#### Synchronize Vehicle Position\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=184&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Synchronize Vehicle Position") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=184&editintro=Template:License_wiki.vg "Edit section's source code: Synchronize Vehicle Position")\]

Teleports the entity on the client without changing the reference point of movement deltas in future [Update Entity Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Update_Entity_Position) packets. Seems to be used to make relative adjustments to vehicle positions; more information needed.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x76`<br>_resource:_<br>`teleport_entity` | Play | Client | Entity ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |
| X | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) |  |
| Y | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) |  |
| Z | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) |  |
| Velocity X | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) |  |
| Velocity Y | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) |  |
| Velocity Z | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) |  |
| Yaw | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | Rotation on the Y axis, in degrees. |
| Pitch | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | Rotation on the Y axis, in degrees. |
| Flags | [Teleport Flags](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Teleport_Flags) |  |
| On Ground | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) |  |

#### Test Instance Block Status\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=185&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Test Instance Block Status") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=185&editintro=Template:License_wiki.vg "Edit section's source code: Test Instance Block Status")\]

Updates the status of the currently open [Test Instance Block](https://minecraft.wiki/w/Test_Instance_Block "Test Instance Block") screen, if any.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x77`<br>_resource:_<br>`test_instance_block_status` | Play | Client | Status | [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) |  |
| Has Size | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) |  |
| Size X | [Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Optional) [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) | Only present if Has Size is true. |
| Size Y | [Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Optional) [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) | Only present if Has Size is true. |
| Size Z | [Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Optional) [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) | Only present if Has Size is true. |

#### Set Ticking State\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=186&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Set Ticking State") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=186&editintro=Template:License_wiki.vg "Edit section's source code: Set Ticking State")\]

Used to adjust the ticking rate of the client, and whether it's frozen.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x78`<br>_resource:_<br>`ticking_state` | Play | Client | Tick rate | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) |  |
| Is frozen | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) |  |

#### Step Tick\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=187&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Step Tick") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=187&editintro=Template:License_wiki.vg "Edit section's source code: Step Tick")\]

Advances the client processing by the specified number of ticks. Has no effect unless client ticking is frozen.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x79`<br>_resource:_<br>`ticking_step` | Play | Client | Tick steps | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |

#### Transfer (play)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=188&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Transfer (play)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=188&editintro=Template:License_wiki.vg "Edit section's source code: Transfer (play)")\]

Notifies the client that it should transfer to the given server. Cookies previously stored are preserved between server transfers.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x7A`<br>_resource:_<br>`transfer` | Play | Client | Host | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) | The hostname or IP of the server. |
| Port | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | The port of the server. |

#### Update Advancements\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=189&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Update Advancements") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=189&editintro=Template:License_wiki.vg "Edit section's source code: Update Advancements")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x7B`<br>_resource:_<br>`update_advancements` | Play | Client | Reset/Clear | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | Whether to reset/clear the current advancements. |
| Advancement mapping | Key | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) | [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) | The identifier of the advancement. |
| Value | Advancement | See below |
| Identifiers | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) | The identifiers of the advancements that should be removed. |
| Progress mapping | Key | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) | [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) | The identifier of the advancement. |
| Value | Advancement progress | See below. |
| Show advancements | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) |  |

Advancement structure:

| Field Name | Field Type | Notes |
| --- | --- | --- |
| Parent id | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) | The identifier of the parent advancement. |
| Display data | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) Advancement display | See below. |
| Nested requirements | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (32767) | Array with a sub-array of criteria. To check if the requirements are met, each sub-array must be tested and mapped with the OR operator, resulting in a boolean array.<br>These booleans must be mapped with the AND operator to get the result. |
| Sends telemetry data | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | Whether the client should include this achievement in the telemetry data when it's completed.<br>The vanilla client only sends data for advancements on the `minecraft` namespace. |

Advancement display:

| Field Name | Field Type | Notes |
| --- | --- | --- |
| Title | [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) |  |
| Description | [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) |  |
| Icon | [Slot](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Slot) |  |
| Frame type | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | 0 = `task`, 1 = `challenge`, 2 = `goal`. |
| Flags | [Int](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Int) | 0x01: has background texture; 0x02: `show_toast`; 0x04: `hidden`. |
| Background texture | [Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Optional) [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) | Background texture location. Only if flags indicates it. |
| X coord | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) |  |
| Y coord | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) |  |

Advancement progress:

| Field Name | Field Type | Notes |
| --- | --- | --- |
| Criteria | Criterion identifier | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) | [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) | The identifier of the criterion. |
| Date of achieving | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) [Long](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Long) | Present if achieved. As returned by [`Date.getTime`](https://docs.oracle.com/javase/6/docs/api/java/util/Date.html#getTime()). |

#### Update Attributes\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=190&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Update Attributes") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=190&editintro=Template:License_wiki.vg "Edit section's source code: Update Attributes")\]

Sets [attributes](https://minecraft.wiki/w/Attribute "Attribute") on the given entity.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x7C`<br>_resource:_<br>`update_attributes` | Play | Client | Entity ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |
| Property | Id | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | ID in the `minecraft:attribute` registry. See also [Attribute#Attributes](https://minecraft.wiki/w/Attribute#Attributes "Attribute"). |
| Value | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) | See below. |
| Modifiers | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of Modifier Data | See [Attribute#Modifiers](https://minecraft.wiki/w/Attribute#Modifiers "Attribute"). Modifier Data defined below. |

_Modifier Data_ structure:

| Field Name | Field Type | Notes |
| --- | --- | --- |
| Id | [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) |  |
| Amount | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) | May be positive or negative. |
| Operation | [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | See below. |

The operation controls how the base value of the modifier is changed.

- 0: Add/subtract amount
- 1: Add/subtract amount percent of the current value
- 2: Multiply by amount percent

All of the 0's are applied first, and then the 1's, and then the 2's.

#### Entity Effect\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=191&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Entity Effect") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=191&editintro=Template:License_wiki.vg "Edit section's source code: Entity Effect")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x7D`<br>_resource:_<br>`update_mob_effect` | Play | Client | Entity ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |
| Effect ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | See [this table](https://minecraft.wiki/w/Status_effect#Effect_list "Status effect"). |
| Amplifier | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Vanilla client displays effect level as Amplifier + 1. |
| Duration | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Duration in ticks. (-1 for infinite) |
| Flags | [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | Bit field, see below. |

[![](https://minecraft.wiki/images/thumb/Info.png/16px-Info.png?24414)](https://minecraft.wiki/w/File:Info.png)

This section is missing information about: What exact effect does the blend bit flag have on the client? What happens if it is used on effects besides DARKNESS?

Please expand the section to include this information. Further details may exist on the [talk page](https://minecraft.wiki/w/Talk:Java_Edition_protocol/Packets).

Within flags:

- 0x01: Is ambient - was the effect spawned from a beacon? All beacon-generated effects are ambient. Ambient effects use a different icon in the HUD (blue border rather than gray). If all effects on an entity are ambient, the ["Is potion effect ambient" living metadata field](https://minecraft.wiki/w/Java_Edition_protocol/Entity_metadata#Living_Entity "Java Edition protocol/Entity metadata") should be set to true. Usually should not be enabled.
- 0x02: Show particles - should all particles from this effect be hidden? Effects with particles hidden are not included in the calculation of the effect color, and are not rendered on the HUD (but are still rendered within the inventory). Usually should be enabled.
- 0x04: Show icon - should the icon be displayed on the client? Usually should be enabled.
- 0x08: Blend - should the effect's hard-coded blending be applied? Currently only used in the DARKNESS effect to apply extra void fog and adjust the gamma value for lighting.

#### Update Recipes\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=192&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Update Recipes") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=192&editintro=Template:License_wiki.vg "Edit section's source code: Update Recipes")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x7E`<br>_resource:_<br>`update_recipes` | Play | Client | Property Sets | Property Set ID | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) | [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) |  |
| Items | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | IDs in the `minecraft:item` registry. |
| Stonecutter Recipes | Ingredients | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) | [ID Set](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:ID_Set) |  |
| Slot Display | [Slot Display](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Slot_Display) |  |

#### Update Tags (play)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=193&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Update Tags (play)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=193&editintro=Template:License_wiki.vg "Edit section's source code: Update Tags (play)")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x7F`<br>_resource:_<br>`update_tags` | Play | Client | Registry to tags map | Registry | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) | [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) | Registry identifier (Vanilla expects tags for the registries `minecraft:block`, `minecraft:item`, `minecraft:fluid`, `minecraft:entity_type`, and `minecraft:game_event`) |
| Tags | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of Tag (See below) |  |

A tag looks like this:

| Field Name | Field Type | Notes |
| --- | --- | --- |
| Tag name | [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) |  |
| Entries | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) of [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Numeric IDs of the given type (block, item, etc.). This list replaces the previous list of IDs for the given tag. If some preexisting tags are left unmentioned, a warning is printed. |

See [Tag](https://minecraft.wiki/w/Tag "Tag") on the Minecraft Wiki for more information, including a list of vanilla tags.

#### Projectile Power\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=194&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Projectile Power") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=194&editintro=Template:License_wiki.vg "Edit section's source code: Projectile Power")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x80`<br>_resource:_<br>`projectile_power` | Play | Client | Entity ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |
| Power | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) |  |

#### Custom Report Details\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=195&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Custom Report Details") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=195&editintro=Template:License_wiki.vg "Edit section's source code: Custom Report Details")\]

Contains a list of key-value text entries that are included in any crash or disconnection report generated during connection to the server.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x81`<br>_resource:_<br>`custom_report_details` | Play | Client | Details | Title | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) (32) | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (128) |  |
| Description | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (4096) |  |

#### Server Links\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=196&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Server Links") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=196&editintro=Template:License_wiki.vg "Edit section's source code: Server Links")\]

This packet contains a list of links that the vanilla client will display in the menu available from the pause menu. Link labels can be built-in or custom (i.e., any text).

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x82`<br>_resource:_<br>`server_links` | Play | Client | Links | Is built-in | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | Determines if the following label is built-in (from enum) or custom (text component). |
| Label | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) / [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) | See below. |
| URL | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) | Valid URL. |

| ID | Name | Notes |
| --- | --- | --- |
| 0 | Bug Report | Displayed on connection error screen; included as a comment in the disconnection report. |
| 1 | Community Guidelines |  |
| 2 | Support |  |
| 3 | Status |  |
| 4 | Feedback |  |
| 5 | Community |  |
| 6 | Website |  |
| 7 | Forums |  |
| 8 | News |  |
| 9 | Announcements |  |

#### Waypoint\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=197&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Waypoint") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=197&editintro=Template:License_wiki.vg "Edit section's source code: Waypoint")\]

Adds, removes, or updates an entry that will be tracked on the player locator bar.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x83`<br>_resource:_<br>`waypoint` | Play | Client | Operation | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | 0: track, 1: untrack, 2: update. |
| Identifier | [Uuid](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Uuid) [or](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:or) [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) | Something that uniquely identifies this specific waypoint. |
| Icon style | [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) | Path to the waypoint style JSON: `assets/<namespace>/waypoint_style/<value>.json.` |
| Color |
| Red | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) | [Unsigned Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Unsigned_Byte) | The color of the waypoint icon. |
| Green | [Unsigned Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Unsigned_Byte) |
| Blue | [Unsigned Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Unsigned_Byte) |
| Waypoint type | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | Defines how the following field is read. |
| Waypoint data | Varies | | Type | Field Name | Field Data | Notes |
| --- | --- | --- | --- |
| 0: Empty | _no fields_ |
| 1: Vec3i | X | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | The position that the waypoint will point to. |
| Y | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |
| Z | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |
| 2: Chunk | X | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | The chunk coordinates that the waypoint will point to. |
| Z | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |
| 3: Azimuth | Angle | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | The angle that will be pointed to, in radians. | |

#### Clear Dialog (play)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=198&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Clear Dialog (play)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=198&editintro=Template:License_wiki.vg "Edit section's source code: Clear Dialog (play)")\]

If we're currently in a dialog screen, then this removes the current screen and switches back to the previous one.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x84`<br>_resource:_<br>`clear_dialog` | Play | Client | _no fields_ |

#### Show Dialog (play)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=199&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Show Dialog (play)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=199&editintro=Template:License_wiki.vg "Edit section's source code: Show Dialog (play)")\]

Show a custom dialog screen to the client.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x85`<br>_resource:_<br>`show_dialog` | Play | Client | Dialog | [ID or](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:ID_or) [NBT](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:NBT) | ID in the `minecraft:dialog` registry, or an inline definition as described at [Registry\_data#Dialog](https://minecraft.wiki/w/Java_Edition_protocol/Registry_data#Dialog "Java Edition protocol/Registry data"). |

### Serverbound\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=200&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Serverbound") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=200&editintro=Template:License_wiki.vg "Edit section's source code: Serverbound")\]

#### Confirm Teleportation\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=201&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Confirm Teleportation") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=201&editintro=Template:License_wiki.vg "Edit section's source code: Confirm Teleportation")\]

Sent by client as confirmation of [Synchronize Player Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Synchronize_Player_Position).

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x00`<br>_resource:_<br>`accept_teleportation` | Play | Server | Teleport ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | The ID given by the [Synchronize Player Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Synchronize_Player_Position) packet. |

#### Query Block Entity Tag\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=202&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Query Block Entity Tag") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=202&editintro=Template:License_wiki.vg "Edit section's source code: Query Block Entity Tag")\]

Used when `F3` + `I` is pressed while looking at a block.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x01`<br>_resource:_<br>`block_entity_tag_query` | Play | Server | Transaction ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | An incremental ID so that the client can verify that the response matches. |
| Location | [Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Position) | The location of the block to check. |

#### Bundle Item Selected\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=203&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Bundle Item Selected") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=203&editintro=Template:License_wiki.vg "Edit section's source code: Bundle Item Selected")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x02`<br>_resource:_<br>`bundle_item_selected` | Play | Server | Slot of Bundle | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |
| Slot in Bundle | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |

#### Change Difficulty\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=204&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Change Difficulty") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=204&editintro=Template:License_wiki.vg "Edit section's source code: Change Difficulty")\]

Must have at least op level 2 to use. Appears to only be used on singleplayer; the difficulty buttons are still disabled in multiplayer.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x03`<br>_resource:_<br>`change_difficulty` | Play | Server | New difficulty | [Unsigned Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Unsigned_Byte) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | 0: peaceful, 1: easy, 2: normal, 3: hard. |

#### Change Game Mode\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=205&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Change Game Mode") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=205&editintro=Template:License_wiki.vg "Edit section's source code: Change Game Mode")\]

Requests for the server to update our game mode. Has no effect on vanilla servers if the client doesn't have the required permissions.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x04`<br>_resource:_<br>`change_game_mode` | Play | Server | Game mode | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | 0: survival, 1: creative, 2: adventure, 3: spectator. |

#### Acknowledge Message\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=206&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Acknowledge Message") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=206&editintro=Template:License_wiki.vg "Edit section's source code: Acknowledge Message")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x05`<br>_resource:_<br>`chat_ack` | Play | Server | Message Count | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |

#### Chat Command\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=207&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Chat Command") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=207&editintro=Template:License_wiki.vg "Edit section's source code: Chat Command")\]

Main page: [Minecraft\_Wiki:Projects/wiki.vg\_merge/Chat](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Chat "Minecraft Wiki:Projects/wiki.vg merge/Chat")

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x06`<br>_resource:_<br>`chat_command` | Play | Server | Command | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (32767) | The command typed by the client excluding the `/`. |

#### Signed Chat Command\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=208&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Signed Chat Command") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=208&editintro=Template:License_wiki.vg "Edit section's source code: Signed Chat Command")\]

Main page: [Minecraft\_Wiki:Projects/wiki.vg\_merge/Chat](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Chat "Minecraft Wiki:Projects/wiki.vg merge/Chat")

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x07`<br>_resource:_<br>`chat_command_signed` | Play | Server | Command | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (32767) | The command typed by the client excluding the `/`. |
| Timestamp | [Long](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Long) | The timestamp that the command was executed. |
| Salt | [Long](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Long) | The salt for the following argument signatures. |
| Array of argument signatures | Argument name | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) (8) | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (16) | The name of the argument that is signed by the following signature. |
| Signature | [Byte Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte_Array) (256) | The signature that verifies the argument. Always 256 bytes and is not length-prefixed. |
| Message Count | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |
| Acknowledged | [Fixed BitSet](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Fixed_BitSet) (20) |  |
| Checksum | [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) |  |

#### Chat Message\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=209&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Chat Message") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=209&editintro=Template:License_wiki.vg "Edit section's source code: Chat Message")\]

Main page: [Minecraft\_Wiki:Projects/wiki.vg\_merge/Chat](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Chat "Minecraft Wiki:Projects/wiki.vg merge/Chat")

Used to send a chat message to the server. The message may not be longer than 256 characters or else the server will kick the client.

The server will broadcast a [Player Chat Message](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Player_Chat_Message) packet with Chat Type `minecraft:chat` to all players that haven't disabled chat (including the player that sent the message). See [Chat#Processing chat](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Chat#Processing_chat "Minecraft Wiki:Projects/wiki.vg merge/Chat") for more information.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x08`<br>_resource:_<br>`chat` | Play | Server | Message | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (256) | Content of the message |
| Timestamp | [Long](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Long) | Number of milliseconds since the epoch (1 Jan 1970, midnight, UTC) |
| Salt | [Long](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Long) | The salt used to verify the signature hash. Randomly generated by the client |
| Signature | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) [Byte Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte_Array) (256) | The signature used to verify the chat message's authentication. When present, always 256 bytes and not length-prefixed.<br>This is a SHA256 with RSA digital signature computed over the following:<br>- The number 1 as a 4-byte int. Always 00 00 00 01.<br>- The player's 16-byte UUID.<br>- The chat session (a 16-byte UUID randomly generated by the client).<br>- The index of the message within this chat session as a 4-byte int. First message is 0, next message is 1, etc. Incremented each time the client sends a chat message.<br>- The salt (from above) as an 8-byte long.<br>- The timestamp (from above) converted from milliseconds to seconds, so divide by 1000, as an 8-byte long.<br>- The length of the message in bytes (from above) as a 4-byte int.<br>- The message bytes.<br>- The number of messages in the last seen set, as a 4-byte int. Always in the range \[0,20\].<br>- For each message in the last seen set, from oldest to newest, the 256-byte signature of that message.<br>The client's chat private key is used for the message signature. |
| Message Count | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Number of signed clientbound chat messages the client has seen from the server since the last serverbound chat message from this client. The server will use this to update its last seen list for the client. |
| Acknowledged | [Fixed BitSet](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Fixed_BitSet) (20) | Bitmask of which message signatures from the last seen set were used to sign this message. The most recent is the highest bit. If there are fewer than 20 messages in the last seen set, the lower bits will be zeros. |
| Checksum | [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) |  |

#### Player Session\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=210&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Player Session") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=210&editintro=Template:License_wiki.vg "Edit section's source code: Player Session")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x09`<br>_resource:_<br>`chat_session_update` | Play | Server | Session Id | [UUID](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:UUID) |  |
| Public Key | Expires At | [Long](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Long) | The time the play session key expires in [epoch](https://en.wikipedia.org/wiki/Unix_time) milliseconds. |
| Public Key | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) (512) of [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | A byte array of an X.509-encoded public key. |
| Key Signature | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) (4096) of [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | The signature consists of the player UUID, the key expiration timestamp, and the public key data. These values are hashed using [SHA-1](https://en.wikipedia.org/wiki/SHA-1) and signed using Mojang's private [RSA](https://en.wikipedia.org/wiki/RSA_(cryptosystem)) key. |

#### Chunk Batch Received\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=211&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Chunk Batch Received") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=211&editintro=Template:License_wiki.vg "Edit section's source code: Chunk Batch Received")\]

Notifies the server that the chunk batch has been received by the client. The server uses the value sent in this packet to adjust the number of chunks to be sent in a batch.

The vanilla server will stop sending further chunk data until the client acknowledges the sent chunk batch. After the first acknowledgement, the server adjusts this number to allow up to 10 unacknowledged batches.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x0A`<br>_resource:_<br>`chunk_batch_received` | Play | Server | Chunks per tick | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | Desired chunks per tick. |

#### Client Status\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=212&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Client Status") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=212&editintro=Template:License_wiki.vg "Edit section's source code: Client Status")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x0B`<br>_resource:_<br>`client_command` | Play | Server | Action ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | See below |

_Action ID_ values:

| Action ID | Action | Notes |
| --- | --- | --- |
| 0 | Perform respawn | Sent when the client is ready to respawn after death. |
| 1 | Request stats | Sent when the client opens the Statistics menu. |

#### Client Tick End\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=213&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Client Tick End") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=213&editintro=Template:License_wiki.vg "Edit section's source code: Client Tick End")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x0C`<br>_resource:_<br>`client_tick_end` | Play | Server | _no fields_ |

#### Client Information (play)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=214&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Client Information (play)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=214&editintro=Template:License_wiki.vg "Edit section's source code: Client Information (play)")\]

Sent when the player connects, or when settings are changed.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x0D`<br>_resource:_<br>`client_information` | Play | Server | Locale | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (16) | e.g. `en_GB`. |
| View Distance | [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | Client-side render distance, in chunks. |
| Chat Mode | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | 0: enabled, 1: commands only, 2: hidden. See [Chat#Client chat mode](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Chat#Client_chat_mode "Minecraft Wiki:Projects/wiki.vg merge/Chat") for more information. |
| Chat Colors | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | “Colors” multiplayer setting. The vanilla server stores this value but does nothing with it (see [MC-64867](https://bugs.mojang.com/browse/MC-64867 "mojira:MC-64867")). Some third-party servers disable all coloring in chat and system messages when it is false. |
| Displayed Skin Parts | [Unsigned Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Unsigned_Byte) | Bit mask, see below. |
| Main Hand | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | 0: Left, 1: Right. |
| Enable text filtering | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | Enables filtering of text on signs and written book titles. The vanilla client sets this according to the `profanityFilterPreferences.profanityFilterOn` account attribute indicated by the [`/player/attributes` Mojang API endpoint](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Mojang_API#Player_Attributes "Minecraft Wiki:Projects/wiki.vg merge/Mojang API"). In offline mode, it is always false. |
| Allow server listings | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | Servers usually list online players; this option should let you not show up in that list. |
| Particle Status | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | 0: all, 1: decreased, 2: minimal |

_Displayed Skin Parts_ flags:

- Bit 0 (0x01): Cape enabled
- Bit 1 (0x02): Jacket enabled
- Bit 2 (0x04): Left Sleeve enabled
- Bit 3 (0x08): Right Sleeve enabled
- Bit 4 (0x10): Left Pants Leg enabled
- Bit 5 (0x20): Right Pants Leg enabled
- Bit 6 (0x40): Hat enabled

The most significant bit (bit 7, 0x80) appears to be unused.

#### Command Suggestions Request\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=215&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Command Suggestions Request") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=215&editintro=Template:License_wiki.vg "Edit section's source code: Command Suggestions Request")\]

Sent when the client needs to tab-complete a `minecraft:ask_server` suggestion type.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x0E`<br>_resource:_<br>`command_suggestion` | Play | Server | Transaction Id | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | The ID of the transaction that the server will send back to the client in the response of this packet. Client generates this and increments it each time it sends another tab completion that doesn't get a response. |
| Text | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (32500) | All the text behind the cursor including the `/` (e.g. to the left of the cursor in left-to-right languages like English). |

#### Acknowledge Configuration\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=216&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Acknowledge Configuration") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=216&editintro=Template:License_wiki.vg "Edit section's source code: Acknowledge Configuration")\]

Sent by the client upon receiving a [Start Configuration](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Start_Configuration) packet from the server.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x0F`<br>_resource:_<br>`configuration_acknowledged` | Play | Server | _no fields_ |

This packet switches the connection state to [configuration](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Configuration).

#### Click Container Button\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=217&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Click Container Button") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=217&editintro=Template:License_wiki.vg "Edit section's source code: Click Container Button")\]

Used when clicking on window buttons. Until 1.14, this was only used by enchantment tables.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x10`<br>_resource:_<br>`container_button_click` | Play | Server | Window ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | The ID of the window sent by [Open Screen](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Open_Screen). |
| Button ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Meaning depends on window type; see below. |

| Window type | ID | Meaning |
| --- | --- | --- |
| Enchantment Table | 0 | Topmost enchantment. |
| 1 | Middle enchantment. |
| 2 | Bottom enchantment. |
| Lectern | 1 | Previous page (which does give a redstone output). |
| 2 | Next page. |
| 3 | Take Book. |
| 100+page | Opened page number - 100 + number. |
| Stonecutter | Recipe button number - 4\*row + col. Depends on the item. |
| Loom | Recipe button number - 4\*row + col. Depends on the item. |

#### Click Container\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=218&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Click Container") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=218&editintro=Template:License_wiki.vg "Edit section's source code: Click Container")\]

This packet is sent by the client when the player clicks on a slot in a window.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x11`<br>_resource:_<br>`container_click` | Play | Server | Window ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | The ID of the window that was clicked. 0 for player inventory. The server ignores any packets targeting a Window ID other than the current one, including ignoring 0 when any other window is open. |
| State ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | The last received State ID from either a [Set Container Slot](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Container_Slot) or a [Set Container Content](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Container_Content) packet. |
| Slot | [Short](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Short) | The clicked slot number, see below. |
| Button | [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | The button used in the click, see below. |
| Mode | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | Inventory operation mode, see below. |
| Array of changed slots | Slot number | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) (128) | [Short](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Short) |  |
| Slot data | [Hashed Slot](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Hashed_Slot) | New data for this slot, in the client's opinion; see below. |
| Carried item | [Hashed Slot](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Hashed_Slot) | Item carried by the cursor. |

See [Inventory](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Inventory "Minecraft Wiki:Projects/wiki.vg merge/Inventory") for further information about how slots are indexed.

After performing the action, the server compares the results to the slot change information included in the packet, as applied on top of the server's view of the container's state prior to the action. For any slots that do not match, it sends [Set Container Slot](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Container_Slot) packets containing the correct results. If State ID does not match the last ID sent by the server, it will instead send a full [Set Container Content](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Container_Content) to resynchronize the client.

When right-clicking on a stack of items, half the stack will be picked up and half left in the slot. If the stack is an odd number, the half left in the slot will be the smaller of the amounts.

The distinct type of click performed by the client is determined by the combination of the Mode and Button fields.

| Mode | Button | Slot | Trigger |
| --- | --- | --- | --- |
| 0 | 0 | Normal | Left mouse click |
| 1 | Normal | Right mouse click |
| 0 | -999 | Left click outside inventory (drop cursor stack) |
| 1 | -999 | Right click outside inventory (drop cursor single item) |
| 1 | 0 | Normal | Shift + left mouse click |
| 1 | Normal | Shift + right mouse click _(identical behavior)_ |
| 2 | 0 | Normal | Number key 1 |
| 1 | Normal | Number key 2 |
| 2 | Normal | Number key 3 |
| ⋮ | ⋮ | ⋮ |
| 8 | Normal | Number key 9 |
| ⋮ | ⋮ | Used to allow swapping with arbitrary inventory slots. Ignored since 1.20.3. |
| 40 | Normal | Offhand swap key F |
| 3 | 2 | Normal | Middle click, only defined for creative players in non-player inventories. |
| 4 | 0 | Normal | Drop key (Q) |
| 1 | Normal | Control + Drop key (Q) |
| 5 | 0 | -999 | Starting left mouse drag |
| 4 | -999 | Starting right mouse drag |
| 8 | -999 | Starting middle mouse drag, only defined for creative players in non-player inventories. |
| 1 | Normal | Add slot for left-mouse drag |
| 5 | Normal | Add slot for right-mouse drag |
| 9 | Normal | Add slot for middle-mouse drag, only defined for creative players in non-player inventories. |
| 2 | -999 | Ending left mouse drag |
| 6 | -999 | Ending right mouse drag |
| 10 | -999 | Ending middle mouse drag, only defined for creative players in non-player inventories. |
| 6 | 0 | Normal | Double click |
| 1 | Normal | Pickup all but check items in reverse order (impossible in vanilla clients) |

Starting from version 1.5, “painting mode” is available for use in inventory windows. It is done by picking up a stack of something (more than 1 item), then holding the mouse button (left, right, or middle) and dragging the held stack over empty (or same type in case of right button) slots. In that case client sends the following to the server after the mouse button release (omitting the first pickup packet, which is sent as usual):

1. packet with mode 5, slot -999, button (0 for left \| 4 for right);
2. packet for every slot painted on, mode is still 5, button (1 \| 5);
3. packet with mode 5, slot -999, button (2 \| 6);

If any of the painting packets other than the “progress” ones are sent out of order (for example, a start, some slots, then another start; or a left-click in the middle) the painting status will be reset.

#### Close Container\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=219&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Close Container") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=219&editintro=Template:License_wiki.vg "Edit section's source code: Close Container")\]

This packet is sent by the client when closing a window.

vanilla clients send a Close Window packet with Window ID 0 to close their inventory, even though there is never an [Open Screen](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Open_Screen) packet for the inventory.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x12`<br>_resource:_<br>`container_close` | Play | Server | Window ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | This is the ID of the window that was closed. 0 for player inventory. |

#### Change Container Slot State\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=220&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Change Container Slot State") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=220&editintro=Template:License_wiki.vg "Edit section's source code: Change Container Slot State")\]

This packet is sent by the client when toggling the state of a Crafter.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x13`<br>_resource:_<br>`container_slot_state_changed` | Play | Server | Slot ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | This is the ID of the slot that was changed. |
| Window ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | This is the ID of the window that was changed. |
| State | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | The new state of the slot. True for enabled, false for disabled. |

#### Cookie Response (play)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=221&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Cookie Response (play)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=221&editintro=Template:License_wiki.vg "Edit section's source code: Cookie Response (play)")\]

Response to a [Cookie Request (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Cookie_Request_(play)) from the server. The vanilla server only accepts responses of up to 5 kiB in size.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x14`<br>_resource:_<br>`cookie_response` | Play | Server | Key | [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) | The identifier of the cookie. |
| Payload | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) (5120) of [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | The data of the cookie. |

#### Serverbound Plugin Message (play)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=222&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Serverbound Plugin Message (play)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=222&editintro=Template:License_wiki.vg "Edit section's source code: Serverbound Plugin Message (play)")\]

Main page: [Minecraft Wiki:Projects/wiki.vg merge/Plugin channels](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Plugin_channels "Minecraft Wiki:Projects/wiki.vg merge/Plugin channels")

Mods and plugins can use this to send their data. Minecraft itself uses some [plugin channels](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Plugin_channels "Minecraft Wiki:Projects/wiki.vg merge/Plugin channels"). These internal channels are in the `minecraft` namespace.

More documentation on this: [https://dinnerbone.com/blog/2012/01/13/minecraft-plugin-channels-messaging/](https://dinnerbone.com/blog/2012/01/13/minecraft-plugin-channels-messaging/)

Note that the length of Data is known only from the packet length, since the packet has no length field of any kind.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x15`<br>_resource:_<br>`custom_payload` | Play | Server | Channel | [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) | Name of the [plugin channel](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Plugin_channels "Minecraft Wiki:Projects/wiki.vg merge/Plugin channels") used to send the data. |
| Data | [Byte Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte_Array) (32767) | Any data, depending on the channel. `minecraft:` channels are documented [here](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Plugin_channels "Minecraft Wiki:Projects/wiki.vg merge/Plugin channels"). The length of this array must be inferred from the packet length. |

In vanilla servers, the maximum data length is 32767 bytes.

#### Debug Sample Subscription\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=223&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Debug Sample Subscription") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=223&editintro=Template:License_wiki.vg "Edit section's source code: Debug Sample Subscription")\]

Subscribes to the specified type of debug sample data, which is then sent periodically to the client via [Debug Sample](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Debug_Sample).

The subscription is retained for 10 seconds (the vanilla server checks that both 10.001 real-time seconds and 201 ticks have elapsed), after which the client is automatically unsubscribed. The vanilla client resends this packet every 5 seconds to keep up the subscription.

The vanilla server only allows subscriptions from players who are server operators.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x16`<br>_resource:_<br>`debug_sample_subscription` | Play | Server | Sample Type | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | The type of debug sample to subscribe to. Can be one of the following:<br>- 0 - Tick time |

#### Edit Book\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=224&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Edit Book") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=224&editintro=Template:License_wiki.vg "Edit section's source code: Edit Book")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x17`<br>_resource:_<br>`edit_book` | Play | Server | Slot | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | The hotbar slot where the written book is located |
| Entries | [Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Array) (100) of [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (1024) | Text from each page. Maximum string length is 1024 chars. |
| Title | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (32) | Title of book. Present if book is being signed, not present if book is being edited. |

#### Query Entity Tag\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=225&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Query Entity Tag") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=225&editintro=Template:License_wiki.vg "Edit section's source code: Query Entity Tag")\]

Used when `F3` + `I` is pressed while looking at an entity.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x18`<br>_resource:_<br>`entity_tag_query` | Play | Server | Transaction ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | An incremental ID so that the client can verify that the response matches. |
| Entity ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | The ID of the entity to query. |

#### Interact\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=226&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Interact") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=226&editintro=Template:License_wiki.vg "Edit section's source code: Interact")\]

This packet is sent from the client to the server when the client attacks or right-clicks another entity (a player, minecart, etc).

A vanilla server only accepts this packet if the entity being attacked/used is visible without obstruction and within a 4-unit radius of the player's position.

The target X, Y, and Z fields represent the difference between the vector location of the cursor at the time of the packet and the entity's position.

Note that middle-click in creative mode is interpreted by the client and sent as a [Set Creative Mode Slot](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Creative_Mode_Slot) packet instead.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x19`<br>_resource:_<br>`interact` | Play | Server | Entity ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | The ID of the entity to interact. Note the special case described below. |
| Type | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | 0: interact, 1: attack, 2: interact at. |
| Target X | [Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Optional) [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | Only if Type is interact at. |
| Target Y | [Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Optional) [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | Only if Type is interact at. |
| Target Z | [Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Optional) [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | Only if Type is interact at. |
| Hand | [Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Optional) [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | Only if Type is interact or interact at; 0: main hand, 1: off hand. |
| Sneak Key Pressed | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | If the client is pressing the sneak key. Has the same effect as a Player Command Press/Release sneak key preceding the interaction, and the state is permanently changed. |

Interaction with the ender dragon is an odd special case characteristic of release deadline–driven design. 8 consecutive entity IDs following the dragon's ID (id \+ 1, id \+ 2, ..., id \+ 8) are reserved for the 8 hitboxes that make up the dragon:

| ID offset | Description |
| --- | --- |
| 0 | The dragon itself (never used in this packet) |
| 1 | Head |
| 2 | Neck |
| 3 | Body |
| 4 | Tail 1 |
| 5 | Tail 2 |
| 6 | Tail 3 |
| 7 | Wing 1 |
| 8 | Wing 2 |

#### Jigsaw Generate\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=227&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Jigsaw Generate") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=227&editintro=Template:License_wiki.vg "Edit section's source code: Jigsaw Generate")\]

Sent when Generate is pressed on the [Jigsaw Block](https://minecraft.wiki/w/Jigsaw_Block "Jigsaw Block") interface.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x1A`<br>_resource:_<br>`jigsaw_generate` | Play | Server | Location | [Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Position) | Block entity location. |
| Levels | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Value of the levels slider/max depth to generate. |
| Keep Jigsaws | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) |  |

#### Serverbound Keep Alive (play)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=228&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Serverbound Keep Alive (play)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=228&editintro=Template:License_wiki.vg "Edit section's source code: Serverbound Keep Alive (play)")\]

The server will frequently send out a keep-alive (see [Clientbound Keep Alive](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Clientbound_Keep_Alive_(play))), each containing a random ID. The client must respond with the same packet.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x1B`<br>_resource:_<br>`keep_alive` | Play | Server | Keep Alive ID | [Long](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Long) |  |

#### Lock Difficulty\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=229&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Lock Difficulty") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=229&editintro=Template:License_wiki.vg "Edit section's source code: Lock Difficulty")\]

Must have at least op level 2 to use. Appears to only be used on singleplayer; the difficulty buttons are still disabled in multiplayer.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x1C`<br>_resource:_<br>`lock_difficulty` | Play | Server | Locked | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) |  |

#### Set Player Position\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=230&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Set Player Position") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=230&editintro=Template:License_wiki.vg "Edit section's source code: Set Player Position")\]

Updates the player's XYZ position on the server.

If the player is in a vehicle, the position is ignored (but in case of [Set Player Position and Rotation](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Player_Position_and_Rotation), the rotation is still used as normal). No validation steps other than value range clamping are performed in this case.

If the player is sleeping, the position (or rotation) is not changed, and a [Synchronize Player Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Synchronize_Player_Position) is sent if the received position deviated from the server's view by more than a meter.

The vanilla server silently clamps the x and z coordinates between -30,000,000 and 30,000,000, and the y coordinate between -20,000,000 and 20,000,000. A similar condition has historically caused a kick for "Illegal position"; this is no longer the case. However, infinite or NaN coordinates (or angles) still result in a kick for `multiplayer.disconnect.invalid_player_movement`.

As of 1.20.6, checking for moving too fast is achieved like this (sic):

- Each server tick, the player's current position is stored.
- When the player moves, the offset from the stored position to the requested position is computed (Δx, Δy, Δz).
- The requested movement distance squared is computed as Δx² + Δy² + Δz².
- The baseline expected movement distance squared is computed based on the player's server-side velocity as Vx² + Vy² + Vz². The player's server-side velocity is a somewhat ill-defined quantity that includes, among other things, gravity, jump velocity and knockback, but _not_ regular horizontal movement. A proper description would bring much of Minecraft's physics engine with it. It is accessible as the `Motion` NBT tag on the player entity.
- The maximum permitted movement distance squared is computed as 100 (300 if the player is using an elytra), multiplied by the number of movement packets received since the last tick, including this one, unless that value is greater than 5, in which case no multiplier is applied.
- If the requested movement distance squared minus the baseline distance squared is more than the maximum squared, the player is moving too fast.

If the player is moving too fast, it is logged that "<player> moved too quickly! " followed by the change in x, y, and z, and the player is teleported back to their current (before this packet) server-side position.

Checking for block collisions is achieved like this:

- A temporary collision-checked move of the player is attempted from its current position to the requested one.
- The offset from the resulting position to the requested position is computed. If the absolute value of the offset on the y axis is less than 0.5, it (only the y component) is rounded down to 0.
- If the magnitude of the offset is greater than 0.25 and the player isn't in creative or spectator mode, it is logged that "<player> moved wrongly!", and the player is teleported back to their current (before this packet) server-side position.
- In addition, if the player's hitbox, stationary at the requested position, would intersect with a block, and they aren't in spectator mode, they are teleported back without a log message.

Checking for illegal flight is achieved like this:

- When a movement packet is received, a flag indicating whether or not the player is floating mid-air is updated. The flag is set if the move test described above detected no collision below the player _and_ the y component of the offset from the player's current position to the requested one is greater than -0.5, unless any of various conditions permitting flight (creative mode, elytra, levitation effect, etc., but not jumping) are met.
- Each server tick, it is checked if the flag has been set for more than 80 consecutive ticks. If so, and the player isn't currently sleeping, dead or riding a vehicle, they are kicked for `multiplayer.disconnect.flying`.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x1D`<br>_resource:_<br>`move_player_pos` | Play | Server | X | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) | Absolute position. |
| Feet Y | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) | Absolute feet position, normally Head Y - 1.62. |
| Z | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) | Absolute position. |
| Flags | [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | Bit field: 0x01: on ground, 0x02: pushing against wall. |

#### Set Player Position and Rotation\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=231&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Set Player Position and Rotation") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=231&editintro=Template:License_wiki.vg "Edit section's source code: Set Player Position and Rotation")\]

A combination of [Move Player Rotation](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Player_Rotation) and [Move Player Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Player_Position).

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x1E`<br>_resource:_<br>`move_player_pos_rot` | Play | Server | X | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) | Absolute position. |
| Feet Y | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) | Absolute feet position, normally Head Y - 1.62. |
| Z | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) | Absolute position. |
| Yaw | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | Absolute rotation on the X Axis, in degrees. |
| Pitch | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | Absolute rotation on the Y Axis, in degrees. |
| Flags | [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | Bit field: 0x01: on ground, 0x02: pushing against wall. |

#### Set Player Rotation\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=232&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Set Player Rotation") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=232&editintro=Template:License_wiki.vg "Edit section's source code: Set Player Rotation")\]

[![](https://minecraft.wiki/images/thumb/Minecraft-trig-yaw.png/300px-Minecraft-trig-yaw.png?c58d0)](https://minecraft.wiki/w/File:Minecraft-trig-yaw.png) The unit circle for yaw[![](https://minecraft.wiki/images/thumb/Yaw.png/300px-Yaw.png?3475f)](https://minecraft.wiki/w/File:Yaw.png) The unit circle of yaw, redrawn

Updates the direction the player is looking in.

Yaw is measured in degrees and does not follow classical trigonometry rules. The unit circle of yaw on the XZ-plane starts at (0, 1) and turns counterclockwise, with 90 at (-1, 0), 180 at (0,-1) and 270 at (1, 0). Additionally, yaw is not clamped to between 0 and 360 degrees; any number is valid, including negative numbers and numbers greater than 360.

Pitch is measured in degrees, where 0 is looking straight ahead, -90 is looking straight up, and 90 is looking straight down.

The yaw and pitch of the player (in degrees), standing at point (x0, y0, z0) and looking towards point (x, y, z) can be calculated with:

```
dx = x-x0
dy = y-y0
dz = z-z0
r = sqrt( dx*dx + dy*dy + dz*dz )
yaw = -atan2(dx,dz)/PI*180
if yaw < 0 then
    yaw = 360 + yaw
pitch = -arcsin(dy/r)/PI*180
```

You can get a unit vector from a given yaw/pitch via:

```
x = -cos(pitch) * sin(yaw)
y = -sin(pitch)
z =  cos(pitch) * cos(yaw)
```

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x1F`<br>_resource:_<br>`move_player_rot` | Play | Server | Yaw | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | Absolute rotation on the X Axis, in degrees. |
| Pitch | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | Absolute rotation on the Y Axis, in degrees. |
| Flags | [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | Bit field: 0x01: on ground, 0x02: pushing against wall. |

#### Set Player Movement Flags\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=233&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Set Player Movement Flags") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=233&editintro=Template:License_wiki.vg "Edit section's source code: Set Player Movement Flags")\]

This packet, as well as [Set Player Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Player_Position), [Set Player Rotation](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Player_Rotation), and [Set Player Position and Rotation](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Player_Position_and_Rotation) are called the “serverbound movement packets”. Vanilla clients will send Move Player Position once every 20 ticks, even for a stationary player.

This packet is used to indicate whether the player is on ground (walking/swimming) or airborne (jumping/falling).

When dropping from a sufficient height, fall damage is applied when this state goes from false to true. The amount of damage applied is based on the point where it last changed from true to false. Note that there are several movement related packets containing this state.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x20`<br>_resource:_<br>`move_player_status_only` | Play | Server | Flags | [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | Bit field: 0x01: on ground, 0x02: pushing against wall. |

#### Move Vehicle (serverbound)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=234&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Move Vehicle (serverbound)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=234&editintro=Template:License_wiki.vg "Edit section's source code: Move Vehicle (serverbound)")\]

Sent when a player moves in a client-side-controlled vehicle. Fields are the same as in [Set Player Position and Rotation](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Player_Position_and_Rotation). Note that all fields use absolute positioning and do not allow for relative positioning.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x21`<br>_resource:_<br>`move_vehicle` | Play | Server | X | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) | Absolute position (X coordinate). |
| Y | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) | Absolute position (Y coordinate). |
| Z | [Double](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Double) | Absolute position (Z coordinate). |
| Yaw | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | Absolute rotation on the vertical axis, in degrees. |
| Pitch | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | Absolute rotation on the horizontal axis, in degrees. |
| On Ground | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | _(This value does not seem to exist)_ |

#### Paddle Boat\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=235&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Paddle Boat") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=235&editintro=Template:License_wiki.vg "Edit section's source code: Paddle Boat")\]

Used to _visually_ update whether boat paddles are turning. The server will update the [Boat entity metadata](https://minecraft.wiki/w/Java_Edition_protocol/Entity_metadata#Boat "Java Edition protocol/Entity metadata") to match the values here.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x22`<br>_resource:_<br>`paddle_boat` | Play | Server | Left paddle turning | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) |  |
| Right paddle turning | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) |  |

Right paddle turning is set to true when the left button or forward button is held, left paddle turning is set to true when the right button or forward button is held.

#### Pick Item From Block\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=236&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Pick Item From Block") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=236&editintro=Template:License_wiki.vg "Edit section's source code: Pick Item From Block")\]

Used for pick block functionality (middle click) on blocks to retrieve items from the inventory in survival or creative mode or create them in creative mode. See [Controls#Pick\_Block](https://minecraft.wiki/w/Controls#Pick_Block "Controls") for more information.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x23`<br>_resource:_<br>`pick_item_from_block` | Play | Server | Location | [Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Position) | The location of the block. |
| Include Data | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | Used to tell the server to include block data in the new stack, works only if in creative mode. |

#### Pick Item From Entity\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=237&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Pick Item From Entity") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=237&editintro=Template:License_wiki.vg "Edit section's source code: Pick Item From Entity")\]

Used for pick block functionality (middle click) on entities to retrieve items from the inventory in survival or creative mode or create them in creative mode. See [Controls#Pick\_Block](https://minecraft.wiki/w/Controls#Pick_Block "Controls") for more information.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x24`<br>_resource:_<br>`pick_item_from_entity` | Play | Server | Entity ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | The ID of the entity to pick. |
| Include Data | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | Unused by the vanilla server. |

#### Ping Request (play)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=238&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Ping Request (play)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=238&editintro=Template:License_wiki.vg "Edit section's source code: Ping Request (play)")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x25`<br>_resource:_<br>`ping_request` | Play | Server | Payload | [Long](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Long) | May be any number. vanilla clients use a system-dependent time value, which is counted in milliseconds. |

#### Place Recipe\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=239&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Place Recipe") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=239&editintro=Template:License_wiki.vg "Edit section's source code: Place Recipe")\]

This packet is sent when a player clicks a recipe in the crafting book that is craftable (white border).

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x26`<br>_resource:_<br>`place_recipe` | Play | Server | Window ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |
| Recipe ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | ID of recipe previously defined in [Recipe Book Add](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Recipe_Book_Add). |
| Make all | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | Affects the amount of items processed; true if shift is down when clicked. |

#### Player Abilities (serverbound)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=240&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Player Abilities (serverbound)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=240&editintro=Template:License_wiki.vg "Edit section's source code: Player Abilities (serverbound)")\]

The vanilla client sends this packet when the player starts/stops flying with the Flags parameter changed accordingly.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x27`<br>_resource:_<br>`player_abilities` | Play | Server | Flags | [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | Bit mask. 0x02: is flying. |

#### Player Action\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=241&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Player Action") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=241&editintro=Template:License_wiki.vg "Edit section's source code: Player Action")\]

Sent when the player mines a block. A vanilla server only accepts digging packets with coordinates within a 6-unit radius between the center of the block and the player's eyes.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x28`<br>_resource:_<br>`player_action` | Play | Server | Status | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | The action the player is taking against the block (see below). |
| Location | [Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Position) | Block position. |
| Face | [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | The face being hit (see below). |
| Sequence | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Block change sequence number (see [#Acknowledge Block Change](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Acknowledge_Block_Change)). |

Status can be one of seven values:

| Value | Meaning | Notes |
| --- | --- | --- |
| 0 | Started digging | Sent when the player starts digging a block. If the block was instamined or the player is in creative mode, the client will _not_ send Status = Finished digging, and will assume the server completed the destruction. To detect this, it is necessary to [calculate the block destruction speed](https://minecraft.wiki/w/Breaking#Speed "Breaking") server-side. |
| 1 | Cancelled digging | Sent when the player lets go of the Mine Block key (default: left click). Face is always set to -Y. |
| 2 | Finished digging | Sent when the client thinks it is finished. |
| 3 | Drop item stack | Triggered by using the Drop Item key (default: Q) with the modifier to drop the entire selected stack (default: Control or Command, depending on OS). Location is always set to 0/0/0, Face is always set to -Y. Sequence is always set to 0. |
| 4 | Drop item | Triggered by using the Drop Item key (default: Q). Location is always set to 0/0/0, Face is always set to -Y. Sequence is always set to 0. |
| 5 | Shoot arrow / finish eating | Indicates that the currently held item should have its state updated, such as eating food, pulling back bows, using buckets, etc. Location is always set to 0/0/0, Face is always set to -Y. Sequence is always set to 0. |
| 6 | Swap item in hand | Used to swap or assign an item to the second hand. Location is always set to 0/0/0, Face is always set to -Y. Sequence is always set to 0. |

The Face field can be one of the following values, representing the face being hit:

| Value | Offset | Face |
| --- | --- | --- |
| 0 | -Y | Bottom |
| 1 | +Y | Top |
| 2 | -Z | North |
| 3 | +Z | South |
| 4 | -X | West |
| 5 | +X | East |

#### Player Command\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=242&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Player Command") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=242&editintro=Template:License_wiki.vg "Edit section's source code: Player Command")\]

Sent by the client to indicate that it has performed certain actions: sprinting, exiting a bed, jumping with a horse, and opening a horse's inventory while riding it.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x29`<br>_resource:_<br>`player_command` | Play | Server | Entity ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Player ID (ignored by the vanilla server) |
| Action ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | The ID of the action, see below. |
| Jump Boost | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Only used by the “start jump with horse” action, in which case it ranges from 0 to 100. In all other cases it is 0. |

Action ID can be one of the following values:

| ID | Action |
| --- | --- |
| 0 | Leave bed |
| 1 | Start sprinting |
| 2 | Stop sprinting |
| 3 | Start jump with horse |
| 4 | Stop jump with horse |
| 5 | Open vehicle inventory |
| 6 | Start flying with elytra |

Leave bed is only sent when the “Leave Bed” button is clicked on the sleep GUI, not when waking up in the morning.

Open vehicle inventory is only sent when pressing the inventory key (default: E) while on a horse or chest boat — all other methods of opening such an inventory (involving right-clicking or shift-right-clicking it) do not use this packet.

#### Player Input\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=243&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Player Input") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=243&editintro=Template:License_wiki.vg "Edit section's source code: Player Input")\]

Sent whenever the player presses or releases certain keys. The flags correspond directly to the states of their corresponding keys—the Sprint flag does not depend on whether the player is actually able to sprint at the moment, etc.

Used by the vanilla server for minecart controls, player inputs in the `entity_properties` [predicate](https://minecraft.wiki/w/Predicate "Predicate"), and sneaking (sprinting is still controlled by [Player Command](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Player_Command)).

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x2A`<br>_resource:_<br>`player_input` | Play | Server | Flags | [Unsigned Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Unsigned_Byte) | Bit mask; see below |

The flags are as follows:

| Hex Mask | Field |
| --- | --- |
| 0x01 | Forward |
| 0x02 | Backward |
| 0x04 | Left |
| 0x08 | Right |
| 0x10 | Jump |
| 0x20 | Sneak |
| 0x40 | Sprint |

#### Player Loaded\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=244&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Player Loaded") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=244&editintro=Template:License_wiki.vg "Edit section's source code: Player Loaded")\]

Sent by the client to indicate that it is ready to start simulating the player. The vanilla client sends this [when the "Loading terrain..." screen is closed](https://minecraft.wiki/w/Java_Edition_protocol/FAQ#%E2%80%A6my_player_isn't_spawning! "Java Edition protocol/FAQ"). (But see the caveat below.)

The vanilla client skips ticking the player entity until the tick on which this packet is sent (the first tick will happen between this packet and the next [Client Tick End](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Client_Tick_End)). Other entities and objects will still be ticked.

Once 60 ticks have elapsed since the last Login or Respawn packet, the vanilla client will start ticking the player and **skip sending this packet completely**, even after the usual conditions for it have been met. This can happen even before the "Start waiting for level chunks" [Game Event](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Game_Event) is received. The loading screen is not affected in any way by this timer (except indirectly by the player falling into the void after ticking has started). Likewise, the vanilla server will assume that the client has loaded if it takes longer than 60 server ticks to send this packet. A more robust way to detect this condition is to count the number of [Client Tick End](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Client_Tick_End) packets sent by the client. The first player tick will occur after 60 Client Tick End packets have been sent. To determine when this counter should be restarted following a respawn, the Respawn packet can be sent in a [bundle](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Bundle_Delimiter) together with a [Ping](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Ping_(play)) packet.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x2B`<br>_resource:_<br>`player_loaded` | Play | Server | _no fields_ |

#### Pong (play)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=245&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Pong (play)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=245&editintro=Template:License_wiki.vg "Edit section's source code: Pong (play)")\]

Response to the clientbound packet ( [Ping](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Ping_(play))) with the same ID.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x2C`<br>_resource:_<br>`pong` | Play | Server | ID | [Int](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Int) | id is the same as the ping packet |

#### Change Recipe Book Settings\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=246&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Change Recipe Book Settings") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=246&editintro=Template:License_wiki.vg "Edit section's source code: Change Recipe Book Settings")\]

Replaces Recipe Book Data, type 1.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x2D`<br>_resource:_<br>`recipe_book_change_settings` | Play | Server | Book ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | 0: crafting, 1: furnace, 2: blast furnace, 3: smoker. |
| Book Open | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) |  |
| Filter Active | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) |  |

#### Set Seen Recipe\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=247&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Set Seen Recipe") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=247&editintro=Template:License_wiki.vg "Edit section's source code: Set Seen Recipe")\]

Sent when recipe is first seen in recipe book. Replaces Recipe Book Data, type 0.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x2E`<br>_resource:_<br>`recipe_book_seen_recipe` | Play | Server | Recipe ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | ID of recipe previously defined in Recipe Book Add. |

#### Rename Item\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=248&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Rename Item") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=248&editintro=Template:License_wiki.vg "Edit section's source code: Rename Item")\]

Sent as a player is renaming an item in an anvil (each keypress in the anvil UI sends a new Rename Item packet). If the new name is empty, then the item loses its custom name (this is different from setting the custom name to the normal name of the item). The item name may be no longer than 50 characters, and if it is longer than that, then the rename is silently ignored.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x2F`<br>_resource:_<br>`rename_item` | Play | Server | Item name | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (32767) | The new name of the item. |

#### Resource Pack Response (play)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=249&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Resource Pack Response (play)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=249&editintro=Template:License_wiki.vg "Edit section's source code: Resource Pack Response (play)")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x30`<br>_resource:_<br>`resource_pack` | Play | Server | UUID | [UUID](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:UUID) | The unique identifier of the resource pack received in the [Add Resource Pack (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Add_Resource_Pack_(play)) request. |
| Result | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | Result ID (see below). |

Result can be one of the following values:

| ID | Result |
| --- | --- |
| 0 | Successfully downloaded |
| 1 | Declined |
| 2 | Failed to download |
| 3 | Accepted |
| 4 | Downloaded |
| 5 | Invalid URL |
| 6 | Failed to reload |
| 7 | Discarded |

#### Seen Advancements\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=250&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Seen Advancements") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=250&editintro=Template:License_wiki.vg "Edit section's source code: Seen Advancements")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x31`<br>_resource:_<br>`seen_advancements` | Play | Server | Action | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | 0: Opened tab, 1: Closed screen. |
| Tab ID | [Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Optional) [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) | Only present if action is Opened tab. |

#### Select Trade\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=251&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Select Trade") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=251&editintro=Template:License_wiki.vg "Edit section's source code: Select Trade")\]

When a player selects a specific trade offered by a villager NPC.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x32`<br>_resource:_<br>`select_trade` | Play | Server | Selected slot | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | The selected slot in the player's current (trading) inventory. |

#### Set Beacon Effect\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=252&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Set Beacon Effect") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=252&editintro=Template:License_wiki.vg "Edit section's source code: Set Beacon Effect")\]

Changes the effect of the current beacon.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x33`<br>_resource:_<br>`set_beacon` | Play | Server | Primary Effect | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | A [Potion ID](https://minecraft.wiki/w/Potion#ID). |
| Secondary Effect | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | A [Potion ID](https://minecraft.wiki/w/Potion#ID). |

#### Set Held Item (serverbound)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=253&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Set Held Item (serverbound)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=253&editintro=Template:License_wiki.vg "Edit section's source code: Set Held Item (serverbound)")\]

Sent when the player changes the slot selection.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x34`<br>_resource:_<br>`set_carried_item` | Play | Server | Slot | [Short](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Short) | The slot which the player has selected (0–8). |

#### Program Command Block\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=254&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Program Command Block") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=254&editintro=Template:License_wiki.vg "Edit section's source code: Program Command Block")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x35`<br>_resource:_<br>`set_command_block` | Play | Server | Location | [Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Position) |  |
| Command | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (32767) |  |
| Mode | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | 0: chain, 1: repeating, 2: impulse. |
| Flags | [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | 0x01: Track Output (if false, the output of the previous command will not be stored within the command block); 0x02: Is conditional; 0x04: Automatic. |

#### Program Command Block Minecart\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=255&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Program Command Block Minecart") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=255&editintro=Template:License_wiki.vg "Edit section's source code: Program Command Block Minecart")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x36`<br>_resource:_<br>`set_command_minecart` | Play | Server | Entity ID | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |
| Command | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (32767) |  |
| Track Output | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | If false, the output of the previous command will not be stored within the command block. |

#### Set Creative Mode Slot\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=256&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Set Creative Mode Slot") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=256&editintro=Template:License_wiki.vg "Edit section's source code: Set Creative Mode Slot")\]

While the user is in the standard inventory (i.e., not a crafting bench) in Creative mode, the player will send this packet.

Clicking in the creative inventory menu is quite different from non-creative inventory management. Picking up an item with the mouse actually deletes the item from the server, and placing an item into a slot or dropping it out of the inventory actually tells the server to create the item from scratch. (This can be verified by clicking an item that you don't mind deleting, then severing the connection to the server; the item will be nowhere to be found when you log back in.) As a result of this implementation strategy, the "Destroy Item" slot is just a client-side implementation detail that means "I don't intend to recreate this item.". Additionally, the long listings of items (by category, etc.) are a client-side interface for choosing which item to create. Picking up an item from such listings sends no packets to the server; only when you put it somewhere does it tell the server to create the item in that location.

This action can be described as "set inventory slot". Picking up an item sets the slot to item ID -1. Placing an item into an inventory slot sets the slot to the specified item. Dropping an item (by clicking outside the window) effectively sets slot -1 to the specified item, which causes the server to spawn the item entity, etc.. All other inventory slots are numbered the same as the non-creative inventory (including slots for the 2x2 crafting menu, even though they aren't visible in the vanilla client).

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x37`<br>_resource:_<br>`set_creative_mode_slot` | Play | Server | Slot | [Short](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Short) | Inventory slot. |
| Clicked Item | [Slot](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Slot) |  |

#### Program Jigsaw Block\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=257&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Program Jigsaw Block") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=257&editintro=Template:License_wiki.vg "Edit section's source code: Program Jigsaw Block")\]

Sent when Done is pressed on the [Jigsaw Block](https://minecraft.wiki/w/Jigsaw_Block "Jigsaw Block") interface.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x38`<br>_resource:_<br>`set_jigsaw_block` | Play | Server | Location | [Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Position) | Block entity location |
| Name | [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) |  |
| Target | [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) |  |
| Pool | [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) |  |
| Final state | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (32767) | "Turns into" on the GUI, `final_state` in NBT. |
| Joint type | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (32767) | `rollable` if the attached piece can be rotated, else `aligned`. |
| Selection priority | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |
| Placement priority | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |

#### Program Structure Block\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=258&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Program Structure Block") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=258&editintro=Template:License_wiki.vg "Edit section's source code: Program Structure Block")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x39`<br>_resource:_<br>`set_structure_block` | Play | Server |
| Location | [Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Position) | Block entity location. |
| Action | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | An additional action to perform beyond simply saving the given data; see below. |
| Mode | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | One of SAVE (0), LOAD (1), CORNER (2), DATA (3). |
| Name | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (32767) |  |
| Offset X | [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | Between -48 and 48. |
| Offset Y | [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | Between -48 and 48. |
| Offset Z | [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | Between -48 and 48. |
| Size X | [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | Between 0 and 48. |
| Size Y | [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | Between 0 and 48. |
| Size Z | [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | Between 0 and 48. |
| Mirror | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | One of NONE (0), LEFT\_RIGHT (1), FRONT\_BACK (2). |
| Rotation | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | One of NONE (0), CLOCKWISE\_90 (1), CLOCKWISE\_180 (2), COUNTERCLOCKWISE\_90 (3). |
| Metadata | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (128) |  |
| Integrity | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | Between 0 and 1. |
| Seed | [VarLong](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarLong) |  |
| Flags | [Byte](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Byte) | 0x01: Ignore entities; 0x02: Show air; 0x04: Show bounding box; 0x08: Strict placement. |

Possible actions:

- 0 - Update data
- 1 - Save the structure
- 2 - Load the structure
- 3 - Detect size

The vanilla client uses update data to indicate no special action should be taken (i.e. the done button).

#### Set Test Block\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=259&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Set Test Block") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=259&editintro=Template:License_wiki.vg "Edit section's source code: Set Test Block")\]

Updates the value of the [Test Block](https://minecraft.wiki/w/Test_Block "Test Block") at the given position.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x3A`<br>_resource:_<br>`set_test_block` | Play | Server | Position | [Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Position) |  |
| Mode | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | 0: start, 1: log, 2: fail, 3: accept |
| Message | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) |  |

#### Update Sign\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=260&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Update Sign") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=260&editintro=Template:License_wiki.vg "Edit section's source code: Update Sign")\]

This message is sent from the client to the server when the “Done” button is pushed after placing a sign.

The server only accepts this packet after [Open Sign Editor](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Open_Sign_Editor), otherwise this packet is silently ignored.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x3B`<br>_resource:_<br>`sign_update` | Play | Server | Location | [Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Position) | Block Coordinates. |
| Is Front Text | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | Whether the updated text is in front or on the back of the sign |
| Line 1 | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (384) | First line of text in the sign. |
| Line 2 | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (384) | Second line of text in the sign. |
| Line 3 | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (384) | Third line of text in the sign. |
| Line 4 | [String](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:String) (384) | Fourth line of text in the sign. |

#### Swing Arm\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=261&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Swing Arm") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=261&editintro=Template:License_wiki.vg "Edit section's source code: Swing Arm")\]

Sent when the player's arm swings.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x3C`<br>_resource:_<br>`swing` | Play | Server | Hand | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | Hand used for the animation. 0: main hand, 1: off hand. |

#### Teleport To Entity\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=262&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Teleport To Entity") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=262&editintro=Template:License_wiki.vg "Edit section's source code: Teleport To Entity")\]

Teleports the player to the given entity. The player must be in spectator mode.

The vanilla client only uses this to teleport to players, but it appears to accept any type of entity. The entity does not need to be in the same dimension as the player; if necessary, the player will be respawned in the right world. If the given entity cannot be found (or isn't loaded), this packet will be ignored.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x3D`<br>_resource:_<br>`teleport_to_entity` | Play | Server | Target Player | [UUID](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:UUID) | UUID of the player to teleport to (can also be an entity UUID). |

#### Test Instance Block Action\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=263&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Test Instance Block Action") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=263&editintro=Template:License_wiki.vg "Edit section's source code: Test Instance Block Action")\]

Tries to perform an action the [Test Instance Block](https://minecraft.wiki/w/Test_Instance_Block "Test Instance Block") at the given position.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x3E`<br>_resource:_<br>`test_instance_block_action` | Play | Server | Position | [Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Position) |  |
| Action | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | 0: init, 1: query, 2: set, 3: reset, 4: save, 5: export, 6: run. |
| Test | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) | ID in the `minecraft:test_instance` registry. |
| Size X | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |
| Size Y | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |
| Size Z | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) |  |
| Rotation | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | 0: none, 1: clockwise 90°, 2: clockwise 180°, 3: counter-clockwise 90°. |
| Ignore Entities | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) |  |
| Status | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | 0: cleared, 1: running, 2: finished. |
| Error Message | [Prefixed Optional](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Prefixed_Optional) [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) |  |

#### Use Item On\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=264&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Use Item On") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=264&editintro=Template:License_wiki.vg "Edit section's source code: Use Item On")\]

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x3F`<br>_resource:_<br>`use_item_on` | Play | Server | Hand | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | The hand from which the block is placed; 0: main hand, 1: off hand. |
| Location | [Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Position) | Block position. |
| Face | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | The face on which the block is placed (as documented at [Player Action](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Player_Action)). |
| Cursor Position X | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | The position of the crosshair on the block, from 0 to 1 increasing from west to east. |
| Cursor Position Y | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | The position of the crosshair on the block, from 0 to 1 increasing from bottom to top. |
| Cursor Position Z | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | The position of the crosshair on the block, from 0 to 1 increasing from north to south. |
| Inside block | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | True when the player's head is inside of a block. |
| World Border Hit | [Boolean](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Boolean) | Seems to always be false, even when interacting with blocks around or outside the world border, or while the player is outside the border. |
| Sequence | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Block change sequence number (see [#Acknowledge Block Change](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Acknowledge_Block_Change)). |

Upon placing a block, this packet is sent once.

The Cursor Position X/Y/Z fields (also known as in-block coordinates) are calculated using raytracing. The unit corresponds to sixteen pixels in the default resource pack. For example, let's say a slab is being placed against the south face of a full block. The Cursor Position X will be higher if the player was pointing near the right (east) edge of the face, lower if pointing near the left. The Cursor Position Y will be used to determine whether it will appear as a bottom slab (values 0.0–0.5) or as a top slab (values 0.5-1.0). The Cursor Position Z should be 1.0 since the player was looking at the southernmost part of the block.

Inside block is true when a player's head (specifically eyes) are inside of a block's collision. In 1.13 and later versions, collision is rather complicated and individual blocks can have multiple collision boxes. For instance, a ring of vines has a non-colliding hole in the middle. This value is only true when the player is directly in the box. In practice, though, this value is only used by scaffolding to place in front of the player when sneaking inside of it (other blocks will place behind when you intersect with them -- try with glass for instance).

#### Use Item\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=265&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Use Item") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=265&editintro=Template:License_wiki.vg "Edit section's source code: Use Item")\]

Sent when pressing the Use Item key (default: right click) with an item in hand.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x40`<br>_resource:_<br>`use_item` | Play | Server | Hand | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) [Enum](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Enum) | Hand used for the animation. 0: main hand, 1: off hand. |
| Sequence | [VarInt](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt) | Block change sequence number (see [#Acknowledge Block Change](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Acknowledge_Block_Change)). |
| Yaw | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | Player head rotation around the Y-Axis. |
| Pitch | [Float](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Float) | Player head rotation around the X-Axis. |

The player's rotation is permanently updated according to the Yaw and Pitch fields before performing the action, unless there is no item in the specified hand.

#### Custom Click Action (play)\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=266&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Custom Click Action (play)") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=266&editintro=Template:License_wiki.vg "Edit section's source code: Custom Click Action (play)")\]

Sent when the client clicks a [Text Component](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component) with the `minecraft:custom` click action. This is meant as an alternative to running a command, but will not have any effect on vanilla servers.

| Packet ID | State | Bound To | Field Name | Field Type | Notes |
| --- | --- | --- | --- | --- | --- |
| _protocol:_<br>`0x41`<br>_resource:_<br>`custom_click_action` | Play | Server | ID | [Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier) | The identifier for the click action. |
| Payload | [NBT](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:NBT) | The data to send with the click action. May be a TAG\_END (0). |

## Navigation\[ [edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?section=267&veaction=edit&editintro=Template:License_wiki.vg "Edit section: Navigation") \| [edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&section=267&editintro=Template:License_wiki.vg "Edit section's source code: Navigation")\]

| \[hide\]<br>- [v](https://minecraft.wiki/w/Template:Navbox_Java_Edition_technical "Template:Navbox Java Edition technical")<br>- [t](https://minecraft.wiki/w/Special:TalkPage/Template:Navbox_Java_Edition_technical "Special:TalkPage/Template:Navbox Java Edition technical")<br>- [e](https://minecraft.wiki/w/Special:EditPage/Template:Navbox_Java_Edition_technical "Special:EditPage/Template:Navbox Java Edition technical")<br>_[Java Edition](https://minecraft.wiki/w/Java_Edition "Java Edition")_ technical |
| --- |
| | \[hide\]General |
| --- |
| Concepts | - [![](https://minecraft.wiki/images/BlockSprite_chest.png?15d81)](https://minecraft.wiki/w/Block_entity "Block entity")[Block entity](https://minecraft.wiki/w/Block_entity "Block entity")<br>- [![](https://minecraft.wiki/images/EnvSprite_map.png?b863e)](https://minecraft.wiki/w/Coordinates "Coordinates")[Coordinates](https://minecraft.wiki/w/Coordinates "Coordinates")<br>- [![](https://minecraft.wiki/images/EffectSprite_infested.png?4562a)](https://minecraft.wiki/w/Crash "Crash")[Crashes](https://minecraft.wiki/w/Crash "Crash")<br>- \[String\] [Loot context](https://minecraft.wiki/w/Loot_context "Loot context")<br>- [![](https://minecraft.wiki/images/EntitySprite_cow.png?893cf)](https://minecraft.wiki/w/Mob_AI "Mob AI")[Mob AI](https://minecraft.wiki/w/Mob_AI "Mob AI")<br>- [![](https://minecraft.wiki/images/BlockSprite_lodestone.png?00f1a)](https://minecraft.wiki/w/Point_of_Interest "Point of Interest")[Point of Interest](https://minecraft.wiki/w/Point_of_Interest "Point of Interest")<br>- ![File directory.png: Sprite image for directory in Minecraft](https://minecraft.wiki/images/thumb/File_directory.png/16px-File_directory.png?8a409)[Resource location](https://minecraft.wiki/w/Resource_location "Resource location")<br>- [![](https://minecraft.wiki/images/BlockSprite_camera.png?7ee99)](https://minecraft.wiki/w/Screenshot "Screenshot")[Screenshot](https://minecraft.wiki/w/Screenshot "Screenshot")<br>- [![](https://minecraft.wiki/images/EnvSprite_scoreboard.png?38feb)](https://minecraft.wiki/w/Statistics "Statistics")[Statistics](https://minecraft.wiki/w/Statistics "Statistics")<br>- [![](https://minecraft.wiki/images/ItemSprite_book.png?791a5)](https://minecraft.wiki/w/Telemetry "Telemetry")[Telemetry](https://minecraft.wiki/w/Telemetry "Telemetry")<br>- [![](https://minecraft.wiki/images/ItemSprite_clock.png?30324)](https://minecraft.wiki/w/Tick "Tick")[Tick](https://minecraft.wiki/w/Tick "Tick")<br>- [![](https://minecraft.wiki/images/BlockSprite_number-block.png?0d8c7)](https://minecraft.wiki/w/UUID "UUID")[UUID](https://minecraft.wiki/w/UUID "UUID")<br>- [![](https://minecraft.wiki/images/BlockSprite_computer.png?e0c37)](https://minecraft.wiki/w/JSON "JSON")[JSON](https://minecraft.wiki/w/JSON "JSON") |
| [General format](https://minecraft.wiki/w/Development_resources "Development resources") | - [![](https://minecraft.wiki/images/BlockSprite_number-block.png?0d8c7)](https://minecraft.wiki/w/Java_Edition_data_values "Java Edition data values")[Data values](https://minecraft.wiki/w/Java_Edition_data_values "Java Edition data values")<br>  - [![](https://minecraft.wiki/images/LegacyBlockSprite_bricks-je1.png?9a58b)](https://minecraft.wiki/w/Java_Edition_Classic_data_values "Java Edition Classic data values")[Classic](https://minecraft.wiki/w/Java_Edition_Classic_data_values "Java Edition Classic data values")<br>  - [![](https://minecraft.wiki/images/EntitySprite_rana.png?3f2f9)](https://minecraft.wiki/w/Java_Edition_Indev_data_values "Java Edition Indev data values")[Indev](https://minecraft.wiki/w/Java_Edition_Indev_data_values "Java Edition Indev data values")<br>  - [![](https://minecraft.wiki/images/BlockSprite_stone.png?e9a91)](https://minecraft.wiki/w/Java_Edition_pre-flattening_data_values "Java Edition pre-flattening data values")[Pre-flattening](https://minecraft.wiki/w/Java_Edition_pre-flattening_data_values "Java Edition pre-flattening data values")<br>- [![](https://minecraft.wiki/images/EnvSprite_item.png?89d23)](https://minecraft.wiki/w/Data_component_format "Data component format")[Data component format](https://minecraft.wiki/w/Data_component_format "Data component format")<br>  - [![](https://minecraft.wiki/images/BlockSprite_test-block-start.png?35191)](https://minecraft.wiki/w/Data_component_predicate "Data component predicate")[Predicate](https://minecraft.wiki/w/Data_component_predicate "Data component predicate")<br>- [![](https://minecraft.wiki/images/EnvSprite_entities.png?94711)](https://minecraft.wiki/w/Entity_format "Entity format")[Entity format](https://minecraft.wiki/w/Entity_format "Entity format")<br>- [![](https://minecraft.wiki/images/ItemSprite_map.png?05f8c)](https://minecraft.wiki/w/Map_item_format "Map item format")[Map item format](https://minecraft.wiki/w/Map_item_format "Map item format")<br>- \[NBT Compound / JSON Object\] [NBT format](https://minecraft.wiki/w/NBT_format "NBT format")<br>- [![](https://minecraft.wiki/images/EffectSprite_particle-healing.png?1357a)](https://minecraft.wiki/w/Particle_format "Particle format")[Particle format](https://minecraft.wiki/w/Particle_format "Particle format")<br>- [![](https://minecraft.wiki/images/ItemSprite_book-and-quill.png?f190b)](https://minecraft.wiki/w/Text_component_format "Text component format")[Text component format](https://minecraft.wiki/w/Text_component_format "Text component format")<br>- [§](https://minecraft.wiki/w/Formatting_codes "Formatting codes") [Formatting codes](https://minecraft.wiki/w/Formatting_codes "Formatting codes")<br>- [![](https://minecraft.wiki/images/thumb/Movement_hint.png/16px-Movement_hint.png?92667)](https://minecraft.wiki/w/Key_codes "Key codes")[Key Codes](https://minecraft.wiki/w/Key_codes "Key codes")<br>- [![](https://minecraft.wiki/images/thumb/Dice.png/14px-Dice.png?a4e84)](https://minecraft.wiki/w/Random_sequence_format "Random sequence format")[Random sequence](https://minecraft.wiki/w/Random_sequence_format "Random sequence format")<br>- [![](https://minecraft.wiki/images/BlockSprite_structure-block.png?381fc)](https://minecraft.wiki/w/Structure_file "Structure file")[Structure file format](https://minecraft.wiki/w/Structure_file "Structure file")<br>  - [![](https://minecraft.wiki/images/EnvSprite_chunk.png?b2cf1)](https://minecraft.wiki/w/Schematic_file_format "Schematic file format")[Schematic file format](https://minecraft.wiki/w/Schematic_file_format "Schematic file format") |
| [World](https://minecraft.wiki/w/World "World") | - [![](https://minecraft.wiki/images/EnvSprite_altitude.png?9b274)](https://minecraft.wiki/w/Heightmap "Heightmap")[Heightmap](https://minecraft.wiki/w/Heightmap "Heightmap")
- [![](https://minecraft.wiki/images/EnvSprite_mountain.png?2e0ae)](https://minecraft.wiki/w/World_seed "World seed")[Seed](https://minecraft.wiki/w/World_seed "World seed")
  - [Anomalous](https://minecraft.wiki/w/Anomalous_world_seeds "Anomalous world seeds")
- [![](https://minecraft.wiki/images/BlockSprite_number-block.png?0d8c7)](https://minecraft.wiki/w/Data_version "Data version")[Data version](https://minecraft.wiki/w/Data_version "Data version")

| Legacy | - [![](https://minecraft.wiki/images/EnvSprite_chunk.png?b2cf1)](https://minecraft.wiki/w/Spawn_chunk "Spawn chunk")[Spawn chunk](https://minecraft.wiki/w/Spawn_chunk "Spawn chunk") |
| [Level format](https://minecraft.wiki/w/Java_Edition_level_format "Java Edition level format") | - [![](https://minecraft.wiki/images/BlockSprite_anvil.png?a26c9)](https://minecraft.wiki/w/Anvil_file_format "Anvil file format")[Anvil file format](https://minecraft.wiki/w/Anvil_file_format "Anvil file format")
- [![](https://minecraft.wiki/images/EnvSprite_chunk.png?b2cf1)](https://minecraft.wiki/w/Chunk_format "Chunk format")[Chunk format](https://minecraft.wiki/w/Chunk_format "Chunk format")
- [![](https://minecraft.wiki/images/EntitySprite_steve.png?856f8)](https://minecraft.wiki/w/Player.dat_format "Player.dat format")[Player format](https://minecraft.wiki/w/Player.dat_format "Player.dat format")
- [![](https://minecraft.wiki/images/BlockSprite_lodestone.png?00f1a)](https://minecraft.wiki/w/Point_of_Interest_format "Point of Interest format")[Point of Interest format](https://minecraft.wiki/w/Point_of_Interest_format "Point of Interest format")
- [![](https://minecraft.wiki/images/EntitySprite_ravager.png?40196)](https://minecraft.wiki/w/Raids.dat_format "Raids.dat format")[raids.dat format](https://minecraft.wiki/w/Raids.dat_format "Raids.dat format")
- [![](https://minecraft.wiki/images/BlockSprite_chain-command-block.png?0afa8)](https://minecraft.wiki/w/Command_storage_format "Command storage format")[Command storage format](https://minecraft.wiki/w/Command_storage_format "Command storage format")
- [![](https://minecraft.wiki/images/EnvSprite_scoreboard.png?38feb)](https://minecraft.wiki/w/Scoreboard#NBT_format "Scoreboard")[Scoreboard format](https://minecraft.wiki/w/Scoreboard#NBT_format "Scoreboard")

| Legacy | - [![](https://minecraft.wiki/images/LegacyBlockSprite_bricks-je1.png?9a58b)](https://minecraft.wiki/w/Java_Edition_Classic_level_format "Java Edition Classic level format")[Classic level format](https://minecraft.wiki/w/Java_Edition_Classic_level_format "Java Edition Classic level format")<br>- [Classic server protocol](https://minecraft.wiki/w/Classic_server_protocol "Classic server protocol")<br>- [![](https://minecraft.wiki/images/EntitySprite_rana.png?3f2f9)](https://minecraft.wiki/w/Java_Edition_Indev_level_format "Java Edition Indev level format")[Indev level format](https://minecraft.wiki/w/Java_Edition_Indev_level_format "Java Edition Indev level format")<br>- [![](https://minecraft.wiki/images/EnvSprite_nether-portal.png?47646)](https://minecraft.wiki/w/Java_Edition_Alpha_level_format "Java Edition Alpha level format")[Alpha level format](https://minecraft.wiki/w/Java_Edition_Alpha_level_format "Java Edition Alpha level format")<br>  - [![](https://minecraft.wiki/images/LegacyItemSprite_oak-door-revision-1.png?b7426)](https://minecraft.wiki/w/Zone_file_format "Zone file format")[Zone file format](https://minecraft.wiki/w/Zone_file_format "Zone file format")<br>- [![](https://minecraft.wiki/images/ItemSprite_locked-map.png?c4112)](https://minecraft.wiki/w/Region_file_format "Region file format")[Region file format](https://minecraft.wiki/w/Region_file_format "Region file format")<br>- [![](https://minecraft.wiki/images/Servers.png?b1dc2)](https://minecraft.wiki/w/Server_level.dat "Server level.dat")[server\_level.dat format](https://minecraft.wiki/w/Server_level.dat "Server level.dat")<br>- [![](https://minecraft.wiki/images/EnvSprite_new-village.png?3e8a5)](https://minecraft.wiki/w/Villages.dat_format "Villages.dat format")[villages.dat format](https://minecraft.wiki/w/Villages.dat_format "Villages.dat format")<br>- [![](https://minecraft.wiki/images/EnvSprite_abandoned-mineshaft.png?fab65)](https://minecraft.wiki/w/Generated_structures_data_file_format "Generated structures data file format")[Generated structures format](https://minecraft.wiki/w/Generated_structures_data_file_format "Generated structures data file format") | | |
| [.minecraft](https://minecraft.wiki/w/.minecraft ".minecraft") | - ![File archive.png: Sprite image for archive in Minecraft](https://minecraft.wiki/images/thumb/File_archive.png/16px-File_archive.png?5ba7d)[client.jar](https://minecraft.wiki/w/Client.jar "Client.jar") <br>  - [![](https://minecraft.wiki/images/BlockSprite_number-block.png?0d8c7)](https://minecraft.wiki/w/Version.json "Version.json")[version.json](https://minecraft.wiki/w/Version.json "Version.json")<br>- [![](https://minecraft.wiki/images/ItemSprite_book-and-quill.png?f190b)](https://minecraft.wiki/w/Client.json "Client.json")[client.json](https://minecraft.wiki/w/Client.json "Client.json")<br>- [![](https://minecraft.wiki/images/BlockSprite_command-block.png?e7078)](https://minecraft.wiki/w/Command_history.txt "Command history.txt")[command\_history.txt](https://minecraft.wiki/w/Command_history.txt "Command history.txt")<br>- [![](https://minecraft.wiki/images/BlockSprite_chest.png?15d81)](https://minecraft.wiki/w/Launcher_profiles.json "Launcher profiles.json")[launcher\_profiles.json](https://minecraft.wiki/w/Launcher_profiles.json "Launcher profiles.json")<br>- [![](https://minecraft.wiki/images/Chat_settings_gear.png?6a179)](https://minecraft.wiki/w/Options.txt "Options.txt")[options.txt](https://minecraft.wiki/w/Options.txt "Options.txt")<br>- [![](https://minecraft.wiki/images/BlockSprite_number-block.png?0d8c7)](https://minecraft.wiki/w/Version_manifest.json "Version manifest.json")[version\_manifest.json](https://minecraft.wiki/w/Version_manifest.json "Version manifest.json")<br>- [![](https://minecraft.wiki/images/ItemSprite_paper.png?565a1)](https://minecraft.wiki/w/Hotbar.nbt_format "Hotbar.nbt format")[hotbar.nbt format](https://minecraft.wiki/w/Hotbar.nbt_format "Hotbar.nbt format")<br>- [![](https://minecraft.wiki/images/Servers.png?b1dc2)](https://minecraft.wiki/w/Servers.dat_format "Servers.dat format")[Server list format](https://minecraft.wiki/w/Servers.dat_format "Servers.dat format") |
| Tools | - `F3` [Debug screen](https://minecraft.wiki/w/Debug_screen "Debug screen") <br>  - [hotkey](https://minecraft.wiki/w/Debug_hotkey "Debug hotkey")<br>  - [renderer](https://minecraft.wiki/w/Debug_renderer "Debug renderer")<br>- [![](https://minecraft.wiki/images/Mojang_logo.svg?0b294)](https://minecraft.wiki/w/Java_Developer_Tools "Java Developer Tools")[Developer Tools](https://minecraft.wiki/w/Java_Developer_Tools "Java Developer Tools")<br>  - [![](https://minecraft.wiki/images/BlockSprite_test-instance-block.png?27a39)](https://minecraft.wiki/w/GameTest "GameTest")[GameTest](https://minecraft.wiki/w/GameTest "GameTest")<br>  - [Debug properties](https://minecraft.wiki/w/Debug_property "Debug property")<br>- [![](https://minecraft.wiki/images/BlockSprite_computer.png?e0c37)](https://minecraft.wiki/w/Obfuscation_map "Obfuscation map")[Obfuscation map](https://minecraft.wiki/w/Obfuscation_map "Obfuscation map") |
| Sound | - [![](https://minecraft.wiki/images/BlockSprite_jukebox-side.png?8477e)](https://minecraft.wiki/w/Block_sound_type "Block sound type")[Block sound type](https://minecraft.wiki/w/Block_sound_type "Block sound type")<br>- [![](https://minecraft.wiki/images/EnvSprite_ambience.png?d7c92)](https://minecraft.wiki/w/Sounds.json "Sounds.json")[sounds.json](https://minecraft.wiki/w/Sounds.json "Sounds.json")<br>- [![](https://minecraft.wiki/images/EnvSprite_chat.png?0dd92)](https://minecraft.wiki/w/Subtitles "Subtitles")[Subtitles](https://minecraft.wiki/w/Subtitles "Subtitles") |
| [Commands](https://minecraft.wiki/w/Commands "Commands") | - [Brigadier](https://minecraft.wiki/w/Brigadier "Brigadier")<br>- [![](https://minecraft.wiki/images/BlockSprite_repeating-command-block.png?1dad0)](https://minecraft.wiki/w/Function_(Java_Edition) "Function (Java Edition)")[Functions](https://minecraft.wiki/w/Function_(Java_Edition) "Function (Java Edition)")<br>**[All commands](https://minecraft.wiki/w/Template:Navbox_commands "Template:Navbox commands")** |
| [Launching](https://minecraft.wiki/w/Minecraft_Launcher "Minecraft Launcher") | - [Mojang API](https://minecraft.wiki/w/Mojang_API "Mojang API")
- [![](https://minecraft.wiki/images/Microsoft_logo.svg?7e87a)](https://minecraft.wiki/w/Microsoft_authentication "Microsoft authentication")[Microsoft authentication](https://minecraft.wiki/w/Microsoft_authentication "Microsoft authentication")
- [![](https://minecraft.wiki/images/thumb/Java_Edition_icon_3.png/16px-Java_Edition_icon_3.png?f7112)](https://minecraft.wiki/w/Quick_Play "Quick Play")[Quick Play](https://minecraft.wiki/w/Quick_Play "Quick Play")

| Legacy | - [Legacy Minecraft authentication](https://minecraft.wiki/w/Legacy_Minecraft_authentication "Legacy Minecraft authentication")<br>- [Yggdrasil](https://minecraft.wiki/w/Yggdrasil "Yggdrasil") | |
| [Protocol](https://minecraft.wiki/w/Java_Edition_protocol "Java Edition protocol") | - [![](https://minecraft.wiki/images/BlockSprite_number-block.png?0d8c7)](https://minecraft.wiki/w/Protocol_version "Protocol version")[Protocol version](https://minecraft.wiki/w/Protocol_version "Protocol version")<br>- [![](https://minecraft.wiki/images/ItemSprite_bundle.png?9eb9f)](https://minecraft.wiki/w/Java_Edition_protocol/Packets "Java Edition protocol/Packets")Packets<br>- [Data types](https://minecraft.wiki/w/Java_Edition_protocol/Data_types "Java Edition protocol/Data types")<br>- [![](https://minecraft.wiki/images/BlockSprite_computer.png?e0c37)](https://minecraft.wiki/w/Java_Edition_protocol/Encryption "Java Edition protocol/Encryption")[Encryption](https://minecraft.wiki/w/Java_Edition_protocol/Encryption "Java Edition protocol/Encryption") |
| [Server](https://minecraft.wiki/w/Server "Server") | - ![File archive.png: Sprite image for archive in Minecraft](https://minecraft.wiki/images/thumb/File_archive.png/16px-File_archive.png?5ba7d)[server.jar](https://minecraft.wiki/w/Server.jar "Server.jar")<br>- [![](https://minecraft.wiki/images/BlockSprite_test-instance-block.png?27a39)](https://minecraft.wiki/w/Server.properties "Server.properties")[server.properties](https://minecraft.wiki/w/Server.properties "Server.properties")<br>- [![](https://minecraft.wiki/images/BlockSprite_test-instance-block.png?27a39)](https://minecraft.wiki/w/Server/Requirements "Server/Requirements")[Server requirements](https://minecraft.wiki/w/Server/Requirements "Server/Requirements")<br>- [![](https://minecraft.wiki/images/BlockSprite_test-block-accept.png?08355)](https://minecraft.wiki/w/Whitelist "Whitelist")[Whitelist](https://minecraft.wiki/w/Whitelist "Whitelist")<br>- [Operator list](https://minecraft.wiki/w/Server#Operator_list "Server")<br>- [Query](https://minecraft.wiki/w/Query "Query")<br>- [RCON](https://minecraft.wiki/w/RCON "RCON")<br>- [Server Management Protocol](https://minecraft.wiki/w/Minecraft_Server_Management_Protocol "Minecraft Server Management Protocol") |
| Legacy | - [al\_version](https://minecraft.wiki/w/Al_version "Al version")<br>- [![](https://minecraft.wiki/images/EnvSprite_item.png?89d23)](https://minecraft.wiki/w/Item_format "Item format")[Item format](https://minecraft.wiki/w/Item_format "Item format") | |
| | \[show\][Data pack](https://minecraft.wiki/w/Data_pack "Data pack") |
| --- |
| Components | - [pack.mcmeta](https://minecraft.wiki/w/Pack.mcmeta "Pack.mcmeta")
- [![](https://minecraft.wiki/images/BlockSprite_number-block.png?0d8c7)](https://minecraft.wiki/w/Pack_format "Pack format")[Pack format](https://minecraft.wiki/w/Pack_format "Pack format")
- [![](https://minecraft.wiki/images/ItemSprite_map.png?05f8c)](https://minecraft.wiki/w/Advancement_definition "Advancement definition")[Advancements](https://minecraft.wiki/w/Advancement_definition "Advancement definition")
- [![](https://minecraft.wiki/images/BlockSprite_repeating-command-block.png?1dad0)](https://minecraft.wiki/w/Function_(Java_Edition) "Function (Java Edition)")[Functions](https://minecraft.wiki/w/Function_(Java_Edition) "Function (Java Edition)")
- [![](https://minecraft.wiki/images/BlockSprite_red-banner.png?cd672)](https://minecraft.wiki/w/Item_modifier "Item modifier")[Item modifier](https://minecraft.wiki/w/Item_modifier "Item modifier")
- [![](https://minecraft.wiki/images/ItemSprite_diamond.png?8f019)](https://minecraft.wiki/w/Loot_table "Loot table")[Loot tables](https://minecraft.wiki/w/Loot_table "Loot table")
- [![](https://minecraft.wiki/images/BlockSprite_test-block-start.png?35191)](https://minecraft.wiki/w/Predicate "Predicate")[Predicate](https://minecraft.wiki/w/Predicate "Predicate")
- [![](https://minecraft.wiki/images/BlockSprite_crafting-table.png?6e126)](https://minecraft.wiki/w/Recipe "Recipe")[Recipe](https://minecraft.wiki/w/Recipe "Recipe")
- [![](https://minecraft.wiki/images/EffectSprite_strength.png?05e79)](https://minecraft.wiki/w/Damage_type "Damage type")[Damage type](https://minecraft.wiki/w/Damage_type "Damage type")
- [![](https://minecraft.wiki/images/EnvSprite_chat.png?0dd92)](https://minecraft.wiki/w/Chat_type "Chat type")[Chat type](https://minecraft.wiki/w/Chat_type "Chat type")
- [![](https://minecraft.wiki/images/ItemSprite_enchanted-book.png?b7877)](https://minecraft.wiki/w/Enchantment_definition "Enchantment definition")[Enchantment](https://minecraft.wiki/w/Enchantment_definition "Enchantment definition")
- [![](https://minecraft.wiki/images/BlockSprite_enchanting-table.png?45e2c)](https://minecraft.wiki/w/Enchantment_provider "Enchantment provider")[Enchantment provider](https://minecraft.wiki/w/Enchantment_provider "Enchantment provider")
- [![](https://minecraft.wiki/images/ItemSprite_painting.png?55d20)](https://minecraft.wiki/w/Painting_variant_definition "Painting variant definition")[Painting variant](https://minecraft.wiki/w/Painting_variant_definition "Painting variant definition")
- [![](https://minecraft.wiki/images/BlockSprite_white-banner.png?9b133)](https://minecraft.wiki/w/Banner_pattern_definition "Banner pattern definition")[Banner pattern](https://minecraft.wiki/w/Banner_pattern_definition "Banner pattern definition")
- [![](https://minecraft.wiki/images/ItemSprite_goat-horn.png?e5a9f)](https://minecraft.wiki/w/Goat_horn_instrument_definition "Goat horn instrument definition")[Goat horn instrument](https://minecraft.wiki/w/Goat_horn_instrument_definition "Goat horn instrument definition")
- [![](https://minecraft.wiki/images/BlockSprite_jukebox.png?86205)](https://minecraft.wiki/w/Jukebox_song_definition "Jukebox song definition")[Jukebox song](https://minecraft.wiki/w/Jukebox_song_definition "Jukebox song definition")
- [![](https://minecraft.wiki/images/BlockSprite_trial-spawner.png?0a3dc)](https://minecraft.wiki/w/Trial_spawner_configuration "Trial spawner configuration")[Trial spawner configuration](https://minecraft.wiki/w/Trial_spawner_configuration "Trial spawner configuration")
- [![](https://minecraft.wiki/images/EntitySprite_pig.png?5435e)](https://minecraft.wiki/w/Mob_variant_definitions "Mob variant definitions")[Mob variants](https://minecraft.wiki/w/Mob_variant_definitions "Mob variant definitions")
- [![](https://minecraft.wiki/images/ItemSprite_paper.png?565a1)](https://minecraft.wiki/w/Dialog "Dialog")[Dialog](https://minecraft.wiki/w/Dialog "Dialog")
- [![](https://minecraft.wiki/images/ItemSprite_wayfinder-armor-trim.png?ffaf0)](https://minecraft.wiki/w/Armor_trim_definition "Armor trim definition")[Armor trim](https://minecraft.wiki/w/Armor_trim_definition "Armor trim definition")
- [Slot sources](https://minecraft.wiki/w/Slot_sources "Slot sources") ​\[ _upcoming [JE 1.21.11](https://minecraft.wiki/w/Java_Edition_1.21.11 "Java Edition 1.21.11")_\]

| [Tag](https://minecraft.wiki/w/Tag_(Java_Edition) "Tag (Java Edition)") | - [![](https://minecraft.wiki/images/BlockSprite_grass-block.png?97c2e)](https://minecraft.wiki/w/Block_tag_(Java_Edition) "Block tag (Java Edition)")[Block](https://minecraft.wiki/w/Block_tag_(Java_Edition) "Block tag (Java Edition)")<br>- [![](https://minecraft.wiki/images/EnvSprite_item.png?89d23)](https://minecraft.wiki/w/Item_tag_(Java_Edition) "Item tag (Java Edition)")[Item](https://minecraft.wiki/w/Item_tag_(Java_Edition) "Item tag (Java Edition)")<br>- [![](https://minecraft.wiki/images/BlockSprite_repeating-command-block.png?1dad0)](https://minecraft.wiki/w/Function_tag_(Java_Edition) "Function tag (Java Edition)")[Function](https://minecraft.wiki/w/Function_tag_(Java_Edition) "Function tag (Java Edition)")<br>- [![](https://minecraft.wiki/images/ItemSprite_water-bucket.png?6e72b)](https://minecraft.wiki/w/Fluid_tag_(Java_Edition) "Fluid tag (Java Edition)")[Fluid](https://minecraft.wiki/w/Fluid_tag_(Java_Edition) "Fluid tag (Java Edition)")<br>- [![](https://minecraft.wiki/images/EnvSprite_entities.png?94711)](https://minecraft.wiki/w/Entity_type_tag_(Java_Edition) "Entity type tag (Java Edition)")[Entity type](https://minecraft.wiki/w/Entity_type_tag_(Java_Edition) "Entity type tag (Java Edition)")<br>- [![](https://minecraft.wiki/images/BlockSprite_sculk-sensor.png?ccbdb)](https://minecraft.wiki/w/Game_event_tag_(Java_Edition) "Game event tag (Java Edition)")[Game event](https://minecraft.wiki/w/Game_event_tag_(Java_Edition) "Game event tag (Java Edition)")<br>- [![](https://minecraft.wiki/images/BiomeSprite_forest.png?abd89)](https://minecraft.wiki/w/Biome_tag_(Java_Edition) "Biome tag (Java Edition)")[Biome](https://minecraft.wiki/w/Biome_tag_(Java_Edition) "Biome tag (Java Edition)")<br>- [![](https://minecraft.wiki/images/EnvSprite_superflat.png?54c14)](https://minecraft.wiki/w/Flat_level_generator_preset_tag_(Java_Edition) "Flat level generator preset tag (Java Edition)")[Flat level generator preset](https://minecraft.wiki/w/Flat_level_generator_preset_tag_(Java_Edition) "Flat level generator preset tag (Java Edition)")<br>- [![](https://minecraft.wiki/images/EnvSprite_mountain.png?2e0ae)](https://minecraft.wiki/w/World_preset_tag_(Java_Edition) "World preset tag (Java Edition)")[World preset](https://minecraft.wiki/w/World_preset_tag_(Java_Edition) "World preset tag (Java Edition)")<br>- [![](https://minecraft.wiki/images/EnvSprite_jungle-pyramid.png?736e3)](https://minecraft.wiki/w/Structure_tag_(Java_Edition) "Structure tag (Java Edition)")[Structure](https://minecraft.wiki/w/Structure_tag_(Java_Edition) "Structure tag (Java Edition)")<br>- [![](https://minecraft.wiki/images/BlockSprite_lodestone.png?00f1a)](https://minecraft.wiki/w/Point_of_interest_type_tag_(Java_Edition) "Point of interest type tag (Java Edition)")[Point of interest type](https://minecraft.wiki/w/Point_of_interest_type_tag_(Java_Edition) "Point of interest type tag (Java Edition)")<br>- [![](https://minecraft.wiki/images/ItemSprite_painting.png?55d20)](https://minecraft.wiki/w/Painting_variant_tag_(Java_Edition) "Painting variant tag (Java Edition)")[Painting variant](https://minecraft.wiki/w/Painting_variant_tag_(Java_Edition) "Painting variant tag (Java Edition)")<br>- [![](https://minecraft.wiki/images/BlockSprite_white-banner.png?9b133)](https://minecraft.wiki/w/Banner_pattern_tag_(Java_Edition) "Banner pattern tag (Java Edition)")[Banner pattern](https://minecraft.wiki/w/Banner_pattern_tag_(Java_Edition) "Banner pattern tag (Java Edition)")<br>- [![](https://minecraft.wiki/images/ItemSprite_goat-horn.png?e5a9f)](https://minecraft.wiki/w/Instrument_tag_(Java_Edition) "Instrument tag (Java Edition)")[Instrument](https://minecraft.wiki/w/Instrument_tag_(Java_Edition) "Instrument tag (Java Edition)")<br>- ![❤️](https://minecraft.wiki/images/Heart_%28icon%29.png?faf83)[Damage type](https://minecraft.wiki/w/Damage_type_tag_(Java_Edition) "Damage type tag (Java Edition)")<br>- [![](https://minecraft.wiki/images/ItemSprite_enchanted-book.png?b7877)](https://minecraft.wiki/w/Enchantment_tag_(Java_Edition) "Enchantment tag (Java Edition)")[Enchantment](https://minecraft.wiki/w/Enchantment_tag_(Java_Edition) "Enchantment tag (Java Edition)")<br>- [![](https://minecraft.wiki/images/ItemSprite_paper.png?565a1)](https://minecraft.wiki/w/Dialog_tag_(Java_Edition) "Dialog tag (Java Edition)")[Dialog](https://minecraft.wiki/w/Dialog_tag_(Java_Edition) "Dialog tag (Java Edition)") |
| [GameTest](https://minecraft.wiki/w/GameTest "GameTest") | - [![](https://minecraft.wiki/images/BlockSprite_test-block-start.png?35191)](https://minecraft.wiki/w/Test_environment_definition "Test environment definition")[Test environment](https://minecraft.wiki/w/Test_environment_definition "Test environment definition")<br>- [![](https://minecraft.wiki/images/BlockSprite_test-instance-block.png?27a39)](https://minecraft.wiki/w/Test_instance_definition "Test instance definition")[Test instance](https://minecraft.wiki/w/Test_instance_definition "Test instance definition") |
| [World generation](https://minecraft.wiki/w/Custom_world_generation "Custom world generation") | - [Dimension](https://minecraft.wiki/w/Dimension_definition "Dimension definition")
- [![](https://minecraft.wiki/images/EnvSprite_nether-portal.png?47646)](https://minecraft.wiki/w/Dimension_type "Dimension type")[Dimension type](https://minecraft.wiki/w/Dimension_type "Dimension type")
- [![](https://minecraft.wiki/images/EnvSprite_mountain.png?2e0ae)](https://minecraft.wiki/w/World_preset_definition "World preset definition")[World preset](https://minecraft.wiki/w/World_preset_definition "World preset definition")
- [![](https://minecraft.wiki/images/EnvSprite_biomes.png?0a976)](https://minecraft.wiki/w/Biome_definition "Biome definition")[Biomes](https://minecraft.wiki/w/Biome_definition "Biome definition")
- [![](https://minecraft.wiki/images/EnvSprite_cave.png?47a17)](https://minecraft.wiki/w/Carver_definition "Carver definition")[Carver](https://minecraft.wiki/w/Carver_definition "Carver definition")
- [![](https://minecraft.wiki/images/thumb/Wrench.png/16px-Wrench.png?4711e)](https://minecraft.wiki/w/Configured_Feature "Configured Feature")[Configured Feature](https://minecraft.wiki/w/Configured_Feature "Configured Feature")
  - [![](https://minecraft.wiki/images/EnvSprite_oak.png?742a4)](https://minecraft.wiki/w/Tree_definition "Tree definition")[Tree](https://minecraft.wiki/w/Tree_definition "Tree definition")
- [![](https://minecraft.wiki/images/EnvSprite_map.png?b863e)](https://minecraft.wiki/w/Placed_feature "Placed feature")[Placed feature](https://minecraft.wiki/w/Placed_feature "Placed feature")
- [Environment attribute](https://minecraft.wiki/w/Environment_attribute "Environment attribute") ​\[ _upcoming [JE 1.21.11](https://minecraft.wiki/w/Java_Edition_1.21.11 "Java Edition 1.21.11")_\]

| [Noise settings](https://minecraft.wiki/w/Noise_settings "Noise settings") | - [![](https://minecraft.wiki/images/EnvSprite_mountain.png?2e0ae)](https://minecraft.wiki/w/Noise_router "Noise router")[Noise router](https://minecraft.wiki/w/Noise_router "Noise router")<br>- [![](https://minecraft.wiki/images/BlockSprite_number-block.png?0d8c7)](https://minecraft.wiki/w/Density_function "Density function")[Density function](https://minecraft.wiki/w/Density_function "Density function")<br>- [Noises](https://minecraft.wiki/w/Noise "Noise")<br>- [![](https://minecraft.wiki/images/EnvSprite_surface.png?75bf7)](https://minecraft.wiki/w/Surface_rule "Surface rule")[Surface rule](https://minecraft.wiki/w/Surface_rule "Surface rule") |
| [Structures](https://minecraft.wiki/w/Structure_definition "Structure definition") | - [![](https://minecraft.wiki/images/EnvSprite_abandoned-mineshaft.png?fab65)](https://minecraft.wiki/w/Structure_set "Structure set")[Structure set](https://minecraft.wiki/w/Structure_set "Structure set")<br>- [![](https://minecraft.wiki/images/BlockSprite_jigsaw.png?ec5e3)](https://minecraft.wiki/w/Template_pool "Template pool")[Template pool](https://minecraft.wiki/w/Template_pool "Template pool")<br>- [![](https://minecraft.wiki/images/BlockSprite_cracked-stone-bricks.png?f3f1d)](https://minecraft.wiki/w/Processor_list "Processor list")[Processor list](https://minecraft.wiki/w/Processor_list "Processor list")<br>- [![](https://minecraft.wiki/images/EnvSprite_nether-fossil.png?93621)](https://minecraft.wiki/w/Structure_file "Structure file")[Structure templates](https://minecraft.wiki/w/Structure_file "Structure file") |
| Removed | - [![](https://minecraft.wiki/images/ItemSprite_iron-pickaxe.png?77536)](https://minecraft.wiki/w/Configured_surface_builder "Configured surface builder")[Configured surface builder](https://minecraft.wiki/w/Configured_surface_builder "Configured surface builder") | | |
| Data packs | - [![](https://minecraft.wiki/images/BlockSprite_deepslate.png?d7361)](https://minecraft.wiki/w/Caves_%26_Cliffs_Prototype_Data_Pack "Caves & Cliffs Prototype Data Pack")[Caves & Cliffs Prototype Data Pack](https://minecraft.wiki/w/Caves_%26_Cliffs_Prototype_Data_Pack "Caves & Cliffs Prototype Data Pack")<br>- [![](https://minecraft.wiki/images/ItemSprite_magical-painting.png?b0bf0)](https://minecraft.wiki/w/Phantom_Frames "Phantom Frames")[Phantom Frames](https://minecraft.wiki/w/Phantom_Frames "Phantom Frames") |
| Tutorials | - [![](https://minecraft.wiki/images/thumb/EnvSprite_autosave.png/16px-EnvSprite_autosave.png?a55e7)](https://minecraft.wiki/w/Tutorial:Installing_a_data_pack "Tutorial:Installing a data pack")[Installing](https://minecraft.wiki/w/Tutorial:Installing_a_data_pack "Tutorial:Installing a data pack")
- [![](https://minecraft.wiki/images/thumb/Wrench.png/16px-Wrench.png?4711e)](https://minecraft.wiki/w/Tutorial:Creating_a_data_pack "Tutorial:Creating a data pack")[Creating](https://minecraft.wiki/w/Tutorial:Creating_a_data_pack "Tutorial:Creating a data pack")
- [Optimizing](https://minecraft.wiki/w/Tutorial:Optimizing_a_data_pack "Tutorial:Optimizing a data pack")
- [![](https://minecraft.wiki/images/BlockSprite_command-block.png?e7078)](https://minecraft.wiki/w/Tutorial:Command_blocks_and_functions "Tutorial:Command blocks and functions")[Command blocks and functions](https://minecraft.wiki/w/Tutorial:Command_blocks_and_functions "Tutorial:Command blocks and functions")
- [Repairing a world corrupted by a data pack](https://minecraft.wiki/w/Tutorial:Repairing_a_world_corrupted_by_a_data_pack "Tutorial:Repairing a world corrupted by a data pack")

| Content | - [![](https://minecraft.wiki/images/ItemSprite_enchanted-book.png?b7877)](https://minecraft.wiki/w/Tutorial:Adding_custom_enchantments "Tutorial:Adding custom enchantments")[Custom enchantments](https://minecraft.wiki/w/Tutorial:Adding_custom_enchantments "Tutorial:Adding custom enchantments")<br>- [![](https://minecraft.wiki/images/ItemSprite_painting.png?55d20)](https://minecraft.wiki/w/Tutorial:Adding_custom_paintings "Tutorial:Adding custom paintings")[Custom paintings](https://minecraft.wiki/w/Tutorial:Adding_custom_paintings "Tutorial:Adding custom paintings")<br>- [![](https://minecraft.wiki/images/ItemSprite_armor-trim.png?1d672)](https://minecraft.wiki/w/Tutorial:Adding_custom_trims "Tutorial:Adding custom trims")[Custom trims](https://minecraft.wiki/w/Tutorial:Adding_custom_trims "Tutorial:Adding custom trims") |

| World generation | - [![](https://minecraft.wiki/images/EnvSprite_other-portal.png?ca57b)](https://minecraft.wiki/w/Tutorial:Adding_a_new_dimension "Tutorial:Adding a new dimension")[New dimension](https://minecraft.wiki/w/Tutorial:Adding_a_new_dimension "Tutorial:Adding a new dimension")<br>- [![](https://minecraft.wiki/images/EnvSprite_lunar-base.png?648e4)](https://minecraft.wiki/w/Tutorial:Custom_structures "Tutorial:Custom structures")[Custom structures](https://minecraft.wiki/w/Tutorial:Custom_structures "Tutorial:Custom structures") | | |
| | \[show\][Resource pack](https://minecraft.wiki/w/Resource_pack "Resource pack") |
| --- |
| Components | - [pack.mcmeta](https://minecraft.wiki/w/Pack.mcmeta "Pack.mcmeta")<br>- [![](https://minecraft.wiki/images/BlockSprite_number-block.png?0d8c7)](https://minecraft.wiki/w/Pack_format "Pack format")[Pack format](https://minecraft.wiki/w/Pack_format "Pack format")<br>- [![](https://minecraft.wiki/images/EnvSprite_language.png?39da2)](https://minecraft.wiki/w/Resource_pack#Language "Resource pack")[Language](https://minecraft.wiki/w/Resource_pack#Language "Resource pack")<br>- [![](https://minecraft.wiki/images/EntitySprite_creeper.png?703e9)](https://minecraft.wiki/w/Model "Model")[Models](https://minecraft.wiki/w/Model "Model")<br>- [![](https://minecraft.wiki/images/BlockSprite_double-stone-slab.png?62750)](https://minecraft.wiki/w/Blockstates_definition "Blockstates definition")[Blockstates](https://minecraft.wiki/w/Blockstates_definition "Blockstates definition")<br>- [![](https://minecraft.wiki/images/EnvSprite_item.png?89d23)](https://minecraft.wiki/w/Items_model_definition "Items model definition")[Items](https://minecraft.wiki/w/Items_model_definition "Items model definition")<br>- [![](https://minecraft.wiki/images/EnvSprite_ambience.png?d7c92)](https://minecraft.wiki/w/Sound "Sound")[Sounds](https://minecraft.wiki/w/Sound "Sound") ( [sounds.json](https://minecraft.wiki/w/Sounds.json "Sounds.json"))<br>- [Shaders](https://minecraft.wiki/w/Shader "Shader")<br>- [![](https://minecraft.wiki/images/EnvSprite_texture-pack.png?a4213)](https://minecraft.wiki/w/Resource_pack#Textures "Resource pack")[Textures](https://minecraft.wiki/w/Resource_pack#Textures "Resource pack")<br>- [![](https://minecraft.wiki/images/ItemSprite_compass.png?2364d)](https://minecraft.wiki/w/Atlas "Atlas")[Atlases](https://minecraft.wiki/w/Atlas "Atlas")<br>- [Aa](https://minecraft.wiki/w/Font "Font") [Fonts](https://minecraft.wiki/w/Font "Font")<br>- [![](https://minecraft.wiki/images/BlockSprite_oak-leaves.png?81553)](https://minecraft.wiki/w/Resource_pack#Colormaps "Resource pack")[Colormaps](https://minecraft.wiki/w/Resource_pack#Colormaps "Resource pack")<br>- ![File file.png: Sprite image for file in Minecraft](https://minecraft.wiki/images/thumb/File_file.png/16px-File_file.png?e19ce)[Texts](https://minecraft.wiki/w/Resource_pack#Texts "Resource pack")<br>- [![](https://minecraft.wiki/images/Locator_Bar_icon_bowtie.png?a8cd8)](https://minecraft.wiki/w/Waypoint_style "Waypoint style")[Waypoint styles](https://minecraft.wiki/w/Waypoint_style "Waypoint style")<br>- [regional\_compliancies.json](https://minecraft.wiki/w/Resource_pack#Regional_compliancies_warnings "Resource pack")<br>- [![](https://minecraft.wiki/images/ItemSprite_all-iron-armor.png?87e31)](https://minecraft.wiki/w/Equipment "Equipment")[Equipment](https://minecraft.wiki/w/Equipment "Equipment") |
| Debug | - [Missing font character](https://minecraft.wiki/w/Missing_font_character "Missing font character")<br>- [![](https://minecraft.wiki/images/BlockSprite_missingno.png?031f4)](https://minecraft.wiki/w/Missing_model "Missing model")[Missing model](https://minecraft.wiki/w/Missing_model "Missing model")<br>- [![](https://minecraft.wiki/images/BlockSprite_missingno.png?031f4)](https://minecraft.wiki/w/Missing_texture "Missing texture")[Missing texture](https://minecraft.wiki/w/Missing_texture "Missing texture") |
| Tutorials | - [![](https://minecraft.wiki/images/thumb/Wrench.png/16px-Wrench.png?4711e)](https://minecraft.wiki/w/Tutorial:Creating_a_resource_pack "Tutorial:Creating a resource pack")[Creating](https://minecraft.wiki/w/Tutorial:Creating_a_resource_pack "Tutorial:Creating a resource pack")<br>- [![](https://minecraft.wiki/images/Download.png?048e3)](https://minecraft.wiki/w/Tutorial:Loading_a_resource_pack "Tutorial:Loading a resource pack")[Loading](https://minecraft.wiki/w/Tutorial:Loading_a_resource_pack "Tutorial:Loading a resource pack")<br>- [![](https://minecraft.wiki/images/EnvSprite_fluids.png?58a6a)](https://minecraft.wiki/w/Tutorial:Models "Tutorial:Models")[Models](https://minecraft.wiki/w/Tutorial:Models "Tutorial:Models")<br>- [![](https://minecraft.wiki/images/EnvSprite_ambience.png?d7c92)](https://minecraft.wiki/w/Tutorial:Sound_directory "Tutorial:Sound directory")[Sound directory](https://minecraft.wiki/w/Tutorial:Sound_directory "Tutorial:Sound directory") | |

[![](https://minecraft.wiki/images/License_cc.svg?7c689)](https://minecraft.wiki/w/File:License_cc.svg)

[![](https://minecraft.wiki/images/License_cc-by.svg?818e9)](https://minecraft.wiki/w/File:License_cc-by.svg)[![](https://minecraft.wiki/images/License_cc-by-sa.svg?75cd1)](https://minecraft.wiki/w/File:License_cc-by-sa.svg)

This article is licensed under a [Creative Commons Attribution-ShareAlike 3.0](http://creativecommons.org/licenses/by-sa/3.0/) license.

This article has been imported from [wiki.vg](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge "Minecraft Wiki:Projects/wiki.vg merge") or is a derivative of such a page. Thus, the wiki's [usual license](https://minecraft.wiki/w/Minecraft_Wiki:Copyrights "Minecraft Wiki:Copyrights") does not apply.

Derivative works must be licensed using the same or a compatible license.

Retrieved from " [https://minecraft.wiki/w/Java\_Edition\_protocol/Packets?oldid=3250085](https://minecraft.wiki/w/Java_Edition_protocol/Packets?oldid=3250085)"

[Categories](https://minecraft.wiki/w/Special:Categories "Special:Categories"):

- [Java Edition](https://minecraft.wiki/w/Category:Java_Edition "Category:Java Edition")
- [Java Edition technical](https://minecraft.wiki/w/Category:Java_Edition_technical "Category:Java Edition technical")
- [Protocol Details](https://minecraft.wiki/w/Category:Protocol_Details "Category:Protocol Details")
- [Java Edition protocol](https://minecraft.wiki/w/Category:Java_Edition_protocol "Category:Java Edition protocol")
- [CC-by-SA pages](https://minecraft.wiki/w/Category:CC-by-SA_pages "Category:CC-by-SA pages")

Hidden categories:

- [Articles to be expanded](https://minecraft.wiki/w/Category:Articles_to_be_expanded "Category:Articles to be expanded")
- [Upcoming](https://minecraft.wiki/w/Category:Upcoming "Category:Upcoming")
- [Java Edition upcoming tag](https://minecraft.wiki/w/Category:Java_Edition_upcoming_tag "Category:Java Edition upcoming tag")

## Navigation menu

### Personal tools

- [Toggle fixed width](https://minecraft.wiki/w/Java_Edition_protocol/Packets "Toggle fixed width")
- [Toggle dark mode](https://minecraft.wiki/w/Java_Edition_protocol/Packets "Toggle dark mode")
- Not logged in
- [Talk](https://minecraft.wiki/w/Special:MyTalk "Talk about edits from this IP address [alt-shift-n]")
- [Contributions](https://minecraft.wiki/w/Special:MyContributions "A list of edits made from this IP address [alt-shift-y]")
- [Create account](https://minecraft.wiki/w/Special:CreateAccount?returnto=Java+Edition+protocol%2FPackets "You are encouraged to create an account and log in; however, it is not mandatory")
- [Log in](https://minecraft.wiki/w/Special:UserLogin?returnto=Java+Edition+protocol%2FPackets "You are encouraged to log in; however, it is not mandatory [alt-shift-o]")

### Namespaces

- [Page](https://minecraft.wiki/w/Java_Edition_protocol/Packets "View the content page [alt-shift-c]")
- [Talk](https://minecraft.wiki/w/Talk:Java_Edition_protocol/Packets "Talk about the content page [alt-shift-t]")

English

### Views

More

- [Read](https://minecraft.wiki/w/Java_Edition_protocol/Packets)
- [Edit](https://minecraft.wiki/w/Java_Edition_protocol/Packets?veaction=edit&editintro=Template:License_wiki.vg "Edit this page [alt-shift-v]")
- [Edit source](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=edit&editintro=Template:License_wiki.vg "Edit the source code of this page [alt-shift-e]")
- [View history](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=history "Past revisions of this page [alt-shift-h]")
- [Refresh](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=purge "Purge this page")

Search

[Visit the main page](https://minecraft.wiki/ "Visit the main page")

### Navigation

- [How to help](https://minecraft.wiki/w/Minecraft_Wiki:How_to_help)
- [Recent changes](https://minecraft.wiki/w/Special:RecentChanges "A list of recent changes in the wiki [alt-shift-r]")
- [Random page](https://minecraft.wiki/w/Special:BlankPage/random?fallback=Special:RandomRootpage "Load a random page [alt-shift-x]")
- [in Minecraft](https://minecraft.wiki/w/Special:BlankPage/random/Main?fallback=Special:RandomRootpage/Main)
- [in Dungeons](https://minecraft.wiki/w/Special:BlankPage/random/Dungeons?fallback=Special:RandomRootpage/MCD)
- [in Legends](https://minecraft.wiki/w/Special:BlankPage/random/Legends?fallback=Special:RandomRootpage/MCL)
- [in Earth](https://minecraft.wiki/w/Special:BlankPage/random/Earth?fallback=Special:RandomRootpage/MCE)
- [in Story Mode](https://minecraft.wiki/w/Special:BlankPage/random/Story_Mode?fallback=Special:RandomRootpage/MCSM)
- [in Tutorial](https://minecraft.wiki/w/Special:RandomRootpage/Tutorial)
- [in Movie](https://minecraft.wiki/w/Special:RandomRootpage/Movie)
- [Wiki Discord](https://discord.gg/fGdE5ZE)

### Wiki community

- [Community portal](https://minecraft.wiki/w/Minecraft_Wiki:Community_portal "About the Minecraft Wiki, what you can do, where to find things")
- [Wiki forum](https://minecraft.wiki/w/Minecraft_Wiki:Forum)
- [Projects](https://minecraft.wiki/w/Minecraft_Wiki:Projects)
- [Wiki rules](https://minecraft.wiki/w/Minecraft_Wiki:Wiki_rules)
- [Style guide](https://minecraft.wiki/w/Minecraft_Wiki:Style_guide)
- [Notability](https://minecraft.wiki/w/Minecraft_Wiki:Notability)
- [Sandbox](https://minecraft.wiki/w/Minecraft_Wiki:Sandbox)
- [Admin noticeboard](https://minecraft.wiki/w/Minecraft_Wiki:Admin_noticeboard)

[X](https://minecraft.wiki/w/Minecraft_Wiki:Indie_Wiki_Buddy)

### Franchise

- [Minecraft](https://minecraft.wiki/)
- [Minecraft Dungeons](https://minecraft.wiki/w/Dungeons:Wiki)
- [Minecraft Legends](https://minecraft.wiki/w/Legends:Wiki)
- [Minecraft Earth](https://minecraft.wiki/w/Earth:Wiki)
- [Minecraft: Story Mode](https://minecraft.wiki/w/Story_Mode:Wiki)
- [A Minecraft Movie](https://minecraft.wiki/w/Movie:Wiki)

### Recent versions

- [Java Edition](https://minecraft.wiki/w/Java_Edition "Java Edition")
- [Latest: 1.21.10](https://minecraft.wiki/w/Java_Edition_1.21.10 "Java Edition 1.21.10")
- [Dev: 25w44a](https://minecraft.wiki/w/Java_Edition_25w44a "Java Edition 25w44a")
- [Upcoming: 1.21.11](https://minecraft.wiki/w/Java_Edition_1.21.11 "Java Edition 1.21.11")
- [Bedrock Edition](https://minecraft.wiki/w/Bedrock_Edition "Bedrock Edition")
- [Latest: 1.21.120](https://minecraft.wiki/w/Bedrock_Edition_1.21.120 "Bedrock Edition 1.21.120")
- [Preview: 1.21.130.25](https://minecraft.wiki/w/Bedrock_Edition_Preview_1.21.130.25 "Bedrock Edition Preview 1.21.130.25")
- [Upcoming: 1.21.130](https://minecraft.wiki/w/Bedrock_Edition_1.21.130 "Bedrock Edition 1.21.130")

### Useful pages

- [Trading](https://minecraft.wiki/w/Trading)
- [Brewing](https://minecraft.wiki/w/Brewing)
- [Enchanting](https://minecraft.wiki/w/Enchanting)
- [Mobs](https://minecraft.wiki/w/Mob)
- [Blocks](https://minecraft.wiki/w/Block)
- [Items](https://minecraft.wiki/w/Item)
- [Crafting](https://minecraft.wiki/w/Crafting)
- [Smelting](https://minecraft.wiki/w/Smelting)
- [Tutorials](https://minecraft.wiki/w/Tutorials)
- [Resource packs](https://minecraft.wiki/w/Resource_pack)
- [Redstone circuits](https://minecraft.wiki/w/Redstone_circuits)

### Useful pages

- [Weapons](https://minecraft.wiki/w/Dungeons:Weapon)
- [Enchanting](https://minecraft.wiki/w/Dungeons:Enchanting)
- [Armor](https://minecraft.wiki/w/Dungeons:Armor)
- [Artifacts](https://minecraft.wiki/w/Dungeons:Artifact)
- [Missions](https://minecraft.wiki/w/Dungeons:Mission_Select)

### Useful pages

- [Friendly units](https://minecraft.wiki/w/Legends:Mob#Allied)
- [Piglins](https://minecraft.wiki/w/Legends:Mob#Hostile)
- [Resources](https://minecraft.wiki/w/Legends:Resource)
- [Biomes](https://minecraft.wiki/w/Legends:Biome)
- [Structures](https://minecraft.wiki/w/Legends:Structure)
- [Lost Legends](https://minecraft.wiki/w/Legends:Lost_Legend)

### Tools

- [What links here](https://minecraft.wiki/w/Special:WhatLinksHere/Java_Edition_protocol/Packets "A list of all wiki pages that link here [alt-shift-j]")
- [Related changes](https://minecraft.wiki/w/Special:RecentChangesLinked/Java_Edition_protocol/Packets "Recent changes in pages linked from this page [alt-shift-k]")
- [Special pages](https://minecraft.wiki/w/Special:SpecialPages "A list of all special pages [alt-shift-q]")
- Printable version
- [Permanent link](https://minecraft.wiki/w/Java_Edition_protocol/Packets?oldid=3250085 "Permanent link to this revision of this page")
- [Page information](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=info "More information about this page")
- [View buckets](https://minecraft.wiki/w/Java_Edition_protocol/Packets?action=bucket "Bucket")

[Not for humans!](https://minecraft.wiki/clanker)

### In other languages

- [Deutsch](https://de.minecraft.wiki/w/Minecraft-Server-Protokoll/Pakete "Minecraft-Server-Protokoll/Pakete – Deutsch")

### Contents

- [(Top)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#)
- [1Definitions](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Definitions)
  - [1.1Data types](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Data_types)
  - [1.2Identifier](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Identifier)
  - [1.3VarInt and VarLong](https://minecraft.wiki/w/Java_Edition_protocol/Packets#VarInt_and_VarLong)
  - [1.4Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Position)
  - [1.5Fixed-point numbers](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Fixed-point_numbers)
  - [1.6Arrays](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Arrays)
    - [1.6.1Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Array)
    - [1.6.2Prefixed Array](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Prefixed_Array)
  - [1.7Bit sets](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Bit_sets)
    - [1.7.1BitSet](https://minecraft.wiki/w/Java_Edition_protocol/Packets#BitSet)
    - [1.7.2Fixed BitSet](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Fixed_BitSet)
  - [1.8Registry references](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Registry_references)
    - [1.8.1ID or X](https://minecraft.wiki/w/Java_Edition_protocol/Packets#ID_or_X)
    - [1.8.2ID Set](https://minecraft.wiki/w/Java_Edition_protocol/Packets#ID_Set)
  - [1.9Registry data](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Registry_data)
    - [1.9.1Sound Event](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Sound_Event)
    - [1.9.2Chat Type](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Chat_Type)
  - [1.10Teleport Flags](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Teleport_Flags)
  - [1.11Chunk Data](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Chunk_Data)
  - [1.12Light Data](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Light_Data)
  - [1.13Game Profile](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Game_Profile)
    - [1.13.1Debug Subscription Event](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Debug_Subscription_Event)
    - [1.13.2Debug Subscription Update](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Debug_Subscription_Update)
    - [1.13.3Debug Subscription Data](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Debug_Subscription_Data)
    - [1.13.4Debug Path Node](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Debug_Path_Node)
    - [1.13.5Debug Structure Info](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Debug_Structure_Info)
  - [1.14Other definitions](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Other_definitions)
- [2Packet format](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Packet_format)
  - [2.1Without compression](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Without_compression)
  - [2.2With compression](https://minecraft.wiki/w/Java_Edition_protocol/Packets#With_compression)
- [3Handshaking](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Handshaking)
  - [3.1Clientbound](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Clientbound)
  - [3.2Serverbound](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Serverbound)
    - [3.2.1Handshake](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Handshake)
    - [3.2.2Legacy Server List Ping](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Legacy_Server_List_Ping)
- [4Status](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Status)
  - [4.1Clientbound](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Clientbound_2)
    - [4.1.1Status Response](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Status_Response)
    - [4.1.2Pong Response (status)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Pong_Response_(status))
  - [4.2Serverbound](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Serverbound_2)
    - [4.2.1Status Request](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Status_Request)
    - [4.2.2Ping Request (status)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Ping_Request_(status))
- [5Login](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Login)
  - [5.1Clientbound](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Clientbound_3)
    - [5.1.1Disconnect (login)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Disconnect_(login))
    - [5.1.2Encryption Request](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Encryption_Request)
    - [5.1.3Login Success](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Login_Success)
    - [5.1.4Set Compression](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Compression)
    - [5.1.5Login Plugin Request](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Login_Plugin_Request)
    - [5.1.6Cookie Request (login)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Cookie_Request_(login))
  - [5.2Serverbound](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Serverbound_3)
    - [5.2.1Login Start](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Login_Start)
    - [5.2.2Encryption Response](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Encryption_Response)
    - [5.2.3Login Plugin Response](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Login_Plugin_Response)
    - [5.2.4Login Acknowledged](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Login_Acknowledged)
    - [5.2.5Cookie Response (login)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Cookie_Response_(login))
- [6Configuration](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Configuration)
  - [6.1Clientbound](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Clientbound_4)
    - [6.1.1Cookie Request (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Cookie_Request_(configuration))
    - [6.1.2Clientbound Plugin Message (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Clientbound_Plugin_Message_(configuration))
    - [6.1.3Disconnect (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Disconnect_(configuration))
    - [6.1.4Finish Configuration](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Finish_Configuration)
    - [6.1.5Clientbound Keep Alive (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Clientbound_Keep_Alive_(configuration))
    - [6.1.6Ping (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Ping_(configuration))
    - [6.1.7Reset Chat](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Reset_Chat)
    - [6.1.8Registry Data](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Registry_Data_2)
    - [6.1.9Remove Resource Pack (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Remove_Resource_Pack_(configuration))
    - [6.1.10Add Resource Pack (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Add_Resource_Pack_(configuration))
    - [6.1.11Store Cookie (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Store_Cookie_(configuration))
    - [6.1.12Transfer (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Transfer_(configuration))
    - [6.1.13Feature Flags](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Feature_Flags)
    - [6.1.14Update Tags (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Update_Tags_(configuration))
    - [6.1.15Clientbound Known Packs](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Clientbound_Known_Packs)
    - [6.1.16Custom Report Details (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Custom_Report_Details_(configuration))
    - [6.1.17Server Links (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Server_Links_(configuration))
    - [6.1.18Clear Dialog (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Clear_Dialog_(configuration))
    - [6.1.19Show Dialog (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Show_Dialog_(configuration))
  - [6.2Serverbound](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Serverbound_4)
    - [6.2.1Client Information (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Client_Information_(configuration))
    - [6.2.2Cookie Response (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Cookie_Response_(configuration))
    - [6.2.3Serverbound Plugin Message (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Serverbound_Plugin_Message_(configuration))
    - [6.2.4Acknowledge Finish Configuration](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Acknowledge_Finish_Configuration)
    - [6.2.5Serverbound Keep Alive (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Serverbound_Keep_Alive_(configuration))
    - [6.2.6Pong (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Pong_(configuration))
    - [6.2.7Resource Pack Response (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Resource_Pack_Response_(configuration))
    - [6.2.8Serverbound Known Packs](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Serverbound_Known_Packs)
    - [6.2.9Custom Click Action (configuration)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Custom_Click_Action_(configuration))
- [7Play](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Play)
  - [7.1Clientbound](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Clientbound_5)
    - [7.1.1Bundle Delimiter](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Bundle_Delimiter)
    - [7.1.2Spawn Entity](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Spawn_Entity)
    - [7.1.3Entity Animation](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Entity_Animation)
    - [7.1.4Award Statistics](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Award_Statistics)
    - [7.1.5Acknowledge Block Change](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Acknowledge_Block_Change)
    - [7.1.6Set Block Destroy Stage](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Block_Destroy_Stage)
    - [7.1.7Block Entity Data](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Block_Entity_Data)
    - [7.1.8Block Action](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Block_Action)
    - [7.1.9Block Update](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Block_Update)
    - [7.1.10Boss Bar](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Boss_Bar)
    - [7.1.11Change Difficulty](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Change_Difficulty)
    - [7.1.12Chunk Batch Finished](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Chunk_Batch_Finished)
    - [7.1.13Chunk Batch Start](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Chunk_Batch_Start)
    - [7.1.14Chunk Biomes](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Chunk_Biomes)
    - [7.1.15Clear Titles](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Clear_Titles)
    - [7.1.16Command Suggestions Response](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Command_Suggestions_Response)
    - [7.1.17Commands](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Commands)
    - [7.1.18Close Container](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Close_Container)
    - [7.1.19Set Container Content](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Container_Content)
    - [7.1.20Set Container Property](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Container_Property)
    - [7.1.21Set Container Slot](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Container_Slot)
    - [7.1.22Cookie Request (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Cookie_Request_(play))
    - [7.1.23Set Cooldown](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Cooldown)
    - [7.1.24Chat Suggestions](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Chat_Suggestions)
    - [7.1.25Clientbound Plugin Message (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Clientbound_Plugin_Message_(play))
    - [7.1.26Damage Event](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Damage_Event)
    - [7.1.27Debug Sample](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Debug_Sample)
    - [7.1.28Delete Message](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Delete_Message)
    - [7.1.29Disconnect (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Disconnect_(play))
    - [7.1.30Disguised Chat Message](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Disguised_Chat_Message)
    - [7.1.31Entity Event](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Entity_Event)
    - [7.1.32Teleport Entity](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Teleport_Entity)
    - [7.1.33Explosion](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Explosion)
    - [7.1.34Unload Chunk](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Unload_Chunk)
    - [7.1.35Game Event](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Game_Event)
    - [7.1.36Open Horse Screen](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Open_Horse_Screen)
    - [7.1.37Hurt Animation](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Hurt_Animation)
    - [7.1.38Initialize World Border](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Initialize_World_Border)
    - [7.1.39Clientbound Keep Alive (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Clientbound_Keep_Alive_(play))
    - [7.1.40Chunk Data and Update Light](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Chunk_Data_and_Update_Light)
    - [7.1.41World Event](https://minecraft.wiki/w/Java_Edition_protocol/Packets#World_Event)
    - [7.1.42Particle](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Particle)
    - [7.1.43Update Light](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Update_Light)
    - [7.1.44Login (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Login_(play))
    - [7.1.45Map Data](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Map_Data)
    - [7.1.46Merchant Offers](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Merchant_Offers)
    - [7.1.47Update Entity Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Update_Entity_Position)
    - [7.1.48Update Entity Position and Rotation](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Update_Entity_Position_and_Rotation)
    - [7.1.49Move Minecart Along Track](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Move_Minecart_Along_Track)
    - [7.1.50Update Entity Rotation](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Update_Entity_Rotation)
    - [7.1.51Move Vehicle (clientbound)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Move_Vehicle_(clientbound))
    - [7.1.52Open Book](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Open_Book)
    - [7.1.53Open Screen](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Open_Screen)
    - [7.1.54Open Sign Editor](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Open_Sign_Editor)
    - [7.1.55Ping (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Ping_(play))
    - [7.1.56Ping Response (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Ping_Response_(play))
    - [7.1.57Place Ghost Recipe](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Place_Ghost_Recipe)
    - [7.1.58Player Abilities (clientbound)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Player_Abilities_(clientbound))
    - [7.1.59Player Chat Message](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Player_Chat_Message)
    - [7.1.60End Combat](https://minecraft.wiki/w/Java_Edition_protocol/Packets#End_Combat)
    - [7.1.61Enter Combat](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Enter_Combat)
    - [7.1.62Combat Death](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Combat_Death)
    - [7.1.63Player Info Remove](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Player_Info_Remove)
    - [7.1.64Player Info Update](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Player_Info_Update)
    - [7.1.65Look At](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Look_At)
    - [7.1.66Synchronize Player Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Synchronize_Player_Position)
    - [7.1.67Player Rotation](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Player_Rotation)
    - [7.1.68Recipe Book Add](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Recipe_Book_Add)
    - [7.1.69Recipe Book Remove](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Recipe_Book_Remove)
    - [7.1.70Recipe Book Settings](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Recipe_Book_Settings)
    - [7.1.71Remove Entities](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Remove_Entities)
    - [7.1.72Remove Entity Effect](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Remove_Entity_Effect)
    - [7.1.73Reset Score](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Reset_Score)
    - [7.1.74Remove Resource Pack (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Remove_Resource_Pack_(play))
    - [7.1.75Add Resource Pack (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Add_Resource_Pack_(play))
    - [7.1.76Respawn](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Respawn)
    - [7.1.77Set Head Rotation](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Head_Rotation)
    - [7.1.78Update Section Blocks](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Update_Section_Blocks)
    - [7.1.79Select Advancements Tab](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Select_Advancements_Tab)
    - [7.1.80Server Data](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Server_Data)
    - [7.1.81Set Action Bar Text](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Action_Bar_Text)
    - [7.1.82Set Border Center](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Border_Center)
    - [7.1.83Set Border Lerp Size](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Border_Lerp_Size)
    - [7.1.84Set Border Size](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Border_Size)
    - [7.1.85Set Border Warning Delay](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Border_Warning_Delay)
    - [7.1.86Set Border Warning Distance](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Border_Warning_Distance)
    - [7.1.87Set Camera](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Camera)
    - [7.1.88Set Center Chunk](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Center_Chunk)
    - [7.1.89Set Render Distance](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Render_Distance)
    - [7.1.90Set Cursor Item](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Cursor_Item)
    - [7.1.91Set Default Spawn Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Default_Spawn_Position)
    - [7.1.92Display Objective](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Display_Objective)
    - [7.1.93Set Entity Metadata](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Entity_Metadata)
    - [7.1.94Link Entities](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Link_Entities)
    - [7.1.95Set Entity Velocity](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Entity_Velocity)
    - [7.1.96Set Equipment](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Equipment)
    - [7.1.97Set Experience](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Experience)
    - [7.1.98Set Health](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Health)
    - [7.1.99Set Held Item (clientbound)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Held_Item_(clientbound))
    - [7.1.100Update Objectives](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Update_Objectives)
    - [7.1.101Set Passengers](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Passengers)
    - [7.1.102Set Player Inventory Slot](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Player_Inventory_Slot)
    - [7.1.103Update Teams](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Update_Teams)
    - [7.1.104Update Score](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Update_Score)
    - [7.1.105Set Simulation Distance](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Simulation_Distance)
    - [7.1.106Set Subtitle Text](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Subtitle_Text)
    - [7.1.107Update Time](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Update_Time)
    - [7.1.108Set Title Text](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Title_Text)
    - [7.1.109Set Title Animation Times](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Title_Animation_Times)
    - [7.1.110Entity Sound Effect](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Entity_Sound_Effect)
    - [7.1.111Sound Effect](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Sound_Effect)
    - [7.1.112Start Configuration](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Start_Configuration)
    - [7.1.113Stop Sound](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Stop_Sound)
    - [7.1.114Store Cookie (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Store_Cookie_(play))
    - [7.1.115System Chat Message](https://minecraft.wiki/w/Java_Edition_protocol/Packets#System_Chat_Message)
    - [7.1.116Set Tab List Header And Footer](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Tab_List_Header_And_Footer)
    - [7.1.117Tag Query Response](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Tag_Query_Response)
    - [7.1.118Pickup Item](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Pickup_Item)
    - [7.1.119Synchronize Vehicle Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Synchronize_Vehicle_Position)
    - [7.1.120Test Instance Block Status](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Test_Instance_Block_Status)
    - [7.1.121Set Ticking State](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Ticking_State)
    - [7.1.122Step Tick](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Step_Tick)
    - [7.1.123Transfer (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Transfer_(play))
    - [7.1.124Update Advancements](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Update_Advancements)
    - [7.1.125Update Attributes](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Update_Attributes)
    - [7.1.126Entity Effect](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Entity_Effect)
    - [7.1.127Update Recipes](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Update_Recipes)
    - [7.1.128Update Tags (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Update_Tags_(play))
    - [7.1.129Projectile Power](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Projectile_Power)
    - [7.1.130Custom Report Details](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Custom_Report_Details)
    - [7.1.131Server Links](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Server_Links)
    - [7.1.132Waypoint](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Waypoint)
    - [7.1.133Clear Dialog (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Clear_Dialog_(play))
    - [7.1.134Show Dialog (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Show_Dialog_(play))
  - [7.2Serverbound](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Serverbound_5)
    - [7.2.1Confirm Teleportation](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Confirm_Teleportation)
    - [7.2.2Query Block Entity Tag](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Query_Block_Entity_Tag)
    - [7.2.3Bundle Item Selected](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Bundle_Item_Selected)
    - [7.2.4Change Difficulty](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Change_Difficulty_2)
    - [7.2.5Change Game Mode](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Change_Game_Mode)
    - [7.2.6Acknowledge Message](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Acknowledge_Message)
    - [7.2.7Chat Command](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Chat_Command)
    - [7.2.8Signed Chat Command](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Signed_Chat_Command)
    - [7.2.9Chat Message](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Chat_Message)
    - [7.2.10Player Session](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Player_Session)
    - [7.2.11Chunk Batch Received](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Chunk_Batch_Received)
    - [7.2.12Client Status](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Client_Status)
    - [7.2.13Client Tick End](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Client_Tick_End)
    - [7.2.14Client Information (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Client_Information_(play))
    - [7.2.15Command Suggestions Request](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Command_Suggestions_Request)
    - [7.2.16Acknowledge Configuration](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Acknowledge_Configuration)
    - [7.2.17Click Container Button](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Click_Container_Button)
    - [7.2.18Click Container](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Click_Container)
    - [7.2.19Close Container](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Close_Container_2)
    - [7.2.20Change Container Slot State](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Change_Container_Slot_State)
    - [7.2.21Cookie Response (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Cookie_Response_(play))
    - [7.2.22Serverbound Plugin Message (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Serverbound_Plugin_Message_(play))
    - [7.2.23Debug Sample Subscription](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Debug_Sample_Subscription)
    - [7.2.24Edit Book](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Edit_Book)
    - [7.2.25Query Entity Tag](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Query_Entity_Tag)
    - [7.2.26Interact](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Interact)
    - [7.2.27Jigsaw Generate](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Jigsaw_Generate)
    - [7.2.28Serverbound Keep Alive (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Serverbound_Keep_Alive_(play))
    - [7.2.29Lock Difficulty](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Lock_Difficulty)
    - [7.2.30Set Player Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Player_Position)
    - [7.2.31Set Player Position and Rotation](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Player_Position_and_Rotation)
    - [7.2.32Set Player Rotation](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Player_Rotation)
    - [7.2.33Set Player Movement Flags](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Player_Movement_Flags)
    - [7.2.34Move Vehicle (serverbound)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Move_Vehicle_(serverbound))
    - [7.2.35Paddle Boat](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Paddle_Boat)
    - [7.2.36Pick Item From Block](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Pick_Item_From_Block)
    - [7.2.37Pick Item From Entity](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Pick_Item_From_Entity)
    - [7.2.38Ping Request (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Ping_Request_(play))
    - [7.2.39Place Recipe](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Place_Recipe)
    - [7.2.40Player Abilities (serverbound)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Player_Abilities_(serverbound))
    - [7.2.41Player Action](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Player_Action)
    - [7.2.42Player Command](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Player_Command)
    - [7.2.43Player Input](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Player_Input)
    - [7.2.44Player Loaded](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Player_Loaded)
    - [7.2.45Pong (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Pong_(play))
    - [7.2.46Change Recipe Book Settings](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Change_Recipe_Book_Settings)
    - [7.2.47Set Seen Recipe](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Seen_Recipe)
    - [7.2.48Rename Item](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Rename_Item)
    - [7.2.49Resource Pack Response (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Resource_Pack_Response_(play))
    - [7.2.50Seen Advancements](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Seen_Advancements)
    - [7.2.51Select Trade](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Select_Trade)
    - [7.2.52Set Beacon Effect](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Beacon_Effect)
    - [7.2.53Set Held Item (serverbound)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Held_Item_(serverbound))
    - [7.2.54Program Command Block](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Program_Command_Block)
    - [7.2.55Program Command Block Minecart](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Program_Command_Block_Minecart)
    - [7.2.56Set Creative Mode Slot](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Creative_Mode_Slot)
    - [7.2.57Program Jigsaw Block](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Program_Jigsaw_Block)
    - [7.2.58Program Structure Block](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Program_Structure_Block)
    - [7.2.59Set Test Block](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Test_Block)
    - [7.2.60Update Sign](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Update_Sign)
    - [7.2.61Swing Arm](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Swing_Arm)
    - [7.2.62Teleport To Entity](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Teleport_To_Entity)
    - [7.2.63Test Instance Block Action](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Test_Instance_Block_Action)
    - [7.2.64Use Item On](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Use_Item_On)
    - [7.2.65Use Item](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Use_Item)
    - [7.2.66Custom Click Action (play)](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Custom_Click_Action_(play))
- [8Navigation](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Navigation)

- This page was last edited on 4 November 2025, at 13:59.
- Content on this site is licensed under [CC BY-NC-SA 3.0](https://creativecommons.org/licenses/by-nc-sa/3.0/) unless otherwise noted; [additional terms may apply](https://meta.weirdgloop.org/w/Project:Copyrights).

  Not an official Minecraft website. We are not associated with Mojang or Microsoft.

- [Privacy policy](https://weirdgloop.org/privacy)
- [About Minecraft Wiki](https://minecraft.wiki/w/Minecraft_Wiki:About)
- [Disclaimers](https://meta.minecraft.wiki/w/General_disclaimer)
- [Terms of Use](https://weirdgloop.org/terms)
- [Contact Weird Gloop](https://minecraft.wiki/w/Special:Contact)
- [Mobile view](https://minecraft.wiki/w/Java_Edition_protocol/Packets?mobileaction=toggle_view_mobile)
- [Edit preview settings](https://minecraft.wiki/w/Java_Edition_protocol/Packets#)

- [![CC BY-NC-SA 3.0](https://meta.weirdgloop.org/images/Creative_Commons_footer.png)](https://creativecommons.org/licenses/by-nc-sa/3.0/)
- [![Hosted by Weird Gloop](https://meta.weirdgloop.org/images/Weird_Gloop_footer_hosted.png)](https://weirdgloop.org/)

##### Give feedback on this wiki article

**NOTE: Feedback regarding in-game features or directed at the game developers belongs on the [Minecraft Feedback site](https://feedback.minecraft.net/). This feedback form is only for the wiki article.**

Feedback will be submitted anonymously to the wiki community.

More info can be found at [Minecraft Wiki:Article feedback](https://minecraft.wiki/w/Minecraft_Wiki:Article_feedback "Minecraft_Wiki:Article_feedback").

SubmitCancel

### Thank you!

Your feedback has been brought up for discussion on this article's talk page and our [Discord](https://discord.gg/fGdE5ZE) server.
