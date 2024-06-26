use std::sync::{Arc, RwLock};

use napi::{
  bindgen_prelude::{BigInt, Error, Uint8Array},
  Result,
};
use napi_derive::napi;

use eip7594::{
  constants,
  prover::ProverContext,
  verifier::{VerifierContext, VerifierError},
};

#[napi]
pub const BYTES_PER_CELL: u32 = constants::BYTES_PER_CELL as u32;
#[napi]
pub const BYTES_PER_COMMITMENT: u32 = constants::BYTES_PER_COMMITMENT as u32;
#[napi]
pub const BYTES_PER_FIELD_ELEMENT: u32 = constants::BYTES_PER_FIELD_ELEMENT as u32;
#[napi]
pub const FIELD_ELEMENTS_PER_BLOB: u32 = constants::FIELD_ELEMENTS_PER_BLOB as u32;
#[napi]
pub const FIELD_ELEMENTS_PER_CELL: u32 = constants::FIELD_ELEMENTS_PER_CELL as u32;
#[napi]
pub const BYTES_PER_BLOB: u32 =
  (constants::FIELD_ELEMENTS_PER_BLOB * constants::BYTES_PER_FIELD_ELEMENT) as u32;

#[napi]
pub struct CellsAndProofs {
  pub cells: Vec<Uint8Array>,
  pub proofs: Vec<Uint8Array>,
}

#[napi]
pub struct ProverContextJs {
  inner: Arc<RwLock<ProverContext>>,
}

#[napi]
impl ProverContextJs {
  #[napi(constructor)]
  pub fn new() -> Self {
    ProverContextJs {
      inner: Arc::new(RwLock::new(ProverContext::new())),
    }
  }

  #[napi]
  pub fn blob_to_kzg_commitment(&self, blob: Uint8Array) -> Result<Uint8Array> {
    let blob = blob.as_ref();
    let prover_context = self
      .inner
      .read()
      .map_err(|err| Error::from_reason(&format!("Failed to acquire lock: {:?}", err)))?;

    let commitment = prover_context.blob_to_kzg_commitment(blob).map_err(|err| {
      Error::from_reason(&format!(
        "failed to compute blob_to_kzg_commitment: {:?}",
        err
      ))
    })?;
    Ok(Uint8Array::from(&commitment))
  }

  #[napi]
  pub async fn async_blob_to_kzg_commitment(&self, blob: Uint8Array) -> Result<Uint8Array> {
    self.blob_to_kzg_commitment(blob)
  }

  #[napi]
  pub fn compute_cells_and_kzg_proofs(&self, blob: Uint8Array) -> Result<CellsAndProofs> {
    let blob = blob.as_ref();
    let prover_context = self
      .inner
      .read()
      .map_err(|err| Error::from_reason(&format!("Failed to acquire lock: {:?}", err)))?;

    let (cells, proofs) = prover_context
      .compute_cells_and_kzg_proofs(blob)
      .map_err(|err| {
        Error::from_reason(&format!(
          "failed to compute compute_cells_and_kzg_proofs: {:?}",
          err
        ))
      })?;

    let cells_uint8array = cells
      .into_iter()
      .map(|cell| Uint8Array::from(cell))
      .collect::<Vec<Uint8Array>>();
    let proofs_uint8array = proofs
      .into_iter()
      .map(|proof| Uint8Array::from(proof))
      .collect::<Vec<Uint8Array>>();

    Ok(CellsAndProofs {
      cells: cells_uint8array,
      proofs: proofs_uint8array,
    })
  }

  #[napi]
  pub async fn async_compute_cells_and_kzg_proofs(
    &self,
    blob: Uint8Array,
  ) -> Result<CellsAndProofs> {
    self.compute_cells_and_kzg_proofs(blob)
  }

  #[napi]
  pub fn compute_cells(&self, blob: Uint8Array) -> Result<Vec<Uint8Array>> {
    let blob = blob.as_ref();
    let prover_context = self
      .inner
      .read()
      .map_err(|err| Error::from_reason(&format!("Failed to acquire lock: {:?}", err)))?;

    let cells = prover_context
      .compute_cells(blob)
      .map_err(|err| Error::from_reason(&format!("failed to compute compute_cells: {:?}", err)))?;

    let cells_uint8array = cells
      .into_iter()
      .map(|cell| Uint8Array::from(cell))
      .collect::<Vec<Uint8Array>>();

    Ok(cells_uint8array)
  }

  #[napi]
  pub async fn async_compute_cells(&self, blob: Uint8Array) -> Result<Vec<Uint8Array>> {
    self.compute_cells(blob)
  }
}

#[napi]
pub struct VerifierContextJs {
  inner: Arc<RwLock<VerifierContext>>,
}

#[napi]
impl VerifierContextJs {
  #[napi(constructor)]
  pub fn new() -> Self {
    VerifierContextJs {
      inner: Arc::new(RwLock::new(VerifierContext::new())),
    }
  }

  #[napi]
  pub fn verify_cell_kzg_proof(
    &self,
    commitment: Uint8Array,
    cell_id: BigInt,
    cell: Uint8Array,
    proof: Uint8Array,
  ) -> Result<bool> {
    let commitment = commitment.as_ref();
    let cell = cell.as_ref();
    let proof = proof.as_ref();
    let cell_id_u64 = bigint_to_u64(cell_id);

    // TODO: this map_err is repeated a few times, we can create a method for it
    let verifier_context = self
      .inner
      .read()
      .map_err(|err| Error::from_reason(&format!("Failed to acquire lock: {:?}", err)))?;

    let valid = verifier_context.verify_cell_kzg_proof(commitment, cell_id_u64, cell, proof);
    match valid {
      Ok(_) => Ok(true),
      Err(VerifierError::InvalidProof) => Ok(false),
      Err(err) => {
        return Err(Error::from_reason(&format!(
          "failed to compute verify_cell_kzg_proof: {:?}",
          err
        )))
      }
    }
  }

  #[napi]
  pub async fn async_verify_cell_kzg_proof(
    &self,
    commitment: Uint8Array,
    cell_id: BigInt,
    cell: Uint8Array,
    proof: Uint8Array,
  ) -> Result<bool> {
    self.verify_cell_kzg_proof(commitment, cell_id, cell, proof)
  }

  #[napi]
  pub fn verify_cell_kzg_proof_batch(
    &self,
    commitments: Vec<Uint8Array>,
    row_indices: Vec<BigInt>,
    column_indices: Vec<BigInt>,
    cells: Vec<Uint8Array>,
    proofs: Vec<Uint8Array>,
  ) -> Result<bool> {
    let row_indices: Vec<_> = row_indices.into_iter().map(bigint_to_u64).collect();
    let column_indices: Vec<_> = column_indices.into_iter().map(bigint_to_u64).collect();

    let commitments: Vec<_> = commitments.iter().map(|comm| comm.as_ref()).collect();
    let cells: Vec<_> = cells.iter().map(|cell| cell.as_ref()).collect();
    let proofs: Vec<_> = proofs.iter().map(|proof| proof.as_ref()).collect();

    let verifier_context = self
      .inner
      .read()
      .map_err(|err| Error::from_reason(&format!("Failed to acquire lock: {:?}", err)))?;

    let valid = verifier_context.verify_cell_kzg_proof_batch(
      commitments,
      row_indices,
      column_indices,
      cells,
      proofs,
    );
    match valid {
      Ok(_) => Ok(true),
      Err(VerifierError::InvalidProof) => Ok(false),
      Err(err) => {
        return Err(Error::from_reason(&format!(
          "failed to compute verify_cell_kzg_proof_batch: {:?}",
          err
        )))
      }
    }
  }

  #[napi]
  pub async fn async_verify_cell_kzg_proof_batch(
    &self,
    commitments: Vec<Uint8Array>,
    row_indices: Vec<BigInt>,
    column_indices: Vec<BigInt>,
    cells: Vec<Uint8Array>,
    proofs: Vec<Uint8Array>,
  ) -> Result<bool> {
    self.verify_cell_kzg_proof_batch(commitments, row_indices, column_indices, cells, proofs)
  }

  #[napi]
  pub fn recover_all_cells(
    &self,
    cell_ids: Vec<BigInt>,
    cells: Vec<Uint8Array>,
  ) -> Result<Vec<Uint8Array>> {
    let cell_ids: Vec<_> = cell_ids.into_iter().map(bigint_to_u64).collect();
    let cells: Vec<_> = cells.iter().map(|cell| cell.as_ref()).collect();

    let verifier_context = self
      .inner
      .read()
      .map_err(|err| Error::from_reason(&format!("Failed to acquire lock: {:?}", err)))?;

    let cells = verifier_context
      .recover_all_cells(cell_ids, cells)
      .map_err(|err| Error::from_reason(&format!("failed to compute compute_cells: {:?}", err)))?;

    let cells_uint8array = cells
      .into_iter()
      .map(|cell| Uint8Array::from(cell))
      .collect::<Vec<Uint8Array>>();

    Ok(cells_uint8array)
  }

  #[napi]
  pub async fn async_recover_all_cells(
    &self,
    cell_ids: Vec<BigInt>,
    cells: Vec<Uint8Array>,
  ) -> Result<Vec<Uint8Array>> {
    self.recover_all_cells(cell_ids, cells)
  }
}

// We use bigint because u64 cannot be used as an argument, see : https://napi.rs/docs/concepts/values.en#bigint
fn bigint_to_u64(value: BigInt) -> u64 {
  let (signed, value_u128, _) = value.get_u128();
  assert!(signed == false, "value should be an unsigned integer");
  value_u128 as u64
}
