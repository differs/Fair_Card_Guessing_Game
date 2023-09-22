#![no_std]

use core::usize;

use codec::{Decode, Encode};
use gstd::{collections::*, MessageId};
use gstd::{prelude::*, ActorId};
use scale_info::TypeInfo;
use gmeta::{InOut, Metadata};


use data_encoding::BASE64;

// use encoding_rs::*;

// use aes_gcm_siv::{ aead::{Aead, KeyInit, OsRng},Aes256GcmSiv, Nonce // Or `Aes128GcmSiv`
// };



#[derive(Clone, Default, Encode, Decode, TypeInfo)]
pub struct CardPlay (
    // 定义结构体字段
    pub BTreeMap<u64, ActorId>,
    pub BTreeMap<u64, gstd::String>,
    pub BTreeMap<u64, String>, // Object.entries {234: "0xdeadbeef..."} => [[234, "0xdeadbeef..."]] Vec<(u64, [u8; 32])>
    pub BTreeMap<u64, (u64, ActorId, u128, u128, String)>,
);

#[derive(Debug, Clone, Encode, Decode, TypeInfo,PartialEq)]

pub enum GameState {
    DealerProofSubmission,
    PlayerBetting,
    PlayerDecryption,
    DealerDecryption,
    RewardDistribution,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo,PartialEq)]
pub enum UserBettingData {
    TheRounds,
    UserId,
    InitBetAmount,
    RealBettingAmount,
    EncryptedBetData,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq)]

pub struct GameStateStruct(pub GameState);

impl CardPlay {

    pub fn init_contract_owner(&mut self, actor_id:ActorId) -> bool {

        self.0.insert(1, actor_id);

        return true;
    }

    pub fn check_contract_owner(&mut self) -> ActorId {

        let actor_id = self.0.get_key_value(&1).expect("Check contract owner error.");

        return *actor_id.1;

    }

    // 实现结构体的方法
    pub fn current_rounds(&mut self) -> u64{
        if self.1.is_empty() == true {
            return 0;
        }
        else {
            let last:u64 = self.1.len().try_into().unwrap();
            // return self.1.last_key_value().;
            return last;
        }
    }

    pub fn last_round(&mut self) -> (u64, String){
        if self.1.is_empty() != true{
            // let id: u64 = 1;
            // let notice: gstd::String = String::from("value"); 
            let last = self.1.last_key_value();
            match last {
                Some((key, value)) => {
                    return (key.clone(), value.clone());
                },
                None => {todo!()},
            }          
    
            // return ;
        }
        else {
            // return ();
            todo!()
        }
    }

    pub fn game_start(&mut self, title: String) {
        // 实现 game_start 函数逻辑
        // let round = 
        let last:u64 = self.1.len().try_into().unwrap();
        let next_rounds = last + 1;
        if self.1.contains_key(&next_rounds) {
            panic!("failed to add url: code exists");
        } else {
            self.1.insert(next_rounds, title);
        }    

    }

    pub fn game_state() -> (){
        // let state = ;
    }

    fn game_stop(&mut self, code: String) {
        // 实现 game_stop 函数逻辑
    }

    pub fn current_round_hash(&mut self, round: u64, base64_encoded_hash: String) {
        if self.2.is_empty() == true{
            self.2.insert(round, base64_encoded_hash.clone());
        }

        if self.2.len() == (self.1.len() - 1){

            self.2.insert(round, base64_encoded_hash.clone());

            // let a = self.1.last_key_value().expect("msg");
            
        } 

        if  self.2.last_key_value() == self.1.last_key_value() {

            panic!(" Hash of this round insert duplicate.")
            
        }

        // 实现 insert_hash 函数逻辑
        // 庄家公布洗牌后的牌序的hash值
    }

    pub fn inquire_current_card_hash(&mut self) -> (u64, String){
            let last = self.2.last_key_value();
            match last {
                Some((key, value)) => {

                    return (key.clone(), value.clone());
                },
                None => {todo!()},
            }
    }

    pub fn bet(&mut self, round: u64, id: ActorId, user_bet_amount: u128, encrypted_bet_data: String) {
        // let betting_data_array = [round, id, bet_amount, encrypted_bet_data];
        // 实现 bet 函数逻辑
        // self.1.insert(next_rounds, title);


        let bet_index:u64 = self.3.len().try_into().unwrap();
        let user_bet_index = bet_index + 1;
        // let user_bet_amount: u128 = 0;
        let mix_amount = user_bet_amount - 0; // 0 - > x : 需要修改为解密后的真实 mix_amount.
        let user_bet_data = (round, id, user_bet_amount, mix_amount, encrypted_bet_data);
        // let use
        self.3.insert(user_bet_index, user_bet_data);


    }

    pub fn refund(&mut self, _base64_encoded_nonce: String, _base64_encoded_betting_data: String, _id: ActorId, _round: u64) {
        // 实现 refund 函数逻辑
        // let actorId: ActorId = msg::source();
        let base64_encoded_nonce = _base64_encoded_nonce;
        // let d = UTF_8.new_decoder_without_bom_handling();
        // let res = d.decode_to_utf8(base64_encoded_nonce, dst, last);
        let nonce = BASE64.decode(base64_encoded_nonce.as_bytes()).expect("decode the nonce error.");
        // let nonce = Encoding::decode_mut(&self, base64_encoded_nonce, None);
        let base64_encoded_betting_data = _base64_encoded_betting_data;
        let betting_data = BASE64.decode(base64_encoded_betting_data.as_bytes()).expect("decode the betting data error");

        // let key = Aes256GcmSiv::generate_key(&mut OsRng);
        // let cipher = Aes256GcmSiv::new(&key);
        // let nonce = Nonce::from_slice(&nonce); // 96-bits; unique per message
        // let ciphertext = cipher.encrypt(nonce, b"plaintext message".as_ref()).expect("msg");

        // // [6666, 8888] : [read_bet_amount, mix_amount].
        // let origin_betting_data = cipher.decrypt(nonce, betting_data.as_ref()).expect("msg");
        // let read_bet_amount = origin_betting_data[0];
        // let mix_amount = origin_betting_data[1];



        // 解密 betting_data 得到一个 [1000, 9999] 这样的数组.
    }

    pub fn insert_cards(&mut self, round: u64, base64_encoded_cards_sequence: String) {
        // 实现 insert_cards 函数逻辑
    }

    pub fn distribute_rewards(&mut self) -> u64{
        // 实现 distribute_rewards 函数逻辑
        // 获取BTreeMap中的所有值
        // let mut all_values: Vec<_,_,_,_> = BTreeMap
        let all_bet_times: u64 = self.3.len().try_into().unwrap();

        return all_bet_times;
    }

}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum Action {
    // GameState,
    GameStart { title: String },
    GameStop { code: String, url: String },
    // GameStop { code: String, url: String },

    // 提交证明扑克牌的顺序的HASH值
    InsertHash { base64_encoded_cards_hash: String },

    // bet_value: 投注金额和混淆金额的和 
    Bet { encrypted_bet_data: String },

    // nonce: 用于解密,获取应该退回的金额, 解密和 refund 同步进行
    Refund { base64_encoded_nonce: String },

    // 提交透明的扑克牌的顺序 和 计算hash值时使用的随机数.
    InsertCards { encoded_cards_sequence: String },

    // 最终获胜者的决定方式可以是线下计算,也可以是在合约中计算.
    DistributeRewards { base64_encoded_cards_array: String },

    // 提取函数
    WithDraw {},


}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum Event {
    // GameState,
    GameStarted { rounds: u64, title: String },
    GameStoped { code: String, url: String },
    InsertedHash { rounds: u64, base64_encoded_cards_hash: String },
    InsertedCards { rounds: u64, encoded_cards_sequence: String },

    // mix amount 应该由 encrypted_bet_data解密获取
    Bet { total_bet_amount: u128, encrypted_bet_data: String },
    Refund { base64_encoded_nonce: String },
    // InsertCards { encoded_cards_sequence: String },
    DistributedRewards { base64_encoded_cards_array: String},
    // GameStarted { code: String, url: String },

    // 提取
    WithDraw {},

}



#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum Query {
    All,
    Rounds(),
    Last(),
    Title(),
    GameState(),
    HashInserted(),
    Beted (),
    CardsInserted(),
    DistributedRewards(),
    BlockNumber,
    BlockTimestamp,
    ProgramId,
    MessageId,
    Whoami,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum Reply {
    All(CardPlay),
    Rounds(u64),
    Last(u64, String),
    Title(String),
    GameState(gstd::Option<GameStateStruct>),
    HashInserted(u64, String),
    Beted (u64,u64,u128,String),
    CardsInserted(String),
    DistributedRewards(u64, ActorId, u128),
    Url(Option<String>),
    Whoami(ActorId),
    BlockNumber(u32),
    BlockTimestamp(u64),
    ProgramId(ActorId),
    MessageId(MessageId),
}


pub struct ProgramMetadata;

/// TODO: 0️⃣ Copy `Metadata` from the first lesson and push changes to the master branch
impl Metadata for ProgramMetadata {
    type Init = ();
    type Handle = InOut<Action, Event>;
    type State = InOut<Query, Reply>;
    type Reply = ();
    type Others = ();
    type Signal = ();
}
