#![allow(dead_code)]

use crate::domain::{
    player::dto::{RsPlayInfo, TransPartyPlayInfo},
    user_player::dto::UserItem,
};

impl RsPlayInfo {
    pub fn add_inventory_item(&mut self, _item: &UserItem, _make_log: bool) -> i32 {
        unimplemented!("migration placeholder: rsPLAYINFO::AddInventoryItem");
    }

    pub fn create_party(&mut self, _play_info_id: u32) -> i32 {
        unimplemented!("migration placeholder: rsPLAYINFO::CreateParty");
    }

    pub fn join_party_player(&mut self, _play_info_id: u32) -> i32 {
        unimplemented!("migration placeholder: rsPLAYINFO::JoinPartyPlayer");
    }

    pub fn delete_party_player(&mut self, _play_info_id: u32) -> i32 {
        unimplemented!("migration placeholder: rsPLAYINFO::DeletePartyPlayer");
    }

    pub fn change_party_player(&mut self, _old_play_info_id: u32, _new_play_info_id: u32) -> i32 {
        unimplemented!("migration placeholder: rsPLAYINFO::ChangePartyPlayer");
    }

    pub fn update_party_player(&mut self) -> i32 {
        unimplemented!("migration placeholder: rsPLAYINFO::UpdatePartyPlayer");
    }

    pub fn update_party_play_info(&mut self) -> i32 {
        unimplemented!("migration placeholder: rsPLAYINFO::UpdatePartyPlayInfo");
    }

    pub fn get_party_money(&mut self, _def_item_info: &UserItem, _play_info_id: u32) -> i32 {
        unimplemented!("migration placeholder: rsPLAYINFO::GetPartyMoney");
    }

    pub fn get_party_exp(&mut self, _char_id: u32, _play_info_id: u32) -> i32 {
        unimplemented!("migration placeholder: rsPLAYINFO::GetPartyExp");
    }

    pub fn party_chatting(&mut self, _play_info_id: u32) -> i32 {
        unimplemented!("migration placeholder: rsPLAYINFO::PartyChatting");
    }

    pub fn update_server_party_player(
        &mut self,
        _trans_party_play_info: &TransPartyPlayInfo,
    ) -> i32 {
        unimplemented!("migration placeholder: rsPLAYINFO::UpdateServerPartyPlayer");
    }

    pub fn leave_party_master(&mut self, _play_info_id: u32) -> i32 {
        unimplemented!("migration placeholder: rsPLAYINFO::LeavePartyMaster");
    }

    pub fn release_party(&mut self) -> i32 {
        unimplemented!("migration placeholder: rsPLAYINFO::ReleaseParty");
    }

    pub fn send_party_message(&mut self, _message: &str, _code: u32) -> i32 {
        unimplemented!("migration placeholder: rsPLAYINFO::SendPartyMessage");
    }

    pub fn send_party_data(&mut self, _play_info_from_id: u32, _data: &[u8]) -> i32 {
        unimplemented!("migration placeholder: rsPLAYINFO::SendPartyData");
    }

    pub fn join_party(&mut self, _play_master_id: u32) -> i32 {
        unimplemented!("migration placeholder: rsPLAYINFO::JoinParty");
    }

    pub fn secede_party(&mut self) -> i32 {
        unimplemented!("migration placeholder: rsPLAYINFO::SecedeParty");
    }

    pub fn disconnect_party_user(&mut self) -> i32 {
        unimplemented!("migration placeholder: rsPLAYINFO::DisconnectPartyUser");
    }

    pub fn add_server_money(&mut self, _money: i32, _where_param: i32) -> i32 {
        unimplemented!("migration placeholder: rsPLAYINFO::AddServerMoney");
    }

    pub fn sub_server_money(&mut self, _money: i32, _where_param: i32) -> i32 {
        unimplemented!("migration placeholder: rsPLAYINFO::SubServerMoney");
    }

    pub fn set_server_money(&mut self, _money: i32, _where_param: i32) -> i32 {
        unimplemented!("migration placeholder: rsPLAYINFO::SetServerMoney");
    }

    pub fn callback_clan_mark_num(&mut self, _clan_mark_num: i32) -> i32 {
        unimplemented!("migration placeholder: rsPLAYINFO::CallBack_ClanMarkNum");
    }
}
