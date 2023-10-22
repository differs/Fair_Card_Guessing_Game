#![no_std]
use gstd::{exec, msg, prelude::*, ActorId};
use vara_bets_io::{VaraBetsStates,Action,Event,Query,Reply, BetsRoundState};
use sha2::{Sha256, Digest};
use data_encoding::BASE64;



static mut VARABETSSTATES: Option<VaraBetsStates> = None;
static mut BETSROUNDSTATE: Option<BetsRoundState> = None;
// static ARR_0: [u8; 32] = [0; 32];
// let mut CONTRACT_OWNER: ActorId = None;
// static CONTRACT_OWNER: Lazy<ActorId> = Lazy::new(|| {
//     ActorId::from_slice(&ARR_0).expect("Init Contract owner error")
// });




#[no_mangle]
extern "C" fn init() {

    unsafe { VARABETSSTATES = Some(VaraBetsStates::default());
            BETSROUNDSTATE = gstd::Some(BetsRoundState::GameEnded);
            assert_eq!(BetsRoundState::GameEnded, BETSROUNDSTATE.unwrap());
     };



}

#[no_mangle]
extern "C" fn handle() {

    let state = unsafe { VARABETSSTATES.as_mut().expect("failed to get state as mut") };
    let action: Action = gstd::msg::load().expect("failed to load action");
    match action {
        Action::GameStart { title } => {
            if unsafe { BETSROUNDSTATE } == Some(BetsRoundState::GameEnded){
                state.game_start(title.clone());
                let rounds = state.current_rounds();

                unsafe { BETSROUNDSTATE = Some(BetsRoundState::DealerProofSubmission) };
                gstd::msg::reply(Event::GameStarted { rounds, title },0).expect("Got error");
            }
        }

        Action::InsertHash { base64_encoded_cards_hash } => {
            if unsafe { BETSROUNDSTATE } == Some(BetsRoundState::DealerProofSubmission) {
                let rounds = state.current_rounds();
                state.current_round_hash(rounds, base64_encoded_cards_hash.clone());
    
                unsafe { 
                    BETSROUNDSTATE = Some(BetsRoundState::PlayerBetting);
                    // debug!("BETSROUNDSTATE: {:?}", BETSROUNDSTATE);                     
                };

                gstd::msg::reply(Event::InsertedHash { rounds, base64_encoded_cards_hash }, 0).expect("Instert Hash error");

    
            }

            // let submitted_hash = base64_encoded_cards_hash.clone();

        },

        Action::Bet { encrypted_bet_data } => {
            if unsafe { 
                BETSROUNDSTATE == Some(BetsRoundState::PlayerBetting)

                // debug!("BETSROUNDSTATE: {:?}", BETSROUNDSTATE);                     
                }
                {
                    // betting data of ActorId
                    let total_bet_amount = msg::value();
                    let actor_id = msg::source();
                    let current_round = state.current_rounds();
                    let _user_bet_amount = total_bet_amount.clone();
                    let base64_encoded_encrypted_betting_data = encrypted_bet_data.clone();
                    // state.current_round_hash(rounds, base64_encoded_cards_hash.clone());

                    state.bet(current_round, actor_id, total_bet_amount,base64_encoded_encrypted_betting_data);
                    // gstd::msg::reply("Betting success", 0).expect("Betting Error");
                    gstd::msg::reply(Event::Bet { total_bet_amount, encrypted_bet_data }, 0).expect("Betting Action Error");

                    if state.3.len() >= 5 {

                        unsafe { BETSROUNDSTATE = Some(BetsRoundState::DealerDecryption) }
                        
                    }
                }
        },

        Action::InsertCards { encoded_cards_sequence  } => {
            if unsafe {
                BETSROUNDSTATE == Some(BetsRoundState::DealerDecryption)

            }{
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
                unsafe { BETSROUNDSTATE = Some(BetsRoundState::RewardDistribution) }
            }
        },

        Action::DistributeRewards { base64_encoded_cards_array } => {

            // assert_eq!(BetsRoundState, Some(BetsRoundState::RewardDistribution));
            if unsafe { BETSROUNDSTATE } == Some(BetsRoundState::RewardDistribution) {


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

                unsafe { BETSROUNDSTATE = Some(BetsRoundState::GameEnded) }
            }
        }

        Action::Refund { base64_encoded_nonce: _ } => {
            if unsafe {
                BETSROUNDSTATE == Some(BetsRoundState::DealerDecryption)
                // BETSROUNDSTATE == Some(BetsRoundState::PlayerDecryption)

                // debug!("BETSROUNDSTATE: {:?}", BETSROUNDSTATE);

            }{
                gstd::msg::reply("Refund Success", 0).expect("Betting Error");

                // let a = state.inquire_current_card_hash().1;
                // let b = BetsRoundStateStruct(BetsRoundState::PlayerDecryption);

                // "31bbe87933d53b1baaf3d13c02d45482af069d82ff92deee935aabc5f1a691f8"
                // 30769590742866
                // ("0x0ab0e6bcd8d1b73f75b88eb946e075bae2a202acec52217103bdfd498e6c7b3a",[30769590742866,30769590742866],1 )
                // let input:(&str, [u64; 2], u128) = ("0x0ab0e6bcd8d1b73f75b88eb946e075bae2a202acec52217103bdfd498e6c7b3a",[*data,*data],1 );
                
                // let inputs= format!("{:?}", input);
                // let input_bytes = inputs.as_bytes();
                // hasher.update(input_bytes);
                // // hasher.update(data.to_le_bytes());
                // let result = hasher.finalize();
                // // hex!(result);
                // // let hash = format!("{:x}", result);
                // result
                            // assert_eq!(hash, "31bbe87933d53b1baaf3d13c02d45482af069d82ff92deee935aabc5f1a691f8")
                let mut hasher = Sha256::new();
                let input:(&str, [u64; 2], u128) = ("0x0ab0e6bcd8d1b73f75b88eb946e075bae2a202acec52217103bdfd498e6c7b3a",[30769590742866,30769590742866],1 );
                let inputs= format!("{:?}", input);
                let input_bytes = inputs.as_bytes();
                hasher.update(input_bytes);
                // hasher.update(data.to_le_bytes());
                let result = hasher.finalize();
                // hex!(result);
                let hash = format!("{:x}", result);
                assert_eq!(hash, "31bbe87933d53b1baaf3d13c02d45482af069d82ff92deee935aabc5f1a691f8")     
                
                // Record actual bet amount

                // Refund mix amount

                // Send inviter shares
     
            }
        },

        Action::WithDraw {} => {            
            let balance: u128 = exec::value_available();
            let admin: ActorId = ActorId::from_bs58("kGfn1RrSZJkTrNbNjpQbvWNE5Szsr3tsTFtmYExrBHFCPLjPy".to_owned()).expect("Get Admin Address Error.");
            assert_eq!(admin, msg::source());

            let _ = gstd::msg::send(msg::source(), "Withdraw Balance.", balance);
        },
    }


}



#[no_mangle]

extern "C" fn state() {
    let query = gstd::msg::load().expect("failed to load query");
    let mut state: VaraBetsStates = unsafe { VARABETSSTATES.as_ref().expect("failed to get contract state").clone() };

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
        Query::Last() => Reply::Last(state.last_round().0, state.last_round().1),
        Query::BetsRoundState() => Reply::BetsRoundState(unsafe { BETSROUNDSTATE.clone() }),
        Query::HashInserted() => Reply::HashInserted(state.inquire_current_card_hash().0, state.inquire_current_card_hash().1),
        Query::Beted() => todo!(),
        Query::CardsInserted() => Reply::CardsInserted(state.4.last_key_value().expect("msg").1.2.clone()),
        Query::DistributedRewards() => todo!(),
        Query::AllBets() => Reply::AllBets(state.3),
    };
    gstd::msg::reply(reply, 0).expect("Failed to share state");

}


