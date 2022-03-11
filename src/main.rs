
use solana_sdk::{
    pubkey::{ Pubkey },
    signer::{
        Signer,
        keypair::{ Keypair, read_keypair_file },
    },
};

use spl_governance::{
    state::{
        enums::{
            VoteThresholdPercentage,
            VoteTipping,
            ProposalState,
        },
        governance::{
            GovernanceConfig,
        },
    },
};

// mod tokens;
mod commands;

use commands::{ Realm, Governance, Proposal, TokenOwner };

const WALLET_FILE_PATH: &'static str = "/home/mich/.config/solana/id.json";

// const GOVERNANCE_KEY_FILE_PATH: &'static str = "/media/mich/speedwork/NeonLabs/artifacts/governance.json";
const GOVERNANCE_KEY_FILE_PATH: &'static str = "/media/mich/speedwork/SolanaProgs/solana-program-library/target/deploy/spl_governance-keypair.json";
const VOTER_WEIGHT_ADDIN_KEY_FILE_PATH: &'static str = "/media/mich/speedwork/SolanaProgs/solana-program-library/target/deploy/spl_governance_addin_mock-keypair.json";
const COMMUTINY_MINT_KEY_FILE_PATH: &'static str = "/media/mich/speedwork/NeonLabs/artifacts/dev/token_mints/USDT.keypair";
// const COMMUTINY_MINT_KEY_FILE_PATH: &'static str = "/media/mich/speedwork/NeonLabs/artifacts/community-mint.json";
const GOVERNED_MINT_KEY_FILE_PATH: &'static str = "/media/mich/speedwork/NeonLabs/artifacts/dev/token_mints/wBAL.keypair";
const VOTER_WEIGHT_RECORD_KEY_FILE_PATH: &'static str = "/media/mich/speedwork/NeonLabs/artifacts/voter-weight-record.keypair";
const MAX_VOTER_WEIGHT_RECORD_KEY_FILE_PATH: &'static str = "/media/mich/speedwork/NeonLabs/artifacts/max-voter-weight-record.keypair";

const VOTER2_KEY_FILE_PATH: &'static str = "/media/mich/speedwork/NeonLabs/artifacts/vw-addin/voter2.json";
const VOTER2_WEIGHT_RECORD_KEY_FILE_PATH: &'static str = "/media/mich/speedwork/NeonLabs/artifacts/vw-addin/voter2-voter-weight-record.keypair";
const VOTER3_KEY_FILE_PATH: &'static str = "/media/mich/speedwork/NeonLabs/artifacts/vw-addin/voter3.json";
const VOTER3_WEIGHT_RECORD_KEY_FILE_PATH: &'static str = "/media/mich/speedwork/NeonLabs/artifacts/vw-addin/voter3-voter-weight-record.keypair";

// const REALM_NAME: &'static str = "Test Realm";
const REALM_NAME: &'static str = "Test Realm 3";
// const REALM_NAME: &'static str = "Test Realm 6";
const PROPOSAL_NAME: &'static str = "Proposal 3";
const PROPOSAL_DESCRIPTION: &'static str = "proposal_3_description";

fn main() {

    let owner_keypair: Keypair = read_keypair_file(WALLET_FILE_PATH).unwrap();
    let owner_pubkey: Pubkey = owner_keypair.pubkey();
    println!("Owner Pubkey: {}", owner_pubkey);
    let voter_keypair: Keypair = read_keypair_file(WALLET_FILE_PATH).unwrap();

    let voter2_keypair: Keypair = read_keypair_file(VOTER2_KEY_FILE_PATH).unwrap();
    let voter2_pubkey: Pubkey = voter2_keypair.pubkey();
    println!("Voter2 Pubkey: {}", voter2_pubkey);

    let voter3_keypair: Keypair = read_keypair_file(VOTER3_KEY_FILE_PATH).unwrap();
    let voter3_pubkey: Pubkey = voter3_keypair.pubkey();
    println!("Voter3 Pubkey: {}", voter3_pubkey);

    let program_keypair: Keypair = read_keypair_file(GOVERNANCE_KEY_FILE_PATH).unwrap();
    let program_id: Pubkey = program_keypair.pubkey();
    println!("Governance Program Id: {}", program_id);

    let community_keypair: Keypair = read_keypair_file(COMMUTINY_MINT_KEY_FILE_PATH).unwrap();
    let community_pubkey: Pubkey = community_keypair.pubkey();
    println!("Community Token Mint Pubkey: {}", community_pubkey);

    let voter_weight_addin_keypair: Keypair = read_keypair_file(VOTER_WEIGHT_ADDIN_KEY_FILE_PATH).unwrap();
    let voter_weight_addin_pubkey: Pubkey = voter_weight_addin_keypair.pubkey();
    println!("Voter Weight Addin Pubkey: {}", voter_weight_addin_pubkey);

    let governed_account_keypair: Keypair = read_keypair_file(GOVERNED_MINT_KEY_FILE_PATH).unwrap();
    let governed_account_pubkey: Pubkey = governed_account_keypair.pubkey();
    println!("Governed Account (Mint) Pubkey: {}", governed_account_pubkey);

    let max_voter_weight_record_keypair: Keypair = read_keypair_file(MAX_VOTER_WEIGHT_RECORD_KEY_FILE_PATH).unwrap();
    let max_voter_weight_record_pubkey: Pubkey = max_voter_weight_record_keypair.pubkey();
    println!("Max Voter Weight Record Pubkey: {}", max_voter_weight_record_pubkey);

    let voter_weight_record_keypair: Keypair = read_keypair_file(VOTER_WEIGHT_RECORD_KEY_FILE_PATH).unwrap();
    let voter_weight_record_pubkey: Pubkey = voter_weight_record_keypair.pubkey();
    println!("Voter Weight Record Pubkey: {}", voter_weight_record_pubkey);

    let voter2_weight_record_keypair: Keypair = read_keypair_file(VOTER2_WEIGHT_RECORD_KEY_FILE_PATH).unwrap();
    let voter2_weight_record_pubkey: Pubkey = voter2_weight_record_keypair.pubkey();
    println!("Voter2 Weight Record Pubkey: {}", voter2_weight_record_pubkey);

    let voter3_weight_record_keypair: Keypair = read_keypair_file(VOTER3_WEIGHT_RECORD_KEY_FILE_PATH).unwrap();
    let voter3_weight_record_pubkey: Pubkey = voter3_weight_record_keypair.pubkey();
    println!("Voter3 Weight Record Pubkey: {}", voter3_weight_record_pubkey);

    let interactor = commands::SplGovernanceInteractor::new("http://localhost:8899", program_id, voter_weight_addin_pubkey);

    let realm: Realm = interactor.create_realm(owner_keypair, &community_pubkey, Some(voter_weight_addin_pubkey), REALM_NAME).unwrap();
    // println!("{:?}", realm);

    println!("Realm Pubkey: {}", interactor.get_realm_address(REALM_NAME));

    let result = interactor.setup_max_voter_weight_record(&realm, max_voter_weight_record_keypair, 10_000_000_000);
    // println!("{:?}", result);

    let token_owner: TokenOwner = interactor.create_token_owner_record(&realm, voter_keypair).unwrap();
    // println!("Token Owner {:?}", token_owner);

    let token_owner: TokenOwner = interactor.setup_voter_weight_record(&realm, token_owner, voter_weight_record_keypair, 7_000_000_000).unwrap();
    // println!("Token Owner {:?}", token_owner);

    let token_owner2: TokenOwner = interactor.create_token_owner_record(&realm, voter2_keypair).unwrap();
    // println!("Token Owner 2 {:?}", token_owner2);

    let token_owner2: TokenOwner = interactor.setup_voter_weight_record(&realm, token_owner2, voter2_weight_record_keypair, 2_000_000_000).unwrap();
    // println!("Token Owner 2 {:?}", token_owner2);

    let token_owner3: TokenOwner = interactor.create_token_owner_record(&realm, voter3_keypair).unwrap();
    // println!("Token Owner 3 {:?}", token_owner3);

    let token_owner3: TokenOwner = interactor.setup_voter_weight_record(&realm, token_owner3, voter3_weight_record_keypair, 1_000_000_000).unwrap();
    // println!("Token Owner 3 {:?}", token_owner3);

    let gov_config: GovernanceConfig =
        GovernanceConfig {
            vote_threshold_percentage: VoteThresholdPercentage::YesVote(1),
            min_community_weight_to_create_proposal: 1,
            min_transaction_hold_up_time: 0,
            max_voting_time: 78200,
            vote_tipping: VoteTipping::Disabled,
            proposal_cool_off_time: 0,
            min_council_weight_to_create_proposal: 0,
        };

    let governance: Governance = interactor.create_governance(&realm, &token_owner, &governed_account_pubkey, gov_config).unwrap();
    // println!("{:?}", governance);

    let proposal_number: u32 = 
        if governance.get_proposal_count() > 0 {
            // governance.get_proposal_count()
            0
        } else {
            0
        };
    let proposal: Proposal = interactor.create_proposal(&realm, &token_owner, &governance, PROPOSAL_NAME, PROPOSAL_DESCRIPTION, proposal_number).unwrap();
    // println!("{:?}", proposal);

    // let result = interactor.add_signatory(&realm, &governance, &proposal, &token_owner);
    // println!("Add signatory {:?}", result);

    let proposal: Proposal = 
        if proposal.data.state == ProposalState::Draft {
            interactor.sign_off_proposal(&realm, &governance, proposal, &token_owner).unwrap()
        } else {
            proposal
        };
    println!("{:?}\n", proposal);

    let result = interactor.cast_vote(&realm, &governance, &proposal, &token_owner, Some(max_voter_weight_record_pubkey), true);
    println!("{:?}", result);

    // let result = interactor.cast_vote(&realm, &governance, &proposal, &token_owner2, Some(max_voter_weight_record_pubkey), false);
    // println!("{:?}", result);

    // let result = interactor.cast_vote(&realm, &governance, &proposal, &token_owner3, Some(max_voter_weight_record_pubkey), true);
    // println!("{:?}", result);

}
