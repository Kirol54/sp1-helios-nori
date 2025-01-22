use anyhow::Result;
use sp1_sdk::SP1ProofWithPublicValues;
use std::{env, fs, path::Path};

pub async fn handle_nori_proof(proof: &SP1ProofWithPublicValues, latest_block: u64) -> Result<()> {
    // Create directory to save the proofs
    let proof_path = format!("./sp1-helios-proofs");
    let proof_dir = Path::new(&proof_path);
    fs::create_dir_all(proof_dir)?;
    let if_mock = if env::var("SP1_PROVER").unwrap_or_default() == "mock" {
        "mock-"
    } else {
        ""
    };
    let filename = format!("{}{}-{}.json", if_mock, latest_block, proof.sp1_version);
    let file_path = proof_dir.join(filename);
    // Save the proof
    std::fs::write(&file_path, serde_json::to_string(&proof).unwrap()).unwrap();
    println!(
        "Proof saved successfully to {}",
        file_path.to_str().unwrap()
    );
    Ok(())
}
