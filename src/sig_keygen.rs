use crate::helpers::rand_scalar;
use crate::polynomial::{Params, Polynomial};
use crate::sig_setup::SigParams;
use curve25519_dalek::{ristretto::CompressedRistretto, ristretto::RistrettoPoint, scalar::Scalar};

/// User's long-term secret material (do NOT derive `Debug` to avoid leaking secrets).
#[derive(Clone)]
pub struct UserSecret {
    pub x: Scalar,
    pub w: Scalar,
    pub u: Scalar,
    pub poly: Polynomial,
    /// The IPA params this polynomial was bound to (g0 == signature g).
    pub pp: Params,
}

/// User's long-term public material.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserPublic {
    /// pk = g^x * h^w * v^u
    pub pk: RistrettoPoint,
    pub cf: RistrettoPoint,
}

impl UserPublic {
    #[inline]
    pub fn pk_compressed(&self) -> CompressedRistretto {
        self.pk.compress()
    }
    #[inline]
    pub fn cf_compressed(&self) -> CompressedRistretto {
        self.cf.compress()
    }
}

/// KeyGen(pp): sample x,w,u and a random polynomial; compute pk = g^x h^w v^u.
/// - Uses `SigParams` so we share the same `g` with IPA (`pp.g0 == g`).
pub fn keygen(sp: &SigParams) -> (UserSecret, UserPublic) {
    let x = rand_scalar();
    let w = rand_scalar();
    let u = rand_scalar();

    let pk = sp.g * x + sp.h * w + sp.v * u;
    let poly = Polynomial::random(sp.d, &sp.pp);

    let public = UserPublic {
        pk,
        cf: poly.cf,
    };

    let secret = UserSecret {
        x,
        w,
        u,
        poly,
        pp: sp.pp.clone(),
    };

    (secret, public)
}
