use litesvm::LiteSVM;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use solana_system_interface::instruction as system_instruction;
use solana_system_interface::program as system_program;

#[test]
fn create_account() {
    let mut svm = LiteSVM::new();
    let user = Keypair::new();

    svm.airdrop(&user.pubkey(), 1_000_000_000).unwrap();

    let balance = svm.get_balance(&user.pubkey()).unwrap();
    assert_eq!(balance, 1_000_000_000);
    println!("Account funded with {} sol", balance as f64 / 1e9)
}

#[test]
fn test_transfer() {
    let mut svm = LiteSVM::new();
    let alice = Keypair::new();
    let bob = Keypair::new();
    svm.airdrop(&alice.pubkey(), 2_000_000_000).unwrap();
    let transfer = system_instruction::transfer(&alice.pubkey(), &bob.pubkey(), 1_000_000_000);
    let tx = Transaction::new_signed_with_payer(
        &[transfer],
        Some(&alice.pubkey()),
        &[&alice],
        svm.latest_blockhash(),
    );
    svm.send_transaction(tx).unwrap();
    assert_eq!(svm.get_balance(&bob.pubkey()).unwrap(), 1_000_000_000);
    assert!(svm.get_balance(&alice.pubkey()).unwrap() < 1_000_000_000);
    println!("Transfer successful!");
}

mod helpers;
use helpers::*;

fn setup_litesvm() -> (LiteSVM, Keypair, Pubkey) {
    let mut svm = LiteSVM::new();
    let payer = Keypair::new();

    svm.airdrop(&payer.pubkey(), 10_000_000_000).unwrap();

    let program_id = PROGRAM_ID_STR.parse::<Pubkey>().unwrap();
    let program_bytes = include_bytes!("../../../target/deploy/license_manager.so");
    let _ = svm.add_program(program_id, program_bytes);

    (svm, payer, program_id)
}

#[test]
fn test_initialize_license_config_with_correct_default_values() {
    let (mut svm, authority, program_id) = setup_litesvm();

    let (license_config, _bump) = Pubkey::find_program_address(&[b"license-config"], &program_id);

    let mut data = vec![];
    data.extend_from_slice(
        &anchor_lang::solana_program::hash::hash(b"global:initialize_license_config").to_bytes()
            [..8],
    );

    let accounts = vec![
        AccountMeta::new(authority.pubkey(), true),
        AccountMeta::new(license_config, false),
        AccountMeta::new_readonly(system_program::ID, false),
    ];

    let instruction = Instruction {
        program_id,
        accounts,
        data,
    };

    let blockhash = svm.latest_blockhash();
    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&authority.pubkey()));
    transaction.sign(&[&authority], blockhash);

    let result = svm.send_transaction(transaction);

    assert!(result.is_ok(), "Transaction failed: {:#?}", result.err());

    let account_data = svm
        .get_account(&license_config)
        .expect("Account should exist");

    let data = &account_data.data;

    let authority_bytes = &data[8..40];
    let authority_from_account = Pubkey::new_from_array(authority_bytes.try_into().unwrap());

    let default_fee = u64::from_le_bytes(data[40..48].try_into().unwrap());
    let platform_fee_bps = u16::from_le_bytes(data[48..50].try_into().unwrap());

    assert_eq!(
        authority_from_account,
        authority.pubkey(),
        "Authority mismatch"
    );
    assert_eq!(
        default_fee, 1_000_000,
        "Default fee should be 1_000_000 (0.001 SOL)"
    );
    assert_eq!(platform_fee_bps, 300, "Platform fee should be 300 bps (3%)");
}

#[test]
fn test_cannot_initialize_twice() {
    let (mut svm, authority, program_id) = setup_litesvm();

    let (license_config, _) = Pubkey::find_program_address(&[b"license-config"], &program_id);

    let mut data = vec![];
    data.extend_from_slice(
        &anchor_lang::solana_program::hash::hash(b"global:initialize_license_config").to_bytes()
            [..8],
    );

    let accounts = vec![
        AccountMeta::new(authority.pubkey(), true),
        AccountMeta::new(license_config, false),
        AccountMeta::new_readonly(system_program::ID, false),
    ];

    let instruction = Instruction {
        program_id,
        accounts,
        data: data.clone(),
    };

    let blockhash = svm.latest_blockhash();
    let mut tx1 = Transaction::new_with_payer(&[instruction.clone()], Some(&authority.pubkey()));
    tx1.sign(&[&authority], blockhash);

    let result1 = svm.send_transaction(tx1);
    assert!(result1.is_ok(), "First initialization should succeed");

    let blockhash2 = svm.latest_blockhash();
    let mut tx2 = Transaction::new_with_payer(&[instruction], Some(&authority.pubkey()));
    tx2.sign(&[&authority], blockhash2);

    let result2 = svm.send_transaction(tx2);
    assert!(
        result2.is_err(),
        "Second initialization should fail - account already exists"
    );
}

#[test]
fn test_initialize_with_insufficient_lamports() {
    let (mut svm, _payer, program_id) = setup_litesvm();

    let broke_authority = Keypair::new();
    svm.airdrop(&broke_authority.pubkey(), 1000).unwrap();

    let (license_config, _) = Pubkey::find_program_address(&[b"license-config"], &program_id);

    let mut data = vec![];
    data.extend_from_slice(
        &anchor_lang::solana_program::hash::hash(b"global:initialize_license_config").to_bytes()
            [..8],
    );

    let accounts = vec![
        AccountMeta::new(broke_authority.pubkey(), true),
        AccountMeta::new(license_config, false),
        AccountMeta::new_readonly(system_program::ID, false),
    ];

    let instruction = Instruction {
        program_id,
        accounts,
        data,
    };

    let blockhash = svm.latest_blockhash();
    let mut transaction =
        Transaction::new_with_payer(&[instruction], Some(&broke_authority.pubkey()));
    transaction.sign(&[&broke_authority], blockhash);

    let result = svm.send_transaction(transaction);

    assert!(
        result.is_err(),
        "Transaction should fail with insufficient funds"
    );
}
