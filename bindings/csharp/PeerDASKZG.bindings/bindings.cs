﻿// <auto-generated>
// This code is generated by csbindgen.
// DON'T CHANGE THIS DIRECTLY.
// </auto-generated>
#pragma warning disable CS8500
#pragma warning disable CS8981
using System;
using System.Runtime.InteropServices;


namespace PeerDASKZG
{
    public static unsafe partial class PeerDASKZG
    {
        const string __DllName = "c_peerdas_kzg";



        [DllImport(__DllName, EntryPoint = "peerdas_context_new", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern PeerDASContext* peerdas_context_new();

        [DllImport(__DllName, EntryPoint = "peerdas_context_new_with_setting", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern PeerDASContext* peerdas_context_new_with_setting(CContextSetting setting);

        [DllImport(__DllName, EntryPoint = "peerdas_context_free", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void peerdas_context_free(PeerDASContext* ctx);

        /// <summary>Safety: - The caller must ensure that the pointers are valid. If pointers are null, this method will return an error. - The caller must ensure that `blob` points to a region of memory that is at least `BYTES_PER_BLOB` bytes. - The caller must ensure that `out` points to a region of memory that is at least `BYTES_PER_COMMITMENT` bytes.</summary>
        [DllImport(__DllName, EntryPoint = "blob_to_kzg_commitment", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern CResult blob_to_kzg_commitment(PeerDASContext* ctx, ulong blob_length, byte* blob, byte* @out);

        /// <summary>Safety: - The caller must ensure that the pointers are valid. If pointers are null, this method will return an error. - The caller must ensure that `blob` points to a region of memory that is at least `BYTES_PER_BLOB` bytes. - The caller must ensure that `out_cells` points to a region of memory that is at least `NUM_BYTES_CELLS` bytes.</summary>
        [DllImport(__DllName, EntryPoint = "compute_cells", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern CResult compute_cells(PeerDASContext* ctx, ulong blob_length, byte* blob, byte* out_cells);

        /// <summary>Safety: - The caller must ensure that the pointers are valid. If pointers are null, this method will return an error. - The caller must ensure that `blob` points to a region of memory that is at least `BYTES_PER_BLOB` bytes. - The caller must ensure that `out_cells` points to a region of memory that is at least `NUM_BYTES_CELLS` bytes. - The caller must ensure that `out_proofs` points to a region of memory that is at least `NUM_BYTES_PROOFS` bytes.</summary>
        [DllImport(__DllName, EntryPoint = "compute_cells_and_kzg_proofs", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern CResult compute_cells_and_kzg_proofs(PeerDASContext* ctx, ulong blob_length, byte* blob, byte* out_cells, byte* out_proofs);

        /// <summary>Safety: - The caller must ensure that the pointers are valid. If pointers are null, this method will return an error. - The caller must ensure that `cell` points to a region of memory that is at least `BYTES_PER_CELL` bytes. - The caller must ensure that `commitment` points to a region of memory that is at least `BYTES_PER_COMMITMENT` bytes. - The caller must ensure that `proof` points to a region of memory that is at least `BYTES_PER_COMMITMENT` bytes. - The caller must ensure that `verified` points to a region of memory that is at least 1 byte.</summary>
        [DllImport(__DllName, EntryPoint = "verify_cell_kzg_proof", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern CResult verify_cell_kzg_proof(PeerDASContext* ctx, ulong cell_length, byte* cell, ulong commitment_length, byte* commitment, ulong cell_id, ulong proof_length, byte* proof, bool* verified);

        /// <summary>Safety: - The caller must ensure that the pointers are valid. If pointers are null, this method will return an error. - The caller must ensure that `row_commitments` points to a region of memory that is at least `row_commitments_length` bytes. - The caller must ensure that `row_indices` points to a region of memory that is at least `num_cells` bytes. - The caller must ensure that `column_indices` points to a region of memory that is at least `num_cells` bytes. - The caller must ensure that `cells` points to a region of memory that is at least `cells_length` bytes. - The caller must ensure that `proofs` points to a region of memory that is at least `num_cells * BYTES_PER_COMMITMENT` bytes. - The caller must ensure that `verified` points to a region of memory that is at least 1 byte.  Note: cells, proofs and row_commitments are expected to be contiguous in memory. ie they have been concatenated together</summary>
        [DllImport(__DllName, EntryPoint = "verify_cell_kzg_proof_batch", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern CResult verify_cell_kzg_proof_batch(PeerDASContext* ctx, ulong row_commitments_length, byte* row_commitments, ulong row_indices_length, ulong* row_indices, ulong column_indices_length, ulong* column_indices, ulong cells_length, byte* cells, ulong proofs_length, byte* proofs, bool* verified);

        /// <summary>Safety: - The caller must ensure that the pointers are valid. If pointers are null, this method will return an error. - The caller must ensure that `cell_ids` points to a region of memory that is at least `num_cells` bytes. - The caller must ensure that `cells` points to a region of memory that is at least `cells_length` bytes. - The caller must ensure that `out_cells` points to a region of memory that is at least `NUM_BYTES_CELLS` bytes.</summary>
        [DllImport(__DllName, EntryPoint = "recover_all_cells", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern CResult recover_all_cells(PeerDASContext* ctx, ulong cells_length, byte* cells, ulong cell_ids_length, ulong* cell_ids, byte* out_cells);


    }

    [StructLayout(LayoutKind.Sequential)]
    public unsafe partial struct PeerDASContext
    {
    }


    public enum CContextSetting : uint
    {
        ProvingOnly,
        VerifyOnly,
        Both,
    }

    public enum CResult : uint
    {
        Ok,
        Err,
    }


}
