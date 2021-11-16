extern crate halo2_franchise;

mod utils;

use halo2_franchise::{franchise::FranchiseCircuit, utils::generate_test_data};
use halo2_franchise::{
    halo2::pasta::EqAffine,
    halo2::plonk::*,
    halo2::poly::commitment::Params,
    halo2::transcript::{Blake2bRead, Blake2bWrite, Challenge255},
};
use wasm_bindgen::prelude::*;
use instant::Instant;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn greet() {
    alert("start");
    let params: Params<EqAffine> = Params::new(10);
    let empty_circuit = FranchiseCircuit::<21>::default();

    // Initialize the proving key
    let vk = keygen_vk(&params, &empty_circuit).expect("keygen_vk should not fail");
    let pk = keygen_pk(&params, vk, &empty_circuit).expect("keygen_pk should not fail");

    let (circuit, public) = generate_test_data::<21>();
    
    alert("proving");
    let start = Instant::now();
    let mut transcript = Blake2bWrite::<_, _, Challenge255<_>>::init(vec![]);
    create_proof(
        &params,
        &pk,
        &[circuit.clone()],
        &[&[&public]],
        &mut transcript,
    )
    .expect("proof generation should not fail");
    let proof = transcript.finalize();
    let elapsed = start.elapsed();
    alert(&format!("end {}ms",elapsed.as_millis()));

    let msm = params.empty_msm();
    let mut transcript = Blake2bRead::<_, _, Challenge255<_>>::init(&proof[..]);
    let guard = verify_proof(&params, pk.get_vk(), msm, &[&[&public]], &mut transcript).unwrap();
    let msm = guard.clone().use_challenges();
    assert!(msm.eval());
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}
