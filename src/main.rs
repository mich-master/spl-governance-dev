
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

use spl_governance::{
    state::{
        enums::{
            VoteThresholdPercentage,
            // VoteWeightSource,
            VoteTipping,
            MintMaxVoteWeightSource,
        },
        governance::{
            GovernanceConfig,
            get_governance_address,
        },
        realm::{
            get_realm_address,
        },
        proposal::{
            VoteType,
        },
        token_owner_record::{
            get_token_owner_record_address,
        },
    },
    instruction::{
        create_realm,
        create_token_owner_record,
        create_governance,
        // set_governance_config,
        create_proposal,
        // cast_vote,
    }
};
use spl_governance_addin_mock::{
    instruction::{
        setup_voter_weight_record,
    }
};

const WALLET_FILE_PATH: &'static str = "/home/mich/.config/solana/id.json";

// const GOVERNANCE_KEY_FILE_PATH: &'static str = "/media/mich/speedwork/NeonLabs/artifacts/governance.json";
const GOVERNANCE_KEY_FILE_PATH: &'static str = "/media/mich/speedwork/SolanaProgs/solana-program-library/target/deploy/spl_governance-keypair.json";
const VOTER_WEIGHT_ADDIN_KEY_FILE_PATH: &'static str = "/media/mich/speedwork/SolanaProgs/solana-program-library/target/deploy/spl_governance_addin_mock-keypair.json";
const COMMUTINY_MINT_KEY_FILE_PATH: &'static str = "/media/mich/speedwork/NeonLabs/artifacts/dev/token_mints/USDT.keypair";

const REALM_NAME: &'static str = "Test Realm 6";
const PROPOSAL_NAME: &'static str = "Proposal 1";
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


    let solana_client = RpcClient::new_with_commitment("http://localhost:8899".to_string(),CommitmentConfig::confirmed());
    // let solana_client = RpcClient::new_with_commitment(NETWORK.get_solana_url().to_string(),CommitmentConfig::confirmed());

    let gov_config: GovernanceConfig =
        GovernanceConfig {
            vote_threshold_percentage: VoteThresholdPercentage::YesVote(1),
            // min_community_tokens_to_create_proposal: 1,
            // min_instruction_hold_up_time: 1,
            // vote_weight_source: VoteWeightSource::Deposit,
            min_community_weight_to_create_proposal: 1,
            min_transaction_hold_up_time: 1,
            max_voting_time: 3600,
            vote_tipping: VoteTipping::Strict,
            proposal_cool_off_time: 0,
            min_council_weight_to_create_proposal: 1,
            // min_council_tokens_to_create_proposal: 1,
        };

    // let realm_authority = Keypair::new();
    
    let create_realm_instruction =
        create_realm(
            &program_id,
            // &realm_authority.pubkey(),
            &owner_pubkey,
            &community_pubkey,
            &owner_pubkey,
            None,
            Some(voter_weight_addin_pubkey),
            None,
            REALM_NAME.to_string(),
            1,
            MintMaxVoteWeightSource::SupplyFraction(10_000_000_000),
        );
    
    let transaction: Transaction =
        Transaction::new_signed_with_payer(
            &[
                create_realm_instruction,
            ],
            Some(&owner_pubkey),
            &[
                &owner_keypair,
            ],
            solana_client.get_latest_blockhash().unwrap(),
        );
    
    let result = solana_client.send_and_confirm_transaction(&transaction);
    println!("{:?}", result);

    let realm_pubkey: Pubkey = get_realm_address(&program_id, REALM_NAME);

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
    
    let result = solana_client.send_and_confirm_transaction(&transaction);
    println!("{:?}", result);

    let token_owner_record_pubkey: Pubkey = get_token_owner_record_address(&program_id, &realm_pubkey, &community_pubkey, &owner_pubkey);

    let voter_weight_record_keypair = Keypair::new();

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
    
    let result = solana_client.send_and_confirm_transaction(&transaction);
    println!("{:?}", result);


    let governed_account_keypair = Keypair::new();
    let governed_account_pubkey: Pubkey = governed_account_keypair.pubkey();
    let governed_account_opt: Option<&Pubkey> = None;

    let create_governance_instruction =
        create_governance(
            &program_id,
            &realm_pubkey,
            governed_account_opt,
            &token_owner_record_pubkey,
            &owner_pubkey,
            &owner_pubkey,
            Some(voter_weight_record_keypair.pubkey()),
            // None,
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
    
    let result = solana_client.send_and_confirm_transaction(&transaction);
    println!("{:?}", result);

    let governance_pubkey: Pubkey = get_governance_address(&program_id, &realm_pubkey, &governed_account_pubkey);
    let proposal_owner_record: Pubkey = token_owner_record_pubkey;
    let voter_weight_record_opt: Option<Pubkey> = None;

    let create_proposal_instruction =
        create_proposal(
            &program_id,
            &governance_pubkey,
            &proposal_owner_record,
            &owner_pubkey,
            // &governance_authority,
            &owner_pubkey,
            voter_weight_record_opt,
            &realm_pubkey,
            PROPOSAL_NAME.to_string(),
            "description link".to_string(),
            &community_pubkey,
            VoteType::SingleChoice,
            vec!["option 1".to_string()],
            false,
            1,
        );
    
    let transaction: Transaction =
        Transaction::new_signed_with_payer(
            &[
                create_proposal_instruction,
            ],
            Some(&owner_pubkey),
            &[
                &owner_keypair,
            ],
            solana_client.get_latest_blockhash().unwrap(),
        );
    
    let result = solana_client.send_and_confirm_transaction(&transaction);
    println!("{:?}", result);

    // let cast_vote_instruction =
    //     cast_vote(
    //         &program_id,
    //         &realm_pubkey,
    //         None,
    //         &token_owner_record_pubkey,
    //         &owner_pubkey,
    //         &owner_pubkey,
    //         Some(voter_weight_record_keypair.pubkey()),
    //         gov_config,
    //     );

    // let transaction: Transaction =
    //     Transaction::new_signed_with_payer(
    //         &[
    //             cast_vote_instruction,
    //         ],
    //         Some(&owner_pubkey),
    //         &[
    //             &owner_keypair,
    //             &voter_weight_record_keypair,
    //             // &governance_keypair,
    //             // &community_keypair,
    //         ],
    //         latest_blockhash,
    //     );
    
    // let result = solana_client.send_and_confirm_transaction(&transaction);
    // println!("{:?}", result);
}
