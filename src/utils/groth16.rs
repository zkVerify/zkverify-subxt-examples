// Copyright 2024, Horizen Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::zk_verify::runtime_types::hp_groth16::data_structures::{Proof, Scalar, G1, G2};
use crate::zk_verify::runtime_types::pallet_groth16_verifier::groth16::{
    Curve, ProofWithCurve, VerificationKeyWithCurve,
};
use crate::zk_verify::settlement_groth16_pallet::calls::types::submit_proof::Pubs;
use serde_json::{de::from_str, Value};

pub fn parse_g1(g1: &Value) -> G1 {
    G1(hex::decode(g1.as_str().unwrap()).unwrap())
}

pub fn parse_g2(g2: &Value) -> G2 {
    G2(hex::decode(g2.as_str().unwrap()).unwrap())
}

pub fn parse_proof(proof: &str, curve: Curve) -> ProofWithCurve {
    let raw_proof: Value = from_str(proof).unwrap();
    let proof = Proof {
        a: parse_g1(&raw_proof["a"]),
        b: parse_g2(&raw_proof["b"]),
        c: parse_g1(&raw_proof["c"]),
    };
    ProofWithCurve { curve, proof }
}

pub fn parse_vk(vk: &str) -> VerificationKeyWithCurve {
    let raw_vk: Value = from_str(vk).unwrap();
    let raw_curve = raw_vk["curve"].as_str().unwrap();
    let curve = {
        if raw_curve == "Bls12_381" {
            Curve::Bls12_381
        } else if raw_curve == "bn128" {
            Curve::Bn254
        } else {
            unreachable!()
        }
    };
    VerificationKeyWithCurve {
        curve,
        alpha_g1: parse_g1(&raw_vk["alphaG1"]),
        beta_g2: parse_g2(&raw_vk["betaG2"]),
        gamma_g2: parse_g2(&raw_vk["gammaG2"]),
        delta_g2: parse_g2(&raw_vk["deltaG2"]),
        gamma_abc_g1: raw_vk["gammaAbcG1"]
            .as_array()
            .unwrap()
            .iter()
            .map(|g1| parse_g1(g1))
            .collect(),
    }
}

pub fn parse_scalar(scalar: &Value) -> Scalar {
    Scalar(hex::decode(scalar.as_str().unwrap()).unwrap())
}

pub fn parse_pubs(pubs: &str) -> Pubs {
    let pubs: Value = from_str(pubs).unwrap();
    pubs.as_array()
        .unwrap()
        .iter()
        .map(|scalar| parse_scalar(scalar))
        .collect()
}
