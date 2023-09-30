#![no_std]
// use ::alloc::collections::BTreeMap;
// use gstd::prelude::*;
use gstd::{debug, exec, msg, prelude::*, ActorId};
use card_io::{CardPlay,Action,Event,Query,Reply,GameStateStruct, GameState};

// use core::convert::AsMut;
use data_encoding::BASE64;
// use once_cell::sync::Lazy;



static mut STATE: Option<CardPlay> = None;
// static mut GAME_STATE: Option<GameStateStruct> = None;

static mut GAME_STATE: Option<GameState> = None;
// static ARR_0: [u8; 32] = [0; 32];
// let mut CONTRACT_OWNER: ActorId = None;
// static CONTRACT_OWNER: Lazy<ActorId> = Lazy::new(|| {
//     ActorId::from_slice(&ARR_0).expect("Init Contract owner error")
// });




#[no_mangle]
extern "C" fn init() {
    // TODO: 0️⃣ Copy the `init` function from the previous lesson and push changes to the master branch
    // unsafe { STATE = Some(car::default()) };

    unsafe { STATE = Some(CardPlay::default());
            GAME_STATE = gstd::Some(GameState::GameEnd);
            assert_eq!(GameState::GameEnd, GAME_STATE.unwrap());            
            
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
            if unsafe { GAME_STATE } == Some(GameState::GameEnd){
                state.game_start(title.clone());
                let rounds = state.current_rounds();

                unsafe { GAME_STATE = Some(GameState::DealerProofSubmission) };
                gstd::msg::reply(Event::GameStarted { rounds, title },0).expect("Got error");
            }
        }

        Action::InsertHash { base64_encoded_cards_hash } => {
            if unsafe { GAME_STATE } == Some(GameState::DealerProofSubmission) {
                let rounds = state.current_rounds();
                state.current_round_hash(rounds, base64_encoded_cards_hash.clone());
    
                unsafe { 
                    GAME_STATE = Some(GameState::PlayerBetting);
                    // debug!("GAME_STATE: {:?}", GAME_STATE);                     
                };

                gstd::msg::reply(Event::InsertedHash { rounds, base64_encoded_cards_hash }, 0).expect("Instert Hash error");

    
            }

            // let submitted_hash = base64_encoded_cards_hash.clone();

        },

        Action::Bet { encrypted_bet_data } => {
            if unsafe { 
                GAME_STATE == Some(GameState::PlayerBetting)

                // debug!("GAME_STATE: {:?}", GAME_STATE);                     
                }
                {
                    // betting data of ActorId
                    let total_bet_amount = msg::value();
                    let actorId = msg::source();
                    let current_round = state.current_rounds();
                    let _user_bet_amount = total_bet_amount.clone();
                    let base64_encoded_encrypted_betting_data = encrypted_bet_data.clone();
                    // state.current_round_hash(rounds, base64_encoded_cards_hash.clone());

                    state.bet(current_round, actorId, total_bet_amount,base64_encoded_encrypted_betting_data);
                    // gstd::msg::reply("Betting success", 0).expect("Betting Error");
                    gstd::msg::reply(Event::Bet { total_bet_amount, encrypted_bet_data }, 0).expect("Betting Action Error");

                    if state.3.len() >= 5 {

                        unsafe { GAME_STATE = Some(GameState::DealerDecryption) }
                        
                    }
                }
        },

        Action::InsertCards { encoded_cards_sequence  } => {
            if unsafe {
                GAME_STATE == Some(GameState::DealerDecryption)

            }{
                // 庄家公开牌序
                // state.current_round_Cards_array(rounds, card_vec.clone());
                let actor_id = msg::source();
                let banker_id = msg::source();

                if actor_id == banker_id {
                    let current_round = state.current_rounds();

                    let current_round_encoded_cards_sequence = encoded_cards_sequence.clone();

                    state.insert_cards(current_round, actor_id, current_round_encoded_cards_sequence);

                    gstd::msg::reply(Event::InsertedCards {  current_round, actor_id, encoded_cards_sequence }, 0).expect("Insert encoded_cards_sequence Got Errors");
                }

                // 
                unsafe { GAME_STATE = Some(GameState::RewardDistribution) }
            }
        },

        Action::DistributeRewards { base64_encoded_cards_array } => {

            // assert_eq!(GameState, Some(GameState::RewardDistribution));
            if unsafe { GAME_STATE } == Some(GameState::RewardDistribution) {


                let actor_id = msg::source();
                let current_round = state.current_rounds();


                let encoded_cards_array = base64_encoded_cards_array.clone();
                state.insert_cards(current_round, actor_id, encoded_cards_array.clone());


                // decode cards array.
                let decoded_cards_arry = BASE64.decode(&encoded_cards_array.as_bytes()).expect("In put card array error.");

                let mut array: [u8; 52] = [0; 52];
                array.copy_from_slice(&decoded_cards_arry);
                assert_eq!(array.len(), 52);
                let gamblers = state.distribute_rewards();
                // assert!(gamblers, 0)
                assert_ne!(gamblers, 0);

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

                let winner_actor_id: ActorId = winner_actor_info.1.1;

                // let winner_actorId = msg::source();

                let balance: u128 = exec::value_available();

                let owner_shares:u128 = balance * 10 / 100;

                let winner_shares:u128 = balance * 90 / 100;

                let owner = ActorId::from_bs58("kGfn1RrSZJkTrNbNjpQbvWNE5Szsr3tsTFtmYExrBHFCPLjPy".to_owned()).expect("msg");
            
                // gstd::msg::so
                let _ = gstd::msg::reply("DistributeRewards Success", 0).expect("DistributeRewards Error");
                // gstd::msg::send_for_reply(program, payload, value, reply_deposit)
                let _ = gstd::msg::send(owner, "Banker's margin", owner_shares);
                let _ = gstd::msg::send(winner_actor_id, "You are the LUCKest one ", winner_shares);  
            }

            unsafe { GAME_STATE = Some(GameState::GameEnd) }


        }

        Action::Refund { base64_encoded_nonce: _ } => {
            if unsafe {
                GAME_STATE == Some(GameState::PlayerDecryption)
                // debug!("GAME_STATE: {:?}", GAME_STATE);

            }{
                gstd::msg::reply("Refund Success", 0).expect("Betting Error");



                // Test type.
                // let a = state.inquire_current_card_hash().1;
                // let b = GameStateStruct(GameState::PlayerDecryption);

            }
        },
        // Action::GameStop { code: _, url: _ } => todo!(),

        Action::WithDraw {  } => {
            
            let balance: u128 = exec::value_available();
            let admin: ActorId = ActorId::from_bs58("kGfn1RrSZJkTrNbNjpQbvWNE5Szsr3tsTFtmYExrBHFCPLjPy".to_owned()).expect("Get Admin Address Error.");
            assert_eq!(admin, msg::source());

            let _ = gstd::msg::send(msg::source(), "Withdraw Balance.", balance);
        },
        // Action::GameStop { code, url } => todo!(),

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
        Query::Title() => Reply::Title(state.last_round().1),
        // Query::Last => todo!(),
        Query::Last() => Reply::Last(state.last_round().0, state.last_round().1),
        Query::GameState() => Reply::GameState(unsafe { GAME_STATE.clone() }),
        Query::HashInserted() => Reply::HashInserted(state.inquire_current_card_hash().0, state.inquire_current_card_hash().1),
        Query::Beted() => todo!(),
        Query::CardsInserted() => Reply::CardsInserted(state.4.last_key_value().expect("msg").1.2.clone()),
        Query::DistributedRewards() => todo!(),
        Query::AllBets() => Reply::AllBets(state.3),
    };
    gstd::msg::reply(reply, 0).expect("Failed to share state");

}


