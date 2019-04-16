//! frodokem976shake
//!
//! These bindings use the clean version from [PQClean][pqc]
//!
//! # Examples
//! ```
//! use pqcrypto_frodo::frodokem976shake::*;
//! let (pk, sk) = keypair();
//! let (ss1, ct) = encapsulate(pk);
//! let ss2 = decapsulate(ct, sk);
//! assert!(ss1 == ss2);
//! ```
//!
//! [pqc]: https://github.com/pqclean/pqclean/

// This file is generated.

use std::mem;

use crate::ffi;
use pqcrypto_traits::kem as primitive;

macro_rules! simple_struct {
    ($type: ident, $size: expr) => {
        pub struct $type([u8; $size]);

        impl $type {
            /// Generates an uninitialized object
            ///
            /// Used to pass to ``ffi`` interfaces.
            ///
            /// Internal use only!
            fn new() -> Self {
                $type(unsafe { mem::uninitialized() })
            }
        }

        impl primitive::$type for $type {
            /// Get this object as a byte slice
            #[inline]
            fn as_bytes(&self) -> &[u8] {
                &self.0
            }

            /// Construct this object from a byte slice
            fn from_bytes(bytes: &[u8]) -> Self {
                let mut array: [u8; $size] = unsafe { mem::uninitialized() };
                array.copy_from_slice(bytes);
                $type(array)
            }
        }

        impl PartialEq for $type {
            /// By no means constant time comparison
            fn eq(&self, other: &Self) -> bool {
                self.0
                    .iter()
                    .zip(other.0.iter())
                    .try_for_each(|(a, b)| if a == b { Ok(()) } else { Err(()) })
                    .is_ok()
            }
        }
    };
}


simple_struct!(PublicKey, ffi::PQCLEAN_FRODOKEM976SHAKE_CLEAN_CRYPTO_PUBLICKEYBYTES);
simple_struct!(SecretKey, ffi::PQCLEAN_FRODOKEM976SHAKE_CLEAN_CRYPTO_SECRETKEYBYTES);
simple_struct!(
    Ciphertext,
    ffi::PQCLEAN_FRODOKEM976SHAKE_CLEAN_CRYPTO_CIPHERTEXTBYTES
);
simple_struct!(SharedSecret, ffi::PQCLEAN_FRODOKEM976SHAKE_CLEAN_CRYPTO_BYTES);

/// Get the number of bytes for a public key
pub const fn public_key_bytes() -> usize {
    ffi::PQCLEAN_FRODOKEM976SHAKE_CLEAN_CRYPTO_PUBLICKEYBYTES
}

/// Get the number of bytes for a secret key
pub const fn secret_key_bytes() -> usize {
    ffi::PQCLEAN_FRODOKEM976SHAKE_CLEAN_CRYPTO_SECRETKEYBYTES
}

/// Get the number of bytes for the encapsulated ciphertext
pub const fn ciphertext_bytes() -> usize {
    ffi::PQCLEAN_FRODOKEM976SHAKE_CLEAN_CRYPTO_CIPHERTEXTBYTES
}

/// Get the number of bytes for the shared secret
pub const fn shared_secret_bytes() -> usize {
    ffi::PQCLEAN_FRODOKEM976SHAKE_CLEAN_CRYPTO_BYTES
}

/// Generate a frodokem976shake keypair
pub fn keypair() -> (PublicKey, SecretKey) {
    let mut pk = PublicKey::new();
    let mut sk = SecretKey::new();
    assert_eq!(
        unsafe {
            ffi::PQCLEAN_FRODOKEM976SHAKE_CLEAN_crypto_kem_keypair(pk.0.as_mut_ptr(), sk.0.as_mut_ptr())
        },
        0
    );
    (pk, sk)
}

/// Encapsulate to a frodokem976shake public key
pub fn encapsulate(pk: PublicKey) -> (SharedSecret, Ciphertext) {
    let mut ss = SharedSecret::new();
    let mut ct = Ciphertext::new();

    assert_eq!(
        unsafe {
            ffi::PQCLEAN_FRODOKEM976SHAKE_CLEAN_crypto_kem_enc(
                ct.0.as_mut_ptr(),
                ss.0.as_mut_ptr(),
                pk.0.as_ptr(),
            )
        },
        0,
    );

    (ss, ct)
}

/// Decapsulate the received frodokem976shake ciphertext
pub fn decapsulate(ct: Ciphertext, sk: SecretKey) -> SharedSecret {
    let mut ss = SharedSecret::new();
    assert_eq!(
        unsafe {
            ffi::PQCLEAN_FRODOKEM976SHAKE_CLEAN_crypto_kem_dec(
                ss.0.as_mut_ptr(),
                ct.0.as_ptr(),
                sk.0.as_ptr(),
            )
        },
        0
    );
    ss
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_kem() {
        let (pk, sk) = keypair();
        let (ss1, ct) = encapsulate(pk);
        let ss2 = decapsulate(ct, sk);
        assert!(ss1.0 == ss2.0, "Difference in shared secrets!");
    }
}