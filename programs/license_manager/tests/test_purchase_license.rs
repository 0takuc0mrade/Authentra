use litesvm::LiteSVM;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use solana_system_interface::program as system_program;

mod helpers;
use helpers::*;

#[test]
fn test_purchase_license_success() {
    let (mut svm, licensee, program_id, license_config) = setup_litesvm_with_config();

    // This mint manager is a mock but its supposed to come from the creator-standard
    // we can use this for testing
    let mint_manager = Keypair::new();
    svm.airdrop(&mint_manager.pubkey(), 1_000_000_000).unwrap();

    let (active_license, _) = Pubkey::find_program_address(
        &[
            b"active-license",
            mint_manager.pubkey().as_ref(),
            licensee.pubkey().as_ref(),
        ],
        &program_id,
    );

    let mut data = vec![];
    data.extend_from_slice(
        // To correctly get the account discriminator dont forget to use the
        // name of the function in lib.rs rather than the ix handler in the ix file.
        &anchor_lang::solana_program::hash::hash(b"global:purchase_license").to_bytes()[..8],
    );
    println!("Instruction discriminator: {:?}", &data[..8]);

    let accounts = vec![
        AccountMeta::new(licensee.pubkey(), true),
        AccountMeta::new_readonly(license_config, false),
        AccountMeta::new(mint_manager.pubkey(), false),
        AccountMeta::new(active_license, false),
        AccountMeta::new_readonly(system_program::ID, false),
    ];

    let instruction = Instruction {
        program_id,
        accounts,
        data,
    };

    let licensee_balance_before = svm.get_balance(&licensee.pubkey()).unwrap();
    let mint_manager_balance_before = svm.get_balance(&mint_manager.pubkey()).unwrap();

    let blockhash = svm.latest_blockhash();
    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&licensee.pubkey()));
    transaction.sign(&[&licensee], blockhash);

    let result = svm.send_transaction(transaction);
    // println!("result of the tx {:#?}", result);
    assert!(
        result.is_ok(),
        "License purchase failed: {:#?}",
        result.err()
    );

    // Verify balances changed correctly
    let licensee_balance_after = svm.get_balance(&licensee.pubkey()).unwrap();
    let mint_manager_balance_after = svm.get_balance(&mint_manager.pubkey()).unwrap();

    let licensee_total_paid = licensee_balance_before - licensee_balance_after;
    let mint_manager_total_received = mint_manager_balance_after - mint_manager_balance_before;

    let active_license_account = svm.get_account(&active_license).unwrap();
    let rent_exemption = active_license_account.lamports;

    println!("Licensee paid total: {} lamports", licensee_total_paid);
    println!(
        "Mint manager received: {} lamports",
        mint_manager_total_received
    );
    println!(
        "Rent exemption for ActiveLicense: {} lamports",
        rent_exemption
    );
    println!("Expected license fee: 1,000,000 lamports");

    // Verify the mint manager received the exact license fee
    assert_eq!(
        mint_manager_total_received, 1_000_000,
        "Mint manager should have received exactly 1,000,000 lamports"
    );

    assert_eq!(
        licensee_total_paid,
        1_005_000 + rent_exemption,
        "Licensee should have paid license fee + rent exemption"
    );

    // Verify active license account was created with correct data
    let account_data = svm
        .get_account(&active_license)
        .expect("Active license account should exist");
    let data = &account_data.data;

    let mint_manager_from_account = Pubkey::new_from_array(data[8..40].try_into().unwrap());
    let licensee_from_account = Pubkey::new_from_array(data[40..72].try_into().unwrap());
    let purchase_amount = u64::from_le_bytes(data[72..80].try_into().unwrap());
    let is_active = data[88] == 1;

    assert_eq!(mint_manager_from_account, mint_manager.pubkey());
    assert_eq!(licensee_from_account, licensee.pubkey());
    assert_eq!(purchase_amount, 1_000_000);
    assert!(is_active, "License should be active");
}

#[test]
fn test_cannot_purchase_license_twice() {
    let (mut svm, licensee, program_id, license_config) = setup_litesvm_with_config();

    let mint_manager = Keypair::new();
    svm.airdrop(&mint_manager.pubkey(), 1_000_000_000).unwrap();

    let (active_license, _) = Pubkey::find_program_address(
        &[
            b"active-license",
            mint_manager.pubkey().as_ref(),
            licensee.pubkey().as_ref(),
        ],
        &program_id,
    );

    let mut data = vec![];
    data.extend_from_slice(
        &anchor_lang::solana_program::hash::hash(b"global:purchase_license").to_bytes()[..8],
    );

    let accounts = vec![
        AccountMeta::new(licensee.pubkey(), true),
        AccountMeta::new_readonly(license_config, false),
        AccountMeta::new(mint_manager.pubkey(), false),
        AccountMeta::new(active_license, false),
        AccountMeta::new_readonly(system_program::ID, false),
    ];

    let instruction = Instruction {
        program_id,
        accounts: accounts.clone(),
        data: data.clone(),
    };

    // First purchase should succeed
    let blockhash = svm.latest_blockhash();
    let mut tx1 = Transaction::new_with_payer(&[instruction.clone()], Some(&licensee.pubkey()));
    tx1.sign(&[&licensee], blockhash);
    let result1 = svm.send_transaction(tx1);
    assert!(result1.is_ok(), "First purchase should succeed",);

    // Second purchase should fail
    let blockhash2 = svm.latest_blockhash();
    let mut tx2 = Transaction::new_with_payer(&[instruction.clone()], Some(&licensee.pubkey()));
    tx2.sign(&[&licensee], blockhash2);
    let result2 = svm.send_transaction(tx2);

    assert!(
        result2.is_err(),
        "Second purchase should fail - license already exists"
    );
}

#[test]
fn test_purchase_license_insufficient_funds() {
    let (mut svm, _licensee, program_id, license_config) = setup_litesvm_with_config();

    // Create a brokie licensee
    let broke_licensee = Keypair::new();
    svm.airdrop(&broke_licensee.pubkey(), 50_000).unwrap();

    let mint_manager = Keypair::new();
    svm.airdrop(&mint_manager.pubkey(), 1_000_000_000).unwrap();

    let (active_license, _) = Pubkey::find_program_address(
        &[
            b"active-license",
            mint_manager.pubkey().as_ref(),
            broke_licensee.pubkey().as_ref(),
        ],
        &program_id,
    );

    let mut data = vec![];
    data.extend_from_slice(
        &anchor_lang::solana_program::hash::hash(b"global:purchase_license").to_bytes()[..8],
    );

    let accounts = vec![
        AccountMeta::new(broke_licensee.pubkey(), true),
        AccountMeta::new_readonly(license_config, false),
        AccountMeta::new(mint_manager.pubkey(), false),
        AccountMeta::new(active_license, false),
        AccountMeta::new_readonly(system_program::ID, false),
    ];

    let instruction = Instruction {
        program_id,
        accounts,
        data,
    };

    let blockhash = svm.latest_blockhash();
    let mut transaction =
        Transaction::new_with_payer(&[instruction], Some(&broke_licensee.pubkey()));
    transaction.sign(&[&broke_licensee], blockhash);

    let result = svm.send_transaction(transaction);

    assert!(
        result.is_err(),
        "Purchase should fail with insufficient funds"
    );
}

#[test]
fn test_purchase_license_without_config() {
    let mut svm = LiteSVM::new();
    let licensee = Keypair::new();
    let mint_manager = Keypair::new();

    svm.airdrop(&licensee.pubkey(), 10_000_000_000).unwrap();
    svm.airdrop(&mint_manager.pubkey(), 1_000_000_000).unwrap();

    let program_id = PROGRAM_ID_STR.parse::<Pubkey>().unwrap();
    let program_bytes = include_bytes!("../../../target/deploy/license_manager.so");
    let _ = svm.add_program(program_id, program_bytes);

    let (active_license, _) = Pubkey::find_program_address(
        &[
            b"active-license",
            mint_manager.pubkey().as_ref(),
            licensee.pubkey().as_ref(),
        ],
        &program_id,
    );

    let mut data = vec![];
    data.extend_from_slice(
        &anchor_lang::solana_program::hash::hash(b"global:purchase_license").to_bytes()[..8],
    );

    let accounts = vec![
        AccountMeta::new(licensee.pubkey(), true),
        AccountMeta::new_readonly(Pubkey::new_unique(), false),
        AccountMeta::new(mint_manager.pubkey(), false),
        AccountMeta::new(active_license, false),
        AccountMeta::new_readonly(system_program::ID, false),
    ];

    let instruction = Instruction {
        program_id,
        accounts,
        data,
    };

    let blockhash = svm.latest_blockhash();
    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&licensee.pubkey()));
    transaction.sign(&[&licensee], blockhash);

    let result = svm.send_transaction(transaction);

    assert!(
        result.is_err(),
        "Purchase should fail without initialized license config"
    );
}
