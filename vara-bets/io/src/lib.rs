#![no_std]

use core::usize;
use codec::{Decode, Encode};
use gstd::{collections::*, MessageId};
use gstd::{prelude::*, ActorId};
use scale_info::TypeInfo;
use gmeta::{InOut, Metadata};


use data_encoding::BASE64;


#[derive(Clone, Default, Encode, Decode, TypeInfo)]
pub struct CardPlay (
    pub BTreeMap<u64, ActorId>,
    pub BTreeMap<u64, gstd::String>,
    pub BTreeMap<u64, String>, // Object.entries {234: "0xdeadbeef..."} => [[234, "0xdeadbeef..."]] Vec<(u64, [u8; 32])>
    pub BTreeMap<u64, (u64, ActorId, u128, u128, String)>,

    // Cards Insert
    pub BTreeMap<u64, (u64, ActorId, String)>
);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo)]

pub enum GameState {
    #[default]
    DealerProofSubmission,
    PlayerBetting,
    PlayerDecryption,
    DealerDecryption,
    RewardDistribution,
    GameEnd,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo,PartialEq)]
pub enum UserBettingData {
    TheRounds,
    UserId,
    InitBetAmount,
    RealBettingAmount,
    EncryptedBetData,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Copy)]

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

    pub fn current_rounds(&mut self) -> u64{
        if self.1.is_empty() == true {
            return 0;
        }
        else {
            let last:u64 = self.1.len().try_into().unwrap();
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
        }
        else {
            todo!()
        }
    }

    pub fn game_start(&mut self, title: String) {
        // let round = 
        let last:u64 = self.1.len().try_into().unwrap();
        let next_rounds = last + 1;
        if self.1.contains_key(&next_rounds) {
            panic!("failed to add url: code exists");
        } else {
            self.1.insert(next_rounds, title);
        }    

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
        // self.1.insert(next_rounds, title);


        let bet_index:u64 = self.3.len().try_into().unwrap();
        let user_bet_index = bet_index + 1;
        // let user_bet_amount: u128 = 0;
        let mix_amount = user_bet_amount - 0; // 0 - > x :
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



        // betting_data  [1000, 9999] 
    }

    pub fn insert_cards(&mut self, round: u64, actor_id: ActorId, encoded_cards_array: String) {

        let index = self.4.len() + 1;


        self.4.insert(index.try_into().unwrap(),(round, actor_id, encoded_cards_array));
    }

    pub fn distribute_rewards(&mut self) -> u64{
        // let mut all_values: Vec<_,_,_,_> = BTreeMap
        let all_bet_times: u64 = self.3.len().try_into().unwrap();

        return all_bet_times;
    }

}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum Action {
    // GameState,
    GameStart { title: String },
    // GameStop { code: String, url: String },
    // GameStop { code: String, url: String },

    InsertHash { base64_encoded_cards_hash: String },

    Bet { encrypted_bet_data: String },

    Refund { base64_encoded_nonce: String },

    InsertCards { encoded_cards_sequence: String },

    DistributeRewards { base64_encoded_cards_array: String },

    WithDraw {},


}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum Event {
    // GameState,
    GameStarted { rounds: u64, title: String },
    // GameStoped { code: String, url: String },
    InsertedHash { rounds: u64, base64_encoded_cards_hash: String },
    InsertedCards { current_round: u64, actor_id: ActorId, encoded_cards_sequence: String },

    Bet { total_bet_amount: u128, encrypted_bet_data: String },
    Refund { base64_encoded_nonce: String },
    // InsertCards { encoded_cards_sequence: String },
    DistributedRewards { base64_encoded_cards_array: String},
    // GameStarted { code: String, url: String },
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
    Beted(),
    AllBets(),
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
    GameState(gstd::Option<GameState>),
    HashInserted(u64, String),
    Beted (u64, ActorId, u128, u128, String),
    AllBets(BTreeMap<u64, (u64, ActorId, u128, u128, String)>),
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
