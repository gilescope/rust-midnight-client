#![allow(missing_docs)]
use subxt::{OnlineClient, SubstrateConfig};
use subxt::blocks::BlockRef;
use subxt::config::substrate::H256;
use mn_zswap::PolyCommit;
use midnight_base_crypto::proofs::Proof;
use mn_ledger::structure::{CallOrDeploy, Transaction as Tx};
use subxt::backend::legacy::LegacyRpcMethods;
use subxt::backend::rpc::RpcClient;
use midnight_serialize::NetworkId;
use midnight_serialize::{Deserializable, Serializable, Version, Versioned};

// use subxt_signer::sr25519::dev;
// Generate an interface that we can use from the node's metadata.
#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod midnight {}

type InnerTx = Tx<Proof<PolyCommit>>;

#[derive(Clone, Debug, PartialEq, Eq, Serializable)]
pub(crate) struct Transaction(InnerTx);

impl Versioned for Transaction {
    const VERSION: Option<Version> = None;
}

impl Deserializable for Transaction {
    fn versioned_deserialize<R: std::io::Read>(
        reader: &mut R,
        _version: Option<&Version>,
    ) -> Result<Self, std::io::Error> {
        Ok(Transaction(<InnerTx as Deserializable>::deserialize(reader)?))
    }
}

pub async fn get_network_id(rpc_url: &str) -> Result<NetworkId, subxt::Error> {
    let api = OnlineClient::<SubstrateConfig>::from_url(rpc_url).await?;
    let storage_query = midnight::storage().midnight().network_id();
    let network_id_vec = api.storage().at_latest().await?.fetch(&storage_query).await?;

    // TODO: Update this when we launch testnet/mainnet
    let network_id = if let Some(val) = network_id_vec {
        if val.0.iter().all(|x| *x == 0) {
            NetworkId::Undeployed
        } else {
            NetworkId::DevNet
        }
    } else {
        NetworkId::Undeployed
    };

    Ok(network_id)
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::new()
        .init();

    // let rpc_client = RpcClient::from_url("ws://127.0.0.1:9944").await?;
    // // Use this to construct our RPC methods:
    // let rpc = LegacyRpcMethods::<SubstrateConfig>::new(rpc_client.clone());
    // let hash = rpc.chain_get_block_hash(Some(1000.into())).await.unwrap().unwrap();

    // find_transaction(HALO2).await;

    let HALO2_GEN = "75e304546116ee6275a629f6b3bd07f027aaaf67fe140d2e2b06f8e378458d0f";
    let DEVNET_TX= "2b3c168822dca4ac5f5a44a9b0a370c0e5ecae0b19edad53e1c837f02d686497";
    let v: Vec<u8> = hex::decode(HALO2_GEN).unwrap();
    let v: [u8; 32] = v.try_into().unwrap();
    let midnight_block_with_tx: H256 = v.into();

    let api = OnlineClient::<SubstrateConfig>::from_url(HALO2).await?;

    let events = api.events().at(midnight_block_with_tx).await?;
    for event in events.iter() {
        let event = event?;
        //duck typed events:
        let pallet = event.pallet_name();
        let variant = event.variant_name();
        let field_values = event.field_values()?;

        println!("{pallet}::{variant}: {field_values}");

        //Strongly typed:
        if let Ok(ev) = event.as_root_event::<midnight::Event>() {
            println!("{ev:?}");
        } else {
            println!("<Cannot decode event>");
        }
    }

    let block = api.blocks().at(BlockRef::from(midnight_block_with_tx)).await?;

    let hash = block.hash();
    println!("{}", hash);

    for raw_ex in block.extrinsics().await.unwrap().iter() {
        let ex = raw_ex.unwrap();
        let pallet_name = ex.pallet_name().unwrap();
        if pallet_name == "Midnight" {
            let midnight_tx = ex.as_extrinsic::<midnight::midnight::calls::types::SendMnTransaction>().unwrap().unwrap();

            type InnerTx = Tx<Proof<PolyCommit>>;

            midnight_serialize::NETWORK_ID.replace(NetworkId::DevNet);
            let unhexed=  hex::decode( midnight_tx.midnight_tx.as_slice()).unwrap();
            let mid_tx: Result<Transaction, std::io::Error> = midnight_serialize::deserialize(&mut unhexed.as_slice());
            println!("{:#?}", mid_tx);


        } else {
            println!("skipping {pallet_name}");
        }
    }

    Ok(())
}

const DEVNET: &str = "wss://rpc.devnet.midnight.network";
const HALO2: &str = "wss://rpc.halo2-qa.dev.midnight.network";

pub async fn find_transaction(url: &str) -> Result<(), Box<dyn std::error::Error>> {

    let api = OnlineClient::<SubstrateConfig>::from_url(url).await?;
    let h = "7b268b3e09dd8660da784e3da53aa2302d5348dcf732868cc8904dcb56ffe3c0";
    let h = "6f0c7c148da9cc6c919e73f372476fce4a3defcded573f60d29ec4cdcf25dad8";//10_000
    let v: Vec<u8> = hex::decode(h).unwrap();
    let v: [u8; 32] = v.try_into().unwrap();
    let hash: H256 = v.into();

    let block = api.blocks().at(hash).await?;

    let hash = block.hash();
    let mut tx_count = block.extrinsics().await?.len();
    let mut parent = block.header().parent_hash;
    println!("hash = {:?} number: {}, tx: {tx_count}", hash,block.number() );

    while tx_count <= 1 {
        let parent_block = api.blocks().at(BlockRef::from(parent)).await?;
        let txs = parent_block.extrinsics().await?;
        let mut boring = 0;
        for tx in txs.iter() {
            let ex = tx.unwrap();
            let name = ex.pallet_name().unwrap();
            println!("{name}");
            if name == "SessionCommitteeManagement" {
                boring += 1;
            }
            if name == "Sudo" {
                boring += 1;
            }
        }
        tx_count = txs.len() - boring;

        println!("parent hash = {:?} number: {}, tx:{tx_count}", parent_block.hash(), parent_block.number());

        parent = parent_block.header().parent_hash;
    }
    Ok(())
}