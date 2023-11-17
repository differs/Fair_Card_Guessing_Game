#![no_std]

use core::usize;
use codec::{Decode, Encode};
use gstd::{collections::*, MessageId};
use gstd::{prelude::*, ActorId};
use scale_info::TypeInfo;
use gmeta::{InOut, Metadata};
use data_encoding::BASE64;


#[derive(Clone, Default, Encode, Decode, TypeInfo)]
pub struct VaraBetsStates {
    // use to ...
    pub rounds:BTreeMap<u64, ActorId>, 
    // 
    pub cureent_round: BTreeMap<u64, gstd::String>,
    // 
    pub cureent_round_hash_submitted: BTreeMap<u64, String>, 
    // 
    pub betting_index: BTreeMap<u64, (u64, ActorId, u128, u128, String)>,
    // Cards Insert
    pub card_seq: BTreeMap<u64, (u64, ActorId, String)>
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo)]

pub enum BetsRoundState {
    #[default]
    GameStarted,
    DealerProofSubmission,
    PlayerBetting,
    PlayerDecryption,
    DealerDecryption,
    RewardDistribution,
    GameEnded,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo,PartialEq)]
pub enum UserBettingData {
    TheRounds,
    UserId,
    InitBetAmount,
    RealBettingAmount,
    EncryptedBetData,
}

// #[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Copy)]

// pub struct GameStateStruct(pub BetsRoundsState);

impl VaraBetsStates {

    pub fn init_contract_owner(&mut self, actor_id:ActorId) -> bool {

        self.rounds.insert(1, actor_id);

        return true;
    }

    pub fn check_contract_owner(&mut self) -> ActorId {

        let actor_id = self.rounds.get_key_value(&1).expect("Check contract owner error.");

        return *actor_id.1;

    }

    pub fn current_rounds(&mut self) -> u64{
        if self.cureent_round.is_empty() == true {
            return 0;
        }
        else {
            let last:u64 = self.cureent_round.len().try_into().unwrap();
            return last;
        }
    }

    pub fn last_round(&mut self) -> (u64, String){
        if self.cureent_round.is_empty() != true{
            // let id: u64 = 1;
            // let notice: gstd::String = String::from("value"); 
            let last = self.cureent_round.last_key_value();
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
        let last:u64 = self.cureent_round.len().try_into().unwrap();
        let next_rounds = last + 1;
        if self.cureent_round.contains_key(&next_rounds) {
            panic!("failed to add url: code exists");
        } else {
            self.cureent_round.insert(next_rounds, title);
        }    

    }


    pub fn current_round_hash(&mut self, round: u64, base64_encoded_hash: String) {
        if self.cureent_round_hash_submitted.is_empty() == true{
            self.cureent_round_hash_submitted.insert(round, base64_encoded_hash.clone());
        }

        if self.cureent_round_hash_submitted.len() == (self.cureent_round.len() - 1){

            self.cureent_round_hash_submitted.insert(round, base64_encoded_hash.clone());

            // let a = self.1.last_key_value().expect("msg");
            
        } 

        if  self.cureent_round_hash_submitted.last_key_value() == self.cureent_round.last_key_value() {

            panic!(" Hash of this round insert duplicate.")
            
        }

    }

    pub fn inquire_current_card_hash(&mut self) -> (u64, String){
            let last = self.cureent_round_hash_submitted.last_key_value();
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


        let bet_index:u64 = self.betting_index.len().try_into().unwrap();
        let user_bet_index = bet_index + 1;
        // let user_bet_amount: u128 = 0;
        let mix_amount = user_bet_amount - 0; // 0 - > x :
        let user_bet_data = (round, id, user_bet_amount, mix_amount, encrypted_bet_data);
        // let use
        self.betting_index.insert(user_bet_index, user_bet_data);


    }

    pub fn refund(&mut self, _base64_encoded_nonce: String, _base64_encoded_betting_data: String, _id: ActorId, _round: u64) {
        // let actorId: ActorId = msg::source();
        let base64_encoded_nonce = _base64_encoded_nonce;
        // let d = UTF_8.new_decoder_without_bom_handling();
        // let res = d.decode_to_utf8(base64_encoded_nonce, dst, last);
        let _nonce = BASE64.decode(base64_encoded_nonce.as_bytes()).expect("decode the nonce error.");
        // let nonce = Encoding::decode_mut(&self, base64_encoded_nonce, None);
        let base64_encoded_betting_data = _base64_encoded_betting_data;
        let _betting_data = BASE64.decode(base64_encoded_betting_data.as_bytes()).expect("decode the betting data error");

    }

    pub fn insert_cards(&mut self, round: u64, actor_id: ActorId, encoded_cards_array: String) {

        let index = self.card_seq.len() + 1;


        self.card_seq.insert(index.try_into().unwrap(),(round, actor_id, encoded_cards_array));
    }

    pub fn distribute_rewards(&mut self) -> u64{
        // let mut all_values: Vec<_,_,_,_> = BTreeMap
        let all_bet_times: u64 = self.betting_index.len().try_into().unwrap();

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
    BetsRoundState(),
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
    All(VaraBetsStates),
    Rounds(u64),
    Last(u64, String),
    Title(String),
    BetsRoundState(gstd::Option<BetsRoundState>),
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
