#![no_std]
// use ::alloc::collections::BTreeMap;
// use gstd::prelude::*;
use gstd::{debug, exec, msg, prelude::*, ActorId, collections::BTreeMap};
use card_io::{CardPlay,Action,Event,Query,Reply,GameStateStruct, GameState};
use core::cmp::max;
use core::convert::AsMut;
use data_encoding::BASE64;
// use once_cell::sync::Lazy;



static mut STATE: Option<CardPlay> = None;
static mut GAME_STATE: Option<GameStateStruct> = None;
static ARR_0: [u8; 32] = [0; 32];
// let mut CONTRACT_OWNER: ActorId = None;
// static CONTRACT_OWNER: Lazy<ActorId> = Lazy::new(|| {
//     ActorId::from_slice(&ARR_0).expect("Init Contract owner error")
// });




#[no_mangle]
extern "C" fn init() {
    // TODO: 0️⃣ Copy the `init` function from the previous lesson and push changes to the master branch
    // unsafe { STATE = Some(car::default()) };

    unsafe { STATE = Some(CardPlay::default());
             GAME_STATE = Some(GameStateStruct(card_io::GameState::DealerDecryption));
            //  let init_contract_owner:ActorId = msg::source();
            //  GAME_STATE = Some()
            // successed = Some(CardPlay::init_contract_owner(&mut self, msg::source()));
            // Some()
            
     };



}

#[no_mangle]
extern "C" fn handle() {

    let state = unsafe { STATE.as_mut().expect("failed to get state as mut") };
    let action: Action = gstd::msg::load().expect("failed to load action");
    match action {
        // Action::Bet { code, url } => {
        //     state.add_url(code.clone(), url.clone());
        //     gstd::msg::reply(Event::Added { code, url }, 0).expect("failed to reply");
        // }
        Action::GameStart { title } => {
            // todo
            // state::game_start(code.clone(), url.clone());
            state.game_start(title.clone());
            let rounds = state.current_rounds();
            // let the_String = String::from("j=,Cggrj33EMn89956004724dcd5a4b7");
            // let bytes = the_String.as_bytes();
            // let a:[u8; 32] = [106, 61, 44, 67, 103, 103, 114, 106, 51, 51, 69, 77, 110, 56, 57, 57, 53, 54, 48, 48, 52, 55, 50, 52, 100, 99, 100, 53, 97, 52, 98, 55];
            // assert_eq!(bytes, a);

            unsafe { GAME_STATE = Some(GameStateStruct(GameState::DealerProofSubmission)) };
            gstd::msg::reply(Event::GameStarted { rounds, title },0).expect("Got error");
            // gstd::msg::reply("hello",100000).expect("Got error");
        }

        Action::InsertHash { base64_encoded_cards_hash } => {
            let rounds = state.current_rounds();
            state.current_round_hash(rounds, base64_encoded_cards_hash.clone());

            unsafe { 
                GAME_STATE = Some(GameStateStruct(GameState::PlayerBetting));
                debug!("GAME_STATE: {:?}", GAME_STATE);                     
            };

            // let submitted_hash = base64_encoded_cards_hash.clone();

            gstd::msg::reply(Event::InsertedHash { rounds, base64_encoded_cards_hash }, 0).expect("Instert Hash error");
        },

        Action::Bet { encrypted_bet_data } => {
            if unsafe { 
                GAME_STATE == Some(GameStateStruct(GameState::PlayerBetting))

                // debug!("GAME_STATE: {:?}", GAME_STATE);                     
                }
                {
                    // betting data of ActorId
                    let total_bet_amount = msg::value();
                    let actorId = msg::source();
                    let current_round = state.current_rounds();
                    let user_bet_amount = total_bet_amount.clone();
                    let base64_encoded_encrypted_betting_data = encrypted_bet_data.clone();
                    // state.current_round_hash(rounds, base64_encoded_cards_hash.clone());

                    state.bet(current_round, actorId, total_bet_amount,base64_encoded_encrypted_betting_data);
                    // gstd::msg::reply("Betting success", 0).expect("Betting Error");
                    gstd::msg::reply(Event::Bet { total_bet_amount, encrypted_bet_data }, 0).expect("Betting Action Error");
                }
        },

        Action::InsertCards { encoded_cards_sequence } => {
            if unsafe {
                GAME_STATE == Some(GameStateStruct(GameState::DealerDecryption))

            }{
                // 庄家公开牌序
                // state.current_round_Cards_array(rounds, card_vec.clone());
                let actorId = msg::source();
                let bankerId = msg::source();

                if actorId == bankerId {
                    let current_round = state.current_rounds();

                    let current_round_encoded_cards_sequence = encoded_cards_sequence.clone();

                    state.insert_cards(current_round, current_round_encoded_cards_sequence);

                    gstd::msg::reply(Event::InsertedCards { encoded_cards_sequence, rounds: todo!() }, 0).expect("Insert encoded_cards_sequence Got Errors");
                }

                // 
            }
        },

        Action::DistributeRewards { base64_encoded_cards_array } => {

            // let creator: ActorId = exec::program_id()
            // let creator:ActorId = exec
            
            // let encoded_card_arr: String = base64_encoded_cards_array.clone();
            // 计算[{ActorId : Value},{ActorId : Value},{ActorId : Value},{ActorId : Value} ],按照 Value 排序

            // 获取 牌序 数组切片中最大数的索引 INDEX_MAX_NUM

            // 获取 [{ActorId : Value},{ActorId : Value},{ActorId : Value},{ActorId : Value} ]中 索引为 INDEX_MAX_NUM的 键值对的ActorId.
            let encoded_cards_array = base64_encoded_cards_array.clone();

            // decode cards array.
            let decoded_cards_arry = BASE64.decode(encoded_cards_array.as_bytes()).expect("In put card array error.");

            let mut array: [u8; 52] = [0; 52];
            array.copy_from_slice(&decoded_cards_arry);
            assert_eq!(array.len(), 52);
            let gamblers = state.distribute_rewards();
            // assert!(gamblers, 0)
            assert_ne!(gamblers, 0);

            // 按照投注的先后顺序发牌.
            // let mut (k: u64, v) = state.3.clone();
            let k: usize = (gamblers - 0).try_into().unwrap();


            let mut max_val = array[0];
            let mut max_index = k;
        
            for i in 1..k {
                if array[i] > max_val {
                    max_val = array[i];
                    max_index = i;
                }
            }            
            assert_ne!(max_index, 0);

            let card_max_index: u64 = max_index.try_into().unwrap();

            let winner_actor_info= state.3.get_key_value(&card_max_index).expect("Got Winner INFO Error");

            let winner_actorId: ActorId = winner_actor_info.1.1;

            // let winner_actorId = msg::source();

            let balance: u128 = exec::value_available();

            let owner_shares:u128 = balance * 10 / 100;

            let winner_shares:u128 = balance * 90 / 100;

            let owner = ActorId::from_bs58("kGfn1RrSZJkTrNbNjpQbvWNE5Szsr3tsTFtmYExrBHFCPLjPy".to_owned()).expect("msg");
        
            // gstd::msg::so
            gstd::msg::reply("DistributeRewards Success", 0).expect("DistributeRewards Error");
            // gstd::msg::send_for_reply(program, payload, value, reply_deposit)
            gstd::msg::send(owner, "Banker's margin", owner_shares);
            gstd::msg::send(winner_actorId, "You are the LUCKest one ", winner_shares);  
            // gstd::msg::re        

        }

        Action::Refund { base64_encoded_nonce } => {
            if unsafe {
                GAME_STATE == Some(GameStateStruct(GameState::PlayerDecryption))
                // debug!("GAME_STATE: {:?}", GAME_STATE);

            }{
                // 玩家解密投注金额,并回退混淆金额
                // debug!("Age: {}", age);

                // 获取玩家发送的金额

                // 获取加密后的数据
                // let crypted_value_string = state.
                // 获取加密所用的nonce.
                // let nonce = state.

                // 调用解密函数. 得出投注金额

                // 存储 [{ActorId : Value},{ActorId : Value},{ActorId : Value},{ActorId : Value} ]

                // 发送 (sum - bet) 到玩家
                // let sum = state.x
                // let bet = decrypted...
                gstd::msg::reply("Refund Success", 0).expect("Betting Error");



                // Test type.
                // let a = state.inquire_current_card_hash().1;
                // let b = GameStateStruct(GameState::PlayerDecryption);

            }
        },
        Action::GameStop { code, url } => todo!(),

        Action::WithDraw {  } => {
            
            let balance: u128 = exec::value_available();
            let admin: ActorId = ActorId::from_bs58("kGfn1RrSZJkTrNbNjpQbvWNE5Szsr3tsTFtmYExrBHFCPLjPy".to_owned()).expect("Get Admin Address Error.");
            assert_eq!(admin, msg::source());

            let _ = gstd::msg::send(msg::source(), "Withdraw Balance.", balance);
        },

        // Action::GameState => todo!(),
    }


}



#[no_mangle]
// static mut ROUND: Option<FairGuessGame> = None;

extern "C" fn state() {
    let query = gstd::msg::load().expect("failed to load query");
    let mut state: CardPlay = unsafe { STATE.as_ref().expect("failed to get contract state").clone() };

    // let mut GA
    let reply = match query {
        Query::All => Reply::All(state.clone()),
        Query::Rounds() => Reply::Rounds(state.current_rounds()),
        Query::Whoami => Reply::Whoami(gstd::msg::source()), // all zero addr
        Query::BlockNumber => Reply::BlockNumber(gstd::exec::block_height()),
        Query::BlockTimestamp => Reply::BlockTimestamp(gstd::exec::block_timestamp()),
        Query::ProgramId => Reply::ProgramId(gstd::exec::program_id()),
        Query::MessageId => Reply::MessageId(gstd::msg::id()),
        Query::Title() => todo!(),
        // Query::Last => todo!(),
        Query::Last() => Reply::Last(state.last_round().0, state.last_round().1),
        Query::GameState() => Reply::GameState(unsafe { GAME_STATE.clone() }),
        // println!("{}","j=,Cggrj33EMn89956004724dcd5a4b7".encode_hex::<String>());
        Query::HashInserted() => Reply::HashInserted(state.inquire_current_card_hash().0, state.inquire_current_card_hash().1),
        // debug!("Age: {}", age);
        // debug!(state.Inquire_current_card_hash().1)
        Query::Beted() => todo!(),
        Query::CardsInserted() => todo!(),
        Query::DistributedRewards() => todo!(),
    };
    gstd::msg::reply(reply, 0).expect("Failed to share state");

}


