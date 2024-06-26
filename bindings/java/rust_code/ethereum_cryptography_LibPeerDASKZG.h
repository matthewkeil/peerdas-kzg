/* DO NOT EDIT THIS FILE - it is machine generated */
#include <jni.h>
/* Header for class ethereum_cryptography_LibPeerDASKZG */

#ifndef _Included_ethereum_cryptography_LibPeerDASKZG
#define _Included_ethereum_cryptography_LibPeerDASKZG
#ifdef __cplusplus
extern "C" {
#endif
#undef ethereum_cryptography_LibPeerDASKZG_BYTES_PER_G1
#define ethereum_cryptography_LibPeerDASKZG_BYTES_PER_G1 48L
#undef ethereum_cryptography_LibPeerDASKZG_BYTES_PER_G2
#define ethereum_cryptography_LibPeerDASKZG_BYTES_PER_G2 96L
#undef ethereum_cryptography_LibPeerDASKZG_BYTES_PER_COMMITMENT
#define ethereum_cryptography_LibPeerDASKZG_BYTES_PER_COMMITMENT 48L
#undef ethereum_cryptography_LibPeerDASKZG_BYTES_PER_PROOF
#define ethereum_cryptography_LibPeerDASKZG_BYTES_PER_PROOF 48L
#undef ethereum_cryptography_LibPeerDASKZG_BYTES_PER_FIELD_ELEMENT
#define ethereum_cryptography_LibPeerDASKZG_BYTES_PER_FIELD_ELEMENT 32L
#undef ethereum_cryptography_LibPeerDASKZG_BITS_PER_FIELD_ELEMENT
#define ethereum_cryptography_LibPeerDASKZG_BITS_PER_FIELD_ELEMENT 255L
#undef ethereum_cryptography_LibPeerDASKZG_FIELD_ELEMENTS_PER_BLOB
#define ethereum_cryptography_LibPeerDASKZG_FIELD_ELEMENTS_PER_BLOB 4096L
#undef ethereum_cryptography_LibPeerDASKZG_BYTES_PER_BLOB
#define ethereum_cryptography_LibPeerDASKZG_BYTES_PER_BLOB 131072L
#undef ethereum_cryptography_LibPeerDASKZG_FIELD_ELEMENTS_PER_EXT_BLOB
#define ethereum_cryptography_LibPeerDASKZG_FIELD_ELEMENTS_PER_EXT_BLOB 8192L
#undef ethereum_cryptography_LibPeerDASKZG_FIELD_ELEMENTS_PER_CELL
#define ethereum_cryptography_LibPeerDASKZG_FIELD_ELEMENTS_PER_CELL 64L
#undef ethereum_cryptography_LibPeerDASKZG_CELLS_PER_EXT_BLOB
#define ethereum_cryptography_LibPeerDASKZG_CELLS_PER_EXT_BLOB 128L
#undef ethereum_cryptography_LibPeerDASKZG_BYTES_PER_CELL
#define ethereum_cryptography_LibPeerDASKZG_BYTES_PER_CELL 2048L
/*
 * Class:     ethereum_cryptography_LibPeerDASKZG
 * Method:    peerDASContextNew
 * Signature: ()J
 */
JNIEXPORT jlong JNICALL Java_ethereum_cryptography_LibPeerDASKZG_peerDASContextNew
  (JNIEnv *, jclass);

/*
 * Class:     ethereum_cryptography_LibPeerDASKZG
 * Method:    peerDASContextDestroy
 * Signature: (J)V
 */
JNIEXPORT void JNICALL Java_ethereum_cryptography_LibPeerDASKZG_peerDASContextDestroy
  (JNIEnv *, jclass, jlong);

/*
 * Class:     ethereum_cryptography_LibPeerDASKZG
 * Method:    computeCells
 * Signature: (J[B)[B
 */
JNIEXPORT jbyteArray JNICALL Java_ethereum_cryptography_LibPeerDASKZG_computeCells
  (JNIEnv *, jclass, jlong, jbyteArray);

/*
 * Class:     ethereum_cryptography_LibPeerDASKZG
 * Method:    computeCellsAndKZGProofs
 * Signature: (J[B)Lethereum/cryptography/CellsAndProofs;
 */
JNIEXPORT jobject JNICALL Java_ethereum_cryptography_LibPeerDASKZG_computeCellsAndKZGProofs
  (JNIEnv *, jclass, jlong, jbyteArray);

/*
 * Class:     ethereum_cryptography_LibPeerDASKZG
 * Method:    blobToKZGCommitment
 * Signature: (J[B)[B
 */
JNIEXPORT jbyteArray JNICALL Java_ethereum_cryptography_LibPeerDASKZG_blobToKZGCommitment
  (JNIEnv *, jclass, jlong, jbyteArray);

/*
 * Class:     ethereum_cryptography_LibPeerDASKZG
 * Method:    verifyCellKZGProof
 * Signature: (J[BJ[B[B)Z
 */
JNIEXPORT jboolean JNICALL Java_ethereum_cryptography_LibPeerDASKZG_verifyCellKZGProof
  (JNIEnv *, jclass, jlong, jbyteArray, jlong, jbyteArray, jbyteArray);

/*
 * Class:     ethereum_cryptography_LibPeerDASKZG
 * Method:    verifyCellKZGProofBatch
 * Signature: (J[B[J[J[B[B)Z
 */
JNIEXPORT jboolean JNICALL Java_ethereum_cryptography_LibPeerDASKZG_verifyCellKZGProofBatch
  (JNIEnv *, jclass, jlong, jbyteArray, jlongArray, jlongArray, jbyteArray, jbyteArray);

/*
 * Class:     ethereum_cryptography_LibPeerDASKZG
 * Method:    recoverAllCells
 * Signature: (J[J[B)[B
 */
JNIEXPORT jbyteArray JNICALL Java_ethereum_cryptography_LibPeerDASKZG_recoverAllCells
  (JNIEnv *, jclass, jlong, jlongArray, jbyteArray);

#ifdef __cplusplus
}
#endif
#endif
