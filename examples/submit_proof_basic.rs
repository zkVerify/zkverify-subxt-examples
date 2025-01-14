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

use subxt::{OnlineClient, SubstrateConfig};
use subxt_examples::{
    utils::groth16::*, zk_verify::settlement_groth16_pallet::calls::types::submit_proof::VkOrHash,
};
use subxt_signer::{bip39::Mnemonic, sr25519::Keypair};

const TEST_PROOF: &'static str = r#"{
  "a": "0c0f61e9a7f4ee6fa1a123d1d9dde329684c6690e339e11cf653f84941cd77c98c93efe202e8652652a7d4c9ac7e496c135a8579b1d56d842b691fc49245ffe5e0c685ed0fa78e6c097b910a6cbc7907baee3d51f0a205f86786246be24cb03f",
  "b": "0d218bd732caa7465450ed9252b80b6700b75b5e159a0bed27853bac2e546bd25f7d8625fd0881f5ddb6abde5ea7759e1634139a2380194f99330476baf41c3fa83e1f7218be2d113ae5fb15f3a8ebf69456c596432a8b7a3338aeb9109beb3101b68b64f72838f020bd0394accbc558b4d9c0ce82055653a7ea5488bc49144aeb86a75b962309215320a22b64f97555132adbc0ab25656f6a79de016a99c6da97d711532ff13bc5bc85861f7965cd26460edf4ab91708bc4634d346a33e6dd1",
  "c": "0268e0eee7ed517f5ff15a790e5ebad73edf31e36e5cd67ec68907c7cce19e714759ab71b616892015513465498166621524f7371e085dac8a78644fd0c890c0de9b8f48a2335a65b990d1a09cf393e1188fbd1d12fc0d4927aea704761655fe"
}"#;

const TEST_VK: &'static str = r#"{
	"curve": "Bls12_381",
	"alphaG1": "065f99c22a0c380e56d5fbf60c62fc5fc0c2b30a1922f14674841d56432cb4224f51ccabb211dc9161feaf167284d3350cb720378f166a7665d96c8f7c38f5f2d8327c2eca3ed28460a8ee2149dd90fb09ca813d6944bac577165857696e0d88",
	"betaG2": "0752636ff1c60c75f1bd86b8f5dc98078cb35476a21235d49818daf3166bec4039639f709e2aeb5c83b9a84f4912701400102edf84d3c3b9e6865e1bf39161b5f27bb5ee903995bb867f7c1cb25253254f76f564ab25f9086622605fcb57239000336c04e2f8d263b12c867c673e157867f875cc6045141ad26288ed3a959e63aea44c8a147a2444f3f0ecf7e8714d6601abe09165332012f9af7241999fb1464dbff2fd3bf77af20970966b1ee2966a9519ccb9f756918b7ff67a539b37c224",
	"gammaG2": "13d36d703e1edbe3a9fdec7e339d53f87ebe333c8b63eae0fb02dd39375f08bf8b9aa8348795b18bb41a45d81c4e55d71488dfa6e4e981b2f40236aeb32b58b4bf6a93d1e9b5b84192aac5a6f6b58c2f2768229b3d44fae5b6f7404388bf0664152cf77f1dd9abc072d7609197b793b7c0ef992caa5820422c7ac4a6434b50d64b8557305b7f7a26ade280c25b1a5eb80697b973c3ec77e21d5624613ff8bc0a72f82c8fdbb81019ad81b3a9331b8be369951ad1f9c90ee274a15c84573c353c",
	"deltaG2": "189018fef55fde63f7ee79f1568ccf019973ff0cc7cc01c53d787c1c5377e6514ea9a6077e85be032f310d652dc3c40d108cb16c1dfc0956dd97ad82154aac02a2b4287038f1da203828443a79ab163a63d07fadc7598433e1ea3a6e2a059138199fc1f38f8d83f24ee55b973800f8c0b0384615cada3d631525ccc962c0648ce1f9080dbf81f3b6fbbe296ea2e2dadb159aef8f6cd9d75a862a555d23ea600aca98e1e9aad008c04cb37a48fb1a5d488c44c38ca11b2df47c97ed8f5a6bec0e",
	"gammaAbcG1": [
	  "08f21e24cc90ac3af0aad9bdbeea81a648f0c853308b1d176d3cc32d8707ab47ef84891fbd4f7fa1ab0b39ea171526a1051946e2fa796293713f338cc3d0a032db7072ce2d38e6acd53f671cf682cd0c2054e51bd9d76badb0c0d30e234c260a",
	  "121eb8512afd7f3dfbf54d845c8df4c67c1156021c7739a26d4ede97f2a2559636c27453cfee1b68ba225ea3d5452647057517ee207660eb7f5e20663957118678cac37ac518a05afe8ff16a4be7031b95c2ad82117e6418564bbf14d3bf99b3",
	  "0931bb224ccea45ac4c2287afcc82b5dfbe8727924bafa7eddbdfbff4a20c957fca2830b2042102235f4f9faecc407590443c37194dc4bb1289007f0ecfc718d2206ae60cc2dd386d84115a431e1fd5f764330369eaf47ac77e9911eb7c1078d"
	]
  }"#;

const TEST_PUBS: &'static str = r#"[
  "0a00000000000000000000000000000000000000000000000000000000000000",
  "0100000000000000000000000000000000000000000000000000000000000000"
]"#;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get access to a zkVerify client. Here we are using our RPC nodes.
    let api = OnlineClient::<SubstrateConfig>::from_url("wss://testnet-rpc.zkverify.io").await?;

    // Instantiate a keypair corresponding to an account with some funds
    let phrase = "bottom drive obey lake curtain smoke basket hold race lonely fit walk";
    let mnemonic = Mnemonic::parse(phrase).unwrap();
    let signer = Keypair::from_phrase(&mnemonic, None).unwrap();

    // // Optional: register a vk
    // let vk = parse_vk(&TEST_VK);
    // let register_groth16_vk_tx = subxt_examples::zk_verify::tx()
    //     .settlement_groth16_pallet()
    //     .register_vk(vk.clone());

    // let vk_hash = api
    //     .tx()
    //     .sign_and_submit_then_watch_default(&register_groth16_vk_tx, &signer)
    //     .await?
    //     .wait_for_finalized_success()
    //     .await?
    //     .find_first::<crate::zk_verify::settlement_groth16_pallet::events::VkRegistered>()?
    //     .unwrap()
    //     .hash;

    // Prepare a Groth16 submit proof call
    let vk = parse_vk(&TEST_VK);
    let proof = parse_proof(&TEST_PROOF, vk.curve.clone());
    let pubs = parse_pubs(&TEST_PUBS);
    let submit_groth16_proof_tx = subxt_examples::zk_verify::tx()
        .settlement_groth16_pallet()
        .submit_proof(
            VkOrHash::Vk(Box::new(vk)),
            // VkOrHash::Hash(vk_hash), If you preregistered the vk
            proof,
            pubs,
            None,
        );

    // Submit Groth16 proof. Wait for transaction to be included in a block and finalized
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&submit_groth16_proof_tx, &signer)
        .await?
        .wait_for_finalized_success()
        .await?;

    // Do something with the events associated to the transaction
    events.iter().for_each(|evt| println!("{:?}", evt.unwrap()));

    // If you need more fine grained control over the tx status
    // // Submit a Groth16 proof
    // let mut submit_status = api
    //     .tx()
    //     .sign_and_submit_then_watch_default(call, &signer)
    //     .await?;

    // // Query tx status
    // while let Some(status) = submit_status.next().await {
    //     match status? {
    //         subxt::tx::TxStatus::Validated => todo!(),
    //         subxt::tx::TxStatus::Broadcasted { num_peers } => todo!(),
    //         subxt::tx::TxStatus::NoLongerInBestBlock => todo!(),
    //         subxt::tx::TxStatus::InBestBlock(tx_in_block) => todo!(),
    //         subxt::tx::TxStatus::InFinalizedBlock(tx_in_block) => todo!(),
    //         subxt::tx::TxStatus::Error { message } => println!("{:?}", message),
    //         subxt::tx::TxStatus::Invalid { message } => println!("{:?}", message),
    //         subxt::tx::TxStatus::Dropped { message } => println!("{:?}", message),
    //     }
    // }

    Ok(())
}
