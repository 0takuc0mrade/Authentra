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
fn test_verify_license_success() {
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

    let purchase_data = anchor_lang::solana_program::hash::hash(b"global:purchase_license")
        .to_bytes()[..8]
        .to_vec();

    let purchase_instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(licensee.pubkey(), true),
            AccountMeta::new_readonly(license_config, false),
            AccountMeta::new(mint_manager.pubkey(), false),
            AccountMeta::new(active_license, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: purchase_data,
    };

    let blockhash = svm.latest_blockhash();
    let mut purchase_tx =
        Transaction::new_with_payer(&[purchase_instruction], Some(&licensee.pubkey()));
    purchase_tx.sign(&[&licensee], blockhash);
    svm.send_transaction(purchase_tx).unwrap();

    let verify_data =
        anchor_lang::solana_program::hash::hash(b"global:verify_license").to_bytes()[..8].to_vec();

    let verify_instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new_readonly(licensee.pubkey(), true),
            AccountMeta::new_readonly(mint_manager.pubkey(), false),
            AccountMeta::new_readonly(active_license, false),
        ],
        data: verify_data,
    };

    let verify_blockhash = svm.latest_blockhash();
    let mut verify_tx =
        Transaction::new_with_payer(&[verify_instruction], Some(&licensee.pubkey()));
    verify_tx.sign(&[&licensee], verify_blockhash);

    let result = svm.send_transaction(verify_tx);
    assert!(
        result.is_ok(),
        "License verification should succeed: {:#?}",
        result.err()
    );

    let license_account = svm
        .get_account(&active_license)
        .expect("Active license should exist");
    let data = &license_account.data;
    let is_active = data[88] == 1;
    assert!(
        is_active,
        "License should still be active after verification"
    );
}

#[test]
fn test_verify_license_without_purchase() {
    let (mut svm, licensee, program_id, _license_config) = setup_litesvm_with_config();

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

    let verify_data =
        anchor_lang::solana_program::hash::hash(b"global:verify_license").to_bytes()[..8].to_vec();

    let verify_instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new_readonly(licensee.pubkey(), true),
            AccountMeta::new(mint_manager.pubkey(), false),
            AccountMeta::new_readonly(active_license, false),
        ],
        data: verify_data,
    };

    let blockhash = svm.latest_blockhash();
    let mut verify_tx =
        Transaction::new_with_payer(&[verify_instruction], Some(&licensee.pubkey()));
    verify_tx.sign(&[&licensee], blockhash);

    let result = svm.send_transaction(verify_tx);
    assert!(
        result.is_err(),
        "Verification should fail without an existing license"
    );
}

#[test]
fn test_verify_license_wrong_licensee() {
    let (mut svm, licensee, program_id, license_config) = setup_litesvm_with_config();

    let mint_manager = Keypair::new();
    let wrong_licensee = Keypair::new();
    svm.airdrop(&mint_manager.pubkey(), 1_000_000_000).unwrap();
    svm.airdrop(&wrong_licensee.pubkey(), 1_000_000_000)
        .unwrap();

    let (active_license, _) = Pubkey::find_program_address(
        &[
            b"active-license",
            mint_manager.pubkey().as_ref(),
            licensee.pubkey().as_ref(),
        ],
        &program_id,
    );

    let purchase_data = anchor_lang::solana_program::hash::hash(b"global:purchase_license")
        .to_bytes()[..8]
        .to_vec();

    let purchase_instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(licensee.pubkey(), true),
            AccountMeta::new_readonly(license_config, false),
            AccountMeta::new(mint_manager.pubkey(), false),
            AccountMeta::new(active_license, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: purchase_data,
    };

    let blockhash = svm.latest_blockhash();
    let mut purchase_tx =
        Transaction::new_with_payer(&[purchase_instruction], Some(&licensee.pubkey()));
    purchase_tx.sign(&[&licensee], blockhash);
    svm.send_transaction(purchase_tx).unwrap();

    let verify_data =
        anchor_lang::solana_program::hash::hash(b"global:verify_license").to_bytes()[..8].to_vec();

    let verify_instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new_readonly(wrong_licensee.pubkey(), true), // Wrong licensee!
            AccountMeta::new(mint_manager.pubkey(), false),
            AccountMeta::new_readonly(active_license, false),
        ],
        data: verify_data,
    };

    let verify_blockhash = svm.latest_blockhash();
    let mut verify_tx =
        Transaction::new_with_payer(&[verify_instruction], Some(&wrong_licensee.pubkey()));
    verify_tx.sign(&[&wrong_licensee], verify_blockhash);

    let result = svm.send_transaction(verify_tx);
    assert!(
        result.is_err(),
        "Verification should fail with wrong licensee"
    );
}

#[test]
fn test_verify_license_wrong_mint_manager() {
    let (mut svm, licensee, program_id, license_config) = setup_litesvm_with_config();

    let mint_manager = Keypair::new();
    let wrong_mint_manager = Keypair::new();
    svm.airdrop(&mint_manager.pubkey(), 1_000_000_000).unwrap();
    svm.airdrop(&wrong_mint_manager.pubkey(), 1_000_000_000)
        .unwrap();

    let (active_license, _) = Pubkey::find_program_address(
        &[
            b"active-license",
            mint_manager.pubkey().as_ref(),
            licensee.pubkey().as_ref(),
        ],
        &program_id,
    );

    let purchase_data = anchor_lang::solana_program::hash::hash(b"global:purchase_license")
        .to_bytes()[..8]
        .to_vec();

    let purchase_instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(licensee.pubkey(), true),
            AccountMeta::new_readonly(license_config, false),
            AccountMeta::new(mint_manager.pubkey(), false),
            AccountMeta::new(active_license, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: purchase_data,
    };

    let blockhash = svm.latest_blockhash();
    let mut purchase_tx =
        Transaction::new_with_payer(&[purchase_instruction], Some(&licensee.pubkey()));
    purchase_tx.sign(&[&licensee], blockhash);
    svm.send_transaction(purchase_tx).unwrap();

    let verify_data =
        anchor_lang::solana_program::hash::hash(b"global:verify_license").to_bytes()[..8].to_vec();

    let verify_instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new_readonly(licensee.pubkey(), true),
            AccountMeta::new(wrong_mint_manager.pubkey(), false),
            AccountMeta::new_readonly(active_license, false),
        ],
        data: verify_data,
    };

    let verify_blockhash = svm.latest_blockhash();
    let mut verify_tx =
        Transaction::new_with_payer(&[verify_instruction], Some(&licensee.pubkey()));
    verify_tx.sign(&[&licensee], verify_blockhash);

    let result = svm.send_transaction(verify_tx);
    assert!(
        result.is_err(),
        "Verification should fail with wrong mint manager"
    );
}
