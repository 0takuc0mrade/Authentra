use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use solana_system_interface::program as system_program;

mod helpers;
use helpers::*;

#[test]
fn test_complete_license_flow() {
    let (mut svm, licensee, program_id, license_config) = setup_litesvm_with_config();

    // Simulate creator registering content
    let creator = Keypair::new();
    let mint_manager = Keypair::new(); // This would come from creator-standard
    svm.airdrop(&creator.pubkey(), 1_000_000_000).unwrap();
    svm.airdrop(&mint_manager.pubkey(), 1_000_000_000).unwrap();

    println!("Starting complete license flow test");

    // Step 1: Purchase license
    let (active_license, _) = Pubkey::find_program_address(
        &[
            b"active-license",
            mint_manager.pubkey().as_ref(),
            licensee.pubkey().as_ref(),
        ],
        &program_id,
    );

    let licensee_balance_before = svm.get_balance(&licensee.pubkey()).unwrap();

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
        data: purchase_data.clone(),
    };

    let purchase_tx = create_fresh_transaction(
        &mut svm,
        vec![purchase_instruction],
        &[&licensee],
        &licensee.pubkey(),
    );

    println!("Purchasing license...");
    let purchase_result = svm.send_transaction(purchase_tx);
    assert!(
        purchase_result.is_ok(),
        "License purchase failed: {:#?}",
        purchase_result.err()
    );

    // Verify payment occurred
    let licensee_balance_after = svm.get_balance(&licensee.pubkey()).unwrap();
    let license_account = svm.get_account(&active_license).unwrap();
    let rent_paid = license_account.lamports;

    let total_paid = licensee_balance_before - licensee_balance_after;
    let license_fee = 1_000_000; // From config
    let expected_min = license_fee + rent_paid;
    let expected_max = license_fee + rent_paid + 10_000; // variance

    assert!(
        total_paid >= expected_min && total_paid <= expected_max,
        "Licensee should pay fee + rent (with small variance). Paid: {}, Expected: {} ± 5,000",
        total_paid,
        license_fee + rent_paid
    );

    println!(
        "Payment verified: {} lamports (fee: {} + rent: {} ± variance)",
        total_paid, license_fee, rent_paid
    );

    // Step 2: Verify license exists and is active
    let license_data = &license_account.data;
    let is_active = license_data[88] == 1;
    let stored_licensee = Pubkey::new_from_array(license_data[40..72].try_into().unwrap());
    let stored_mint_manager = Pubkey::new_from_array(license_data[8..40].try_into().unwrap());

    assert!(is_active, "License should be active after purchase");
    assert_eq!(
        stored_licensee,
        licensee.pubkey(),
        "Licensee should be stored correctly"
    );
    assert_eq!(
        stored_mint_manager,
        mint_manager.pubkey(),
        "Mint manager should be stored correctly"
    );

    println!("✅ License account created and active");

    // Step 3: Verify the license
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

    let verify_tx = create_fresh_transaction(
        &mut svm,
        vec![verify_instruction],
        &[&licensee],
        &licensee.pubkey(),
    );

    println!("Verifying license...");
    let verify_result = svm.send_transaction(verify_tx);
    assert!(
        verify_result.is_ok(),
        "License verification failed: {:#?}",
        verify_result.err()
    );

    println!("License verified successfully");

    // Step 4: Test that license cannot be purchased again

    let purchase_instruction_2 = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(licensee.pubkey(), true),
            AccountMeta::new_readonly(license_config, false),
            AccountMeta::new(mint_manager.pubkey(), false),
            AccountMeta::new(active_license, false), // Account already exists!
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: purchase_data,
    };

    let purchase_tx_2 = create_fresh_transaction(
        &mut svm,
        vec![purchase_instruction_2],
        &[&licensee],
        &licensee.pubkey(),
    );

    println!("Attempting duplicate purchase...");
    let duplicate_result = svm.send_transaction(purchase_tx_2);
    assert!(duplicate_result.is_err(), "Duplicate purchase should fail");

    println!("Duplicate purchase correctly prevented");

    println!("Complete license flow test passed!");
}
