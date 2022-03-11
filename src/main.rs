
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::{ Pubkey },
    // instruction::{ Instruction },
    transaction::{ Transaction },
    signer::{
        Signer,
        keypair::{ Keypair, read_keypair_file },
    },
};

use solana_client::rpc_client::{ RpcClient };

use borsh::{BorshDeserialize};

use spl_governance::{
    state::{
        enums::{
            VoteThresholdPercentage,
            // VoteWeightSource,
            VoteTipping,
            MintMaxVoteWeightSource,
            ProposalState,
        },
        governance::{
            GovernanceConfig,
            GovernanceV2,
            get_governance_address,
        },
        realm::{
            RealmV2,
            get_realm_address,
        },
        proposal::{
            VoteType,
            ProposalV2,
            get_proposal_address,
        },
        token_owner_record::{
            TokenOwnerRecordV2,
            get_token_owner_record_address,
        },
        vote_record::{
            Vote,
            VoteChoice,
        },
    },
    instruction::{
        create_realm,
        create_token_owner_record,
        create_governance,
        // set_governance_config,
        create_proposal,
        sign_off_proposal,
        cast_vote,
    }
};
use spl_governance_addin_api::{
    max_voter_weight::{
        MaxVoterWeightRecord,
    },
    voter_weight::{
        VoterWeightRecord,
    },
};
use spl_governance_addin_mock::{
    instruction::{
        setup_voter_weight_record,
        setup_max_voter_weight_record,
    }
};

mod tokens;
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

// const REALM_NAME: &'static str = "Test Realm";
const REALM_NAME: &'static str = "Test Realm 3";
// const REALM_NAME: &'static str = "Test Realm 6";
const PROPOSAL_NAME: &'static str = "Proposal 1";
const PROPOSAL_DESCRIPTION: &'static str = "proposal_1_description";
// const NETWORK: Network = Network::Local;

fn main() {

    let owner_keypair: Keypair = read_keypair_file(WALLET_FILE_PATH).unwrap();
    let owner_pubkey: Pubkey = owner_keypair.pubkey();
    println!("Owner Pubkey: {}", owner_pubkey);

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

    let interactor = commands::SplGovernanceInteractor::new("http://localhost:8899", program_id, voter_weight_addin_pubkey);

    // let realm_pubkey: Pubkey = interactor.get_realm_address(REALM_NAME);
    let realm: Realm = interactor.create_realm(owner_keypair, &community_pubkey, Some(voter_weight_addin_pubkey), REALM_NAME).unwrap();
        // if interactor.account_exists(&realm_pubkey) {
        //     interactor.read_realm(owner_keypair, &community_pubkey, Some(voter_weight_addin_pubkey), REALM_NAME).unwrap()
        // } else {
        //     interactor.create_realm(owner_keypair, &community_pubkey, Some(voter_weight_addin_pubkey), REALM_NAME).unwrap()
        // };
    println!("{:?}", realm);

    println!("Realm Pubkey: {}", interactor.get_realm_address(REALM_NAME));

    // let token_owner_record_pubkey: Pubkey = interactor.get_token_owner_record_address(&owner_pubkey, &community_pubkey, REALM_NAME);
    let token_owner: TokenOwner = interactor.create_token_owner_record(&realm).unwrap();
        // if interactor.account_exists(&realm_pubkey) {
        //     interactor.create_token_owner_record(&realm).unwrap()
        // } else {
        //     interactor.create_token_owner_record(&realm).unwrap()
        // };
    println!("{:?}", token_owner);

    // println!("Token Owner Record Pubkey: {}", interactor.get_token_owner_record_address(&owner_pubkey, &community_pubkey, REALM_NAME));
    
    // let result = interactor.setup_max_voter_weight_record(&realm, &max_voter_weight_record_pubkey, 10_000_000_000);
    let result = interactor.setup_max_voter_weight_record(&realm, max_voter_weight_record_keypair, 10_000_000_000);
    println!("{:?}", result);

    let token_owner: TokenOwner = interactor.setup_voter_weight_record(&realm, token_owner, voter_weight_record_keypair, 7_000_000_000).unwrap();
    println!("{:?}", token_owner);
    // let result = interactor.setup_voter_weight_record(&owner_keypair, &community_pubkey, &voter_weight_record_pubkey, REALM_NAME, 700_000_000);
    // println!("{:?}", result);

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
    println!("{:?}", governance);

    let proposal_number: u32 = 
        if governance.get_proposal_count() > 0 {
            1
        } else {
            0
        };
    let proposal: Proposal = interactor.create_proposal(&realm, &token_owner, &governance, PROPOSAL_NAME, PROPOSAL_DESCRIPTION, proposal_number).unwrap();
    println!("{:?}", proposal);

    let proposal: Proposal = 
        if proposal.data.state == ProposalState::Draft {
            interactor.sign_off_proposal(&realm, &governance, proposal, &token_owner).unwrap()
        } else {
            proposal
        };
    println!("{:?}", proposal);
    // return;

    let result = interactor.cast_vote(&realm, &governance, &proposal, &token_owner, Some(max_voter_weight_record_pubkey), true);
    println!("{:?}", result);

    return;

    let solana_client = RpcClient::new_with_commitment("http://localhost:8899".to_string(),CommitmentConfig::confirmed());
    // let solana_client = RpcClient::new_with_commitment("https://api.devnet.solana.com".to_string(),CommitmentConfig::confirmed());

    // tokens::create_accounts_mint_liquidity(&solana_client, &owner_keypair, &community_keypair, &community_pubkey);
    // return;

    // let realm_authority = Keypair::new();
    
    // let create_realm_instruction =
    //     create_realm(
    //         &program_id,
    //         // &realm_authority.pubkey(),
    //         &owner_pubkey,
    //         &community_pubkey,
    //         &owner_pubkey,
    //         None,
    //         Some(voter_weight_addin_pubkey),
    //         // None,
    //         Some(voter_weight_addin_pubkey),
    //         // None,
    //         REALM_NAME.to_string(),
    //         0,
    //         MintMaxVoteWeightSource::SupplyFraction(10_000_000_000),
    //     );
    
    // let transaction: Transaction =
    //     Transaction::new_signed_with_payer(
    //         &[
    //             create_realm_instruction,
    //         ],
    //         Some(&owner_pubkey),
    //         &[
    //             &owner_keypair,
    //         ],
    //         solana_client.get_latest_blockhash().unwrap(),
    //     );
    
    // let result = solana_client.send_and_confirm_transaction(&transaction);
    // println!("{:?}", result);

    let realm_pubkey: Pubkey = get_realm_address(&program_id, REALM_NAME);
    println!("Realm Pubkey: {}", realm_pubkey);

    let mut dt: &[u8] = &solana_client.get_account_data(&realm_pubkey).unwrap();
    let realm_v2: RealmV2 = RealmV2::deserialize(&mut dt).unwrap();
    println!("RealmV2: {:?}", realm_v2);
    return;

    let create_token_owner_record_instruction =
        create_token_owner_record(
            &program_id,
            &realm_pubkey,
            &owner_pubkey,
            &community_pubkey,
            &owner_pubkey,
        );
    
    let transaction: Transaction =
        Transaction::new_signed_with_payer(
            &[
                create_token_owner_record_instruction,
            ],
            Some(&owner_pubkey),
            &[
                &owner_keypair,
            ],
            solana_client.get_latest_blockhash().unwrap(),
        );
    
    // let result = solana_client.send_and_confirm_transaction(&transaction);
    // println!("{:?}", result);

    let token_owner_record_pubkey: Pubkey = get_token_owner_record_address(&program_id, &realm_pubkey, &community_pubkey, &owner_pubkey);
    println!("Token Owner Record Pubkey: {}", token_owner_record_pubkey);

    let mut dt: &[u8] = &solana_client.get_account_data(&token_owner_record_pubkey).unwrap();
    let token_owner_record: TokenOwnerRecordV2 = TokenOwnerRecordV2::deserialize(&mut dt).unwrap();
    println!("TokenOwnerRecordV2: {:?}",token_owner_record);
    // return;


    let setup_max_voter_weight_record_instruction =
        setup_max_voter_weight_record(
            &voter_weight_addin_pubkey,
            &realm_pubkey,
            &community_pubkey,
            &max_voter_weight_record_pubkey,
            &owner_pubkey,
            1000,
            None,
        );

    let transaction: Transaction =
        Transaction::new_signed_with_payer(
            &[
                setup_max_voter_weight_record_instruction,
            ],
            Some(&owner_pubkey),
            &[
                &owner_keypair,
                &max_voter_weight_record_keypair,
            ],
            solana_client.get_latest_blockhash().unwrap(),
        );
    
    // let result = solana_client.send_and_confirm_transaction(&transaction);
    // println!("{:?}", result);

    let mut dt: &[u8] = &solana_client.get_account_data(&max_voter_weight_record_pubkey).unwrap();
    let max_voter_weight_record: MaxVoterWeightRecord = MaxVoterWeightRecord::deserialize(&mut dt).unwrap();
    println!("MaxVoterWeightRecord: {:?}",max_voter_weight_record);

    let setup_voter_weight_record_instruction =
        setup_voter_weight_record(
            &voter_weight_addin_pubkey,
            &realm_pubkey,
            // &owner_pubkey,
            &community_pubkey,
            &owner_pubkey,
            &voter_weight_record_keypair.pubkey(),
            &owner_pubkey,
            1000,
            None,
            None,
            None,
        );

    let transaction: Transaction =
        Transaction::new_signed_with_payer(
            &[
                setup_voter_weight_record_instruction,
            ],
            Some(&owner_pubkey),
            &[
                &owner_keypair,
                &voter_weight_record_keypair,
            ],
            solana_client.get_latest_blockhash().unwrap(),
        );
    
    // let result = solana_client.send_and_confirm_transaction(&transaction);
    // println!("{:?}", result);

    let mut dt: &[u8] = &solana_client.get_account_data(&voter_weight_record_pubkey).unwrap();
    let voter_weight_record: VoterWeightRecord = VoterWeightRecord::deserialize(&mut dt).unwrap();
    println!("VoterWeightRecord: {:?}",voter_weight_record);

    // let governed_account_keypair = Keypair::new();
    // let governed_account_pubkey: Pubkey = governed_account_keypair.pubkey();
    // println!("Governed Account Pubkey: {}", governed_account_pubkey);
    let governed_account_opt: Option<&Pubkey> = Some(&governed_account_pubkey);

    let create_governance_instruction =
        create_governance(
            &program_id,
            &realm_pubkey,
            governed_account_opt,
            &token_owner_record_pubkey,
            &owner_pubkey,
            &owner_pubkey,
            // None,
            Some(voter_weight_record_keypair.pubkey()),
            gov_config,
        );
    
    let transaction: Transaction =
        Transaction::new_signed_with_payer(
            &[
                create_governance_instruction,
            ],
            Some(&owner_pubkey),
            &[
                &owner_keypair,
            ],
            solana_client.get_latest_blockhash().unwrap(),
        );
    
    // let result = solana_client.send_and_confirm_transaction(&transaction);
    // println!("{:?}", result);

    let governance_pubkey: Pubkey = get_governance_address(&program_id, &realm_pubkey, &governed_account_pubkey);
    println!("Governance Pubkey: {}", governance_pubkey);

    let mut dt: &[u8] = &solana_client.get_account_data(&governance_pubkey).unwrap();
    let governance_v2: GovernanceV2 = GovernanceV2::deserialize(&mut dt).unwrap();
    println!("GovernanceV2: {:?}",governance_v2);

    let proposal_owner_record: Pubkey = token_owner_record_pubkey;
    let voter_weight_record_opt: Option<Pubkey> = Some(voter_weight_record_keypair.pubkey());
    // let voter_weight_record_opt: Option<Pubkey> = None;

    let create_proposal_instruction =
        create_proposal(
            &program_id,
            &governance_pubkey,
            &proposal_owner_record,
            &owner_pubkey,
            // &governance_authority,
            // &community_pubkey,
            &owner_pubkey,
            voter_weight_record_opt,
            &realm_pubkey,
            PROPOSAL_NAME.to_string(),
            "description link_0".to_string(),
            &community_pubkey,
            VoteType::SingleChoice,
            vec!["Yes".to_string()],
            true,
            governance_v2.proposals_count,
        );
    
    let transaction: Transaction =
        Transaction::new_signed_with_payer(
            &[
                create_proposal_instruction,
            ],
            Some(&owner_pubkey),
            &[
                &owner_keypair,
                // &community_keypair,
            ],
            solana_client.get_latest_blockhash().unwrap(),
        );
    
    // let result = solana_client.send_and_confirm_transaction(&transaction);
    // println!("{:?}", result);

    // let proposal_pubkey: Pubkey = get_proposal_address(&program_id, &governance_pubkey, &community_pubkey, &vec![(governance_v2.proposals_count-1) as u8]);
    let proposal_index: [u8; 4] = [1,0,0,0];
    let proposal_pubkey: Pubkey = get_proposal_address(&program_id, &governance_pubkey, &community_pubkey, &proposal_index);
    println!("Proposal Pubkey: {}", governance_pubkey);

    let mut dt: &[u8] = &solana_client.get_account_data(&proposal_pubkey).unwrap();
    let proposal_v2: ProposalV2 = ProposalV2::deserialize(&mut dt).unwrap();
    println!("ProposalV2: {:?}", proposal_v2);

    let sign_off_proposal_instruction =
        sign_off_proposal(
            &program_id,
            &realm_pubkey,
            &governance_pubkey,
            &proposal_pubkey,
            &owner_pubkey,
            Some(&proposal_owner_record),
            // None,
        );

    let transaction: Transaction =
        Transaction::new_signed_with_payer(
            &[
                sign_off_proposal_instruction,
            ],
            Some(&owner_pubkey),
            &[
                &owner_keypair,
            ],
            solana_client.get_latest_blockhash().unwrap(),
        );
    
    // let result = solana_client.send_and_confirm_transaction(&transaction);
    // println!("{:?}", result);

    let vote_choice_item: VoteChoice =
        VoteChoice {
            rank: 0,
            weight_percentage: 100,
        };
    let cast_vote_instruction =
        cast_vote(
            &program_id,
            &realm_pubkey,
            &governance_pubkey,
            &proposal_pubkey,
            &proposal_v2.token_owner_record,
            // None,
            &token_owner_record_pubkey,
            &owner_pubkey,
            &community_pubkey,
            &owner_pubkey,
            Some(voter_weight_record_pubkey),
            Some(max_voter_weight_record_pubkey),
            Vote::Approve(vec![vote_choice_item]),
            // Vote::Deny,
        );

    let transaction: Transaction =
        Transaction::new_signed_with_payer(
            &[
                cast_vote_instruction,
            ],
            Some(&owner_pubkey),
            &[
                &owner_keypair,
            ],
            solana_client.get_latest_blockhash().unwrap(),
        );
    
    let result = solana_client.send_and_confirm_transaction(&transaction);
    println!("{:?}", result);
}
