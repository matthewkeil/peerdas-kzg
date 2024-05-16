/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export class CellsAndProofs {
  cells: Array<Uint8Array>
  proofs: Array<Uint8Array>
}
export class ProverContextJs {
  constructor()
  blobToKzgCommitment(blob: Uint8Array): Uint8Array
  asyncBlobToKzgCommitment(blob: Uint8Array): Promise<Uint8Array>
  computeCellsAndKzgProofs(blob: Uint8Array): CellsAndProofs
  asyncComputeCellsAndKzgProofs(blob: Uint8Array): Promise<CellsAndProofs>
  computeCells(blob: Uint8Array): Array<Uint8Array>
  asyncComputeCells(blob: Uint8Array): Promise<Array<Uint8Array>>
}
export class VerifierContextJs {
  constructor()
  verifyCellKzgProof(commitment: Uint8Array, cellId: bigint, cell: Uint8Array, proof: Uint8Array): boolean
  asyncVerifyCellKzgProof(commitment: Uint8Array, cellId: bigint, cell: Uint8Array, proof: Uint8Array): Promise<boolean>
}
