use litesvm::LiteSVM;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use solana_system_interface::program as system_program;

pub const PROGRAM_ID_STR: &str = "46768WTgs123tGwaS1XJ4dVd8eSUheh3rSvzkuBVM4y5";

pub fn setup_litesvm_with_config() -> (LiteSVM, Keypair, Pubkey, Pubkey) {
    let mut svm = LiteSVM::new();
    let authority = Keypair::new();
    let licensee = Keypair::new();

    // Fund both accounts
    svm.airdrop(&authority.pubkey(), 10_000_000_000).unwrap();
    svm.airdrop(&licensee.pubkey(), 10_000_000_000).unwrap();

    let program_id = PROGRAM_ID_STR.parse::<Pubkey>().unwrap();
    let program_bytes = include_bytes!("../../../target/deploy/license_manager.so");
    let _ = svm.add_program(program_id, program_bytes);

    // First, initialize license config
    let (license_config, _) = Pubkey::find_program_address(&[b"license-config"], &program_id);

    let init_data = anchor_lang::solana_program::hash::hash(b"global:initialize_license_config")
        .to_bytes()[..8]
        .to_vec();

    let init_instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(authority.pubkey(), true),
            AccountMeta::new(license_config, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: init_data,
    };

    let blockhash = svm.latest_blockhash();
    let mut init_tx = Transaction::new_with_payer(&[init_instruction], Some(&authority.pubkey()));
    init_tx.sign(&[&authority], blockhash);
    svm.send_transaction(init_tx).unwrap();

    (svm, licensee, program_id, license_config)
}

pub fn create_fresh_transaction(
    svm: &mut LiteSVM,
    instructions: Vec<Instruction>,
    signers: &[&Keypair],
    payer: &Pubkey,
) -> Transaction {
    let blockhash = svm.latest_blockhash();
    let mut transaction = Transaction::new_with_payer(&instructions, Some(payer));
    transaction.sign(signers, blockhash);
    transaction
}
