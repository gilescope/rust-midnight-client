//Generated with cargo expand
#[allow(dead_code, unused_imports, non_camel_case_types)]
#[allow(clippy::all)]
#[allow(rustdoc::broken_intra_doc_links)]
pub mod api {
    #[allow(unused_imports)]
    mod root_mod {
        pub use super::*;
    }
    pub static PALLETS: [&str; 10usize] = [
        "System",
        "Timestamp",
        "Aura",
        "Grandpa",
        "Sidechain",
        "Midnight",
        "Balances",
        "Sudo",
        "SessionCommitteeManagement",
        "Session",
    ];
    pub static RUNTIME_APIS: [&str; 15usize] = [
        "GenesisBuilder",
        "Core",
        "MidnightRuntimeApi",
        "Metadata",
        "BlockBuilder",
        "TaggedTransactionQueue",
        "OffchainWorkerApi",
        "AuraApi",
        "SessionKeys",
        "GrandpaApi",
        "AccountNonceApi",
        "GetSidechainParams",
        "GetSidechainStatus",
        "SlotApi",
        "SessionValidatorManagementApi",
    ];
    #[doc = r" The error type returned when there is a runtime issue."]
    pub type DispatchError = runtime_types::sp_runtime::DispatchError;
    #[doc = r" The outer event enum."]
    pub type Event = runtime_types::midnight_node_runtime::RuntimeEvent;
    #[doc = r" The outer extrinsic enum."]
    pub type Call = runtime_types::midnight_node_runtime::RuntimeCall;
    #[doc = r" The outer error enum representing the DispatchError's Module variant."]
    pub type Error = runtime_types::midnight_node_runtime::RuntimeError;
    pub fn constants() -> ConstantsApi {
        ConstantsApi
    }
    pub fn storage() -> StorageApi {
        StorageApi
    }
    pub fn tx() -> TransactionApi {
        TransactionApi
    }
    pub fn apis() -> runtime_apis::RuntimeApi {
        runtime_apis::RuntimeApi
    }
    pub mod runtime_apis {
        use super::root_mod;
        use super::runtime_types;
        use subxt::ext::subxt_core::ext::codec::Encode;
        pub struct RuntimeApi;
        impl RuntimeApi {
            pub fn genesis_builder(&self) -> genesis_builder::GenesisBuilder {
                genesis_builder::GenesisBuilder
            }
            pub fn core(&self) -> core::Core {
                core::Core
            }
            pub fn midnight_runtime_api(&self) -> midnight_runtime_api::MidnightRuntimeApi {
                midnight_runtime_api::MidnightRuntimeApi
            }
            pub fn metadata(&self) -> metadata::Metadata {
                metadata::Metadata
            }
            pub fn block_builder(&self) -> block_builder::BlockBuilder {
                block_builder::BlockBuilder
            }
            pub fn tagged_transaction_queue(
                &self,
            ) -> tagged_transaction_queue::TaggedTransactionQueue {
                tagged_transaction_queue::TaggedTransactionQueue
            }
            pub fn offchain_worker_api(&self) -> offchain_worker_api::OffchainWorkerApi {
                offchain_worker_api::OffchainWorkerApi
            }
            pub fn aura_api(&self) -> aura_api::AuraApi {
                aura_api::AuraApi
            }
            pub fn session_keys(&self) -> session_keys::SessionKeys {
                session_keys::SessionKeys
            }
            pub fn grandpa_api(&self) -> grandpa_api::GrandpaApi {
                grandpa_api::GrandpaApi
            }
            pub fn account_nonce_api(&self) -> account_nonce_api::AccountNonceApi {
                account_nonce_api::AccountNonceApi
            }
            pub fn get_sidechain_params(&self) -> get_sidechain_params::GetSidechainParams {
                get_sidechain_params::GetSidechainParams
            }
            pub fn get_sidechain_status(&self) -> get_sidechain_status::GetSidechainStatus {
                get_sidechain_status::GetSidechainStatus
            }
            pub fn slot_api(&self) -> slot_api::SlotApi {
                slot_api::SlotApi
            }
            pub fn session_validator_management_api(
                &self,
            ) -> session_validator_management_api::SessionValidatorManagementApi {
                session_validator_management_api::SessionValidatorManagementApi
            }
        }
        pub mod genesis_builder {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " API to interact with RuntimeGenesisConfig for the runtime"]
            pub struct GenesisBuilder;
            impl GenesisBuilder {
                #[doc = " Creates the default `RuntimeGenesisConfig` and returns it as a JSON blob."]
                #[doc = ""]
                #[doc = " This function instantiates the default `RuntimeGenesisConfig` struct for the runtime and serializes it into a JSON"]
                #[doc = " blob. It returns a `Vec<u8>` containing the JSON representation of the default `RuntimeGenesisConfig`."]
                pub fn create_default_config(
                    &self,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::CreateDefaultConfig,
                    types::create_default_config::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "GenesisBuilder",
                        "create_default_config",
                        types::CreateDefaultConfig {},
                        [
                            238u8, 5u8, 139u8, 81u8, 184u8, 155u8, 221u8, 118u8, 190u8, 76u8,
                            229u8, 67u8, 132u8, 89u8, 83u8, 80u8, 56u8, 171u8, 169u8, 64u8, 123u8,
                            20u8, 129u8, 159u8, 28u8, 135u8, 84u8, 52u8, 192u8, 98u8, 104u8, 214u8,
                        ],
                    )
                }
                #[doc = " Build `RuntimeGenesisConfig` from a JSON blob not using any defaults and store it in the storage."]
                #[doc = ""]
                #[doc = " This function deserializes the full `RuntimeGenesisConfig` from the given JSON blob and puts it into the storage."]
                #[doc = " If the provided JSON blob is incorrect or incomplete or the deserialization fails, an error is returned."]
                #[doc = " It is recommended to log any errors encountered during the process."]
                #[doc = ""]
                #[doc = " Please note that provided json blob must contain all `RuntimeGenesisConfig` fields, no defaults will be used."]
                pub fn build_config(
                    &self,
                    json: types::build_config::Json,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::BuildConfig,
                    types::build_config::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "GenesisBuilder",
                        "build_config",
                        types::BuildConfig { json },
                        [
                            6u8, 98u8, 68u8, 125u8, 157u8, 26u8, 107u8, 86u8, 213u8, 227u8, 26u8,
                            229u8, 122u8, 161u8, 229u8, 114u8, 123u8, 192u8, 66u8, 231u8, 148u8,
                            175u8, 5u8, 185u8, 248u8, 88u8, 40u8, 122u8, 230u8, 209u8, 170u8,
                            254u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod create_default_config {
                    use super::runtime_types;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output =
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct CreateDefaultConfig {}
                pub mod build_config {
                    use super::runtime_types;
                    pub type Json =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ::core::result::Result<
                            (),
                            ::subxt::ext::subxt_core::alloc::string::String,
                        >;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct BuildConfig {
                    pub json: build_config::Json,
                }
            }
        }
        pub mod core {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " The `Core` runtime api that every Substrate runtime needs to implement."]
            pub struct Core;
            impl Core {
                #[doc = " Returns the version of the runtime."]
                pub fn version(
                    &self,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::Version,
                    types::version::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "Core",
                        "version",
                        types::Version {},
                        [
                            76u8, 202u8, 17u8, 117u8, 189u8, 237u8, 239u8, 237u8, 151u8, 17u8,
                            125u8, 159u8, 218u8, 92u8, 57u8, 238u8, 64u8, 147u8, 40u8, 72u8, 157u8,
                            116u8, 37u8, 195u8, 156u8, 27u8, 123u8, 173u8, 178u8, 102u8, 136u8,
                            6u8,
                        ],
                    )
                }
                #[doc = " Execute the given block."]
                pub fn execute_block(
                    &self,
                    block: types::execute_block::Block,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::ExecuteBlock,
                    types::execute_block::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "Core",
                        "execute_block",
                        types::ExecuteBlock { block },
                        [
                            133u8, 135u8, 228u8, 65u8, 106u8, 27u8, 85u8, 158u8, 112u8, 254u8,
                            93u8, 26u8, 102u8, 201u8, 118u8, 216u8, 249u8, 247u8, 91u8, 74u8, 56u8,
                            208u8, 231u8, 115u8, 131u8, 29u8, 209u8, 6u8, 65u8, 57u8, 214u8, 125u8,
                        ],
                    )
                }
                #[doc = " Initialize a block with the given header and return the runtime executive mode."]
                pub fn initialize_block(
                    &self,
                    header: types::initialize_block::Header,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::InitializeBlock,
                    types::initialize_block::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "Core",
                        "initialize_block",
                        types::InitializeBlock { header },
                        [
                            132u8, 169u8, 113u8, 112u8, 80u8, 139u8, 113u8, 35u8, 41u8, 81u8, 36u8,
                            35u8, 37u8, 202u8, 29u8, 207u8, 205u8, 229u8, 145u8, 7u8, 133u8, 94u8,
                            25u8, 108u8, 233u8, 86u8, 234u8, 29u8, 236u8, 57u8, 56u8, 186u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod version {
                    use super::runtime_types;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = runtime_types::sp_version::RuntimeVersion;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Version {}
                pub mod execute_block {
                    use super::runtime_types;
                    pub type Block = runtime_types :: sp_runtime :: generic :: block :: Block < runtime_types :: sp_runtime :: generic :: header :: Header < :: core :: primitive :: u32 > , :: subxt :: ext :: subxt_core :: utils :: UncheckedExtrinsic < :: subxt :: ext :: subxt_core :: utils :: MultiAddress < :: subxt :: ext :: subxt_core :: utils :: AccountId32 , () > , runtime_types :: midnight_node_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight ,) > > ;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ();
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ExecuteBlock {
                    pub block: execute_block::Block,
                }
                pub mod initialize_block {
                    use super::runtime_types;
                    pub type Header =
                        runtime_types::sp_runtime::generic::header::Header<::core::primitive::u32>;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = runtime_types::sp_runtime::ExtrinsicInclusionMode;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct InitializeBlock {
                    pub header: initialize_block::Header,
                }
            }
        }
        pub mod midnight_runtime_api {
            use super::root_mod;
            use super::runtime_types;
            pub struct MidnightRuntimeApi;
            impl MidnightRuntimeApi {
                pub fn get_contract_state_json(
                    &self,
                    contract_addres: types::get_contract_state_json::ContractAddres,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::GetContractStateJson,
                    types::get_contract_state_json::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "MidnightRuntimeApi",
                        "get_contract_state_json",
                        types::GetContractStateJson { contract_addres },
                        [
                            141u8, 136u8, 182u8, 33u8, 42u8, 216u8, 77u8, 42u8, 86u8, 64u8, 252u8,
                            158u8, 40u8, 80u8, 218u8, 8u8, 130u8, 252u8, 249u8, 0u8, 28u8, 216u8,
                            238u8, 236u8, 17u8, 182u8, 231u8, 113u8, 137u8, 231u8, 52u8, 233u8,
                        ],
                    )
                }
                pub fn get_contract_state(
                    &self,
                    contract_addres: types::get_contract_state::ContractAddres,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::GetContractState,
                    types::get_contract_state::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "MidnightRuntimeApi",
                        "get_contract_state",
                        types::GetContractState { contract_addres },
                        [
                            189u8, 123u8, 31u8, 135u8, 63u8, 49u8, 79u8, 9u8, 3u8, 13u8, 146u8,
                            208u8, 82u8, 149u8, 16u8, 221u8, 209u8, 196u8, 169u8, 202u8, 255u8,
                            90u8, 172u8, 112u8, 208u8, 92u8, 196u8, 255u8, 74u8, 46u8, 0u8, 239u8,
                        ],
                    )
                }
                pub fn get_decoded_transactions(
                    &self,
                    extrinsics: types::get_decoded_transactions::Extrinsics,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::GetDecodedTransactions,
                    types::get_decoded_transactions::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "MidnightRuntimeApi",
                        "get_decoded_transactions",
                        types::GetDecodedTransactions { extrinsics },
                        [
                            12u8, 248u8, 167u8, 151u8, 211u8, 5u8, 215u8, 23u8, 215u8, 19u8, 245u8,
                            186u8, 162u8, 121u8, 57u8, 118u8, 223u8, 35u8, 229u8, 139u8, 109u8,
                            229u8, 2u8, 159u8, 163u8, 177u8, 121u8, 238u8, 85u8, 219u8, 231u8,
                            138u8,
                        ],
                    )
                }
                pub fn get_zswap_chain_state(
                    &self,
                    contract_addres: types::get_zswap_chain_state::ContractAddres,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::GetZswapChainState,
                    types::get_zswap_chain_state::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "MidnightRuntimeApi",
                        "get_zswap_chain_state",
                        types::GetZswapChainState { contract_addres },
                        [
                            182u8, 218u8, 194u8, 202u8, 142u8, 32u8, 92u8, 1u8, 201u8, 195u8,
                            236u8, 132u8, 113u8, 203u8, 74u8, 56u8, 115u8, 76u8, 171u8, 160u8,
                            103u8, 203u8, 167u8, 234u8, 55u8, 206u8, 202u8, 191u8, 199u8, 36u8,
                            249u8, 43u8,
                        ],
                    )
                }
                pub fn get_network_id(
                    &self,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::GetNetworkId,
                    types::get_network_id::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "MidnightRuntimeApi",
                        "get_network_id",
                        types::GetNetworkId {},
                        [
                            224u8, 25u8, 252u8, 217u8, 77u8, 248u8, 167u8, 240u8, 153u8, 37u8,
                            151u8, 198u8, 139u8, 246u8, 158u8, 80u8, 183u8, 198u8, 144u8, 165u8,
                            157u8, 163u8, 95u8, 215u8, 237u8, 195u8, 76u8, 34u8, 237u8, 159u8, 9u8,
                            187u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod get_contract_state_json {
                    use super::runtime_types;
                    pub type ContractAddres =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output =
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct GetContractStateJson {
                    pub contract_addres: get_contract_state_json::ContractAddres,
                }
                pub mod get_contract_state {
                    use super::runtime_types;
                    pub type ContractAddres =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output =
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct GetContractState {
                    pub contract_addres: get_contract_state::ContractAddres,
                }
                pub mod get_decoded_transactions {
                    use super::runtime_types;
                    pub type Extrinsics = :: subxt :: ext :: subxt_core :: alloc :: vec :: Vec < :: subxt :: ext :: subxt_core :: utils :: UncheckedExtrinsic < :: subxt :: ext :: subxt_core :: utils :: MultiAddress < :: subxt :: ext :: subxt_core :: utils :: AccountId32 , () > , runtime_types :: midnight_node_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight ,) > > ;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ::subxt::ext::subxt_core::alloc::vec::Vec<
                            runtime_types::midnight_node_runtime::TransactionType,
                        >;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct GetDecodedTransactions {
                    pub extrinsics: get_decoded_transactions::Extrinsics,
                }
                pub mod get_zswap_chain_state {
                    use super::runtime_types;
                    pub type ContractAddres =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output =
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct GetZswapChainState {
                    pub contract_addres: get_zswap_chain_state::ContractAddres,
                }
                pub mod get_network_id {
                    use super::runtime_types;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output =
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct GetNetworkId {}
            }
        }
        pub mod metadata {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " The `Metadata` api trait that returns metadata for the runtime."]
            pub struct Metadata;
            impl Metadata {
                #[doc = " Returns the metadata of a runtime."]
                pub fn metadata(
                    &self,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::Metadata,
                    types::metadata::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "Metadata",
                        "metadata",
                        types::Metadata {},
                        [
                            231u8, 24u8, 67u8, 152u8, 23u8, 26u8, 188u8, 82u8, 229u8, 6u8, 185u8,
                            27u8, 175u8, 68u8, 83u8, 122u8, 69u8, 89u8, 185u8, 74u8, 248u8, 87u8,
                            217u8, 124u8, 193u8, 252u8, 199u8, 186u8, 196u8, 179u8, 179u8, 96u8,
                        ],
                    )
                }
                #[doc = " Returns the metadata at a given version."]
                #[doc = ""]
                #[doc = " If the given `version` isn't supported, this will return `None`."]
                #[doc = " Use [`Self::metadata_versions`] to find out about supported metadata version of the runtime."]
                pub fn metadata_at_version(
                    &self,
                    version: types::metadata_at_version::Version,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::MetadataAtVersion,
                    types::metadata_at_version::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "Metadata",
                        "metadata_at_version",
                        types::MetadataAtVersion { version },
                        [
                            131u8, 53u8, 212u8, 234u8, 16u8, 25u8, 120u8, 252u8, 153u8, 153u8,
                            216u8, 28u8, 54u8, 113u8, 52u8, 236u8, 146u8, 68u8, 142u8, 8u8, 10u8,
                            169u8, 131u8, 142u8, 204u8, 38u8, 48u8, 108u8, 134u8, 86u8, 226u8,
                            61u8,
                        ],
                    )
                }
                #[doc = " Returns the supported metadata versions."]
                #[doc = ""]
                #[doc = " This can be used to call `metadata_at_version`."]
                pub fn metadata_versions(
                    &self,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::MetadataVersions,
                    types::metadata_versions::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "Metadata",
                        "metadata_versions",
                        types::MetadataVersions {},
                        [
                            23u8, 144u8, 137u8, 91u8, 188u8, 39u8, 231u8, 208u8, 252u8, 218u8,
                            224u8, 176u8, 77u8, 32u8, 130u8, 212u8, 223u8, 76u8, 100u8, 190u8,
                            82u8, 94u8, 190u8, 8u8, 82u8, 244u8, 225u8, 179u8, 85u8, 176u8, 56u8,
                            16u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod metadata {
                    use super::runtime_types;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = runtime_types::sp_core::OpaqueMetadata;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Metadata {}
                pub mod metadata_at_version {
                    use super::runtime_types;
                    pub type Version = ::core::primitive::u32;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output =
                            ::core::option::Option<runtime_types::sp_core::OpaqueMetadata>;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct MetadataAtVersion {
                    pub version: metadata_at_version::Version,
                }
                pub mod metadata_versions {
                    use super::runtime_types;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output =
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u32>;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct MetadataVersions {}
            }
        }
        pub mod block_builder {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " The `BlockBuilder` api trait that provides the required functionality for building a block."]
            pub struct BlockBuilder;
            impl BlockBuilder {
                #[doc = " Apply the given extrinsic."]
                #[doc = ""]
                #[doc = " Returns an inclusion outcome which specifies if this extrinsic is included in"]
                #[doc = " this block or not."]
                pub fn apply_extrinsic(
                    &self,
                    extrinsic: types::apply_extrinsic::Extrinsic,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::ApplyExtrinsic,
                    types::apply_extrinsic::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "BlockBuilder",
                        "apply_extrinsic",
                        types::ApplyExtrinsic { extrinsic },
                        [
                            72u8, 54u8, 139u8, 3u8, 118u8, 136u8, 65u8, 47u8, 6u8, 105u8, 125u8,
                            223u8, 160u8, 29u8, 103u8, 74u8, 79u8, 149u8, 48u8, 90u8, 237u8, 2u8,
                            97u8, 201u8, 123u8, 34u8, 167u8, 37u8, 187u8, 35u8, 176u8, 97u8,
                        ],
                    )
                }
                #[doc = " Finish the current block."]
                pub fn finalize_block(
                    &self,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::FinalizeBlock,
                    types::finalize_block::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "BlockBuilder",
                        "finalize_block",
                        types::FinalizeBlock {},
                        [
                            244u8, 207u8, 24u8, 33u8, 13u8, 69u8, 9u8, 249u8, 145u8, 143u8, 122u8,
                            96u8, 197u8, 55u8, 64u8, 111u8, 238u8, 224u8, 34u8, 201u8, 27u8, 146u8,
                            232u8, 99u8, 191u8, 30u8, 114u8, 16u8, 32u8, 220u8, 58u8, 62u8,
                        ],
                    )
                }
                #[doc = " Generate inherent extrinsics. The inherent data will vary from chain to chain."]
                pub fn inherent_extrinsics(
                    &self,
                    inherent: types::inherent_extrinsics::Inherent,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::InherentExtrinsics,
                    types::inherent_extrinsics::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "BlockBuilder",
                        "inherent_extrinsics",
                        types::InherentExtrinsics { inherent },
                        [
                            254u8, 110u8, 245u8, 201u8, 250u8, 192u8, 27u8, 228u8, 151u8, 213u8,
                            166u8, 89u8, 94u8, 81u8, 189u8, 234u8, 64u8, 18u8, 245u8, 80u8, 29u8,
                            18u8, 140u8, 129u8, 113u8, 236u8, 135u8, 55u8, 79u8, 159u8, 175u8,
                            183u8,
                        ],
                    )
                }
                #[doc = " Check that the inherents are valid. The inherent data will vary from chain to chain."]
                pub fn check_inherents(
                    &self,
                    block: types::check_inherents::Block,
                    data: types::check_inherents::Data,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::CheckInherents,
                    types::check_inherents::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "BlockBuilder",
                        "check_inherents",
                        types::CheckInherents { block, data },
                        [
                            153u8, 134u8, 1u8, 215u8, 139u8, 11u8, 53u8, 51u8, 210u8, 175u8, 197u8,
                            28u8, 38u8, 209u8, 175u8, 247u8, 142u8, 157u8, 50u8, 151u8, 164u8,
                            191u8, 181u8, 118u8, 80u8, 97u8, 160u8, 248u8, 110u8, 217u8, 181u8,
                            234u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod apply_extrinsic {
                    use super::runtime_types;
                    pub type Extrinsic = :: subxt :: ext :: subxt_core :: utils :: UncheckedExtrinsic < :: subxt :: ext :: subxt_core :: utils :: MultiAddress < :: subxt :: ext :: subxt_core :: utils :: AccountId32 , () > , runtime_types :: midnight_node_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight ,) > ;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = :: core :: result :: Result < :: core :: result :: Result < () , runtime_types :: sp_runtime :: DispatchError > , runtime_types :: sp_runtime :: transaction_validity :: TransactionValidityError > ;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ApplyExtrinsic {
                    pub extrinsic: apply_extrinsic::Extrinsic,
                }
                pub mod finalize_block {
                    use super::runtime_types;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                        >;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct FinalizeBlock {}
                pub mod inherent_extrinsics {
                    use super::runtime_types;
                    pub type Inherent = runtime_types::sp_inherents::InherentData;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = :: subxt :: ext :: subxt_core :: alloc :: vec :: Vec < :: subxt :: ext :: subxt_core :: utils :: UncheckedExtrinsic < :: subxt :: ext :: subxt_core :: utils :: MultiAddress < :: subxt :: ext :: subxt_core :: utils :: AccountId32 , () > , runtime_types :: midnight_node_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight ,) > > ;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct InherentExtrinsics {
                    pub inherent: inherent_extrinsics::Inherent,
                }
                pub mod check_inherents {
                    use super::runtime_types;
                    pub type Block = runtime_types :: sp_runtime :: generic :: block :: Block < runtime_types :: sp_runtime :: generic :: header :: Header < :: core :: primitive :: u32 > , :: subxt :: ext :: subxt_core :: utils :: UncheckedExtrinsic < :: subxt :: ext :: subxt_core :: utils :: MultiAddress < :: subxt :: ext :: subxt_core :: utils :: AccountId32 , () > , runtime_types :: midnight_node_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight ,) > > ;
                    pub type Data = runtime_types::sp_inherents::InherentData;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = runtime_types::sp_inherents::CheckInherentsResult;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct CheckInherents {
                    pub block: check_inherents::Block,
                    pub data: check_inherents::Data,
                }
            }
        }
        pub mod tagged_transaction_queue {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " The `TaggedTransactionQueue` api trait for interfering with the transaction queue."]
            pub struct TaggedTransactionQueue;
            impl TaggedTransactionQueue {
                #[doc = " Validate the transaction."]
                #[doc = ""]
                #[doc = " This method is invoked by the transaction pool to learn details about given transaction."]
                #[doc = " The implementation should make sure to verify the correctness of the transaction"]
                #[doc = " against current state. The given `block_hash` corresponds to the hash of the block"]
                #[doc = " that is used as current state."]
                #[doc = ""]
                #[doc = " Note that this call may be performed by the pool multiple times and transactions"]
                #[doc = " might be verified in any possible order."]
                pub fn validate_transaction(
                    &self,
                    source: types::validate_transaction::Source,
                    tx: types::validate_transaction::Tx,
                    block_hash: types::validate_transaction::BlockHash,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::ValidateTransaction,
                    types::validate_transaction::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "TaggedTransactionQueue",
                        "validate_transaction",
                        types::ValidateTransaction {
                            source,
                            tx,
                            block_hash,
                        },
                        [
                            196u8, 50u8, 90u8, 49u8, 109u8, 251u8, 200u8, 35u8, 23u8, 150u8, 140u8,
                            143u8, 232u8, 164u8, 133u8, 89u8, 32u8, 240u8, 115u8, 39u8, 95u8, 70u8,
                            162u8, 76u8, 122u8, 73u8, 151u8, 144u8, 234u8, 120u8, 100u8, 29u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod validate_transaction {
                    use super::runtime_types;
                    pub type Source =
                        runtime_types::sp_runtime::transaction_validity::TransactionSource;
                    pub type Tx = :: subxt :: ext :: subxt_core :: utils :: UncheckedExtrinsic < :: subxt :: ext :: subxt_core :: utils :: MultiAddress < :: subxt :: ext :: subxt_core :: utils :: AccountId32 , () > , runtime_types :: midnight_node_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight ,) > ;
                    pub type BlockHash = ::subxt::ext::subxt_core::utils::H256;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = :: core :: result :: Result < runtime_types :: sp_runtime :: transaction_validity :: ValidTransaction , runtime_types :: sp_runtime :: transaction_validity :: TransactionValidityError > ;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ValidateTransaction {
                    pub source: validate_transaction::Source,
                    pub tx: validate_transaction::Tx,
                    pub block_hash: validate_transaction::BlockHash,
                }
            }
        }
        pub mod offchain_worker_api {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " The offchain worker api."]
            pub struct OffchainWorkerApi;
            impl OffchainWorkerApi {
                #[doc = " Starts the off-chain task for given block header."]
                pub fn offchain_worker(
                    &self,
                    header: types::offchain_worker::Header,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::OffchainWorker,
                    types::offchain_worker::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "OffchainWorkerApi",
                        "offchain_worker",
                        types::OffchainWorker { header },
                        [
                            10u8, 135u8, 19u8, 153u8, 33u8, 216u8, 18u8, 242u8, 33u8, 140u8, 4u8,
                            223u8, 200u8, 130u8, 103u8, 118u8, 137u8, 24u8, 19u8, 127u8, 161u8,
                            29u8, 184u8, 111u8, 222u8, 111u8, 253u8, 73u8, 45u8, 31u8, 79u8, 60u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod offchain_worker {
                    use super::runtime_types;
                    pub type Header =
                        runtime_types::sp_runtime::generic::header::Header<::core::primitive::u32>;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ();
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct OffchainWorker {
                    pub header: offchain_worker::Header,
                }
            }
        }
        pub mod aura_api {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " API necessary for block authorship with aura."]
            pub struct AuraApi;
            impl AuraApi {
                #[doc = " Returns the slot duration for Aura."]
                #[doc = ""]
                #[doc = " Currently, only the value provided by this type at genesis will be used."]
                pub fn slot_duration(
                    &self,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::SlotDuration,
                    types::slot_duration::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "AuraApi",
                        "slot_duration",
                        types::SlotDuration {},
                        [
                            233u8, 210u8, 132u8, 172u8, 100u8, 125u8, 239u8, 92u8, 114u8, 82u8,
                            7u8, 110u8, 179u8, 196u8, 10u8, 19u8, 211u8, 15u8, 174u8, 2u8, 91u8,
                            73u8, 133u8, 100u8, 205u8, 201u8, 191u8, 60u8, 163u8, 122u8, 215u8,
                            10u8,
                        ],
                    )
                }
                #[doc = " Return the current set of authorities."]
                pub fn authorities(
                    &self,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::Authorities,
                    types::authorities::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "AuraApi",
                        "authorities",
                        types::Authorities {},
                        [
                            96u8, 136u8, 226u8, 244u8, 105u8, 189u8, 8u8, 250u8, 71u8, 230u8, 37u8,
                            123u8, 218u8, 47u8, 179u8, 16u8, 170u8, 181u8, 165u8, 77u8, 102u8,
                            51u8, 43u8, 51u8, 186u8, 84u8, 49u8, 15u8, 208u8, 226u8, 129u8, 230u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod slot_duration {
                    use super::runtime_types;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = runtime_types::sp_consensus_slots::SlotDuration;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct SlotDuration {}
                pub mod authorities {
                    use super::runtime_types;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ::subxt::ext::subxt_core::alloc::vec::Vec<
                            runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
                        >;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Authorities {}
            }
        }
        pub mod session_keys {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " Session keys runtime api."]
            pub struct SessionKeys;
            impl SessionKeys {
                #[doc = " Generate a set of session keys with optionally using the given seed."]
                #[doc = " The keys should be stored within the keystore exposed via runtime"]
                #[doc = " externalities."]
                #[doc = ""]
                #[doc = " The seed needs to be a valid `utf8` string."]
                #[doc = ""]
                #[doc = " Returns the concatenated SCALE encoded public keys."]
                pub fn generate_session_keys(
                    &self,
                    seed: types::generate_session_keys::Seed,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::GenerateSessionKeys,
                    types::generate_session_keys::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "SessionKeys",
                        "generate_session_keys",
                        types::GenerateSessionKeys { seed },
                        [
                            96u8, 171u8, 164u8, 166u8, 175u8, 102u8, 101u8, 47u8, 133u8, 95u8,
                            102u8, 202u8, 83u8, 26u8, 238u8, 47u8, 126u8, 132u8, 22u8, 11u8, 33u8,
                            190u8, 175u8, 94u8, 58u8, 245u8, 46u8, 80u8, 195u8, 184u8, 107u8, 65u8,
                        ],
                    )
                }
                #[doc = " Decode the given public session keys."]
                #[doc = ""]
                #[doc = " Returns the list of public raw public keys + key type."]
                pub fn decode_session_keys(
                    &self,
                    encoded: types::decode_session_keys::Encoded,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::DecodeSessionKeys,
                    types::decode_session_keys::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "SessionKeys",
                        "decode_session_keys",
                        types::DecodeSessionKeys { encoded },
                        [
                            57u8, 242u8, 18u8, 51u8, 132u8, 110u8, 238u8, 255u8, 39u8, 194u8, 8u8,
                            54u8, 198u8, 178u8, 75u8, 151u8, 148u8, 176u8, 144u8, 197u8, 87u8,
                            29u8, 179u8, 235u8, 176u8, 78u8, 252u8, 103u8, 72u8, 203u8, 151u8,
                            248u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod generate_session_keys {
                    use super::runtime_types;
                    pub type Seed = ::core::option::Option<
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    >;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output =
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct GenerateSessionKeys {
                    pub seed: generate_session_keys::Seed,
                }
                pub mod decode_session_keys {
                    use super::runtime_types;
                    pub type Encoded =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ::core::option::Option<
                            ::subxt::ext::subxt_core::alloc::vec::Vec<(
                                ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                                runtime_types::sp_core::crypto::KeyTypeId,
                            )>,
                        >;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct DecodeSessionKeys {
                    pub encoded: decode_session_keys::Encoded,
                }
            }
        }
        pub mod grandpa_api {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " APIs for integrating the GRANDPA finality gadget into runtimes."]
            #[doc = " This should be implemented on the runtime side."]
            #[doc = ""]
            #[doc = " This is primarily used for negotiating authority-set changes for the"]
            #[doc = " gadget. GRANDPA uses a signaling model of changing authority sets:"]
            #[doc = " changes should be signaled with a delay of N blocks, and then automatically"]
            #[doc = " applied in the runtime after those N blocks have passed."]
            #[doc = ""]
            #[doc = " The consensus protocol will coordinate the handoff externally."]
            pub struct GrandpaApi;
            impl GrandpaApi {
                #[doc = " Get the current GRANDPA authorities and weights. This should not change except"]
                #[doc = " for when changes are scheduled and the corresponding delay has passed."]
                #[doc = ""]
                #[doc = " When called at block B, it will return the set of authorities that should be"]
                #[doc = " used to finalize descendants of this block (B+1, B+2, ...). The block B itself"]
                #[doc = " is finalized by the authorities from block B-1."]
                pub fn grandpa_authorities(
                    &self,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::GrandpaAuthorities,
                    types::grandpa_authorities::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "GrandpaApi",
                        "grandpa_authorities",
                        types::GrandpaAuthorities {},
                        [
                            166u8, 76u8, 160u8, 101u8, 242u8, 145u8, 213u8, 10u8, 16u8, 130u8,
                            230u8, 196u8, 125u8, 152u8, 92u8, 143u8, 119u8, 223u8, 140u8, 189u8,
                            203u8, 95u8, 52u8, 105u8, 147u8, 107u8, 135u8, 228u8, 62u8, 178u8,
                            128u8, 33u8,
                        ],
                    )
                }
                #[doc = " Submits an unsigned extrinsic to report an equivocation. The caller"]
                #[doc = " must provide the equivocation proof and a key ownership proof"]
                #[doc = " (should be obtained using `generate_key_ownership_proof`). The"]
                #[doc = " extrinsic will be unsigned and should only be accepted for local"]
                #[doc = " authorship (not to be broadcast to the network). This method returns"]
                #[doc = " `None` when creation of the extrinsic fails, e.g. if equivocation"]
                #[doc = " reporting is disabled for the given runtime (i.e. this method is"]
                #[doc = " hardcoded to return `None`). Only useful in an offchain context."]
                pub fn submit_report_equivocation_unsigned_extrinsic(
                    &self,
                    equivocation_proof : types :: submit_report_equivocation_unsigned_extrinsic :: EquivocationProof,
                    key_owner_proof : types :: submit_report_equivocation_unsigned_extrinsic :: KeyOwnerProof,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::SubmitReportEquivocationUnsignedExtrinsic,
                    types::submit_report_equivocation_unsigned_extrinsic::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "GrandpaApi",
                        "submit_report_equivocation_unsigned_extrinsic",
                        types::SubmitReportEquivocationUnsignedExtrinsic {
                            equivocation_proof,
                            key_owner_proof,
                        },
                        [
                            112u8, 94u8, 150u8, 250u8, 132u8, 127u8, 185u8, 24u8, 113u8, 62u8,
                            28u8, 171u8, 83u8, 9u8, 41u8, 228u8, 92u8, 137u8, 29u8, 190u8, 214u8,
                            232u8, 100u8, 66u8, 100u8, 168u8, 149u8, 122u8, 93u8, 17u8, 236u8,
                            104u8,
                        ],
                    )
                }
                #[doc = " Generates a proof of key ownership for the given authority in the"]
                #[doc = " given set. An example usage of this module is coupled with the"]
                #[doc = " session historical module to prove that a given authority key is"]
                #[doc = " tied to a given staking identity during a specific session. Proofs"]
                #[doc = " of key ownership are necessary for submitting equivocation reports."]
                #[doc = " NOTE: even though the API takes a `set_id` as parameter the current"]
                #[doc = " implementations ignore this parameter and instead rely on this"]
                #[doc = " method being called at the correct block height, i.e. any point at"]
                #[doc = " which the given set id is live on-chain. Future implementations will"]
                #[doc = " instead use indexed data through an offchain worker, not requiring"]
                #[doc = " older states to be available."]
                pub fn generate_key_ownership_proof(
                    &self,
                    set_id: types::generate_key_ownership_proof::SetId,
                    authority_id: types::generate_key_ownership_proof::AuthorityId,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::GenerateKeyOwnershipProof,
                    types::generate_key_ownership_proof::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "GrandpaApi",
                        "generate_key_ownership_proof",
                        types::GenerateKeyOwnershipProof {
                            set_id,
                            authority_id,
                        },
                        [
                            40u8, 126u8, 113u8, 27u8, 245u8, 45u8, 123u8, 138u8, 12u8, 3u8, 125u8,
                            186u8, 151u8, 53u8, 186u8, 93u8, 13u8, 150u8, 163u8, 176u8, 206u8,
                            89u8, 244u8, 127u8, 182u8, 85u8, 203u8, 41u8, 101u8, 183u8, 209u8,
                            179u8,
                        ],
                    )
                }
                #[doc = " Get current GRANDPA authority set id."]
                pub fn current_set_id(
                    &self,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::CurrentSetId,
                    types::current_set_id::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "GrandpaApi",
                        "current_set_id",
                        types::CurrentSetId {},
                        [
                            42u8, 230u8, 120u8, 211u8, 156u8, 245u8, 109u8, 86u8, 100u8, 146u8,
                            234u8, 205u8, 41u8, 183u8, 109u8, 42u8, 17u8, 33u8, 156u8, 25u8, 139u8,
                            84u8, 101u8, 75u8, 232u8, 198u8, 87u8, 136u8, 218u8, 233u8, 103u8,
                            156u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod grandpa_authorities {
                    use super::runtime_types;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ::subxt::ext::subxt_core::alloc::vec::Vec<(
                            runtime_types::sp_consensus_grandpa::app::Public,
                            ::core::primitive::u64,
                        )>;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct GrandpaAuthorities {}
                pub mod submit_report_equivocation_unsigned_extrinsic {
                    use super::runtime_types;
                    pub type EquivocationProof =
                        runtime_types::sp_consensus_grandpa::EquivocationProof<
                            ::subxt::ext::subxt_core::utils::H256,
                            ::core::primitive::u32,
                        >;
                    pub type KeyOwnerProof =
                        runtime_types::sp_consensus_grandpa::OpaqueKeyOwnershipProof;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ::core::option::Option<()>;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct SubmitReportEquivocationUnsignedExtrinsic {
                    pub equivocation_proof:
                        submit_report_equivocation_unsigned_extrinsic::EquivocationProof,
                    pub key_owner_proof:
                        submit_report_equivocation_unsigned_extrinsic::KeyOwnerProof,
                }
                pub mod generate_key_ownership_proof {
                    use super::runtime_types;
                    pub type SetId = ::core::primitive::u64;
                    pub type AuthorityId = runtime_types::sp_consensus_grandpa::app::Public;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ::core::option::Option<
                            runtime_types::sp_consensus_grandpa::OpaqueKeyOwnershipProof,
                        >;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct GenerateKeyOwnershipProof {
                    pub set_id: generate_key_ownership_proof::SetId,
                    pub authority_id: generate_key_ownership_proof::AuthorityId,
                }
                pub mod current_set_id {
                    use super::runtime_types;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ::core::primitive::u64;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct CurrentSetId {}
            }
        }
        pub mod account_nonce_api {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " The API to query account nonce."]
            pub struct AccountNonceApi;
            impl AccountNonceApi {
                #[doc = " Get current account nonce of given `AccountId`."]
                pub fn account_nonce(
                    &self,
                    account: types::account_nonce::Account,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::AccountNonce,
                    types::account_nonce::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "AccountNonceApi",
                        "account_nonce",
                        types::AccountNonce { account },
                        [
                            231u8, 82u8, 7u8, 227u8, 131u8, 2u8, 215u8, 252u8, 173u8, 82u8, 11u8,
                            103u8, 200u8, 25u8, 114u8, 116u8, 79u8, 229u8, 152u8, 150u8, 236u8,
                            37u8, 101u8, 26u8, 220u8, 146u8, 182u8, 101u8, 73u8, 55u8, 191u8,
                            171u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod account_nonce {
                    use super::runtime_types;
                    pub type Account = ::subxt::ext::subxt_core::utils::AccountId32;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ::core::primitive::u32;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct AccountNonce {
                    pub account: account_nonce::Account,
                }
            }
        }
        pub mod get_sidechain_params {
            use super::root_mod;
            use super::runtime_types;
            pub struct GetSidechainParams;
            impl GetSidechainParams {
                pub fn sidechain_params(
                    &self,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::SidechainParams,
                    types::sidechain_params::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "GetSidechainParams",
                        "sidechain_params",
                        types::SidechainParams {},
                        [
                            165u8, 31u8, 22u8, 105u8, 186u8, 86u8, 252u8, 238u8, 41u8, 70u8, 195u8,
                            134u8, 196u8, 152u8, 94u8, 216u8, 112u8, 236u8, 75u8, 207u8, 210u8,
                            129u8, 111u8, 3u8, 118u8, 144u8, 43u8, 208u8, 159u8, 66u8, 137u8,
                            177u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod sidechain_params {
                    use super::runtime_types;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = runtime_types::sidechain_domain::SidechainParams;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct SidechainParams {}
            }
        }
        pub mod get_sidechain_status {
            use super::root_mod;
            use super::runtime_types;
            pub struct GetSidechainStatus;
            impl GetSidechainStatus {
                pub fn get_sidechain_status(
                    &self,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::GetSidechainStatus,
                    types::get_sidechain_status::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "GetSidechainStatus",
                        "get_sidechain_status",
                        types::GetSidechainStatus {},
                        [
                            88u8, 40u8, 13u8, 225u8, 14u8, 152u8, 49u8, 242u8, 158u8, 136u8, 176u8,
                            47u8, 143u8, 112u8, 254u8, 187u8, 187u8, 181u8, 240u8, 18u8, 139u8,
                            56u8, 147u8, 210u8, 41u8, 47u8, 236u8, 197u8, 60u8, 70u8, 146u8, 167u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod get_sidechain_status {
                    use super::runtime_types;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = runtime_types::pallet_sidechain::types::SidechainStatus;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct GetSidechainStatus {}
            }
        }
        pub mod slot_api {
            use super::root_mod;
            use super::runtime_types;
            pub struct SlotApi;
            impl SlotApi {
                pub fn slot_config(
                    &self,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::SlotConfig,
                    types::slot_config::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "SlotApi",
                        "slot_config",
                        types::SlotConfig {},
                        [
                            110u8, 28u8, 227u8, 158u8, 191u8, 127u8, 173u8, 20u8, 234u8, 20u8,
                            65u8, 158u8, 203u8, 203u8, 172u8, 39u8, 184u8, 211u8, 95u8, 240u8,
                            109u8, 86u8, 204u8, 38u8, 216u8, 129u8, 130u8, 8u8, 25u8, 247u8, 173u8,
                            7u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod slot_config {
                    use super::runtime_types;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = runtime_types::sidechain_slots::ScSlotConfig;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct SlotConfig {}
            }
        }
        pub mod session_validator_management_api {
            use super::root_mod;
            use super::runtime_types;
            pub struct SessionValidatorManagementApi;
            impl SessionValidatorManagementApi {
                pub fn get_current_committee(
                    &self,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::GetCurrentCommittee,
                    types::get_current_committee::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "SessionValidatorManagementApi",
                        "get_current_committee",
                        types::GetCurrentCommittee {},
                        [
                            154u8, 21u8, 148u8, 215u8, 27u8, 61u8, 33u8, 105u8, 59u8, 204u8, 162u8,
                            155u8, 43u8, 42u8, 72u8, 48u8, 159u8, 73u8, 34u8, 149u8, 88u8, 251u8,
                            97u8, 22u8, 116u8, 164u8, 184u8, 53u8, 114u8, 35u8, 35u8, 223u8,
                        ],
                    )
                }
                pub fn get_next_committee(
                    &self,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::GetNextCommittee,
                    types::get_next_committee::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "SessionValidatorManagementApi",
                        "get_next_committee",
                        types::GetNextCommittee {},
                        [
                            209u8, 210u8, 158u8, 120u8, 92u8, 74u8, 17u8, 97u8, 45u8, 121u8, 56u8,
                            25u8, 50u8, 126u8, 95u8, 37u8, 192u8, 189u8, 151u8, 68u8, 119u8, 130u8,
                            14u8, 64u8, 13u8, 9u8, 64u8, 234u8, 161u8, 240u8, 69u8, 26u8,
                        ],
                    )
                }
                pub fn get_next_unset_epoch_number(
                    &self,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::GetNextUnsetEpochNumber,
                    types::get_next_unset_epoch_number::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "SessionValidatorManagementApi",
                        "get_next_unset_epoch_number",
                        types::GetNextUnsetEpochNumber {},
                        [
                            18u8, 52u8, 69u8, 147u8, 243u8, 90u8, 109u8, 159u8, 1u8, 154u8, 55u8,
                            75u8, 186u8, 240u8, 83u8, 115u8, 221u8, 93u8, 165u8, 144u8, 163u8,
                            150u8, 145u8, 252u8, 148u8, 125u8, 164u8, 206u8, 252u8, 140u8, 20u8,
                            203u8,
                        ],
                    )
                }
                pub fn calculate_committee(
                    &self,
                    authority_selection_inputs : types :: calculate_committee :: AuthoritySelectionInputs,
                    sidechain_epoch: types::calculate_committee::SidechainEpoch,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::CalculateCommittee,
                    types::calculate_committee::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "SessionValidatorManagementApi",
                        "calculate_committee",
                        types::CalculateCommittee {
                            authority_selection_inputs,
                            sidechain_epoch,
                        },
                        [
                            138u8, 185u8, 51u8, 103u8, 175u8, 110u8, 15u8, 97u8, 227u8, 22u8,
                            195u8, 230u8, 242u8, 145u8, 216u8, 247u8, 27u8, 240u8, 46u8, 67u8,
                            179u8, 164u8, 212u8, 254u8, 43u8, 0u8, 244u8, 121u8, 34u8, 45u8, 61u8,
                            126u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod get_current_committee {
                    use super::runtime_types;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = (runtime_types :: sidechain_domain :: ScEpochNumber , :: subxt :: ext :: subxt_core :: alloc :: vec :: Vec < runtime_types :: sidechain_inherents :: cross_chain_signatures :: crypto :: cross_chain_app :: Public > ,) ;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct GetCurrentCommittee {}
                pub mod get_next_committee {
                    use super::runtime_types;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = :: core :: option :: Option < (runtime_types :: sidechain_domain :: ScEpochNumber , :: subxt :: ext :: subxt_core :: alloc :: vec :: Vec < runtime_types :: sidechain_inherents :: cross_chain_signatures :: crypto :: cross_chain_app :: Public > ,) > ;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct GetNextCommittee {}
                pub mod get_next_unset_epoch_number {
                    use super::runtime_types;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = runtime_types::sidechain_domain::ScEpochNumber;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct GetNextUnsetEpochNumber {}
                pub mod calculate_committee {
                    use super::runtime_types;
                    pub type AuthoritySelectionInputs = runtime_types :: authority_selection_inherents :: authority_selection_inputs :: AuthoritySelectionInputs ;
                    pub type SidechainEpoch = runtime_types::sidechain_domain::ScEpochNumber;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = :: core :: option :: Option < :: subxt :: ext :: subxt_core :: alloc :: vec :: Vec < (runtime_types :: sidechain_inherents :: cross_chain_signatures :: crypto :: cross_chain_app :: Public , runtime_types :: midnight_node_runtime :: opaque :: SessionKeys ,) > > ;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct CalculateCommittee {
                    pub authority_selection_inputs: calculate_committee::AuthoritySelectionInputs,
                    pub sidechain_epoch: calculate_committee::SidechainEpoch,
                }
            }
        }
    }
    pub fn custom() -> CustomValuesApi {
        CustomValuesApi
    }
    pub struct CustomValuesApi;
    impl CustomValuesApi {}
    pub struct ConstantsApi;
    impl ConstantsApi {
        pub fn system(&self) -> system::constants::ConstantsApi {
            system::constants::ConstantsApi
        }
        pub fn timestamp(&self) -> timestamp::constants::ConstantsApi {
            timestamp::constants::ConstantsApi
        }
        pub fn grandpa(&self) -> grandpa::constants::ConstantsApi {
            grandpa::constants::ConstantsApi
        }
        pub fn balances(&self) -> balances::constants::ConstantsApi {
            balances::constants::ConstantsApi
        }
        pub fn session_committee_management(
            &self,
        ) -> session_committee_management::constants::ConstantsApi {
            session_committee_management::constants::ConstantsApi
        }
    }
    pub struct StorageApi;
    impl StorageApi {
        pub fn system(&self) -> system::storage::StorageApi {
            system::storage::StorageApi
        }
        pub fn timestamp(&self) -> timestamp::storage::StorageApi {
            timestamp::storage::StorageApi
        }
        pub fn aura(&self) -> aura::storage::StorageApi {
            aura::storage::StorageApi
        }
        pub fn grandpa(&self) -> grandpa::storage::StorageApi {
            grandpa::storage::StorageApi
        }
        pub fn sidechain(&self) -> sidechain::storage::StorageApi {
            sidechain::storage::StorageApi
        }
        pub fn midnight(&self) -> midnight::storage::StorageApi {
            midnight::storage::StorageApi
        }
        pub fn balances(&self) -> balances::storage::StorageApi {
            balances::storage::StorageApi
        }
        pub fn sudo(&self) -> sudo::storage::StorageApi {
            sudo::storage::StorageApi
        }
        pub fn session_committee_management(
            &self,
        ) -> session_committee_management::storage::StorageApi {
            session_committee_management::storage::StorageApi
        }
        pub fn session(&self) -> session::storage::StorageApi {
            session::storage::StorageApi
        }
    }
    pub struct TransactionApi;
    impl TransactionApi {
        pub fn system(&self) -> system::calls::TransactionApi {
            system::calls::TransactionApi
        }
        pub fn timestamp(&self) -> timestamp::calls::TransactionApi {
            timestamp::calls::TransactionApi
        }
        pub fn grandpa(&self) -> grandpa::calls::TransactionApi {
            grandpa::calls::TransactionApi
        }
        pub fn midnight(&self) -> midnight::calls::TransactionApi {
            midnight::calls::TransactionApi
        }
        pub fn balances(&self) -> balances::calls::TransactionApi {
            balances::calls::TransactionApi
        }
        pub fn sudo(&self) -> sudo::calls::TransactionApi {
            sudo::calls::TransactionApi
        }
        pub fn session_committee_management(
            &self,
        ) -> session_committee_management::calls::TransactionApi {
            session_committee_management::calls::TransactionApi
        }
    }
    #[doc = r" check whether the metadata provided is aligned with this statically generated code."]
    pub fn is_codegen_valid_for(metadata: &::subxt::ext::subxt_core::Metadata) -> bool {
        let runtime_metadata_hash = metadata
            .hasher()
            .only_these_pallets(&PALLETS)
            .only_these_runtime_apis(&RUNTIME_APIS)
            .hash();
        runtime_metadata_hash
            == [
                174u8, 85u8, 94u8, 27u8, 83u8, 166u8, 175u8, 233u8, 199u8, 126u8, 84u8, 130u8,
                156u8, 62u8, 127u8, 154u8, 94u8, 78u8, 43u8, 98u8, 244u8, 145u8, 42u8, 245u8,
                188u8, 150u8, 231u8, 107u8, 214u8, 149u8, 111u8, 84u8,
            ]
    }
    pub mod system {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Error for the System pallet"]
        pub type Error = runtime_types::frame_system::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::frame_system::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Make some on-chain remark."]
                #[doc = ""]
                #[doc = "Can be executed by every `origin`."]
                pub struct Remark {
                    pub remark: remark::Remark,
                }
                pub mod remark {
                    use super::runtime_types;
                    pub type Remark =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for Remark {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "remark";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Set the number of pages in the WebAssembly environment's heap."]
                pub struct SetHeapPages {
                    pub pages: set_heap_pages::Pages,
                }
                pub mod set_heap_pages {
                    use super::runtime_types;
                    pub type Pages = ::core::primitive::u64;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetHeapPages {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "set_heap_pages";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Set the new runtime code."]
                pub struct SetCode {
                    pub code: set_code::Code,
                }
                pub mod set_code {
                    use super::runtime_types;
                    pub type Code =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetCode {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "set_code";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Set the new runtime code without doing any checks of the given `code`."]
                #[doc = ""]
                #[doc = "Note that runtime upgrades will not run if this is called with a not-increasing spec"]
                #[doc = "version!"]
                pub struct SetCodeWithoutChecks {
                    pub code: set_code_without_checks::Code,
                }
                pub mod set_code_without_checks {
                    use super::runtime_types;
                    pub type Code =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetCodeWithoutChecks {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "set_code_without_checks";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Set some items of storage."]
                pub struct SetStorage {
                    pub items: set_storage::Items,
                }
                pub mod set_storage {
                    use super::runtime_types;
                    pub type Items = ::subxt::ext::subxt_core::alloc::vec::Vec<(
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    )>;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetStorage {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "set_storage";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Kill some items from storage."]
                pub struct KillStorage {
                    pub keys: kill_storage::Keys,
                }
                pub mod kill_storage {
                    use super::runtime_types;
                    pub type Keys = ::subxt::ext::subxt_core::alloc::vec::Vec<
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    >;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for KillStorage {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "kill_storage";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Kill all storage items with a key that starts with the given prefix."]
                #[doc = ""]
                #[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
                #[doc = "the prefix we are removing to accurately calculate the weight of this function."]
                pub struct KillPrefix {
                    pub prefix: kill_prefix::Prefix,
                    pub subkeys: kill_prefix::Subkeys,
                }
                pub mod kill_prefix {
                    use super::runtime_types;
                    pub type Prefix =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub type Subkeys = ::core::primitive::u32;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for KillPrefix {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "kill_prefix";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Make some on-chain remark and emit event."]
                pub struct RemarkWithEvent {
                    pub remark: remark_with_event::Remark,
                }
                pub mod remark_with_event {
                    use super::runtime_types;
                    pub type Remark =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for RemarkWithEvent {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "remark_with_event";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
                #[doc = "later."]
                #[doc = ""]
                #[doc = "This call requires Root origin."]
                pub struct AuthorizeUpgrade {
                    pub code_hash: authorize_upgrade::CodeHash,
                }
                pub mod authorize_upgrade {
                    use super::runtime_types;
                    pub type CodeHash = ::subxt::ext::subxt_core::utils::H256;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for AuthorizeUpgrade {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "authorize_upgrade";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
                #[doc = "later."]
                #[doc = ""]
                #[doc = "WARNING: This authorizes an upgrade that will take place without any safety checks, for"]
                #[doc = "example that the spec name remains the same and that the version number increases. Not"]
                #[doc = "recommended for normal use. Use `authorize_upgrade` instead."]
                #[doc = ""]
                #[doc = "This call requires Root origin."]
                pub struct AuthorizeUpgradeWithoutChecks {
                    pub code_hash: authorize_upgrade_without_checks::CodeHash,
                }
                pub mod authorize_upgrade_without_checks {
                    use super::runtime_types;
                    pub type CodeHash = ::subxt::ext::subxt_core::utils::H256;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for AuthorizeUpgradeWithoutChecks {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "authorize_upgrade_without_checks";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Provide the preimage (runtime binary) `code` for an upgrade that has been authorized."]
                #[doc = ""]
                #[doc = "If the authorization required a version check, this call will ensure the spec name"]
                #[doc = "remains unchanged and that the spec version has increased."]
                #[doc = ""]
                #[doc = "Depending on the runtime's `OnSetCode` configuration, this function may directly apply"]
                #[doc = "the new `code` in the same block or attempt to schedule the upgrade."]
                #[doc = ""]
                #[doc = "All origins are allowed."]
                pub struct ApplyAuthorizedUpgrade {
                    pub code: apply_authorized_upgrade::Code,
                }
                pub mod apply_authorized_upgrade {
                    use super::runtime_types;
                    pub type Code =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ApplyAuthorizedUpgrade {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "apply_authorized_upgrade";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Make some on-chain remark."]
                #[doc = ""]
                #[doc = "Can be executed by every `origin`."]
                pub fn remark(
                    &self,
                    remark: types::remark::Remark,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::Remark>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "remark",
                        types::Remark { remark },
                        [
                            43u8, 126u8, 180u8, 174u8, 141u8, 48u8, 52u8, 125u8, 166u8, 212u8,
                            216u8, 98u8, 100u8, 24u8, 132u8, 71u8, 101u8, 64u8, 246u8, 169u8, 33u8,
                            250u8, 147u8, 208u8, 2u8, 40u8, 129u8, 209u8, 232u8, 207u8, 207u8,
                            13u8,
                        ],
                    )
                }
                #[doc = "Set the number of pages in the WebAssembly environment's heap."]
                pub fn set_heap_pages(
                    &self,
                    pages: types::set_heap_pages::Pages,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetHeapPages>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "set_heap_pages",
                        types::SetHeapPages { pages },
                        [
                            188u8, 191u8, 99u8, 216u8, 219u8, 109u8, 141u8, 50u8, 78u8, 235u8,
                            215u8, 242u8, 195u8, 24u8, 111u8, 76u8, 229u8, 64u8, 99u8, 225u8,
                            134u8, 121u8, 81u8, 209u8, 127u8, 223u8, 98u8, 215u8, 150u8, 70u8,
                            57u8, 147u8,
                        ],
                    )
                }
                #[doc = "Set the new runtime code."]
                pub fn set_code(
                    &self,
                    code: types::set_code::Code,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetCode>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "set_code",
                        types::SetCode { code },
                        [
                            233u8, 248u8, 88u8, 245u8, 28u8, 65u8, 25u8, 169u8, 35u8, 237u8, 19u8,
                            203u8, 136u8, 160u8, 18u8, 3u8, 20u8, 197u8, 81u8, 169u8, 244u8, 188u8,
                            27u8, 147u8, 147u8, 236u8, 65u8, 25u8, 3u8, 143u8, 182u8, 22u8,
                        ],
                    )
                }
                #[doc = "Set the new runtime code without doing any checks of the given `code`."]
                #[doc = ""]
                #[doc = "Note that runtime upgrades will not run if this is called with a not-increasing spec"]
                #[doc = "version!"]
                pub fn set_code_without_checks(
                    &self,
                    code: types::set_code_without_checks::Code,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetCodeWithoutChecks>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "set_code_without_checks",
                        types::SetCodeWithoutChecks { code },
                        [
                            82u8, 212u8, 157u8, 44u8, 70u8, 0u8, 143u8, 15u8, 109u8, 109u8, 107u8,
                            157u8, 141u8, 42u8, 169u8, 11u8, 15u8, 186u8, 252u8, 138u8, 10u8,
                            147u8, 15u8, 178u8, 247u8, 229u8, 213u8, 98u8, 207u8, 231u8, 119u8,
                            115u8,
                        ],
                    )
                }
                #[doc = "Set some items of storage."]
                pub fn set_storage(
                    &self,
                    items: types::set_storage::Items,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetStorage>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "set_storage",
                        types::SetStorage { items },
                        [
                            141u8, 216u8, 52u8, 222u8, 223u8, 136u8, 123u8, 181u8, 19u8, 75u8,
                            163u8, 102u8, 229u8, 189u8, 158u8, 142u8, 95u8, 235u8, 240u8, 49u8,
                            150u8, 76u8, 78u8, 137u8, 126u8, 88u8, 183u8, 88u8, 231u8, 146u8,
                            234u8, 43u8,
                        ],
                    )
                }
                #[doc = "Kill some items from storage."]
                pub fn kill_storage(
                    &self,
                    keys: types::kill_storage::Keys,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::KillStorage>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "kill_storage",
                        types::KillStorage { keys },
                        [
                            73u8, 63u8, 196u8, 36u8, 144u8, 114u8, 34u8, 213u8, 108u8, 93u8, 209u8,
                            234u8, 153u8, 185u8, 33u8, 91u8, 187u8, 195u8, 223u8, 130u8, 58u8,
                            156u8, 63u8, 47u8, 228u8, 249u8, 216u8, 139u8, 143u8, 177u8, 41u8,
                            35u8,
                        ],
                    )
                }
                #[doc = "Kill all storage items with a key that starts with the given prefix."]
                #[doc = ""]
                #[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
                #[doc = "the prefix we are removing to accurately calculate the weight of this function."]
                pub fn kill_prefix(
                    &self,
                    prefix: types::kill_prefix::Prefix,
                    subkeys: types::kill_prefix::Subkeys,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::KillPrefix>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "kill_prefix",
                        types::KillPrefix { prefix, subkeys },
                        [
                            184u8, 57u8, 139u8, 24u8, 208u8, 87u8, 108u8, 215u8, 198u8, 189u8,
                            175u8, 242u8, 167u8, 215u8, 97u8, 63u8, 110u8, 166u8, 238u8, 98u8,
                            67u8, 236u8, 111u8, 110u8, 234u8, 81u8, 102u8, 5u8, 182u8, 5u8, 214u8,
                            85u8,
                        ],
                    )
                }
                #[doc = "Make some on-chain remark and emit event."]
                pub fn remark_with_event(
                    &self,
                    remark: types::remark_with_event::Remark,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::RemarkWithEvent>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "remark_with_event",
                        types::RemarkWithEvent { remark },
                        [
                            120u8, 120u8, 153u8, 92u8, 184u8, 85u8, 34u8, 2u8, 174u8, 206u8, 105u8,
                            228u8, 233u8, 130u8, 80u8, 246u8, 228u8, 59u8, 234u8, 240u8, 4u8, 49u8,
                            147u8, 170u8, 115u8, 91u8, 149u8, 200u8, 228u8, 181u8, 8u8, 154u8,
                        ],
                    )
                }
                #[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
                #[doc = "later."]
                #[doc = ""]
                #[doc = "This call requires Root origin."]
                pub fn authorize_upgrade(
                    &self,
                    code_hash: types::authorize_upgrade::CodeHash,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::AuthorizeUpgrade>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "authorize_upgrade",
                        types::AuthorizeUpgrade { code_hash },
                        [
                            4u8, 14u8, 76u8, 107u8, 209u8, 129u8, 9u8, 39u8, 193u8, 17u8, 84u8,
                            254u8, 170u8, 214u8, 24u8, 155u8, 29u8, 184u8, 249u8, 241u8, 109u8,
                            58u8, 145u8, 131u8, 109u8, 63u8, 38u8, 165u8, 107u8, 215u8, 217u8,
                            172u8,
                        ],
                    )
                }
                #[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
                #[doc = "later."]
                #[doc = ""]
                #[doc = "WARNING: This authorizes an upgrade that will take place without any safety checks, for"]
                #[doc = "example that the spec name remains the same and that the version number increases. Not"]
                #[doc = "recommended for normal use. Use `authorize_upgrade` instead."]
                #[doc = ""]
                #[doc = "This call requires Root origin."]
                pub fn authorize_upgrade_without_checks(
                    &self,
                    code_hash: types::authorize_upgrade_without_checks::CodeHash,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<
                    types::AuthorizeUpgradeWithoutChecks,
                > {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "authorize_upgrade_without_checks",
                        types::AuthorizeUpgradeWithoutChecks { code_hash },
                        [
                            126u8, 126u8, 55u8, 26u8, 47u8, 55u8, 66u8, 8u8, 167u8, 18u8, 29u8,
                            136u8, 146u8, 14u8, 189u8, 117u8, 16u8, 227u8, 162u8, 61u8, 149u8,
                            197u8, 104u8, 184u8, 185u8, 161u8, 99u8, 154u8, 80u8, 125u8, 181u8,
                            233u8,
                        ],
                    )
                }
                #[doc = "Provide the preimage (runtime binary) `code` for an upgrade that has been authorized."]
                #[doc = ""]
                #[doc = "If the authorization required a version check, this call will ensure the spec name"]
                #[doc = "remains unchanged and that the spec version has increased."]
                #[doc = ""]
                #[doc = "Depending on the runtime's `OnSetCode` configuration, this function may directly apply"]
                #[doc = "the new `code` in the same block or attempt to schedule the upgrade."]
                #[doc = ""]
                #[doc = "All origins are allowed."]
                pub fn apply_authorized_upgrade(
                    &self,
                    code: types::apply_authorized_upgrade::Code,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<
                    types::ApplyAuthorizedUpgrade,
                > {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "apply_authorized_upgrade",
                        types::ApplyAuthorizedUpgrade { code },
                        [
                            232u8, 107u8, 127u8, 38u8, 230u8, 29u8, 97u8, 4u8, 160u8, 191u8, 222u8,
                            156u8, 245u8, 102u8, 196u8, 141u8, 44u8, 163u8, 98u8, 68u8, 125u8,
                            32u8, 124u8, 101u8, 108u8, 93u8, 211u8, 52u8, 0u8, 231u8, 33u8, 227u8,
                        ],
                    )
                }
            }
        }
        #[doc = "Event for the System pallet."]
        pub type Event = runtime_types::frame_system::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "An extrinsic completed successfully."]
            pub struct ExtrinsicSuccess {
                pub dispatch_info: extrinsic_success::DispatchInfo,
            }
            pub mod extrinsic_success {
                use super::runtime_types;
                pub type DispatchInfo = runtime_types::frame_support::dispatch::DispatchInfo;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for ExtrinsicSuccess {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicSuccess";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "An extrinsic failed."]
            pub struct ExtrinsicFailed {
                pub dispatch_error: extrinsic_failed::DispatchError,
                pub dispatch_info: extrinsic_failed::DispatchInfo,
            }
            pub mod extrinsic_failed {
                use super::runtime_types;
                pub type DispatchError = runtime_types::sp_runtime::DispatchError;
                pub type DispatchInfo = runtime_types::frame_support::dispatch::DispatchInfo;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for ExtrinsicFailed {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicFailed";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "`:code` was updated."]
            pub struct CodeUpdated;
            impl ::subxt::ext::subxt_core::events::StaticEvent for CodeUpdated {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "CodeUpdated";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "A new account was created."]
            pub struct NewAccount {
                pub account: new_account::Account,
            }
            pub mod new_account {
                use super::runtime_types;
                pub type Account = ::subxt::ext::subxt_core::utils::AccountId32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for NewAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "NewAccount";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "An account was reaped."]
            pub struct KilledAccount {
                pub account: killed_account::Account,
            }
            pub mod killed_account {
                use super::runtime_types;
                pub type Account = ::subxt::ext::subxt_core::utils::AccountId32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for KilledAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "KilledAccount";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "On on-chain remark happened."]
            pub struct Remarked {
                pub sender: remarked::Sender,
                pub hash: remarked::Hash,
            }
            pub mod remarked {
                use super::runtime_types;
                pub type Sender = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Hash = ::subxt::ext::subxt_core::utils::H256;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Remarked {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "Remarked";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "An upgrade was authorized."]
            pub struct UpgradeAuthorized {
                pub code_hash: upgrade_authorized::CodeHash,
                pub check_version: upgrade_authorized::CheckVersion,
            }
            pub mod upgrade_authorized {
                use super::runtime_types;
                pub type CodeHash = ::subxt::ext::subxt_core::utils::H256;
                pub type CheckVersion = ::core::primitive::bool;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for UpgradeAuthorized {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "UpgradeAuthorized";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod account {
                    use super::runtime_types;
                    pub type Account = runtime_types::frame_system::AccountInfo<
                        ::core::primitive::u32,
                        runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
                    >;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                pub mod extrinsic_count {
                    use super::runtime_types;
                    pub type ExtrinsicCount = ::core::primitive::u32;
                }
                pub mod inherents_applied {
                    use super::runtime_types;
                    pub type InherentsApplied = ::core::primitive::bool;
                }
                pub mod block_weight {
                    use super::runtime_types;
                    pub type BlockWeight = runtime_types::frame_support::dispatch::PerDispatchClass<
                        runtime_types::sp_weights::weight_v2::Weight,
                    >;
                }
                pub mod all_extrinsics_len {
                    use super::runtime_types;
                    pub type AllExtrinsicsLen = ::core::primitive::u32;
                }
                pub mod block_hash {
                    use super::runtime_types;
                    pub type BlockHash = ::subxt::ext::subxt_core::utils::H256;
                    pub type Param0 = ::core::primitive::u32;
                }
                pub mod extrinsic_data {
                    use super::runtime_types;
                    pub type ExtrinsicData =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub type Param0 = ::core::primitive::u32;
                }
                pub mod number {
                    use super::runtime_types;
                    pub type Number = ::core::primitive::u32;
                }
                pub mod parent_hash {
                    use super::runtime_types;
                    pub type ParentHash = ::subxt::ext::subxt_core::utils::H256;
                }
                pub mod digest {
                    use super::runtime_types;
                    pub type Digest = runtime_types::sp_runtime::generic::digest::Digest;
                }
                pub mod events {
                    use super::runtime_types;
                    pub type Events = ::subxt::ext::subxt_core::alloc::vec::Vec<
                        runtime_types::frame_system::EventRecord<
                            runtime_types::midnight_node_runtime::RuntimeEvent,
                            ::subxt::ext::subxt_core::utils::H256,
                        >,
                    >;
                }
                pub mod event_count {
                    use super::runtime_types;
                    pub type EventCount = ::core::primitive::u32;
                }
                pub mod event_topics {
                    use super::runtime_types;
                    pub type EventTopics = ::subxt::ext::subxt_core::alloc::vec::Vec<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::H256;
                }
                pub mod last_runtime_upgrade {
                    use super::runtime_types;
                    pub type LastRuntimeUpgrade =
                        runtime_types::frame_system::LastRuntimeUpgradeInfo;
                }
                pub mod upgraded_to_u32_ref_count {
                    use super::runtime_types;
                    pub type UpgradedToU32RefCount = ::core::primitive::bool;
                }
                pub mod upgraded_to_triple_ref_count {
                    use super::runtime_types;
                    pub type UpgradedToTripleRefCount = ::core::primitive::bool;
                }
                pub mod execution_phase {
                    use super::runtime_types;
                    pub type ExecutionPhase = runtime_types::frame_system::Phase;
                }
                pub mod authorized_upgrade {
                    use super::runtime_types;
                    pub type AuthorizedUpgrade =
                        runtime_types::frame_system::CodeUpgradeAuthorization;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The full account information for a particular account ID."]
                pub fn account_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::account::Account,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "Account",
                        (),
                        [
                            14u8, 233u8, 115u8, 214u8, 0u8, 109u8, 222u8, 121u8, 162u8, 65u8, 60u8,
                            175u8, 209u8, 79u8, 222u8, 124u8, 22u8, 235u8, 138u8, 176u8, 133u8,
                            124u8, 90u8, 158u8, 85u8, 45u8, 37u8, 174u8, 47u8, 79u8, 47u8, 166u8,
                        ],
                    )
                }
                #[doc = " The full account information for a particular account ID."]
                pub fn account(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::account::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::account::Param0,
                    >,
                    types::account::Account,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "Account",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            14u8, 233u8, 115u8, 214u8, 0u8, 109u8, 222u8, 121u8, 162u8, 65u8, 60u8,
                            175u8, 209u8, 79u8, 222u8, 124u8, 22u8, 235u8, 138u8, 176u8, 133u8,
                            124u8, 90u8, 158u8, 85u8, 45u8, 37u8, 174u8, 47u8, 79u8, 47u8, 166u8,
                        ],
                    )
                }
                #[doc = " Total extrinsics count for the current block."]
                pub fn extrinsic_count(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::extrinsic_count::ExtrinsicCount,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "ExtrinsicCount",
                        (),
                        [
                            102u8, 76u8, 236u8, 42u8, 40u8, 231u8, 33u8, 222u8, 123u8, 147u8,
                            153u8, 148u8, 234u8, 203u8, 181u8, 119u8, 6u8, 187u8, 177u8, 199u8,
                            120u8, 47u8, 137u8, 254u8, 96u8, 100u8, 165u8, 182u8, 249u8, 230u8,
                            159u8, 79u8,
                        ],
                    )
                }
                #[doc = " Whether all inherents have been applied."]
                pub fn inherents_applied(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::inherents_applied::InherentsApplied,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "InherentsApplied",
                        (),
                        [
                            132u8, 249u8, 142u8, 252u8, 8u8, 103u8, 80u8, 120u8, 50u8, 6u8, 188u8,
                            223u8, 101u8, 55u8, 165u8, 189u8, 172u8, 249u8, 165u8, 230u8, 183u8,
                            109u8, 34u8, 65u8, 185u8, 150u8, 29u8, 8u8, 186u8, 129u8, 135u8, 239u8,
                        ],
                    )
                }
                #[doc = " The current weight for the block."]
                pub fn block_weight(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::block_weight::BlockWeight,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "BlockWeight",
                        (),
                        [
                            158u8, 46u8, 228u8, 89u8, 210u8, 214u8, 84u8, 154u8, 50u8, 68u8, 63u8,
                            62u8, 43u8, 42u8, 99u8, 27u8, 54u8, 42u8, 146u8, 44u8, 241u8, 216u8,
                            229u8, 30u8, 216u8, 255u8, 165u8, 238u8, 181u8, 130u8, 36u8, 102u8,
                        ],
                    )
                }
                #[doc = " Total length (in bytes) for all extrinsics put together, for the current block."]
                pub fn all_extrinsics_len(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::all_extrinsics_len::AllExtrinsicsLen,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "AllExtrinsicsLen",
                        (),
                        [
                            117u8, 86u8, 61u8, 243u8, 41u8, 51u8, 102u8, 214u8, 137u8, 100u8,
                            243u8, 185u8, 122u8, 174u8, 187u8, 117u8, 86u8, 189u8, 63u8, 135u8,
                            101u8, 218u8, 203u8, 201u8, 237u8, 254u8, 128u8, 183u8, 169u8, 221u8,
                            242u8, 65u8,
                        ],
                    )
                }
                #[doc = " Map of block numbers to block hashes."]
                pub fn block_hash_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::block_hash::BlockHash,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "BlockHash",
                        (),
                        [
                            217u8, 32u8, 215u8, 253u8, 24u8, 182u8, 207u8, 178u8, 157u8, 24u8,
                            103u8, 100u8, 195u8, 165u8, 69u8, 152u8, 112u8, 181u8, 56u8, 192u8,
                            164u8, 16u8, 20u8, 222u8, 28u8, 214u8, 144u8, 142u8, 146u8, 69u8,
                            202u8, 118u8,
                        ],
                    )
                }
                #[doc = " Map of block numbers to block hashes."]
                pub fn block_hash(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::block_hash::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::block_hash::Param0,
                    >,
                    types::block_hash::BlockHash,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "BlockHash",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            217u8, 32u8, 215u8, 253u8, 24u8, 182u8, 207u8, 178u8, 157u8, 24u8,
                            103u8, 100u8, 195u8, 165u8, 69u8, 152u8, 112u8, 181u8, 56u8, 192u8,
                            164u8, 16u8, 20u8, 222u8, 28u8, 214u8, 144u8, 142u8, 146u8, 69u8,
                            202u8, 118u8,
                        ],
                    )
                }
                #[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
                pub fn extrinsic_data_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::extrinsic_data::ExtrinsicData,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "ExtrinsicData",
                        (),
                        [
                            160u8, 180u8, 122u8, 18u8, 196u8, 26u8, 2u8, 37u8, 115u8, 232u8, 133u8,
                            220u8, 106u8, 245u8, 4u8, 129u8, 42u8, 84u8, 241u8, 45u8, 199u8, 179u8,
                            128u8, 61u8, 170u8, 137u8, 231u8, 156u8, 247u8, 57u8, 47u8, 38u8,
                        ],
                    )
                }
                #[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
                pub fn extrinsic_data(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::extrinsic_data::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::extrinsic_data::Param0,
                    >,
                    types::extrinsic_data::ExtrinsicData,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "ExtrinsicData",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            160u8, 180u8, 122u8, 18u8, 196u8, 26u8, 2u8, 37u8, 115u8, 232u8, 133u8,
                            220u8, 106u8, 245u8, 4u8, 129u8, 42u8, 84u8, 241u8, 45u8, 199u8, 179u8,
                            128u8, 61u8, 170u8, 137u8, 231u8, 156u8, 247u8, 57u8, 47u8, 38u8,
                        ],
                    )
                }
                #[doc = " The current block number being processed. Set by `execute_block`."]
                pub fn number(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::number::Number,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "Number",
                        (),
                        [
                            30u8, 194u8, 177u8, 90u8, 194u8, 232u8, 46u8, 180u8, 85u8, 129u8, 14u8,
                            9u8, 8u8, 8u8, 23u8, 95u8, 230u8, 5u8, 13u8, 105u8, 125u8, 2u8, 22u8,
                            200u8, 78u8, 93u8, 115u8, 28u8, 150u8, 113u8, 48u8, 53u8,
                        ],
                    )
                }
                #[doc = " Hash of the previous block."]
                pub fn parent_hash(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::parent_hash::ParentHash,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "ParentHash",
                        (),
                        [
                            26u8, 130u8, 11u8, 216u8, 155u8, 71u8, 128u8, 170u8, 30u8, 153u8, 21u8,
                            192u8, 62u8, 93u8, 137u8, 80u8, 120u8, 81u8, 202u8, 94u8, 248u8, 125u8,
                            71u8, 82u8, 141u8, 229u8, 32u8, 56u8, 73u8, 50u8, 101u8, 78u8,
                        ],
                    )
                }
                #[doc = " Digest of the current block, also part of the block header."]
                pub fn digest(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::digest::Digest,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "Digest",
                        (),
                        [
                            61u8, 64u8, 237u8, 91u8, 145u8, 232u8, 17u8, 254u8, 181u8, 16u8, 234u8,
                            91u8, 51u8, 140u8, 254u8, 131u8, 98u8, 135u8, 21u8, 37u8, 251u8, 20u8,
                            58u8, 92u8, 123u8, 141u8, 14u8, 227u8, 146u8, 46u8, 222u8, 117u8,
                        ],
                    )
                }
                #[doc = " Events deposited for the current block."]
                #[doc = ""]
                #[doc = " NOTE: The item is unbound and should therefore never be read on chain."]
                #[doc = " It could otherwise inflate the PoV size of a block."]
                #[doc = ""]
                #[doc = " Events have a large in-memory size. Box the events to not go out-of-memory"]
                #[doc = " just in case someone still reads them from within the runtime."]
                pub fn events(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::events::Events,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "Events",
                        (),
                        [
                            206u8, 118u8, 98u8, 178u8, 90u8, 196u8, 154u8, 48u8, 231u8, 102u8,
                            204u8, 216u8, 25u8, 190u8, 156u8, 100u8, 19u8, 31u8, 111u8, 22u8, 70u8,
                            55u8, 30u8, 171u8, 153u8, 246u8, 6u8, 158u8, 212u8, 138u8, 30u8, 67u8,
                        ],
                    )
                }
                #[doc = " The number of events in the `Events<T>` list."]
                pub fn event_count(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::event_count::EventCount,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "EventCount",
                        (),
                        [
                            175u8, 24u8, 252u8, 184u8, 210u8, 167u8, 146u8, 143u8, 164u8, 80u8,
                            151u8, 205u8, 189u8, 189u8, 55u8, 220u8, 47u8, 101u8, 181u8, 33u8,
                            254u8, 131u8, 13u8, 143u8, 3u8, 244u8, 245u8, 45u8, 2u8, 210u8, 79u8,
                            133u8,
                        ],
                    )
                }
                #[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
                #[doc = " of events in the `<Events<T>>` list."]
                #[doc = ""]
                #[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
                #[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
                #[doc = " in case of changes fetch the list of events of interest."]
                #[doc = ""]
                #[doc = " The value has the type `(BlockNumberFor<T>, EventIndex)` because if we used only just"]
                #[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
                #[doc = " no notification will be triggered thus the event might be lost."]
                pub fn event_topics_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::event_topics::EventTopics,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "EventTopics",
                        (),
                        [
                            40u8, 225u8, 14u8, 75u8, 44u8, 176u8, 76u8, 34u8, 143u8, 107u8, 69u8,
                            133u8, 114u8, 13u8, 172u8, 250u8, 141u8, 73u8, 12u8, 65u8, 217u8, 63u8,
                            120u8, 241u8, 48u8, 106u8, 143u8, 161u8, 128u8, 100u8, 166u8, 59u8,
                        ],
                    )
                }
                #[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
                #[doc = " of events in the `<Events<T>>` list."]
                #[doc = ""]
                #[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
                #[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
                #[doc = " in case of changes fetch the list of events of interest."]
                #[doc = ""]
                #[doc = " The value has the type `(BlockNumberFor<T>, EventIndex)` because if we used only just"]
                #[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
                #[doc = " no notification will be triggered thus the event might be lost."]
                pub fn event_topics(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::event_topics::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::event_topics::Param0,
                    >,
                    types::event_topics::EventTopics,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "EventTopics",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            40u8, 225u8, 14u8, 75u8, 44u8, 176u8, 76u8, 34u8, 143u8, 107u8, 69u8,
                            133u8, 114u8, 13u8, 172u8, 250u8, 141u8, 73u8, 12u8, 65u8, 217u8, 63u8,
                            120u8, 241u8, 48u8, 106u8, 143u8, 161u8, 128u8, 100u8, 166u8, 59u8,
                        ],
                    )
                }
                #[doc = " Stores the `spec_version` and `spec_name` of when the last runtime upgrade happened."]
                pub fn last_runtime_upgrade(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::last_runtime_upgrade::LastRuntimeUpgrade,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "LastRuntimeUpgrade",
                        (),
                        [
                            137u8, 29u8, 175u8, 75u8, 197u8, 208u8, 91u8, 207u8, 156u8, 87u8,
                            148u8, 68u8, 91u8, 140u8, 22u8, 233u8, 1u8, 229u8, 56u8, 34u8, 40u8,
                            194u8, 253u8, 30u8, 163u8, 39u8, 54u8, 209u8, 13u8, 27u8, 139u8, 184u8,
                        ],
                    )
                }
                #[doc = " True if we have upgraded so that `type RefCount` is `u32`. False (default) if not."]
                pub fn upgraded_to_u32_ref_count(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::upgraded_to_u32_ref_count::UpgradedToU32RefCount,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "UpgradedToU32RefCount",
                        (),
                        [
                            229u8, 73u8, 9u8, 132u8, 186u8, 116u8, 151u8, 171u8, 145u8, 29u8, 34u8,
                            130u8, 52u8, 146u8, 124u8, 175u8, 79u8, 189u8, 147u8, 230u8, 234u8,
                            107u8, 124u8, 31u8, 2u8, 22u8, 86u8, 190u8, 4u8, 147u8, 50u8, 245u8,
                        ],
                    )
                }
                #[doc = " True if we have upgraded so that AccountInfo contains three types of `RefCount`. False"]
                #[doc = " (default) if not."]
                pub fn upgraded_to_triple_ref_count(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::upgraded_to_triple_ref_count::UpgradedToTripleRefCount,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "UpgradedToTripleRefCount",
                        (),
                        [
                            97u8, 66u8, 124u8, 243u8, 27u8, 167u8, 147u8, 81u8, 254u8, 201u8,
                            101u8, 24u8, 40u8, 231u8, 14u8, 179u8, 154u8, 163u8, 71u8, 81u8, 185u8,
                            167u8, 82u8, 254u8, 189u8, 3u8, 101u8, 207u8, 206u8, 194u8, 155u8,
                            151u8,
                        ],
                    )
                }
                #[doc = " The execution phase of the block."]
                pub fn execution_phase(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::execution_phase::ExecutionPhase,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "ExecutionPhase",
                        (),
                        [
                            191u8, 129u8, 100u8, 134u8, 126u8, 116u8, 154u8, 203u8, 220u8, 200u8,
                            0u8, 26u8, 161u8, 250u8, 133u8, 205u8, 146u8, 24u8, 5u8, 156u8, 158u8,
                            35u8, 36u8, 253u8, 52u8, 235u8, 86u8, 167u8, 35u8, 100u8, 119u8, 27u8,
                        ],
                    )
                }
                #[doc = " `Some` if a code upgrade has been authorized."]
                pub fn authorized_upgrade(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::authorized_upgrade::AuthorizedUpgrade,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "AuthorizedUpgrade",
                        (),
                        [
                            165u8, 97u8, 27u8, 138u8, 2u8, 28u8, 55u8, 92u8, 96u8, 96u8, 168u8,
                            169u8, 55u8, 178u8, 44u8, 127u8, 58u8, 140u8, 206u8, 178u8, 1u8, 37u8,
                            214u8, 213u8, 251u8, 123u8, 5u8, 111u8, 90u8, 148u8, 217u8, 135u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " Block & extrinsics weights: base values and limits."]
                pub fn block_weights(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    runtime_types::frame_system::limits::BlockWeights,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "System",
                        "BlockWeights",
                        [
                            176u8, 124u8, 225u8, 136u8, 25u8, 73u8, 247u8, 33u8, 82u8, 206u8, 85u8,
                            190u8, 127u8, 102u8, 71u8, 11u8, 185u8, 8u8, 58u8, 0u8, 94u8, 55u8,
                            163u8, 177u8, 104u8, 59u8, 60u8, 136u8, 246u8, 116u8, 0u8, 239u8,
                        ],
                    )
                }
                #[doc = " The maximum length of a block (in bytes)."]
                pub fn block_length(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    runtime_types::frame_system::limits::BlockLength,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "System",
                        "BlockLength",
                        [
                            23u8, 242u8, 225u8, 39u8, 225u8, 67u8, 152u8, 41u8, 155u8, 104u8, 68u8,
                            229u8, 185u8, 133u8, 10u8, 143u8, 184u8, 152u8, 234u8, 44u8, 140u8,
                            96u8, 166u8, 235u8, 162u8, 160u8, 72u8, 7u8, 35u8, 194u8, 3u8, 37u8,
                        ],
                    )
                }
                #[doc = " Maximum number of block number to block hash mappings to keep (oldest pruned first)."]
                pub fn block_hash_count(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u32,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "System",
                        "BlockHashCount",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The weight of runtime database operations the runtime can invoke."]
                pub fn db_weight(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    runtime_types::sp_weights::RuntimeDbWeight,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "System",
                        "DbWeight",
                        [
                            42u8, 43u8, 178u8, 142u8, 243u8, 203u8, 60u8, 173u8, 118u8, 111u8,
                            200u8, 170u8, 102u8, 70u8, 237u8, 187u8, 198u8, 120u8, 153u8, 232u8,
                            183u8, 76u8, 74u8, 10u8, 70u8, 243u8, 14u8, 218u8, 213u8, 126u8, 29u8,
                            177u8,
                        ],
                    )
                }
                #[doc = " Get the chain's in-code version."]
                pub fn version(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    runtime_types::sp_version::RuntimeVersion,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "System",
                        "Version",
                        [
                            219u8, 45u8, 162u8, 245u8, 177u8, 246u8, 48u8, 126u8, 191u8, 157u8,
                            228u8, 83u8, 111u8, 133u8, 183u8, 13u8, 148u8, 108u8, 92u8, 102u8,
                            72u8, 205u8, 74u8, 242u8, 233u8, 79u8, 20u8, 170u8, 72u8, 202u8, 158u8,
                            165u8,
                        ],
                    )
                }
                #[doc = " The designated SS58 prefix of this chain."]
                #[doc = ""]
                #[doc = " This replaces the \"ss58Format\" property declared in the chain spec. Reason is"]
                #[doc = " that the runtime should know about the prefix in order to make use of it as"]
                #[doc = " an identifier of the chain."]
                pub fn ss58_prefix(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u16,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "System",
                        "SS58Prefix",
                        [
                            116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8, 169u8, 167u8, 227u8,
                            41u8, 144u8, 11u8, 236u8, 82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8,
                            90u8, 208u8, 135u8, 15u8, 117u8, 10u8, 123u8, 128u8, 193u8, 29u8, 70u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod timestamp {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::pallet_timestamp::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Set the current time."]
                #[doc = ""]
                #[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
                #[doc = "phase, if this call hasn't been invoked by that time."]
                #[doc = ""]
                #[doc = "The timestamp should be greater than the previous one by the amount specified by"]
                #[doc = "[`Config::MinimumPeriod`]."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _None_."]
                #[doc = ""]
                #[doc = "This dispatch class is _Mandatory_ to ensure it gets executed in the block. Be aware"]
                #[doc = "that changing the complexity of this call could result exhausting the resources in a"]
                #[doc = "block to execute any other calls."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
                #[doc = "- 1 storage read and 1 storage mutation (codec `O(1)` because of `DidUpdate::take` in"]
                #[doc = "  `on_finalize`)"]
                #[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
                pub struct Set {
                    #[codec(compact)]
                    pub now: set::Now,
                }
                pub mod set {
                    use super::runtime_types;
                    pub type Now = ::core::primitive::u64;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for Set {
                    const PALLET: &'static str = "Timestamp";
                    const CALL: &'static str = "set";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Set the current time."]
                #[doc = ""]
                #[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
                #[doc = "phase, if this call hasn't been invoked by that time."]
                #[doc = ""]
                #[doc = "The timestamp should be greater than the previous one by the amount specified by"]
                #[doc = "[`Config::MinimumPeriod`]."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _None_."]
                #[doc = ""]
                #[doc = "This dispatch class is _Mandatory_ to ensure it gets executed in the block. Be aware"]
                #[doc = "that changing the complexity of this call could result exhausting the resources in a"]
                #[doc = "block to execute any other calls."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
                #[doc = "- 1 storage read and 1 storage mutation (codec `O(1)` because of `DidUpdate::take` in"]
                #[doc = "  `on_finalize`)"]
                #[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
                pub fn set(
                    &self,
                    now: types::set::Now,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::Set>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Timestamp",
                        "set",
                        types::Set { now },
                        [
                            37u8, 95u8, 49u8, 218u8, 24u8, 22u8, 0u8, 95u8, 72u8, 35u8, 155u8,
                            199u8, 213u8, 54u8, 207u8, 22u8, 185u8, 193u8, 221u8, 70u8, 18u8,
                            200u8, 4u8, 231u8, 195u8, 173u8, 6u8, 122u8, 11u8, 203u8, 231u8, 227u8,
                        ],
                    )
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod now {
                    use super::runtime_types;
                    pub type Now = ::core::primitive::u64;
                }
                pub mod did_update {
                    use super::runtime_types;
                    pub type DidUpdate = ::core::primitive::bool;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The current time for the current block."]
                pub fn now(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::now::Now,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Timestamp",
                        "Now",
                        (),
                        [
                            44u8, 50u8, 80u8, 30u8, 195u8, 146u8, 123u8, 238u8, 8u8, 163u8, 187u8,
                            92u8, 61u8, 39u8, 51u8, 29u8, 173u8, 169u8, 217u8, 158u8, 85u8, 187u8,
                            141u8, 26u8, 12u8, 115u8, 51u8, 11u8, 200u8, 244u8, 138u8, 152u8,
                        ],
                    )
                }
                #[doc = " Whether the timestamp has been updated in this block."]
                #[doc = ""]
                #[doc = " This value is updated to `true` upon successful submission of a timestamp by a node."]
                #[doc = " It is then checked at the end of each block execution in the `on_finalize` hook."]
                pub fn did_update(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::did_update::DidUpdate,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Timestamp",
                        "DidUpdate",
                        (),
                        [
                            229u8, 175u8, 246u8, 102u8, 237u8, 158u8, 212u8, 229u8, 238u8, 214u8,
                            205u8, 160u8, 164u8, 252u8, 195u8, 75u8, 139u8, 110u8, 22u8, 34u8,
                            248u8, 204u8, 107u8, 46u8, 20u8, 200u8, 238u8, 167u8, 71u8, 41u8,
                            214u8, 140u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The minimum period between blocks."]
                #[doc = ""]
                #[doc = " Be aware that this is different to the *expected* period that the block production"]
                #[doc = " apparatus provides. Your chosen consensus system will generally work with this to"]
                #[doc = " determine a sensible block time. For example, in the Aura pallet it will be double this"]
                #[doc = " period on default settings."]
                pub fn minimum_period(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u64,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Timestamp",
                        "MinimumPeriod",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod aura {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod authorities {
                    use super::runtime_types;
                    pub type Authorities =
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
                        >;
                }
                pub mod current_slot {
                    use super::runtime_types;
                    pub type CurrentSlot = runtime_types::sp_consensus_slots::Slot;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The current authority set."]
                pub fn authorities(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::authorities::Authorities,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Aura",
                        "Authorities",
                        (),
                        [
                            232u8, 129u8, 167u8, 104u8, 47u8, 188u8, 238u8, 164u8, 6u8, 29u8,
                            129u8, 45u8, 64u8, 182u8, 194u8, 47u8, 0u8, 73u8, 63u8, 102u8, 204u8,
                            94u8, 111u8, 96u8, 137u8, 7u8, 141u8, 110u8, 180u8, 80u8, 228u8, 16u8,
                        ],
                    )
                }
                #[doc = " The current slot of this block."]
                #[doc = ""]
                #[doc = " This will be set in `on_initialize`."]
                pub fn current_slot(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::current_slot::CurrentSlot,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Aura",
                        "CurrentSlot",
                        (),
                        [
                            112u8, 199u8, 115u8, 248u8, 217u8, 242u8, 45u8, 231u8, 178u8, 53u8,
                            236u8, 167u8, 219u8, 238u8, 81u8, 243u8, 39u8, 140u8, 68u8, 19u8,
                            201u8, 169u8, 211u8, 133u8, 135u8, 213u8, 150u8, 105u8, 60u8, 252u8,
                            43u8, 57u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod grandpa {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "The `Error` enum of this pallet."]
        pub type Error = runtime_types::pallet_grandpa::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::pallet_grandpa::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
                #[doc = "equivocation proof and validate the given key ownership proof"]
                #[doc = "against the extracted offender. If both are valid, the offence"]
                #[doc = "will be reported."]
                pub struct ReportEquivocation {
                    pub equivocation_proof: ::subxt::ext::subxt_core::alloc::boxed::Box<
                        report_equivocation::EquivocationProof,
                    >,
                    pub key_owner_proof: report_equivocation::KeyOwnerProof,
                }
                pub mod report_equivocation {
                    use super::runtime_types;
                    pub type EquivocationProof =
                        runtime_types::sp_consensus_grandpa::EquivocationProof<
                            ::subxt::ext::subxt_core::utils::H256,
                            ::core::primitive::u32,
                        >;
                    pub type KeyOwnerProof = runtime_types::sp_core::Void;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ReportEquivocation {
                    const PALLET: &'static str = "Grandpa";
                    const CALL: &'static str = "report_equivocation";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
                #[doc = "equivocation proof and validate the given key ownership proof"]
                #[doc = "against the extracted offender. If both are valid, the offence"]
                #[doc = "will be reported."]
                #[doc = ""]
                #[doc = "This extrinsic must be called unsigned and it is expected that only"]
                #[doc = "block authors will call it (validated in `ValidateUnsigned`), as such"]
                #[doc = "if the block author is defined it will be defined as the equivocation"]
                #[doc = "reporter."]
                pub struct ReportEquivocationUnsigned {
                    pub equivocation_proof: ::subxt::ext::subxt_core::alloc::boxed::Box<
                        report_equivocation_unsigned::EquivocationProof,
                    >,
                    pub key_owner_proof: report_equivocation_unsigned::KeyOwnerProof,
                }
                pub mod report_equivocation_unsigned {
                    use super::runtime_types;
                    pub type EquivocationProof =
                        runtime_types::sp_consensus_grandpa::EquivocationProof<
                            ::subxt::ext::subxt_core::utils::H256,
                            ::core::primitive::u32,
                        >;
                    pub type KeyOwnerProof = runtime_types::sp_core::Void;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ReportEquivocationUnsigned {
                    const PALLET: &'static str = "Grandpa";
                    const CALL: &'static str = "report_equivocation_unsigned";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Note that the current authority set of the GRANDPA finality gadget has stalled."]
                #[doc = ""]
                #[doc = "This will trigger a forced authority set change at the beginning of the next session, to"]
                #[doc = "be enacted `delay` blocks after that. The `delay` should be high enough to safely assume"]
                #[doc = "that the block signalling the forced change will not be re-orged e.g. 1000 blocks."]
                #[doc = "The block production rate (which may be slowed down because of finality lagging) should"]
                #[doc = "be taken into account when choosing the `delay`. The GRANDPA voters based on the new"]
                #[doc = "authority will start voting on top of `best_finalized_block_number` for new finalized"]
                #[doc = "blocks. `best_finalized_block_number` should be the highest of the latest finalized"]
                #[doc = "block of all validators of the new authority set."]
                #[doc = ""]
                #[doc = "Only callable by root."]
                pub struct NoteStalled {
                    pub delay: note_stalled::Delay,
                    pub best_finalized_block_number: note_stalled::BestFinalizedBlockNumber,
                }
                pub mod note_stalled {
                    use super::runtime_types;
                    pub type Delay = ::core::primitive::u32;
                    pub type BestFinalizedBlockNumber = ::core::primitive::u32;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for NoteStalled {
                    const PALLET: &'static str = "Grandpa";
                    const CALL: &'static str = "note_stalled";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
                #[doc = "equivocation proof and validate the given key ownership proof"]
                #[doc = "against the extracted offender. If both are valid, the offence"]
                #[doc = "will be reported."]
                pub fn report_equivocation(
                    &self,
                    equivocation_proof: types::report_equivocation::EquivocationProof,
                    key_owner_proof: types::report_equivocation::KeyOwnerProof,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ReportEquivocation>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Grandpa",
                        "report_equivocation",
                        types::ReportEquivocation {
                            equivocation_proof: ::subxt::ext::subxt_core::alloc::boxed::Box::new(
                                equivocation_proof,
                            ),
                            key_owner_proof,
                        },
                        [
                            158u8, 70u8, 189u8, 51u8, 231u8, 191u8, 199u8, 33u8, 64u8, 156u8, 71u8,
                            243u8, 122u8, 199u8, 216u8, 10u8, 45u8, 73u8, 198u8, 141u8, 31u8,
                            209u8, 58u8, 164u8, 219u8, 124u8, 242u8, 26u8, 114u8, 52u8, 65u8,
                            106u8,
                        ],
                    )
                }
                #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
                #[doc = "equivocation proof and validate the given key ownership proof"]
                #[doc = "against the extracted offender. If both are valid, the offence"]
                #[doc = "will be reported."]
                #[doc = ""]
                #[doc = "This extrinsic must be called unsigned and it is expected that only"]
                #[doc = "block authors will call it (validated in `ValidateUnsigned`), as such"]
                #[doc = "if the block author is defined it will be defined as the equivocation"]
                #[doc = "reporter."]
                pub fn report_equivocation_unsigned(
                    &self,
                    equivocation_proof: types::report_equivocation_unsigned::EquivocationProof,
                    key_owner_proof: types::report_equivocation_unsigned::KeyOwnerProof,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<
                    types::ReportEquivocationUnsigned,
                > {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Grandpa",
                        "report_equivocation_unsigned",
                        types::ReportEquivocationUnsigned {
                            equivocation_proof: ::subxt::ext::subxt_core::alloc::boxed::Box::new(
                                equivocation_proof,
                            ),
                            key_owner_proof,
                        },
                        [
                            53u8, 23u8, 255u8, 215u8, 105u8, 11u8, 67u8, 177u8, 234u8, 248u8,
                            183u8, 57u8, 230u8, 239u8, 54u8, 238u8, 115u8, 170u8, 153u8, 18u8,
                            55u8, 195u8, 85u8, 98u8, 109u8, 194u8, 57u8, 225u8, 139u8, 237u8,
                            171u8, 152u8,
                        ],
                    )
                }
                #[doc = "Note that the current authority set of the GRANDPA finality gadget has stalled."]
                #[doc = ""]
                #[doc = "This will trigger a forced authority set change at the beginning of the next session, to"]
                #[doc = "be enacted `delay` blocks after that. The `delay` should be high enough to safely assume"]
                #[doc = "that the block signalling the forced change will not be re-orged e.g. 1000 blocks."]
                #[doc = "The block production rate (which may be slowed down because of finality lagging) should"]
                #[doc = "be taken into account when choosing the `delay`. The GRANDPA voters based on the new"]
                #[doc = "authority will start voting on top of `best_finalized_block_number` for new finalized"]
                #[doc = "blocks. `best_finalized_block_number` should be the highest of the latest finalized"]
                #[doc = "block of all validators of the new authority set."]
                #[doc = ""]
                #[doc = "Only callable by root."]
                pub fn note_stalled(
                    &self,
                    delay: types::note_stalled::Delay,
                    best_finalized_block_number: types::note_stalled::BestFinalizedBlockNumber,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::NoteStalled>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Grandpa",
                        "note_stalled",
                        types::NoteStalled {
                            delay,
                            best_finalized_block_number,
                        },
                        [
                            158u8, 25u8, 64u8, 114u8, 131u8, 139u8, 227u8, 132u8, 42u8, 107u8,
                            40u8, 249u8, 18u8, 93u8, 254u8, 86u8, 37u8, 67u8, 250u8, 35u8, 241u8,
                            194u8, 209u8, 20u8, 39u8, 75u8, 186u8, 21u8, 48u8, 124u8, 151u8, 31u8,
                        ],
                    )
                }
            }
        }
        #[doc = "The `Event` enum of this pallet"]
        pub type Event = runtime_types::pallet_grandpa::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "New authority set has been applied."]
            pub struct NewAuthorities {
                pub authority_set: new_authorities::AuthoritySet,
            }
            pub mod new_authorities {
                use super::runtime_types;
                pub type AuthoritySet = ::subxt::ext::subxt_core::alloc::vec::Vec<(
                    runtime_types::sp_consensus_grandpa::app::Public,
                    ::core::primitive::u64,
                )>;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for NewAuthorities {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "NewAuthorities";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Current authority set has been paused."]
            pub struct Paused;
            impl ::subxt::ext::subxt_core::events::StaticEvent for Paused {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "Paused";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Current authority set has been resumed."]
            pub struct Resumed;
            impl ::subxt::ext::subxt_core::events::StaticEvent for Resumed {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "Resumed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod state {
                    use super::runtime_types;
                    pub type State =
                        runtime_types::pallet_grandpa::StoredState<::core::primitive::u32>;
                }
                pub mod pending_change {
                    use super::runtime_types;
                    pub type PendingChange =
                        runtime_types::pallet_grandpa::StoredPendingChange<::core::primitive::u32>;
                }
                pub mod next_forced {
                    use super::runtime_types;
                    pub type NextForced = ::core::primitive::u32;
                }
                pub mod stalled {
                    use super::runtime_types;
                    pub type Stalled = (::core::primitive::u32, ::core::primitive::u32);
                }
                pub mod current_set_id {
                    use super::runtime_types;
                    pub type CurrentSetId = ::core::primitive::u64;
                }
                pub mod set_id_session {
                    use super::runtime_types;
                    pub type SetIdSession = ::core::primitive::u32;
                    pub type Param0 = ::core::primitive::u64;
                }
                pub mod authorities {
                    use super::runtime_types;
                    pub type Authorities =
                        runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<(
                            runtime_types::sp_consensus_grandpa::app::Public,
                            ::core::primitive::u64,
                        )>;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " State of the current authority set."]
                pub fn state(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::state::State,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Grandpa",
                        "State",
                        (),
                        [
                            73u8, 71u8, 112u8, 83u8, 238u8, 75u8, 44u8, 9u8, 180u8, 33u8, 30u8,
                            121u8, 98u8, 96u8, 61u8, 133u8, 16u8, 70u8, 30u8, 249u8, 34u8, 148u8,
                            15u8, 239u8, 164u8, 157u8, 52u8, 27u8, 144u8, 52u8, 223u8, 109u8,
                        ],
                    )
                }
                #[doc = " Pending change: (signaled at, scheduled change)."]
                pub fn pending_change(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::pending_change::PendingChange,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Grandpa",
                        "PendingChange",
                        (),
                        [
                            150u8, 194u8, 185u8, 248u8, 239u8, 43u8, 141u8, 253u8, 61u8, 106u8,
                            74u8, 164u8, 209u8, 204u8, 206u8, 200u8, 32u8, 38u8, 11u8, 78u8, 84u8,
                            243u8, 181u8, 142u8, 179u8, 151u8, 81u8, 204u8, 244u8, 150u8, 137u8,
                            250u8,
                        ],
                    )
                }
                #[doc = " next block number where we can force a change."]
                pub fn next_forced(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::next_forced::NextForced,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Grandpa",
                        "NextForced",
                        (),
                        [
                            3u8, 231u8, 56u8, 18u8, 87u8, 112u8, 227u8, 126u8, 180u8, 131u8, 255u8,
                            141u8, 82u8, 34u8, 61u8, 47u8, 234u8, 37u8, 95u8, 62u8, 33u8, 235u8,
                            231u8, 122u8, 125u8, 8u8, 223u8, 95u8, 255u8, 204u8, 40u8, 97u8,
                        ],
                    )
                }
                #[doc = " `true` if we are currently stalled."]
                pub fn stalled(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::stalled::Stalled,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Grandpa",
                        "Stalled",
                        (),
                        [
                            6u8, 81u8, 205u8, 142u8, 195u8, 48u8, 0u8, 247u8, 108u8, 170u8, 10u8,
                            249u8, 72u8, 206u8, 32u8, 103u8, 109u8, 57u8, 51u8, 21u8, 144u8, 204u8,
                            79u8, 8u8, 191u8, 185u8, 38u8, 34u8, 118u8, 223u8, 75u8, 241u8,
                        ],
                    )
                }
                #[doc = " The number of changes (both in terms of keys and underlying economic responsibilities)"]
                #[doc = " in the \"set\" of Grandpa validators from genesis."]
                pub fn current_set_id(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::current_set_id::CurrentSetId,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Grandpa",
                        "CurrentSetId",
                        (),
                        [
                            234u8, 215u8, 218u8, 42u8, 30u8, 76u8, 129u8, 40u8, 125u8, 137u8,
                            207u8, 47u8, 46u8, 213u8, 159u8, 50u8, 175u8, 81u8, 155u8, 123u8,
                            246u8, 175u8, 156u8, 68u8, 22u8, 113u8, 135u8, 137u8, 163u8, 18u8,
                            115u8, 73u8,
                        ],
                    )
                }
                #[doc = " A mapping from grandpa set ID to the index of the *most recent* session for which its"]
                #[doc = " members were responsible."]
                #[doc = ""]
                #[doc = " This is only used for validating equivocation proofs. An equivocation proof must"]
                #[doc = " contains a key-ownership proof for a given session, therefore we need a way to tie"]
                #[doc = " together sessions and GRANDPA set ids, i.e. we need to validate that a validator"]
                #[doc = " was the owner of a given key on a given session, and what the active set ID was"]
                #[doc = " during that session."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: `SetId` is not under user control."]
                pub fn set_id_session_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::set_id_session::SetIdSession,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Grandpa",
                        "SetIdSession",
                        (),
                        [
                            47u8, 0u8, 239u8, 121u8, 187u8, 213u8, 254u8, 50u8, 238u8, 10u8, 162u8,
                            65u8, 189u8, 166u8, 37u8, 74u8, 82u8, 81u8, 160u8, 20u8, 180u8, 253u8,
                            238u8, 18u8, 209u8, 203u8, 38u8, 148u8, 16u8, 105u8, 72u8, 169u8,
                        ],
                    )
                }
                #[doc = " A mapping from grandpa set ID to the index of the *most recent* session for which its"]
                #[doc = " members were responsible."]
                #[doc = ""]
                #[doc = " This is only used for validating equivocation proofs. An equivocation proof must"]
                #[doc = " contains a key-ownership proof for a given session, therefore we need a way to tie"]
                #[doc = " together sessions and GRANDPA set ids, i.e. we need to validate that a validator"]
                #[doc = " was the owner of a given key on a given session, and what the active set ID was"]
                #[doc = " during that session."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: `SetId` is not under user control."]
                pub fn set_id_session(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::set_id_session::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::set_id_session::Param0,
                    >,
                    types::set_id_session::SetIdSession,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Grandpa",
                        "SetIdSession",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            47u8, 0u8, 239u8, 121u8, 187u8, 213u8, 254u8, 50u8, 238u8, 10u8, 162u8,
                            65u8, 189u8, 166u8, 37u8, 74u8, 82u8, 81u8, 160u8, 20u8, 180u8, 253u8,
                            238u8, 18u8, 209u8, 203u8, 38u8, 148u8, 16u8, 105u8, 72u8, 169u8,
                        ],
                    )
                }
                #[doc = " The current list of authorities."]
                pub fn authorities(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::authorities::Authorities,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Grandpa",
                        "Authorities",
                        (),
                        [
                            67u8, 196u8, 244u8, 13u8, 246u8, 245u8, 198u8, 98u8, 81u8, 55u8, 182u8,
                            187u8, 214u8, 5u8, 181u8, 76u8, 251u8, 213u8, 144u8, 166u8, 36u8,
                            153u8, 234u8, 181u8, 252u8, 55u8, 198u8, 175u8, 55u8, 211u8, 105u8,
                            85u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " Max Authorities in use"]
                pub fn max_authorities(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u32,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Grandpa",
                        "MaxAuthorities",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum number of nominators for each validator."]
                pub fn max_nominators(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u32,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Grandpa",
                        "MaxNominators",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum number of entries to keep in the set id to session index mapping."]
                #[doc = ""]
                #[doc = " Since the `SetIdSession` map is only used for validating equivocations this"]
                #[doc = " value should relate to the bonding duration of whatever staking system is"]
                #[doc = " being used (if any). If equivocation handling is not enabled then this value"]
                #[doc = " can be zero."]
                pub fn max_set_id_session_entries(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u64,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Grandpa",
                        "MaxSetIdSessionEntries",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod sidechain {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod epoch_number {
                    use super::runtime_types;
                    pub type EpochNumber = runtime_types::sidechain_domain::ScEpochNumber;
                }
                pub mod chain_id {
                    use super::runtime_types;
                    pub type ChainId = ::core::primitive::u8;
                }
                pub mod threshold_numerator {
                    use super::runtime_types;
                    pub type ThresholdNumerator = ::core::primitive::u64;
                }
                pub mod threshold_denominator {
                    use super::runtime_types;
                    pub type ThresholdDenominator = ::core::primitive::u64;
                }
                pub mod genesis_committee_utxo {
                    use super::runtime_types;
                    pub type GenesisCommitteeUtxo = runtime_types::sidechain_domain::UtxoId;
                }
                pub mod governance_authority {
                    use super::runtime_types;
                    pub type GovernanceAuthority =
                        runtime_types::sidechain_domain::MainchainAddressHash;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn epoch_number(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::epoch_number::EpochNumber,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Sidechain",
                        "EpochNumber",
                        (),
                        [
                            169u8, 245u8, 25u8, 130u8, 0u8, 133u8, 35u8, 2u8, 179u8, 121u8, 41u8,
                            231u8, 187u8, 155u8, 223u8, 210u8, 120u8, 202u8, 152u8, 24u8, 138u8,
                            99u8, 163u8, 96u8, 107u8, 56u8, 234u8, 163u8, 7u8, 108u8, 89u8, 6u8,
                        ],
                    )
                }
                pub fn chain_id(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::chain_id::ChainId,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Sidechain",
                        "ChainId",
                        (),
                        [
                            142u8, 61u8, 228u8, 19u8, 201u8, 47u8, 210u8, 49u8, 79u8, 80u8, 163u8,
                            30u8, 55u8, 213u8, 215u8, 172u8, 128u8, 88u8, 249u8, 213u8, 101u8,
                            27u8, 254u8, 231u8, 28u8, 37u8, 64u8, 94u8, 89u8, 240u8, 29u8, 219u8,
                        ],
                    )
                }
                pub fn threshold_numerator(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::threshold_numerator::ThresholdNumerator,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Sidechain",
                        "ThresholdNumerator",
                        (),
                        [
                            141u8, 230u8, 0u8, 50u8, 32u8, 201u8, 235u8, 201u8, 153u8, 144u8,
                            179u8, 47u8, 235u8, 21u8, 146u8, 180u8, 124u8, 65u8, 31u8, 14u8, 102u8,
                            140u8, 41u8, 10u8, 141u8, 170u8, 172u8, 87u8, 145u8, 112u8, 9u8, 177u8,
                        ],
                    )
                }
                pub fn threshold_denominator(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::threshold_denominator::ThresholdDenominator,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Sidechain",
                        "ThresholdDenominator",
                        (),
                        [
                            211u8, 214u8, 173u8, 197u8, 185u8, 40u8, 72u8, 74u8, 120u8, 73u8,
                            179u8, 64u8, 148u8, 114u8, 182u8, 46u8, 9u8, 45u8, 57u8, 235u8, 23u8,
                            205u8, 7u8, 12u8, 54u8, 87u8, 240u8, 114u8, 66u8, 148u8, 216u8, 144u8,
                        ],
                    )
                }
                pub fn genesis_committee_utxo(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::genesis_committee_utxo::GenesisCommitteeUtxo,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Sidechain",
                        "GenesisCommitteeUtxo",
                        (),
                        [
                            75u8, 180u8, 138u8, 57u8, 51u8, 41u8, 209u8, 118u8, 26u8, 188u8, 190u8,
                            46u8, 50u8, 166u8, 80u8, 2u8, 249u8, 229u8, 174u8, 53u8, 231u8, 135u8,
                            173u8, 251u8, 113u8, 221u8, 172u8, 213u8, 60u8, 10u8, 99u8, 61u8,
                        ],
                    )
                }
                #[doc = " Main chain address hash of the Governance Authority."]
                #[doc = " With current trustless-sidechain it should never change."]
                pub fn governance_authority(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::governance_authority::GovernanceAuthority,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Sidechain",
                        "GovernanceAuthority",
                        (),
                        [
                            216u8, 130u8, 90u8, 100u8, 115u8, 124u8, 239u8, 47u8, 32u8, 223u8,
                            191u8, 149u8, 16u8, 105u8, 8u8, 244u8, 150u8, 17u8, 54u8, 29u8, 171u8,
                            133u8, 20u8, 188u8, 152u8, 94u8, 25u8, 34u8, 207u8, 83u8, 227u8, 34u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod midnight {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "The `Error` enum of this pallet."]
        pub type Error = runtime_types::pallet_midnight::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::pallet_midnight::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct SendMnTransaction {
                    pub midnight_tx: send_mn_transaction::MidnightTx,
                }
                pub mod send_mn_transaction {
                    use super::runtime_types;
                    pub type MidnightTx =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SendMnTransaction {
                    const PALLET: &'static str = "Midnight";
                    const CALL: &'static str = "send_mn_transaction";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                pub fn send_mn_transaction(
                    &self,
                    midnight_tx: types::send_mn_transaction::MidnightTx,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SendMnTransaction>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Midnight",
                        "send_mn_transaction",
                        types::SendMnTransaction { midnight_tx },
                        [
                            30u8, 219u8, 177u8, 72u8, 75u8, 62u8, 191u8, 249u8, 44u8, 133u8, 160u8,
                            164u8, 141u8, 99u8, 152u8, 207u8, 255u8, 213u8, 245u8, 6u8, 251u8,
                            99u8, 224u8, 203u8, 63u8, 66u8, 61u8, 50u8, 138u8, 90u8, 180u8, 34u8,
                        ],
                    )
                }
            }
        }
        #[doc = "The `Event` enum of this pallet"]
        pub type Event = runtime_types::pallet_midnight::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "A contract was called."]
            pub struct ContractCall(pub contract_call::Field0);
            pub mod contract_call {
                use super::runtime_types;
                pub type Field0 = runtime_types::pallet_midnight::pallet::CallDetails;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for ContractCall {
                const PALLET: &'static str = "Midnight";
                const EVENT: &'static str = "ContractCall";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "A contract has been deployed."]
            pub struct ContractDeploy(pub contract_deploy::Field0);
            pub mod contract_deploy {
                use super::runtime_types;
                pub type Field0 = runtime_types::pallet_midnight::pallet::DeploymentDetails;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for ContractDeploy {
                const PALLET: &'static str = "Midnight";
                const EVENT: &'static str = "ContractDeploy";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "A transaction has been applied (both the guaranteed and conditional part)."]
            pub struct TxApplied(pub tx_applied::Field0);
            pub mod tx_applied {
                use super::runtime_types;
                pub type Field0 = runtime_types::pallet_midnight::pallet::TxAppliedDetails;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for TxApplied {
                const PALLET: &'static str = "Midnight";
                const EVENT: &'static str = "TxApplied";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Only guaranteed transactions have been applied."]
            pub struct TxOnlyGuaranteedApplied(pub tx_only_guaranteed_applied::Field0);
            pub mod tx_only_guaranteed_applied {
                use super::runtime_types;
                pub type Field0 = runtime_types::pallet_midnight::pallet::TxAppliedDetails;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for TxOnlyGuaranteedApplied {
                const PALLET: &'static str = "Midnight";
                const EVENT: &'static str = "TxOnlyGuaranteedApplied";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod state {
                    use super::runtime_types;
                    pub type State = runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >;
                }
                pub mod network_id {
                    use super::runtime_types;
                    pub type NetworkId =
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn state(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::state::State,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Midnight",
                        "State",
                        (),
                        [
                            108u8, 162u8, 237u8, 105u8, 46u8, 248u8, 244u8, 80u8, 91u8, 109u8,
                            17u8, 191u8, 80u8, 184u8, 104u8, 117u8, 75u8, 141u8, 229u8, 197u8,
                            186u8, 91u8, 138u8, 191u8, 234u8, 230u8, 110u8, 205u8, 143u8, 180u8,
                            66u8, 176u8,
                        ],
                    )
                }
                pub fn network_id(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::network_id::NetworkId,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Midnight",
                        "NetworkId",
                        (),
                        [
                            96u8, 119u8, 161u8, 231u8, 43u8, 41u8, 144u8, 248u8, 254u8, 246u8,
                            14u8, 98u8, 57u8, 202u8, 188u8, 174u8, 158u8, 229u8, 53u8, 29u8, 125u8,
                            202u8, 240u8, 57u8, 28u8, 171u8, 167u8, 30u8, 138u8, 210u8, 75u8, 86u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod balances {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "The `Error` enum of this pallet."]
        pub type Error = runtime_types::pallet_balances::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::pallet_balances::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Transfer some liquid free balance to another account."]
                #[doc = ""]
                #[doc = "`transfer_allow_death` will set the `FreeBalance` of the sender and receiver."]
                #[doc = "If the sender's account is below the existential deposit as a result"]
                #[doc = "of the transfer, the account will be reaped."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
                pub struct TransferAllowDeath {
                    pub dest: transfer_allow_death::Dest,
                    #[codec(compact)]
                    pub value: transfer_allow_death::Value,
                }
                pub mod transfer_allow_death {
                    use super::runtime_types;
                    pub type Dest = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type Value = ::core::primitive::u128;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for TransferAllowDeath {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "transfer_allow_death";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Exactly as `transfer_allow_death`, except the origin must be root and the source account"]
                #[doc = "may be specified."]
                pub struct ForceTransfer {
                    pub source: force_transfer::Source,
                    pub dest: force_transfer::Dest,
                    #[codec(compact)]
                    pub value: force_transfer::Value,
                }
                pub mod force_transfer {
                    use super::runtime_types;
                    pub type Source = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type Dest = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type Value = ::core::primitive::u128;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ForceTransfer {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "force_transfer";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Same as the [`transfer_allow_death`] call, but with a check that the transfer will not"]
                #[doc = "kill the origin account."]
                #[doc = ""]
                #[doc = "99% of the time you want [`transfer_allow_death`] instead."]
                #[doc = ""]
                #[doc = "[`transfer_allow_death`]: struct.Pallet.html#method.transfer"]
                pub struct TransferKeepAlive {
                    pub dest: transfer_keep_alive::Dest,
                    #[codec(compact)]
                    pub value: transfer_keep_alive::Value,
                }
                pub mod transfer_keep_alive {
                    use super::runtime_types;
                    pub type Dest = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type Value = ::core::primitive::u128;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for TransferKeepAlive {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "transfer_keep_alive";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Transfer the entire transferable balance from the caller account."]
                #[doc = ""]
                #[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
                #[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
                #[doc = "transferred by this function. To ensure that this function results in a killed account,"]
                #[doc = "you might need to prepare the account by removing any reference counters, storage"]
                #[doc = "deposits, etc..."]
                #[doc = ""]
                #[doc = "The dispatch origin of this call must be Signed."]
                #[doc = ""]
                #[doc = "- `dest`: The recipient of the transfer."]
                #[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
                #[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
                #[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
                #[doc = "  keep the sender account alive (true)."]
                pub struct TransferAll {
                    pub dest: transfer_all::Dest,
                    pub keep_alive: transfer_all::KeepAlive,
                }
                pub mod transfer_all {
                    use super::runtime_types;
                    pub type Dest = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type KeepAlive = ::core::primitive::bool;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for TransferAll {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "transfer_all";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Unreserve some balance from a user by force."]
                #[doc = ""]
                #[doc = "Can only be called by ROOT."]
                pub struct ForceUnreserve {
                    pub who: force_unreserve::Who,
                    pub amount: force_unreserve::Amount,
                }
                pub mod force_unreserve {
                    use super::runtime_types;
                    pub type Who = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type Amount = ::core::primitive::u128;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ForceUnreserve {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "force_unreserve";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Upgrade a specified account."]
                #[doc = ""]
                #[doc = "- `origin`: Must be `Signed`."]
                #[doc = "- `who`: The account to be upgraded."]
                #[doc = ""]
                #[doc = "This will waive the transaction fee if at least all but 10% of the accounts needed to"]
                #[doc = "be upgraded. (We let some not have to be upgraded just in order to allow for the"]
                #[doc = "possibililty of churn)."]
                pub struct UpgradeAccounts {
                    pub who: upgrade_accounts::Who,
                }
                pub mod upgrade_accounts {
                    use super::runtime_types;
                    pub type Who = ::subxt::ext::subxt_core::alloc::vec::Vec<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                    >;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for UpgradeAccounts {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "upgrade_accounts";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Set the regular balance of a given account."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call is `root`."]
                pub struct ForceSetBalance {
                    pub who: force_set_balance::Who,
                    #[codec(compact)]
                    pub new_free: force_set_balance::NewFree,
                }
                pub mod force_set_balance {
                    use super::runtime_types;
                    pub type Who = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type NewFree = ::core::primitive::u128;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ForceSetBalance {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "force_set_balance";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Adjust the total issuance in a saturating way."]
                #[doc = ""]
                #[doc = "Can only be called by root and always needs a positive `delta`."]
                #[doc = ""]
                #[doc = "# Example"]
                pub struct ForceAdjustTotalIssuance {
                    pub direction: force_adjust_total_issuance::Direction,
                    #[codec(compact)]
                    pub delta: force_adjust_total_issuance::Delta,
                }
                pub mod force_adjust_total_issuance {
                    use super::runtime_types;
                    pub type Direction = runtime_types::pallet_balances::types::AdjustmentDirection;
                    pub type Delta = ::core::primitive::u128;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ForceAdjustTotalIssuance {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "force_adjust_total_issuance";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Transfer some liquid free balance to another account."]
                #[doc = ""]
                #[doc = "`transfer_allow_death` will set the `FreeBalance` of the sender and receiver."]
                #[doc = "If the sender's account is below the existential deposit as a result"]
                #[doc = "of the transfer, the account will be reaped."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
                pub fn transfer_allow_death(
                    &self,
                    dest: types::transfer_allow_death::Dest,
                    value: types::transfer_allow_death::Value,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::TransferAllowDeath>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "transfer_allow_death",
                        types::TransferAllowDeath { dest, value },
                        [
                            51u8, 166u8, 195u8, 10u8, 139u8, 218u8, 55u8, 130u8, 6u8, 194u8, 35u8,
                            140u8, 27u8, 205u8, 214u8, 222u8, 102u8, 43u8, 143u8, 145u8, 86u8,
                            219u8, 210u8, 147u8, 13u8, 39u8, 51u8, 21u8, 237u8, 179u8, 132u8,
                            130u8,
                        ],
                    )
                }
                #[doc = "Exactly as `transfer_allow_death`, except the origin must be root and the source account"]
                #[doc = "may be specified."]
                pub fn force_transfer(
                    &self,
                    source: types::force_transfer::Source,
                    dest: types::force_transfer::Dest,
                    value: types::force_transfer::Value,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ForceTransfer>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "force_transfer",
                        types::ForceTransfer {
                            source,
                            dest,
                            value,
                        },
                        [
                            154u8, 93u8, 222u8, 27u8, 12u8, 248u8, 63u8, 213u8, 224u8, 86u8, 250u8,
                            153u8, 249u8, 102u8, 83u8, 160u8, 79u8, 125u8, 105u8, 222u8, 77u8,
                            180u8, 90u8, 105u8, 81u8, 217u8, 60u8, 25u8, 213u8, 51u8, 185u8, 96u8,
                        ],
                    )
                }
                #[doc = "Same as the [`transfer_allow_death`] call, but with a check that the transfer will not"]
                #[doc = "kill the origin account."]
                #[doc = ""]
                #[doc = "99% of the time you want [`transfer_allow_death`] instead."]
                #[doc = ""]
                #[doc = "[`transfer_allow_death`]: struct.Pallet.html#method.transfer"]
                pub fn transfer_keep_alive(
                    &self,
                    dest: types::transfer_keep_alive::Dest,
                    value: types::transfer_keep_alive::Value,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::TransferKeepAlive>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "transfer_keep_alive",
                        types::TransferKeepAlive { dest, value },
                        [
                            245u8, 14u8, 190u8, 193u8, 32u8, 210u8, 74u8, 92u8, 25u8, 182u8, 76u8,
                            55u8, 247u8, 83u8, 114u8, 75u8, 143u8, 236u8, 117u8, 25u8, 54u8, 157u8,
                            208u8, 207u8, 233u8, 89u8, 70u8, 161u8, 235u8, 242u8, 222u8, 59u8,
                        ],
                    )
                }
                #[doc = "Transfer the entire transferable balance from the caller account."]
                #[doc = ""]
                #[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
                #[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
                #[doc = "transferred by this function. To ensure that this function results in a killed account,"]
                #[doc = "you might need to prepare the account by removing any reference counters, storage"]
                #[doc = "deposits, etc..."]
                #[doc = ""]
                #[doc = "The dispatch origin of this call must be Signed."]
                #[doc = ""]
                #[doc = "- `dest`: The recipient of the transfer."]
                #[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
                #[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
                #[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
                #[doc = "  keep the sender account alive (true)."]
                pub fn transfer_all(
                    &self,
                    dest: types::transfer_all::Dest,
                    keep_alive: types::transfer_all::KeepAlive,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::TransferAll>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "transfer_all",
                        types::TransferAll { dest, keep_alive },
                        [
                            105u8, 132u8, 49u8, 144u8, 195u8, 250u8, 34u8, 46u8, 213u8, 248u8,
                            112u8, 188u8, 81u8, 228u8, 136u8, 18u8, 67u8, 172u8, 37u8, 38u8, 238u8,
                            9u8, 34u8, 15u8, 67u8, 34u8, 148u8, 195u8, 223u8, 29u8, 154u8, 6u8,
                        ],
                    )
                }
                #[doc = "Unreserve some balance from a user by force."]
                #[doc = ""]
                #[doc = "Can only be called by ROOT."]
                pub fn force_unreserve(
                    &self,
                    who: types::force_unreserve::Who,
                    amount: types::force_unreserve::Amount,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ForceUnreserve>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "force_unreserve",
                        types::ForceUnreserve { who, amount },
                        [
                            142u8, 151u8, 64u8, 205u8, 46u8, 64u8, 62u8, 122u8, 108u8, 49u8, 223u8,
                            140u8, 120u8, 153u8, 35u8, 165u8, 187u8, 38u8, 157u8, 200u8, 123u8,
                            199u8, 198u8, 168u8, 208u8, 159u8, 39u8, 134u8, 92u8, 103u8, 84u8,
                            171u8,
                        ],
                    )
                }
                #[doc = "Upgrade a specified account."]
                #[doc = ""]
                #[doc = "- `origin`: Must be `Signed`."]
                #[doc = "- `who`: The account to be upgraded."]
                #[doc = ""]
                #[doc = "This will waive the transaction fee if at least all but 10% of the accounts needed to"]
                #[doc = "be upgraded. (We let some not have to be upgraded just in order to allow for the"]
                #[doc = "possibililty of churn)."]
                pub fn upgrade_accounts(
                    &self,
                    who: types::upgrade_accounts::Who,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::UpgradeAccounts>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "upgrade_accounts",
                        types::UpgradeAccounts { who },
                        [
                            66u8, 200u8, 179u8, 104u8, 65u8, 2u8, 101u8, 56u8, 130u8, 161u8, 224u8,
                            233u8, 255u8, 124u8, 70u8, 122u8, 8u8, 49u8, 103u8, 178u8, 68u8, 47u8,
                            214u8, 166u8, 217u8, 116u8, 178u8, 50u8, 212u8, 164u8, 98u8, 226u8,
                        ],
                    )
                }
                #[doc = "Set the regular balance of a given account."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call is `root`."]
                pub fn force_set_balance(
                    &self,
                    who: types::force_set_balance::Who,
                    new_free: types::force_set_balance::NewFree,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ForceSetBalance>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "force_set_balance",
                        types::ForceSetBalance { who, new_free },
                        [
                            114u8, 229u8, 59u8, 204u8, 180u8, 83u8, 17u8, 4u8, 59u8, 4u8, 55u8,
                            39u8, 151u8, 196u8, 124u8, 60u8, 209u8, 65u8, 193u8, 11u8, 44u8, 164u8,
                            116u8, 93u8, 169u8, 30u8, 199u8, 165u8, 55u8, 231u8, 223u8, 43u8,
                        ],
                    )
                }
                #[doc = "Adjust the total issuance in a saturating way."]
                #[doc = ""]
                #[doc = "Can only be called by root and always needs a positive `delta`."]
                #[doc = ""]
                #[doc = "# Example"]
                pub fn force_adjust_total_issuance(
                    &self,
                    direction: types::force_adjust_total_issuance::Direction,
                    delta: types::force_adjust_total_issuance::Delta,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<
                    types::ForceAdjustTotalIssuance,
                > {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "force_adjust_total_issuance",
                        types::ForceAdjustTotalIssuance { direction, delta },
                        [
                            208u8, 134u8, 56u8, 133u8, 232u8, 164u8, 10u8, 213u8, 53u8, 193u8,
                            190u8, 63u8, 236u8, 186u8, 96u8, 122u8, 104u8, 87u8, 173u8, 38u8, 58u8,
                            176u8, 21u8, 78u8, 42u8, 106u8, 46u8, 248u8, 251u8, 190u8, 150u8,
                            202u8,
                        ],
                    )
                }
            }
        }
        #[doc = "The `Event` enum of this pallet"]
        pub type Event = runtime_types::pallet_balances::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "An account was created with some free balance."]
            pub struct Endowed {
                pub account: endowed::Account,
                pub free_balance: endowed::FreeBalance,
            }
            pub mod endowed {
                use super::runtime_types;
                pub type Account = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type FreeBalance = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Endowed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Endowed";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
            #[doc = "resulting in an outright loss."]
            pub struct DustLost {
                pub account: dust_lost::Account,
                pub amount: dust_lost::Amount,
            }
            pub mod dust_lost {
                use super::runtime_types;
                pub type Account = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for DustLost {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "DustLost";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Transfer succeeded."]
            pub struct Transfer {
                pub from: transfer::From,
                pub to: transfer::To,
                pub amount: transfer::Amount,
            }
            pub mod transfer {
                use super::runtime_types;
                pub type From = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type To = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Transfer {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Transfer";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "A balance was set by root."]
            pub struct BalanceSet {
                pub who: balance_set::Who,
                pub free: balance_set::Free,
            }
            pub mod balance_set {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Free = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for BalanceSet {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "BalanceSet";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Some balance was reserved (moved from free to reserved)."]
            pub struct Reserved {
                pub who: reserved::Who,
                pub amount: reserved::Amount,
            }
            pub mod reserved {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Reserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Reserved";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Some balance was unreserved (moved from reserved to free)."]
            pub struct Unreserved {
                pub who: unreserved::Who,
                pub amount: unreserved::Amount,
            }
            pub mod unreserved {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Unreserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Unreserved";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Some balance was moved from the reserve of the first account to the second account."]
            #[doc = "Final argument indicates the destination balance type."]
            pub struct ReserveRepatriated {
                pub from: reserve_repatriated::From,
                pub to: reserve_repatriated::To,
                pub amount: reserve_repatriated::Amount,
                pub destination_status: reserve_repatriated::DestinationStatus,
            }
            pub mod reserve_repatriated {
                use super::runtime_types;
                pub type From = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type To = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
                pub type DestinationStatus =
                    runtime_types::frame_support::traits::tokens::misc::BalanceStatus;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for ReserveRepatriated {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "ReserveRepatriated";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Some amount was deposited (e.g. for transaction fees)."]
            pub struct Deposit {
                pub who: deposit::Who,
                pub amount: deposit::Amount,
            }
            pub mod deposit {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Deposit {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Deposit";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
            pub struct Withdraw {
                pub who: withdraw::Who,
                pub amount: withdraw::Amount,
            }
            pub mod withdraw {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Withdraw {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Withdraw";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
            pub struct Slashed {
                pub who: slashed::Who,
                pub amount: slashed::Amount,
            }
            pub mod slashed {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Slashed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Slashed";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Some amount was minted into an account."]
            pub struct Minted {
                pub who: minted::Who,
                pub amount: minted::Amount,
            }
            pub mod minted {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Minted {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Minted";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Some amount was burned from an account."]
            pub struct Burned {
                pub who: burned::Who,
                pub amount: burned::Amount,
            }
            pub mod burned {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Burned {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Burned";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Some amount was suspended from an account (it can be restored later)."]
            pub struct Suspended {
                pub who: suspended::Who,
                pub amount: suspended::Amount,
            }
            pub mod suspended {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Suspended {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Suspended";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Some amount was restored into an account."]
            pub struct Restored {
                pub who: restored::Who,
                pub amount: restored::Amount,
            }
            pub mod restored {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Restored {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Restored";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "An account was upgraded."]
            pub struct Upgraded {
                pub who: upgraded::Who,
            }
            pub mod upgraded {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Upgraded {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Upgraded";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Total issuance was increased by `amount`, creating a credit to be balanced."]
            pub struct Issued {
                pub amount: issued::Amount,
            }
            pub mod issued {
                use super::runtime_types;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Issued {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Issued";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Total issuance was decreased by `amount`, creating a debt to be balanced."]
            pub struct Rescinded {
                pub amount: rescinded::Amount,
            }
            pub mod rescinded {
                use super::runtime_types;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Rescinded {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Rescinded";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Some balance was locked."]
            pub struct Locked {
                pub who: locked::Who,
                pub amount: locked::Amount,
            }
            pub mod locked {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Locked {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Locked";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Some balance was unlocked."]
            pub struct Unlocked {
                pub who: unlocked::Who,
                pub amount: unlocked::Amount,
            }
            pub mod unlocked {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Unlocked {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Unlocked";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Some balance was frozen."]
            pub struct Frozen {
                pub who: frozen::Who,
                pub amount: frozen::Amount,
            }
            pub mod frozen {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Frozen {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Frozen";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Some balance was thawed."]
            pub struct Thawed {
                pub who: thawed::Who,
                pub amount: thawed::Amount,
            }
            pub mod thawed {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Thawed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Thawed";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "The `TotalIssuance` was forcefully changed."]
            pub struct TotalIssuanceForced {
                pub old: total_issuance_forced::Old,
                pub new: total_issuance_forced::New,
            }
            pub mod total_issuance_forced {
                use super::runtime_types;
                pub type Old = ::core::primitive::u128;
                pub type New = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for TotalIssuanceForced {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "TotalIssuanceForced";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod total_issuance {
                    use super::runtime_types;
                    pub type TotalIssuance = ::core::primitive::u128;
                }
                pub mod inactive_issuance {
                    use super::runtime_types;
                    pub type InactiveIssuance = ::core::primitive::u128;
                }
                pub mod account {
                    use super::runtime_types;
                    pub type Account =
                        runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                pub mod locks {
                    use super::runtime_types;
                    pub type Locks =
                        runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
                            runtime_types::pallet_balances::types::BalanceLock<
                                ::core::primitive::u128,
                            >,
                        >;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                pub mod reserves {
                    use super::runtime_types;
                    pub type Reserves = runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::types::ReserveData<
                            [::core::primitive::u8; 8usize],
                            ::core::primitive::u128,
                        >,
                    >;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                pub mod holds {
                    use super::runtime_types;
                    pub type Holds = runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::types::IdAmount<
                            (),
                            ::core::primitive::u128,
                        >,
                    >;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                pub mod freezes {
                    use super::runtime_types;
                    pub type Freezes = runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::types::IdAmount<
                            (),
                            ::core::primitive::u128,
                        >,
                    >;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The total units issued in the system."]
                pub fn total_issuance(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::total_issuance::TotalIssuance,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "TotalIssuance",
                        (),
                        [
                            116u8, 70u8, 119u8, 194u8, 69u8, 37u8, 116u8, 206u8, 171u8, 70u8,
                            171u8, 210u8, 226u8, 111u8, 184u8, 204u8, 206u8, 11u8, 68u8, 72u8,
                            255u8, 19u8, 194u8, 11u8, 27u8, 194u8, 81u8, 204u8, 59u8, 224u8, 202u8,
                            185u8,
                        ],
                    )
                }
                #[doc = " The total units of outstanding deactivated balance in the system."]
                pub fn inactive_issuance(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::inactive_issuance::InactiveIssuance,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "InactiveIssuance",
                        (),
                        [
                            212u8, 185u8, 19u8, 50u8, 250u8, 72u8, 173u8, 50u8, 4u8, 104u8, 161u8,
                            249u8, 77u8, 247u8, 204u8, 248u8, 11u8, 18u8, 57u8, 4u8, 82u8, 110u8,
                            30u8, 216u8, 16u8, 37u8, 87u8, 67u8, 189u8, 235u8, 214u8, 155u8,
                        ],
                    )
                }
                #[doc = " The Balances pallet example of storing the balance of an account."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " You can also store the balance of an account in the `System` pallet."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "   type AccountStore = System"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
                #[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
                #[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
                #[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
                pub fn account_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::account::Account,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Account",
                        (),
                        [
                            213u8, 38u8, 200u8, 69u8, 218u8, 0u8, 112u8, 181u8, 160u8, 23u8, 96u8,
                            90u8, 3u8, 88u8, 126u8, 22u8, 103u8, 74u8, 64u8, 69u8, 29u8, 247u8,
                            18u8, 17u8, 234u8, 143u8, 189u8, 22u8, 247u8, 194u8, 154u8, 249u8,
                        ],
                    )
                }
                #[doc = " The Balances pallet example of storing the balance of an account."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " You can also store the balance of an account in the `System` pallet."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "   type AccountStore = System"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
                #[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
                #[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
                #[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
                pub fn account(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::account::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::account::Param0,
                    >,
                    types::account::Account,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Account",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            213u8, 38u8, 200u8, 69u8, 218u8, 0u8, 112u8, 181u8, 160u8, 23u8, 96u8,
                            90u8, 3u8, 88u8, 126u8, 22u8, 103u8, 74u8, 64u8, 69u8, 29u8, 247u8,
                            18u8, 17u8, 234u8, 143u8, 189u8, 22u8, 247u8, 194u8, 154u8, 249u8,
                        ],
                    )
                }
                #[doc = " Any liquidity locks on some account balances."]
                #[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
                pub fn locks_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::locks::Locks,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Locks",
                        (),
                        [
                            10u8, 223u8, 55u8, 0u8, 249u8, 69u8, 168u8, 41u8, 75u8, 35u8, 120u8,
                            167u8, 18u8, 132u8, 9u8, 20u8, 91u8, 51u8, 27u8, 69u8, 136u8, 187u8,
                            13u8, 220u8, 163u8, 122u8, 26u8, 141u8, 174u8, 249u8, 85u8, 37u8,
                        ],
                    )
                }
                #[doc = " Any liquidity locks on some account balances."]
                #[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
                pub fn locks(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::locks::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::locks::Param0,
                    >,
                    types::locks::Locks,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Locks",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            10u8, 223u8, 55u8, 0u8, 249u8, 69u8, 168u8, 41u8, 75u8, 35u8, 120u8,
                            167u8, 18u8, 132u8, 9u8, 20u8, 91u8, 51u8, 27u8, 69u8, 136u8, 187u8,
                            13u8, 220u8, 163u8, 122u8, 26u8, 141u8, 174u8, 249u8, 85u8, 37u8,
                        ],
                    )
                }
                #[doc = " Named reserves on some account balances."]
                pub fn reserves_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::reserves::Reserves,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Reserves",
                        (),
                        [
                            112u8, 10u8, 241u8, 77u8, 64u8, 187u8, 106u8, 159u8, 13u8, 153u8,
                            140u8, 178u8, 182u8, 50u8, 1u8, 55u8, 149u8, 92u8, 196u8, 229u8, 170u8,
                            106u8, 193u8, 88u8, 255u8, 244u8, 2u8, 193u8, 62u8, 235u8, 204u8, 91u8,
                        ],
                    )
                }
                #[doc = " Named reserves on some account balances."]
                pub fn reserves(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::reserves::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::reserves::Param0,
                    >,
                    types::reserves::Reserves,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Reserves",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            112u8, 10u8, 241u8, 77u8, 64u8, 187u8, 106u8, 159u8, 13u8, 153u8,
                            140u8, 178u8, 182u8, 50u8, 1u8, 55u8, 149u8, 92u8, 196u8, 229u8, 170u8,
                            106u8, 193u8, 88u8, 255u8, 244u8, 2u8, 193u8, 62u8, 235u8, 204u8, 91u8,
                        ],
                    )
                }
                #[doc = " Holds on account balances."]
                pub fn holds_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::holds::Holds,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Holds",
                        (),
                        [
                            53u8, 126u8, 215u8, 237u8, 42u8, 223u8, 188u8, 150u8, 230u8, 107u8,
                            95u8, 24u8, 26u8, 235u8, 158u8, 149u8, 193u8, 191u8, 10u8, 194u8,
                            231u8, 59u8, 35u8, 167u8, 186u8, 89u8, 43u8, 126u8, 215u8, 117u8, 1u8,
                            202u8,
                        ],
                    )
                }
                #[doc = " Holds on account balances."]
                pub fn holds(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::holds::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::holds::Param0,
                    >,
                    types::holds::Holds,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Holds",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            53u8, 126u8, 215u8, 237u8, 42u8, 223u8, 188u8, 150u8, 230u8, 107u8,
                            95u8, 24u8, 26u8, 235u8, 158u8, 149u8, 193u8, 191u8, 10u8, 194u8,
                            231u8, 59u8, 35u8, 167u8, 186u8, 89u8, 43u8, 126u8, 215u8, 117u8, 1u8,
                            202u8,
                        ],
                    )
                }
                #[doc = " Freeze locks on account balances."]
                pub fn freezes_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::freezes::Freezes,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Freezes",
                        (),
                        [
                            69u8, 49u8, 165u8, 76u8, 135u8, 142u8, 179u8, 118u8, 50u8, 109u8, 53u8,
                            112u8, 110u8, 94u8, 30u8, 93u8, 173u8, 38u8, 27u8, 142u8, 19u8, 5u8,
                            163u8, 4u8, 68u8, 218u8, 179u8, 224u8, 118u8, 218u8, 115u8, 64u8,
                        ],
                    )
                }
                #[doc = " Freeze locks on account balances."]
                pub fn freezes(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::freezes::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::freezes::Param0,
                    >,
                    types::freezes::Freezes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Freezes",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            69u8, 49u8, 165u8, 76u8, 135u8, 142u8, 179u8, 118u8, 50u8, 109u8, 53u8,
                            112u8, 110u8, 94u8, 30u8, 93u8, 173u8, 38u8, 27u8, 142u8, 19u8, 5u8,
                            163u8, 4u8, 68u8, 218u8, 179u8, 224u8, 118u8, 218u8, 115u8, 64u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The minimum amount required to keep an account open. MUST BE GREATER THAN ZERO!"]
                #[doc = ""]
                #[doc = " If you *really* need it to be zero, you can enable the feature `insecure_zero_ed` for"]
                #[doc = " this pallet. However, you do so at your own risk: this will open up a major DoS vector."]
                #[doc = " In case you have multiple sources of provider references, you may also get unexpected"]
                #[doc = " behaviour if you set this to zero."]
                #[doc = ""]
                #[doc = " Bottom line: Do yourself a favour and make it at least one!"]
                pub fn existential_deposit(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u128,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Balances",
                        "ExistentialDeposit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The maximum number of locks that should exist on an account."]
                #[doc = " Not strictly enforced, but used for weight estimation."]
                pub fn max_locks(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u32,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Balances",
                        "MaxLocks",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum number of named reserves that can exist on an account."]
                pub fn max_reserves(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u32,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Balances",
                        "MaxReserves",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum number of individual freeze locks that can exist on an account at any time."]
                pub fn max_freezes(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u32,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Balances",
                        "MaxFreezes",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod sudo {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Error for the Sudo pallet."]
        pub type Error = runtime_types::pallet_sudo::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::pallet_sudo::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                pub struct Sudo {
                    pub call: ::subxt::ext::subxt_core::alloc::boxed::Box<sudo::Call>,
                }
                pub mod sudo {
                    use super::runtime_types;
                    pub type Call = runtime_types::midnight_node_runtime::RuntimeCall;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for Sudo {
                    const PALLET: &'static str = "Sudo";
                    const CALL: &'static str = "sudo";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                #[doc = "This function does not check the weight of the call, and instead allows the"]
                #[doc = "Sudo user to specify the weight of the call."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                pub struct SudoUncheckedWeight {
                    pub call:
                        ::subxt::ext::subxt_core::alloc::boxed::Box<sudo_unchecked_weight::Call>,
                    pub weight: sudo_unchecked_weight::Weight,
                }
                pub mod sudo_unchecked_weight {
                    use super::runtime_types;
                    pub type Call = runtime_types::midnight_node_runtime::RuntimeCall;
                    pub type Weight = runtime_types::sp_weights::weight_v2::Weight;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SudoUncheckedWeight {
                    const PALLET: &'static str = "Sudo";
                    const CALL: &'static str = "sudo_unchecked_weight";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
                #[doc = "key."]
                pub struct SetKey {
                    pub new: set_key::New,
                }
                pub mod set_key {
                    use super::runtime_types;
                    pub type New = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetKey {
                    const PALLET: &'static str = "Sudo";
                    const CALL: &'static str = "set_key";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
                #[doc = "a given account."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                pub struct SudoAs {
                    pub who: sudo_as::Who,
                    pub call: ::subxt::ext::subxt_core::alloc::boxed::Box<sudo_as::Call>,
                }
                pub mod sudo_as {
                    use super::runtime_types;
                    pub type Who = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type Call = runtime_types::midnight_node_runtime::RuntimeCall;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SudoAs {
                    const PALLET: &'static str = "Sudo";
                    const CALL: &'static str = "sudo_as";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Permanently removes the sudo key."]
                #[doc = ""]
                #[doc = "**This cannot be un-done.**"]
                pub struct RemoveKey;
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for RemoveKey {
                    const PALLET: &'static str = "Sudo";
                    const CALL: &'static str = "remove_key";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                pub fn sudo(
                    &self,
                    call: types::sudo::Call,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::Sudo>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Sudo",
                        "sudo",
                        types::Sudo {
                            call: ::subxt::ext::subxt_core::alloc::boxed::Box::new(call),
                        },
                        [
                            203u8, 157u8, 63u8, 51u8, 38u8, 223u8, 128u8, 222u8, 32u8, 239u8,
                            132u8, 30u8, 84u8, 152u8, 32u8, 218u8, 47u8, 120u8, 105u8, 101u8,
                            207u8, 154u8, 236u8, 141u8, 92u8, 124u8, 157u8, 68u8, 148u8, 173u8,
                            117u8, 250u8,
                        ],
                    )
                }
                #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                #[doc = "This function does not check the weight of the call, and instead allows the"]
                #[doc = "Sudo user to specify the weight of the call."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                pub fn sudo_unchecked_weight(
                    &self,
                    call: types::sudo_unchecked_weight::Call,
                    weight: types::sudo_unchecked_weight::Weight,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SudoUncheckedWeight>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Sudo",
                        "sudo_unchecked_weight",
                        types::SudoUncheckedWeight {
                            call: ::subxt::ext::subxt_core::alloc::boxed::Box::new(call),
                            weight,
                        },
                        [
                            61u8, 21u8, 44u8, 235u8, 94u8, 1u8, 82u8, 140u8, 144u8, 185u8, 198u8,
                            167u8, 228u8, 126u8, 249u8, 236u8, 223u8, 76u8, 189u8, 15u8, 215u8,
                            177u8, 196u8, 54u8, 95u8, 62u8, 100u8, 97u8, 182u8, 170u8, 111u8, 86u8,
                        ],
                    )
                }
                #[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
                #[doc = "key."]
                pub fn set_key(
                    &self,
                    new: types::set_key::New,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetKey>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Sudo",
                        "set_key",
                        types::SetKey { new },
                        [
                            9u8, 73u8, 39u8, 205u8, 188u8, 127u8, 143u8, 54u8, 128u8, 94u8, 8u8,
                            227u8, 197u8, 44u8, 70u8, 93u8, 228u8, 196u8, 64u8, 165u8, 226u8,
                            158u8, 101u8, 192u8, 22u8, 193u8, 102u8, 84u8, 21u8, 35u8, 92u8, 198u8,
                        ],
                    )
                }
                #[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
                #[doc = "a given account."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                pub fn sudo_as(
                    &self,
                    who: types::sudo_as::Who,
                    call: types::sudo_as::Call,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SudoAs>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Sudo",
                        "sudo_as",
                        types::SudoAs {
                            who,
                            call: ::subxt::ext::subxt_core::alloc::boxed::Box::new(call),
                        },
                        [
                            232u8, 149u8, 195u8, 124u8, 219u8, 80u8, 194u8, 122u8, 12u8, 152u8,
                            108u8, 115u8, 112u8, 239u8, 171u8, 2u8, 172u8, 232u8, 177u8, 131u8,
                            81u8, 26u8, 103u8, 201u8, 72u8, 157u8, 172u8, 32u8, 193u8, 164u8,
                            209u8, 135u8,
                        ],
                    )
                }
                #[doc = "Permanently removes the sudo key."]
                #[doc = ""]
                #[doc = "**This cannot be un-done.**"]
                pub fn remove_key(
                    &self,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::RemoveKey>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Sudo",
                        "remove_key",
                        types::RemoveKey {},
                        [
                            133u8, 253u8, 54u8, 175u8, 202u8, 239u8, 5u8, 198u8, 180u8, 138u8,
                            25u8, 28u8, 109u8, 40u8, 30u8, 56u8, 126u8, 100u8, 52u8, 205u8, 250u8,
                            191u8, 61u8, 195u8, 172u8, 142u8, 184u8, 239u8, 247u8, 10u8, 211u8,
                            79u8,
                        ],
                    )
                }
            }
        }
        #[doc = "The `Event` enum of this pallet"]
        pub type Event = runtime_types::pallet_sudo::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "A sudo call just took place."]
            pub struct Sudid {
                pub sudo_result: sudid::SudoResult,
            }
            pub mod sudid {
                use super::runtime_types;
                pub type SudoResult =
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Sudid {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "Sudid";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "The sudo key has been updated."]
            pub struct KeyChanged {
                pub old: key_changed::Old,
                pub new: key_changed::New,
            }
            pub mod key_changed {
                use super::runtime_types;
                pub type Old = ::core::option::Option<::subxt::ext::subxt_core::utils::AccountId32>;
                pub type New = ::subxt::ext::subxt_core::utils::AccountId32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for KeyChanged {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "KeyChanged";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "The key was permanently removed."]
            pub struct KeyRemoved;
            impl ::subxt::ext::subxt_core::events::StaticEvent for KeyRemoved {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "KeyRemoved";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "A [sudo_as](Pallet::sudo_as) call just took place."]
            pub struct SudoAsDone {
                pub sudo_result: sudo_as_done::SudoResult,
            }
            pub mod sudo_as_done {
                use super::runtime_types;
                pub type SudoResult =
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for SudoAsDone {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "SudoAsDone";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod key {
                    use super::runtime_types;
                    pub type Key = ::subxt::ext::subxt_core::utils::AccountId32;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The `AccountId` of the sudo key."]
                pub fn key(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::key::Key,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Sudo",
                        "Key",
                        (),
                        [
                            72u8, 14u8, 225u8, 162u8, 205u8, 247u8, 227u8, 105u8, 116u8, 57u8, 4u8,
                            31u8, 84u8, 137u8, 227u8, 228u8, 133u8, 245u8, 206u8, 227u8, 117u8,
                            36u8, 252u8, 151u8, 107u8, 15u8, 180u8, 4u8, 4u8, 152u8, 195u8, 144u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod session_committee_management {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "The `Error` enum of this pallet."]
        pub type Error = runtime_types::pallet_session_validator_management::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::pallet_session_validator_management::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "'for_epoch_number' parameter is needed only for validation purposes, because we need to make sure that"]
                #[doc = "check_inherent uses the same epoch_number as was used to create inherent data."]
                #[doc = "Alternative approach would be to put epoch number inside InherentData. However, sidechain"]
                #[doc = "epoch number is set in Runtime, thus, inherent data provider doesn't have to know about it."]
                #[doc = "On top of that, the latter approach is slightly more complicated to code."]
                pub struct Set {
                    pub validators: set::Validators,
                    pub for_epoch_number: set::ForEpochNumber,
                }
                pub mod set {
                    use super::runtime_types;
                    pub type Validators = runtime_types :: bounded_collections :: bounded_vec :: BoundedVec < (runtime_types :: sidechain_inherents :: cross_chain_signatures :: crypto :: cross_chain_app :: Public , runtime_types :: midnight_node_runtime :: opaque :: SessionKeys ,) > ;
                    pub type ForEpochNumber = runtime_types::sidechain_domain::ScEpochNumber;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for Set {
                    const PALLET: &'static str = "SessionCommitteeManagement";
                    const CALL: &'static str = "set";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "'for_epoch_number' parameter is needed only for validation purposes, because we need to make sure that"]
                #[doc = "check_inherent uses the same epoch_number as was used to create inherent data."]
                #[doc = "Alternative approach would be to put epoch number inside InherentData. However, sidechain"]
                #[doc = "epoch number is set in Runtime, thus, inherent data provider doesn't have to know about it."]
                #[doc = "On top of that, the latter approach is slightly more complicated to code."]
                pub fn set(
                    &self,
                    validators: types::set::Validators,
                    for_epoch_number: types::set::ForEpochNumber,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::Set>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "SessionCommitteeManagement",
                        "set",
                        types::Set {
                            validators,
                            for_epoch_number,
                        },
                        [
                            12u8, 227u8, 171u8, 55u8, 24u8, 88u8, 9u8, 140u8, 241u8, 76u8, 211u8,
                            88u8, 112u8, 19u8, 208u8, 220u8, 242u8, 147u8, 66u8, 47u8, 90u8, 87u8,
                            205u8, 150u8, 172u8, 85u8, 138u8, 237u8, 145u8, 22u8, 207u8, 187u8,
                        ],
                    )
                }
            }
        }
        #[doc = "The `Event` enum of this pallet"]
        pub type Event = runtime_types::pallet_session_validator_management::pallet::Event;
        pub mod events {
            use super::runtime_types;
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod current_committee {
                    use super::runtime_types;
                    pub type CurrentCommittee = runtime_types :: pallet_session_validator_management :: pallet :: CommitteeInfo < runtime_types :: sidechain_domain :: ScEpochNumber , runtime_types :: sidechain_inherents :: cross_chain_signatures :: crypto :: cross_chain_app :: Public , runtime_types :: midnight_node_runtime :: opaque :: SessionKeys > ;
                }
                pub mod next_committee {
                    use super::runtime_types;
                    pub type NextCommittee = runtime_types :: pallet_session_validator_management :: pallet :: CommitteeInfo < runtime_types :: sidechain_domain :: ScEpochNumber , runtime_types :: sidechain_inherents :: cross_chain_signatures :: crypto :: cross_chain_app :: Public , runtime_types :: midnight_node_runtime :: opaque :: SessionKeys > ;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn current_committee(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::current_committee::CurrentCommittee,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "SessionCommitteeManagement",
                        "CurrentCommittee",
                        (),
                        [
                            17u8, 184u8, 251u8, 103u8, 132u8, 21u8, 151u8, 137u8, 145u8, 244u8,
                            225u8, 129u8, 225u8, 253u8, 21u8, 92u8, 32u8, 232u8, 154u8, 113u8, 4u8,
                            113u8, 55u8, 202u8, 251u8, 199u8, 128u8, 22u8, 183u8, 222u8, 24u8,
                            92u8,
                        ],
                    )
                }
                pub fn next_committee(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::next_committee::NextCommittee,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "SessionCommitteeManagement",
                        "NextCommittee",
                        (),
                        [
                            236u8, 114u8, 9u8, 163u8, 64u8, 137u8, 229u8, 246u8, 113u8, 117u8,
                            236u8, 183u8, 229u8, 238u8, 162u8, 89u8, 166u8, 88u8, 55u8, 153u8,
                            24u8, 180u8, 157u8, 91u8, 140u8, 76u8, 206u8, 28u8, 163u8, 186u8, 91u8,
                            112u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn max_validators(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u32,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "SessionCommitteeManagement",
                        "MaxValidators",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod session {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "The `Event` enum of this pallet"]
        pub type Event = runtime_types::pallet_sidechains_session::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "New session has happened. Note that the argument is the session index, not the"]
            #[doc = "block number as the type might suggest."]
            pub struct NewSession {
                pub session_index: new_session::SessionIndex,
            }
            pub mod new_session {
                use super::runtime_types;
                pub type SessionIndex = ::core::primitive::u32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for NewSession {
                const PALLET: &'static str = "Session";
                const EVENT: &'static str = "NewSession";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod validators {
                    use super::runtime_types;
                    pub type Validators = ::subxt::ext::subxt_core::alloc::vec::Vec<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                    >;
                }
                pub mod validators_and_keys {
                    use super::runtime_types;
                    pub type ValidatorsAndKeys = ::subxt::ext::subxt_core::alloc::vec::Vec<(
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        runtime_types::midnight_node_runtime::opaque::SessionKeys,
                    )>;
                }
                pub mod current_index {
                    use super::runtime_types;
                    pub type CurrentIndex = ::core::primitive::u32;
                }
                pub mod disabled_validators {
                    use super::runtime_types;
                    pub type DisabledValidators =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u32>;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn validators(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::validators::Validators,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Session",
                        "Validators",
                        (),
                        [
                            50u8, 86u8, 154u8, 222u8, 249u8, 209u8, 156u8, 22u8, 155u8, 25u8,
                            133u8, 194u8, 210u8, 50u8, 38u8, 28u8, 139u8, 201u8, 90u8, 139u8,
                            115u8, 12u8, 12u8, 141u8, 4u8, 178u8, 201u8, 241u8, 223u8, 234u8, 6u8,
                            86u8,
                        ],
                    )
                }
                pub fn validators_and_keys(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::validators_and_keys::ValidatorsAndKeys,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Session",
                        "ValidatorsAndKeys",
                        (),
                        [
                            29u8, 99u8, 251u8, 229u8, 89u8, 209u8, 200u8, 153u8, 215u8, 186u8,
                            48u8, 82u8, 149u8, 56u8, 244u8, 29u8, 137u8, 30u8, 29u8, 149u8, 194u8,
                            219u8, 135u8, 238u8, 51u8, 21u8, 133u8, 9u8, 175u8, 225u8, 110u8, 36u8,
                        ],
                    )
                }
                #[doc = " Current index of the session."]
                pub fn current_index(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::current_index::CurrentIndex,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Session",
                        "CurrentIndex",
                        (),
                        [
                            167u8, 151u8, 125u8, 150u8, 159u8, 21u8, 78u8, 217u8, 237u8, 183u8,
                            135u8, 65u8, 187u8, 114u8, 188u8, 206u8, 16u8, 32u8, 69u8, 208u8,
                            134u8, 159u8, 232u8, 224u8, 243u8, 27u8, 31u8, 166u8, 145u8, 44u8,
                            221u8, 230u8,
                        ],
                    )
                }
                #[doc = " Indices of disabled validators."]
                #[doc = ""]
                #[doc = " The vec is always kept sorted so that we can find whether a given validator is"]
                #[doc = " disabled using binary search. It gets cleared when `on_session_ending` returns"]
                #[doc = " a new set of identities."]
                pub fn disabled_validators(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::disabled_validators::DisabledValidators,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Session",
                        "DisabledValidators",
                        (),
                        [
                            213u8, 19u8, 168u8, 234u8, 187u8, 200u8, 180u8, 97u8, 234u8, 189u8,
                            36u8, 233u8, 158u8, 184u8, 45u8, 35u8, 129u8, 213u8, 133u8, 8u8, 104u8,
                            183u8, 46u8, 68u8, 154u8, 240u8, 132u8, 22u8, 247u8, 11u8, 54u8, 221u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod runtime_types {
        use super::runtime_types;
        pub mod authority_selection_inherents {
            use super::runtime_types;
            pub mod authority_selection_inputs {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct AuthoritySelectionInputs {
                    pub d_parameter: runtime_types::sidechain_domain::DParameter,
                    pub permissioned_candidates: ::subxt::ext::subxt_core::alloc::vec::Vec<
                        runtime_types::sidechain_domain::PermissionedCandidateData,
                    >,
                    pub registered_candidates: ::subxt::ext::subxt_core::alloc::vec::Vec<
                        runtime_types::sidechain_domain::CandidateRegistrationsWithStake,
                    >,
                    pub epoch_nonce: runtime_types::sidechain_domain::EpochNonce,
                }
            }
        }
        pub mod bounded_collections {
            use super::runtime_types;
            pub mod bounded_vec {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct BoundedVec<_0>(pub ::subxt::ext::subxt_core::alloc::vec::Vec<_0>);
            }
            pub mod weak_bounded_vec {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct WeakBoundedVec<_0>(pub ::subxt::ext::subxt_core::alloc::vec::Vec<_0>);
            }
        }
        pub mod finality_grandpa {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Equivocation<_0, _1, _2> {
                pub round_number: ::core::primitive::u64,
                pub identity: _0,
                pub first: (_1, _2),
                pub second: (_1, _2),
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Precommit<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Prevote<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
        }
        pub mod frame_support {
            use super::runtime_types;
            pub mod dispatch {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum DispatchClass {
                    #[codec(index = 0)]
                    Normal,
                    #[codec(index = 1)]
                    Operational,
                    #[codec(index = 2)]
                    Mandatory,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct DispatchInfo {
                    pub weight: runtime_types::sp_weights::weight_v2::Weight,
                    pub class: runtime_types::frame_support::dispatch::DispatchClass,
                    pub pays_fee: runtime_types::frame_support::dispatch::Pays,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Pays {
                    #[codec(index = 0)]
                    Yes,
                    #[codec(index = 1)]
                    No,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct PerDispatchClass<_0> {
                    pub normal: _0,
                    pub operational: _0,
                    pub mandatory: _0,
                }
            }
            pub mod traits {
                use super::runtime_types;
                pub mod tokens {
                    use super::runtime_types;
                    pub mod misc {
                        use super::runtime_types;
                        #[derive(
                            :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                            :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                            :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                            :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                            Debug,
                        )]
                        # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                        #[decode_as_type(
                            crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                        )]
                        #[encode_as_type(
                            crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                        )]
                        pub enum BalanceStatus {
                            #[codec(index = 0)]
                            Free,
                            #[codec(index = 1)]
                            Reserved,
                        }
                    }
                }
            }
        }
        pub mod frame_system {
            use super::runtime_types;
            pub mod extensions {
                use super::runtime_types;
                pub mod check_genesis {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct CheckGenesis;
                }
                pub mod check_mortality {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
                }
                pub mod check_non_zero_sender {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct CheckNonZeroSender;
                }
                pub mod check_nonce {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct CheckNonce(#[codec(compact)] pub ::core::primitive::u32);
                }
                pub mod check_spec_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct CheckSpecVersion;
                }
                pub mod check_tx_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct CheckTxVersion;
                }
                pub mod check_weight {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct CheckWeight;
                }
            }
            pub mod limits {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct BlockLength {
                    pub max: runtime_types::frame_support::dispatch::PerDispatchClass<
                        ::core::primitive::u32,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct BlockWeights {
                    pub base_block: runtime_types::sp_weights::weight_v2::Weight,
                    pub max_block: runtime_types::sp_weights::weight_v2::Weight,
                    pub per_class: runtime_types::frame_support::dispatch::PerDispatchClass<
                        runtime_types::frame_system::limits::WeightsPerClass,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct WeightsPerClass {
                    pub base_extrinsic: runtime_types::sp_weights::weight_v2::Weight,
                    pub max_extrinsic:
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                    pub max_total:
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                    pub reserved:
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Make some on-chain remark."]
                    #[doc = ""]
                    #[doc = "Can be executed by every `origin`."]
                    remark {
                        remark: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Set the number of pages in the WebAssembly environment's heap."]
                    set_heap_pages { pages: ::core::primitive::u64 },
                    #[codec(index = 2)]
                    #[doc = "Set the new runtime code."]
                    set_code {
                        code: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 3)]
                    #[doc = "Set the new runtime code without doing any checks of the given `code`."]
                    #[doc = ""]
                    #[doc = "Note that runtime upgrades will not run if this is called with a not-increasing spec"]
                    #[doc = "version!"]
                    set_code_without_checks {
                        code: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    #[doc = "Set some items of storage."]
                    set_storage {
                        items: ::subxt::ext::subxt_core::alloc::vec::Vec<(
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        )>,
                    },
                    #[codec(index = 5)]
                    #[doc = "Kill some items from storage."]
                    kill_storage {
                        keys: ::subxt::ext::subxt_core::alloc::vec::Vec<
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        >,
                    },
                    #[codec(index = 6)]
                    #[doc = "Kill all storage items with a key that starts with the given prefix."]
                    #[doc = ""]
                    #[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
                    #[doc = "the prefix we are removing to accurately calculate the weight of this function."]
                    kill_prefix {
                        prefix: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        subkeys: ::core::primitive::u32,
                    },
                    #[codec(index = 7)]
                    #[doc = "Make some on-chain remark and emit event."]
                    remark_with_event {
                        remark: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 9)]
                    #[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
                    #[doc = "later."]
                    #[doc = ""]
                    #[doc = "This call requires Root origin."]
                    authorize_upgrade {
                        code_hash: ::subxt::ext::subxt_core::utils::H256,
                    },
                    #[codec(index = 10)]
                    #[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
                    #[doc = "later."]
                    #[doc = ""]
                    #[doc = "WARNING: This authorizes an upgrade that will take place without any safety checks, for"]
                    #[doc = "example that the spec name remains the same and that the version number increases. Not"]
                    #[doc = "recommended for normal use. Use `authorize_upgrade` instead."]
                    #[doc = ""]
                    #[doc = "This call requires Root origin."]
                    authorize_upgrade_without_checks {
                        code_hash: ::subxt::ext::subxt_core::utils::H256,
                    },
                    #[codec(index = 11)]
                    #[doc = "Provide the preimage (runtime binary) `code` for an upgrade that has been authorized."]
                    #[doc = ""]
                    #[doc = "If the authorization required a version check, this call will ensure the spec name"]
                    #[doc = "remains unchanged and that the spec version has increased."]
                    #[doc = ""]
                    #[doc = "Depending on the runtime's `OnSetCode` configuration, this function may directly apply"]
                    #[doc = "the new `code` in the same block or attempt to schedule the upgrade."]
                    #[doc = ""]
                    #[doc = "All origins are allowed."]
                    apply_authorized_upgrade {
                        code: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Error for the System pallet"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The name of specification does not match between the current runtime"]
                    #[doc = "and the new runtime."]
                    InvalidSpecName,
                    #[codec(index = 1)]
                    #[doc = "The specification version is not allowed to decrease between the current runtime"]
                    #[doc = "and the new runtime."]
                    SpecVersionNeedsToIncrease,
                    #[codec(index = 2)]
                    #[doc = "Failed to extract the runtime version from the new runtime."]
                    #[doc = ""]
                    #[doc = "Either calling `Core_version` or decoding `RuntimeVersion` failed."]
                    FailedToExtractRuntimeVersion,
                    #[codec(index = 3)]
                    #[doc = "Suicide called when the account has non-default composite data."]
                    NonDefaultComposite,
                    #[codec(index = 4)]
                    #[doc = "There is a non-zero reference count preventing the account from being purged."]
                    NonZeroRefCount,
                    #[codec(index = 5)]
                    #[doc = "The origin filter prevent the call to be dispatched."]
                    CallFiltered,
                    #[codec(index = 6)]
                    #[doc = "A multi-block migration is ongoing and prevents the current code from being replaced."]
                    MultiBlockMigrationsOngoing,
                    #[codec(index = 7)]
                    #[doc = "No upgrade authorized."]
                    NothingAuthorized,
                    #[codec(index = 8)]
                    #[doc = "The submitted code is not authorized."]
                    Unauthorized,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Event for the System pallet."]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "An extrinsic completed successfully."]
                    ExtrinsicSuccess {
                        dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
                    },
                    #[codec(index = 1)]
                    #[doc = "An extrinsic failed."]
                    ExtrinsicFailed {
                        dispatch_error: runtime_types::sp_runtime::DispatchError,
                        dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
                    },
                    #[codec(index = 2)]
                    #[doc = "`:code` was updated."]
                    CodeUpdated,
                    #[codec(index = 3)]
                    #[doc = "A new account was created."]
                    NewAccount {
                        account: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 4)]
                    #[doc = "An account was reaped."]
                    KilledAccount {
                        account: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 5)]
                    #[doc = "On on-chain remark happened."]
                    Remarked {
                        sender: ::subxt::ext::subxt_core::utils::AccountId32,
                        hash: ::subxt::ext::subxt_core::utils::H256,
                    },
                    #[codec(index = 6)]
                    #[doc = "An upgrade was authorized."]
                    UpgradeAuthorized {
                        code_hash: ::subxt::ext::subxt_core::utils::H256,
                        check_version: ::core::primitive::bool,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct AccountInfo<_0, _1> {
                pub nonce: _0,
                pub consumers: ::core::primitive::u32,
                pub providers: ::core::primitive::u32,
                pub sufficients: ::core::primitive::u32,
                pub data: _1,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct CodeUpgradeAuthorization {
                pub code_hash: ::subxt::ext::subxt_core::utils::H256,
                pub check_version: ::core::primitive::bool,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct EventRecord<_0, _1> {
                pub phase: runtime_types::frame_system::Phase,
                pub event: _0,
                pub topics: ::subxt::ext::subxt_core::alloc::vec::Vec<_1>,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct LastRuntimeUpgradeInfo {
                #[codec(compact)]
                pub spec_version: ::core::primitive::u32,
                pub spec_name: ::subxt::ext::subxt_core::alloc::string::String,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum Phase {
                #[codec(index = 0)]
                ApplyExtrinsic(::core::primitive::u32),
                #[codec(index = 1)]
                Finalization,
                #[codec(index = 2)]
                Initialization,
            }
        }
        pub mod midnight_node_runtime {
            use super::runtime_types;
            pub mod opaque {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct SessionKeys {
                    pub aura: runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
                    pub grandpa: runtime_types::sp_consensus_grandpa::app::Public,
                }
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Runtime;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum RuntimeCall {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Call),
                #[codec(index = 1)]
                Timestamp(runtime_types::pallet_timestamp::pallet::Call),
                #[codec(index = 3)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Call),
                #[codec(index = 5)]
                Midnight(runtime_types::pallet_midnight::pallet::Call),
                #[codec(index = 6)]
                Balances(runtime_types::pallet_balances::pallet::Call),
                #[codec(index = 7)]
                Sudo(runtime_types::pallet_sudo::pallet::Call),
                #[codec(index = 8)]
                SessionCommitteeManagement(
                    runtime_types::pallet_session_validator_management::pallet::Call,
                ),
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum RuntimeError {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Error),
                #[codec(index = 3)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Error),
                #[codec(index = 5)]
                Midnight(runtime_types::pallet_midnight::pallet::Error),
                #[codec(index = 6)]
                Balances(runtime_types::pallet_balances::pallet::Error),
                #[codec(index = 7)]
                Sudo(runtime_types::pallet_sudo::pallet::Error),
                #[codec(index = 8)]
                SessionCommitteeManagement(
                    runtime_types::pallet_session_validator_management::pallet::Error,
                ),
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum RuntimeEvent {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Event),
                #[codec(index = 3)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Event),
                #[codec(index = 5)]
                Midnight(runtime_types::pallet_midnight::pallet::Event),
                #[codec(index = 6)]
                Balances(runtime_types::pallet_balances::pallet::Event),
                #[codec(index = 7)]
                Sudo(runtime_types::pallet_sudo::pallet::Event),
                #[codec(index = 8)]
                SessionCommitteeManagement(
                    runtime_types::pallet_session_validator_management::pallet::Event,
                ),
                #[codec(index = 30)]
                Session(runtime_types::pallet_sidechains_session::pallet::Event),
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum TransactionType {
                #[codec(index = 0)]
                MidnightTx(::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>),
                #[codec(index = 1)]
                TimestampTx(::core::primitive::u64),
                #[codec(index = 2)]
                UnknownTx,
            }
        }
        pub mod pallet_balances {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Transfer some liquid free balance to another account."]
                    #[doc = ""]
                    #[doc = "`transfer_allow_death` will set the `FreeBalance` of the sender and receiver."]
                    #[doc = "If the sender's account is below the existential deposit as a result"]
                    #[doc = "of the transfer, the account will be reaped."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
                    transfer_allow_death {
                        dest: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "Exactly as `transfer_allow_death`, except the origin must be root and the source account"]
                    #[doc = "may be specified."]
                    force_transfer {
                        source: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        dest: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "Same as the [`transfer_allow_death`] call, but with a check that the transfer will not"]
                    #[doc = "kill the origin account."]
                    #[doc = ""]
                    #[doc = "99% of the time you want [`transfer_allow_death`] instead."]
                    #[doc = ""]
                    #[doc = "[`transfer_allow_death`]: struct.Pallet.html#method.transfer"]
                    transfer_keep_alive {
                        dest: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    #[doc = "Transfer the entire transferable balance from the caller account."]
                    #[doc = ""]
                    #[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
                    #[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
                    #[doc = "transferred by this function. To ensure that this function results in a killed account,"]
                    #[doc = "you might need to prepare the account by removing any reference counters, storage"]
                    #[doc = "deposits, etc..."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be Signed."]
                    #[doc = ""]
                    #[doc = "- `dest`: The recipient of the transfer."]
                    #[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
                    #[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
                    #[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
                    #[doc = "  keep the sender account alive (true)."]
                    transfer_all {
                        dest: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        keep_alive: ::core::primitive::bool,
                    },
                    #[codec(index = 5)]
                    #[doc = "Unreserve some balance from a user by force."]
                    #[doc = ""]
                    #[doc = "Can only be called by ROOT."]
                    force_unreserve {
                        who: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    #[doc = "Upgrade a specified account."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must be `Signed`."]
                    #[doc = "- `who`: The account to be upgraded."]
                    #[doc = ""]
                    #[doc = "This will waive the transaction fee if at least all but 10% of the accounts needed to"]
                    #[doc = "be upgraded. (We let some not have to be upgraded just in order to allow for the"]
                    #[doc = "possibililty of churn)."]
                    upgrade_accounts {
                        who: ::subxt::ext::subxt_core::alloc::vec::Vec<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        >,
                    },
                    #[codec(index = 8)]
                    #[doc = "Set the regular balance of a given account."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call is `root`."]
                    force_set_balance {
                        who: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        new_free: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    #[doc = "Adjust the total issuance in a saturating way."]
                    #[doc = ""]
                    #[doc = "Can only be called by root and always needs a positive `delta`."]
                    #[doc = ""]
                    #[doc = "# Example"]
                    force_adjust_total_issuance {
                        direction: runtime_types::pallet_balances::types::AdjustmentDirection,
                        #[codec(compact)]
                        delta: ::core::primitive::u128,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Vesting balance too high to send value."]
                    VestingBalance,
                    #[codec(index = 1)]
                    #[doc = "Account liquidity restrictions prevent withdrawal."]
                    LiquidityRestrictions,
                    #[codec(index = 2)]
                    #[doc = "Balance too low to send value."]
                    InsufficientBalance,
                    #[codec(index = 3)]
                    #[doc = "Value too low to create account due to existential deposit."]
                    ExistentialDeposit,
                    #[codec(index = 4)]
                    #[doc = "Transfer/payment would kill account."]
                    Expendability,
                    #[codec(index = 5)]
                    #[doc = "A vesting schedule already exists for this account."]
                    ExistingVestingSchedule,
                    #[codec(index = 6)]
                    #[doc = "Beneficiary account must pre-exist."]
                    DeadAccount,
                    #[codec(index = 7)]
                    #[doc = "Number of named reserves exceed `MaxReserves`."]
                    TooManyReserves,
                    #[codec(index = 8)]
                    #[doc = "Number of holds exceed `VariantCountOf<T::RuntimeHoldReason>`."]
                    TooManyHolds,
                    #[codec(index = 9)]
                    #[doc = "Number of freezes exceed `MaxFreezes`."]
                    TooManyFreezes,
                    #[codec(index = 10)]
                    #[doc = "The issuance cannot be modified since it is already deactivated."]
                    IssuanceDeactivated,
                    #[codec(index = 11)]
                    #[doc = "The delta cannot be zero."]
                    DeltaZero,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "An account was created with some free balance."]
                    Endowed {
                        account: ::subxt::ext::subxt_core::utils::AccountId32,
                        free_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    #[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
                    #[doc = "resulting in an outright loss."]
                    DustLost {
                        account: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "Transfer succeeded."]
                    Transfer {
                        from: ::subxt::ext::subxt_core::utils::AccountId32,
                        to: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "A balance was set by root."]
                    BalanceSet {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        free: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    #[doc = "Some balance was reserved (moved from free to reserved)."]
                    Reserved {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    #[doc = "Some balance was unreserved (moved from reserved to free)."]
                    Unreserved {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    #[doc = "Some balance was moved from the reserve of the first account to the second account."]
                    #[doc = "Final argument indicates the destination balance type."]
                    ReserveRepatriated {
                        from: ::subxt::ext::subxt_core::utils::AccountId32,
                        to: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                        destination_status:
                            runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
                    },
                    #[codec(index = 7)]
                    #[doc = "Some amount was deposited (e.g. for transaction fees)."]
                    Deposit {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    #[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
                    Withdraw {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    #[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
                    Slashed {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 10)]
                    #[doc = "Some amount was minted into an account."]
                    Minted {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 11)]
                    #[doc = "Some amount was burned from an account."]
                    Burned {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 12)]
                    #[doc = "Some amount was suspended from an account (it can be restored later)."]
                    Suspended {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 13)]
                    #[doc = "Some amount was restored into an account."]
                    Restored {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 14)]
                    #[doc = "An account was upgraded."]
                    Upgraded {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 15)]
                    #[doc = "Total issuance was increased by `amount`, creating a credit to be balanced."]
                    Issued { amount: ::core::primitive::u128 },
                    #[codec(index = 16)]
                    #[doc = "Total issuance was decreased by `amount`, creating a debt to be balanced."]
                    Rescinded { amount: ::core::primitive::u128 },
                    #[codec(index = 17)]
                    #[doc = "Some balance was locked."]
                    Locked {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 18)]
                    #[doc = "Some balance was unlocked."]
                    Unlocked {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 19)]
                    #[doc = "Some balance was frozen."]
                    Frozen {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 20)]
                    #[doc = "Some balance was thawed."]
                    Thawed {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 21)]
                    #[doc = "The `TotalIssuance` was forcefully changed."]
                    TotalIssuanceForced {
                        old: ::core::primitive::u128,
                        new: ::core::primitive::u128,
                    },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct AccountData<_0> {
                    pub free: _0,
                    pub reserved: _0,
                    pub frozen: _0,
                    pub flags: runtime_types::pallet_balances::types::ExtraFlags,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum AdjustmentDirection {
                    #[codec(index = 0)]
                    Increase,
                    #[codec(index = 1)]
                    Decrease,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct BalanceLock<_0> {
                    pub id: [::core::primitive::u8; 8usize],
                    pub amount: _0,
                    pub reasons: runtime_types::pallet_balances::types::Reasons,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ExtraFlags(pub ::core::primitive::u128);
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct IdAmount<_0, _1> {
                    pub id: _0,
                    pub amount: _1,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Reasons {
                    #[codec(index = 0)]
                    Fee,
                    #[codec(index = 1)]
                    Misc,
                    #[codec(index = 2)]
                    All,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ReserveData<_0, _1> {
                    pub id: _0,
                    pub amount: _1,
                }
            }
        }
        pub mod pallet_grandpa {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
                    #[doc = "equivocation proof and validate the given key ownership proof"]
                    #[doc = "against the extracted offender. If both are valid, the offence"]
                    #[doc = "will be reported."]
                    report_equivocation {
                        equivocation_proof: ::subxt::ext::subxt_core::alloc::boxed::Box<
                            runtime_types::sp_consensus_grandpa::EquivocationProof<
                                ::subxt::ext::subxt_core::utils::H256,
                                ::core::primitive::u32,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_core::Void,
                    },
                    #[codec(index = 1)]
                    #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
                    #[doc = "equivocation proof and validate the given key ownership proof"]
                    #[doc = "against the extracted offender. If both are valid, the offence"]
                    #[doc = "will be reported."]
                    #[doc = ""]
                    #[doc = "This extrinsic must be called unsigned and it is expected that only"]
                    #[doc = "block authors will call it (validated in `ValidateUnsigned`), as such"]
                    #[doc = "if the block author is defined it will be defined as the equivocation"]
                    #[doc = "reporter."]
                    report_equivocation_unsigned {
                        equivocation_proof: ::subxt::ext::subxt_core::alloc::boxed::Box<
                            runtime_types::sp_consensus_grandpa::EquivocationProof<
                                ::subxt::ext::subxt_core::utils::H256,
                                ::core::primitive::u32,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_core::Void,
                    },
                    #[codec(index = 2)]
                    #[doc = "Note that the current authority set of the GRANDPA finality gadget has stalled."]
                    #[doc = ""]
                    #[doc = "This will trigger a forced authority set change at the beginning of the next session, to"]
                    #[doc = "be enacted `delay` blocks after that. The `delay` should be high enough to safely assume"]
                    #[doc = "that the block signalling the forced change will not be re-orged e.g. 1000 blocks."]
                    #[doc = "The block production rate (which may be slowed down because of finality lagging) should"]
                    #[doc = "be taken into account when choosing the `delay`. The GRANDPA voters based on the new"]
                    #[doc = "authority will start voting on top of `best_finalized_block_number` for new finalized"]
                    #[doc = "blocks. `best_finalized_block_number` should be the highest of the latest finalized"]
                    #[doc = "block of all validators of the new authority set."]
                    #[doc = ""]
                    #[doc = "Only callable by root."]
                    note_stalled {
                        delay: ::core::primitive::u32,
                        best_finalized_block_number: ::core::primitive::u32,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Attempt to signal GRANDPA pause when the authority set isn't live"]
                    #[doc = "(either paused or already pending pause)."]
                    PauseFailed,
                    #[codec(index = 1)]
                    #[doc = "Attempt to signal GRANDPA resume when the authority set isn't paused"]
                    #[doc = "(either live or already pending resume)."]
                    ResumeFailed,
                    #[codec(index = 2)]
                    #[doc = "Attempt to signal GRANDPA change with one already pending."]
                    ChangePending,
                    #[codec(index = 3)]
                    #[doc = "Cannot signal forced change so soon after last."]
                    TooSoon,
                    #[codec(index = 4)]
                    #[doc = "A key ownership proof provided as part of an equivocation report is invalid."]
                    InvalidKeyOwnershipProof,
                    #[codec(index = 5)]
                    #[doc = "An equivocation proof provided as part of an equivocation report is invalid."]
                    InvalidEquivocationProof,
                    #[codec(index = 6)]
                    #[doc = "A given equivocation report is valid but already previously reported."]
                    DuplicateOffenceReport,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "New authority set has been applied."]
                    NewAuthorities {
                        authority_set: ::subxt::ext::subxt_core::alloc::vec::Vec<(
                            runtime_types::sp_consensus_grandpa::app::Public,
                            ::core::primitive::u64,
                        )>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Current authority set has been paused."]
                    Paused,
                    #[codec(index = 2)]
                    #[doc = "Current authority set has been resumed."]
                    Resumed,
                }
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct StoredPendingChange<_0> {
                pub scheduled_at: _0,
                pub delay: _0,
                pub next_authorities:
                    runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_consensus_grandpa::app::Public,
                        ::core::primitive::u64,
                    )>,
                pub forced: ::core::option::Option<_0>,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum StoredState<_0> {
                #[codec(index = 0)]
                Live,
                #[codec(index = 1)]
                PendingPause { scheduled_at: _0, delay: _0 },
                #[codec(index = 2)]
                Paused,
                #[codec(index = 3)]
                PendingResume { scheduled_at: _0, delay: _0 },
            }
        }
        pub mod pallet_midnight {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    send_mn_transaction {
                        midnight_tx:
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct CallDetails {
                    pub tx_hash: [::core::primitive::u8; 32usize],
                    pub contract_address:
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct DeploymentDetails {
                    pub tx_hash: [::core::primitive::u8; 32usize],
                    pub contract_address:
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    UnableToLoadNetworkId,
                    #[codec(index = 1)]
                    UnableToDeserializeTransaction,
                    #[codec(index = 2)]
                    UnableToDeserializeContractAddresses,
                    #[codec(index = 3)]
                    UnableToApplyTransaction,
                    #[codec(index = 4)]
                    UnableToSerializeLedgerState,
                    #[codec(index = 5)]
                    UnableToSerializeContractAddresses,
                    #[codec(index = 6)]
                    OutOfStateStorage,
                    #[codec(index = 7)]
                    InvalidTransaction,
                    #[codec(index = 8)]
                    CorruptedStateAfterTransaction,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A contract was called."]
                    ContractCall(runtime_types::pallet_midnight::pallet::CallDetails),
                    #[codec(index = 1)]
                    #[doc = "A contract has been deployed."]
                    ContractDeploy(runtime_types::pallet_midnight::pallet::DeploymentDetails),
                    #[codec(index = 2)]
                    #[doc = "A transaction has been applied (both the guaranteed and conditional part)."]
                    TxApplied(runtime_types::pallet_midnight::pallet::TxAppliedDetails),
                    #[codec(index = 3)]
                    #[doc = "Only guaranteed transactions have been applied."]
                    TxOnlyGuaranteedApplied(
                        runtime_types::pallet_midnight::pallet::TxAppliedDetails,
                    ),
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct TxAppliedDetails {
                    pub tx_hash: [::core::primitive::u8; 32usize],
                }
            }
        }
        pub mod pallet_session_validator_management {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    # [codec (index = 0)] # [doc = "'for_epoch_number' parameter is needed only for validation purposes, because we need to make sure that"] # [doc = "check_inherent uses the same epoch_number as was used to create inherent data."] # [doc = "Alternative approach would be to put epoch number inside InherentData. However, sidechain"] # [doc = "epoch number is set in Runtime, thus, inherent data provider doesn't have to know about it."] # [doc = "On top of that, the latter approach is slightly more complicated to code."] set { validators : runtime_types :: bounded_collections :: bounded_vec :: BoundedVec < (runtime_types :: sidechain_inherents :: cross_chain_signatures :: crypto :: cross_chain_app :: Public , runtime_types :: midnight_node_runtime :: opaque :: SessionKeys ,) > , for_epoch_number : runtime_types :: sidechain_domain :: ScEpochNumber , } , }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct CommitteeInfo<_0, _1, _2> {
                    pub epoch: _0,
                    pub committee:
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<(_1, _2)>,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidEpoch,
                    #[codec(index = 1)]
                    UnnecessarySetCall,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {}
            }
        }
        pub mod pallet_sidechain {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct SidechainStatus {
                    pub epoch: runtime_types::sidechain_domain::ScEpochNumber,
                    pub slot: runtime_types::sidechain_domain::ScSlotNumber,
                    pub slots_per_epoch: ::core::primitive::u32,
                }
            }
        }
        pub mod pallet_sidechains_session {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "New session has happened. Note that the argument is the session index, not the"]
                    #[doc = "block number as the type might suggest."]
                    NewSession {
                        session_index: ::core::primitive::u32,
                    },
                }
            }
        }
        pub mod pallet_sudo {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                    sudo {
                        call: ::subxt::ext::subxt_core::alloc::boxed::Box<
                            runtime_types::midnight_node_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 1)]
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                    #[doc = "This function does not check the weight of the call, and instead allows the"]
                    #[doc = "Sudo user to specify the weight of the call."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    sudo_unchecked_weight {
                        call: ::subxt::ext::subxt_core::alloc::boxed::Box<
                            runtime_types::midnight_node_runtime::RuntimeCall,
                        >,
                        weight: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 2)]
                    #[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
                    #[doc = "key."]
                    set_key {
                        new: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 3)]
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
                    #[doc = "a given account."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    sudo_as {
                        who: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        call: ::subxt::ext::subxt_core::alloc::boxed::Box<
                            runtime_types::midnight_node_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 4)]
                    #[doc = "Permanently removes the sudo key."]
                    #[doc = ""]
                    #[doc = "**This cannot be un-done.**"]
                    remove_key,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Error for the Sudo pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Sender must be the Sudo account."]
                    RequireSudo,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A sudo call just took place."]
                    Sudid {
                        sudo_result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 1)]
                    #[doc = "The sudo key has been updated."]
                    KeyChanged {
                        old: ::core::option::Option<::subxt::ext::subxt_core::utils::AccountId32>,
                        new: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 2)]
                    #[doc = "The key was permanently removed."]
                    KeyRemoved,
                    #[codec(index = 3)]
                    #[doc = "A [sudo_as](Pallet::sudo_as) call just took place."]
                    SudoAsDone {
                        sudo_result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                }
            }
        }
        pub mod pallet_timestamp {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Set the current time."]
                    #[doc = ""]
                    #[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
                    #[doc = "phase, if this call hasn't been invoked by that time."]
                    #[doc = ""]
                    #[doc = "The timestamp should be greater than the previous one by the amount specified by"]
                    #[doc = "[`Config::MinimumPeriod`]."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _None_."]
                    #[doc = ""]
                    #[doc = "This dispatch class is _Mandatory_ to ensure it gets executed in the block. Be aware"]
                    #[doc = "that changing the complexity of this call could result exhausting the resources in a"]
                    #[doc = "block to execute any other calls."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
                    #[doc = "- 1 storage read and 1 storage mutation (codec `O(1)` because of `DidUpdate::take` in"]
                    #[doc = "  `on_finalize`)"]
                    #[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
                    set {
                        #[codec(compact)]
                        now: ::core::primitive::u64,
                    },
                }
            }
        }
        pub mod sidechain_domain {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct AuraPublicKey(
                pub ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
            );
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct CandidateRegistrations {
                pub mainchain_pub_key: runtime_types::sidechain_domain::MainchainPublicKey,
                pub registrations: ::subxt::ext::subxt_core::alloc::vec::Vec<
                    runtime_types::sidechain_domain::RegistrationData,
                >,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct CandidateRegistrationsWithStake {
                pub stake_delegation: runtime_types::sidechain_domain::StakeDelegation,
                pub candidate_registrations:
                    runtime_types::sidechain_domain::CandidateRegistrations,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct CrossChainPublicKey(
                pub ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
            );
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct CrossChainSignature(
                pub ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
            );
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct DParameter {
                pub num_permissioned_candidates: ::core::primitive::u16,
                pub num_registered_candidates: ::core::primitive::u16,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct EpochNonce(
                pub ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
            );
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct GrandpaPublicKey(
                pub ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
            );
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct MainchainAddressHash(pub [::core::primitive::u8; 28usize]);
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct MainchainPublicKey(pub [::core::primitive::u8; 32usize]);
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct MainchainSignature(
                pub ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
            );
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: CompactAs,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct McBlockNumber(pub ::core::primitive::u32);
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: CompactAs,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct McEpochNumber(pub ::core::primitive::u32);
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: CompactAs,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct McSlotNumber(pub ::core::primitive::u64);
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct McTxHash(pub [::core::primitive::u8; 32usize]);
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: CompactAs,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct McTxIndexInBlock(pub ::core::primitive::u32);
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct PermissionedCandidateData {
                pub sidechain_public_key: runtime_types::sidechain_domain::SidechainPublicKey,
                pub aura_public_key: runtime_types::sidechain_domain::AuraPublicKey,
                pub grandpa_public_key: runtime_types::sidechain_domain::GrandpaPublicKey,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct RegistrationData {
                pub consumed_input: runtime_types::sidechain_domain::UtxoId,
                pub sidechain_signature: runtime_types::sidechain_domain::SidechainSignature,
                pub mainchain_signature: runtime_types::sidechain_domain::MainchainSignature,
                pub cross_chain_signature: runtime_types::sidechain_domain::CrossChainSignature,
                pub sidechain_pub_key: runtime_types::sidechain_domain::SidechainPublicKey,
                pub cross_chain_pub_key: runtime_types::sidechain_domain::CrossChainPublicKey,
                pub utxo_info: runtime_types::sidechain_domain::UtxoInfo,
                pub tx_inputs: ::subxt::ext::subxt_core::alloc::vec::Vec<
                    runtime_types::sidechain_domain::UtxoId,
                >,
                pub aura_pub_key: runtime_types::sidechain_domain::AuraPublicKey,
                pub grandpa_pub_key: runtime_types::sidechain_domain::GrandpaPublicKey,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: CompactAs,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct ScEpochNumber(pub ::core::primitive::u64);
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: CompactAs,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct ScSlotNumber(pub ::core::primitive::u64);
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct SidechainParams {
                pub chain_id: ::core::primitive::u16,
                pub genesis_committee_utxo: runtime_types::sidechain_domain::UtxoId,
                pub threshold_numerator: ::core::primitive::u64,
                pub threshold_denominator: ::core::primitive::u64,
                pub governance_authority: runtime_types::sidechain_domain::MainchainAddressHash,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct SidechainPublicKey(
                pub ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
            );
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct SidechainSignature(
                pub ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
            );
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: CompactAs,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct StakeDelegation(pub ::core::primitive::u64);
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct UtxoId {
                pub tx_hash: runtime_types::sidechain_domain::McTxHash,
                pub index: runtime_types::sidechain_domain::UtxoIndex,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: CompactAs,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct UtxoIndex(pub ::core::primitive::u16);
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct UtxoInfo {
                pub utxo_id: runtime_types::sidechain_domain::UtxoId,
                pub epoch_number: runtime_types::sidechain_domain::McEpochNumber,
                pub block_number: runtime_types::sidechain_domain::McBlockNumber,
                pub slot_number: runtime_types::sidechain_domain::McSlotNumber,
                pub tx_index_within_block: runtime_types::sidechain_domain::McTxIndexInBlock,
            }
        }
        pub mod sidechain_inherents {
            use super::runtime_types;
            pub mod cross_chain_signatures {
                use super::runtime_types;
                pub mod crypto {
                    use super::runtime_types;
                    pub mod cross_chain_app {
                        use super::runtime_types;
                        #[derive(
                            :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                            :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                            :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                            :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                            Debug,
                        )]
                        # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                        #[decode_as_type(
                            crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                        )]
                        #[encode_as_type(
                            crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                        )]
                        pub struct Public(pub runtime_types::sp_core::ecdsa::Public);
                    }
                }
            }
        }
        pub mod sidechain_slots {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct ScSlotConfig {
                pub slots_per_epoch: runtime_types::sidechain_slots::SlotsPerEpoch,
                pub slot_duration: runtime_types::sp_consensus_slots::SlotDuration,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: CompactAs,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct SlotsPerEpoch(pub ::core::primitive::u32);
        }
        pub mod sp_arithmetic {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum ArithmeticError {
                #[codec(index = 0)]
                Underflow,
                #[codec(index = 1)]
                Overflow,
                #[codec(index = 2)]
                DivisionByZero,
            }
        }
        pub mod sp_consensus_aura {
            use super::runtime_types;
            pub mod sr25519 {
                use super::runtime_types;
                pub mod app_sr25519 {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct Public(pub runtime_types::sp_core::sr25519::Public);
                }
            }
        }
        pub mod sp_consensus_grandpa {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Public(pub runtime_types::sp_core::ed25519::Public);
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Signature(pub runtime_types::sp_core::ed25519::Signature);
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum Equivocation<_0, _1> {
                #[codec(index = 0)]
                Prevote(
                    runtime_types::finality_grandpa::Equivocation<
                        runtime_types::sp_consensus_grandpa::app::Public,
                        runtime_types::finality_grandpa::Prevote<_0, _1>,
                        runtime_types::sp_consensus_grandpa::app::Signature,
                    >,
                ),
                #[codec(index = 1)]
                Precommit(
                    runtime_types::finality_grandpa::Equivocation<
                        runtime_types::sp_consensus_grandpa::app::Public,
                        runtime_types::finality_grandpa::Precommit<_0, _1>,
                        runtime_types::sp_consensus_grandpa::app::Signature,
                    >,
                ),
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct EquivocationProof<_0, _1> {
                pub set_id: ::core::primitive::u64,
                pub equivocation: runtime_types::sp_consensus_grandpa::Equivocation<_0, _1>,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct OpaqueKeyOwnershipProof(
                pub ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
            );
        }
        pub mod sp_consensus_slots {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: CompactAs,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Slot(pub ::core::primitive::u64);
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: CompactAs,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct SlotDuration(pub ::core::primitive::u64);
        }
        pub mod sp_core {
            use super::runtime_types;
            pub mod crypto {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct KeyTypeId(pub [::core::primitive::u8; 4usize]);
            }
            pub mod ecdsa {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Public(pub [::core::primitive::u8; 33usize]);
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Signature(pub [::core::primitive::u8; 65usize]);
            }
            pub mod ed25519 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            pub mod sr25519 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct OpaqueMetadata(
                pub ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
            );
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum Void {}
        }
        pub mod sp_inherents {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct CheckInherentsResult {
                pub okay: ::core::primitive::bool,
                pub fatal_error: ::core::primitive::bool,
                pub errors: runtime_types::sp_inherents::InherentData,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct InherentData {
                pub data: ::subxt::ext::subxt_core::utils::KeyedVec<
                    [::core::primitive::u8; 8usize],
                    ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                >,
            }
        }
        pub mod sp_runtime {
            use super::runtime_types;
            pub mod generic {
                use super::runtime_types;
                pub mod block {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct Block<_0, _1> {
                        pub header: _0,
                        pub extrinsics: ::subxt::ext::subxt_core::alloc::vec::Vec<_1>,
                    }
                }
                pub mod digest {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct Digest {
                        pub logs: ::subxt::ext::subxt_core::alloc::vec::Vec<
                            runtime_types::sp_runtime::generic::digest::DigestItem,
                        >,
                    }
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum DigestItem {
                        #[codec(index = 6)]
                        PreRuntime(
                            [::core::primitive::u8; 4usize],
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 4)]
                        Consensus(
                            [::core::primitive::u8; 4usize],
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 5)]
                        Seal(
                            [::core::primitive::u8; 4usize],
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 0)]
                        Other(::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>),
                        #[codec(index = 8)]
                        RuntimeEnvironmentUpdated,
                    }
                }
                pub mod era {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum Era {
                        #[codec(index = 0)]
                        Immortal,
                        #[codec(index = 1)]
                        Mortal1(::core::primitive::u8),
                        #[codec(index = 2)]
                        Mortal2(::core::primitive::u8),
                        #[codec(index = 3)]
                        Mortal3(::core::primitive::u8),
                        #[codec(index = 4)]
                        Mortal4(::core::primitive::u8),
                        #[codec(index = 5)]
                        Mortal5(::core::primitive::u8),
                        #[codec(index = 6)]
                        Mortal6(::core::primitive::u8),
                        #[codec(index = 7)]
                        Mortal7(::core::primitive::u8),
                        #[codec(index = 8)]
                        Mortal8(::core::primitive::u8),
                        #[codec(index = 9)]
                        Mortal9(::core::primitive::u8),
                        #[codec(index = 10)]
                        Mortal10(::core::primitive::u8),
                        #[codec(index = 11)]
                        Mortal11(::core::primitive::u8),
                        #[codec(index = 12)]
                        Mortal12(::core::primitive::u8),
                        #[codec(index = 13)]
                        Mortal13(::core::primitive::u8),
                        #[codec(index = 14)]
                        Mortal14(::core::primitive::u8),
                        #[codec(index = 15)]
                        Mortal15(::core::primitive::u8),
                        #[codec(index = 16)]
                        Mortal16(::core::primitive::u8),
                        #[codec(index = 17)]
                        Mortal17(::core::primitive::u8),
                        #[codec(index = 18)]
                        Mortal18(::core::primitive::u8),
                        #[codec(index = 19)]
                        Mortal19(::core::primitive::u8),
                        #[codec(index = 20)]
                        Mortal20(::core::primitive::u8),
                        #[codec(index = 21)]
                        Mortal21(::core::primitive::u8),
                        #[codec(index = 22)]
                        Mortal22(::core::primitive::u8),
                        #[codec(index = 23)]
                        Mortal23(::core::primitive::u8),
                        #[codec(index = 24)]
                        Mortal24(::core::primitive::u8),
                        #[codec(index = 25)]
                        Mortal25(::core::primitive::u8),
                        #[codec(index = 26)]
                        Mortal26(::core::primitive::u8),
                        #[codec(index = 27)]
                        Mortal27(::core::primitive::u8),
                        #[codec(index = 28)]
                        Mortal28(::core::primitive::u8),
                        #[codec(index = 29)]
                        Mortal29(::core::primitive::u8),
                        #[codec(index = 30)]
                        Mortal30(::core::primitive::u8),
                        #[codec(index = 31)]
                        Mortal31(::core::primitive::u8),
                        #[codec(index = 32)]
                        Mortal32(::core::primitive::u8),
                        #[codec(index = 33)]
                        Mortal33(::core::primitive::u8),
                        #[codec(index = 34)]
                        Mortal34(::core::primitive::u8),
                        #[codec(index = 35)]
                        Mortal35(::core::primitive::u8),
                        #[codec(index = 36)]
                        Mortal36(::core::primitive::u8),
                        #[codec(index = 37)]
                        Mortal37(::core::primitive::u8),
                        #[codec(index = 38)]
                        Mortal38(::core::primitive::u8),
                        #[codec(index = 39)]
                        Mortal39(::core::primitive::u8),
                        #[codec(index = 40)]
                        Mortal40(::core::primitive::u8),
                        #[codec(index = 41)]
                        Mortal41(::core::primitive::u8),
                        #[codec(index = 42)]
                        Mortal42(::core::primitive::u8),
                        #[codec(index = 43)]
                        Mortal43(::core::primitive::u8),
                        #[codec(index = 44)]
                        Mortal44(::core::primitive::u8),
                        #[codec(index = 45)]
                        Mortal45(::core::primitive::u8),
                        #[codec(index = 46)]
                        Mortal46(::core::primitive::u8),
                        #[codec(index = 47)]
                        Mortal47(::core::primitive::u8),
                        #[codec(index = 48)]
                        Mortal48(::core::primitive::u8),
                        #[codec(index = 49)]
                        Mortal49(::core::primitive::u8),
                        #[codec(index = 50)]
                        Mortal50(::core::primitive::u8),
                        #[codec(index = 51)]
                        Mortal51(::core::primitive::u8),
                        #[codec(index = 52)]
                        Mortal52(::core::primitive::u8),
                        #[codec(index = 53)]
                        Mortal53(::core::primitive::u8),
                        #[codec(index = 54)]
                        Mortal54(::core::primitive::u8),
                        #[codec(index = 55)]
                        Mortal55(::core::primitive::u8),
                        #[codec(index = 56)]
                        Mortal56(::core::primitive::u8),
                        #[codec(index = 57)]
                        Mortal57(::core::primitive::u8),
                        #[codec(index = 58)]
                        Mortal58(::core::primitive::u8),
                        #[codec(index = 59)]
                        Mortal59(::core::primitive::u8),
                        #[codec(index = 60)]
                        Mortal60(::core::primitive::u8),
                        #[codec(index = 61)]
                        Mortal61(::core::primitive::u8),
                        #[codec(index = 62)]
                        Mortal62(::core::primitive::u8),
                        #[codec(index = 63)]
                        Mortal63(::core::primitive::u8),
                        #[codec(index = 64)]
                        Mortal64(::core::primitive::u8),
                        #[codec(index = 65)]
                        Mortal65(::core::primitive::u8),
                        #[codec(index = 66)]
                        Mortal66(::core::primitive::u8),
                        #[codec(index = 67)]
                        Mortal67(::core::primitive::u8),
                        #[codec(index = 68)]
                        Mortal68(::core::primitive::u8),
                        #[codec(index = 69)]
                        Mortal69(::core::primitive::u8),
                        #[codec(index = 70)]
                        Mortal70(::core::primitive::u8),
                        #[codec(index = 71)]
                        Mortal71(::core::primitive::u8),
                        #[codec(index = 72)]
                        Mortal72(::core::primitive::u8),
                        #[codec(index = 73)]
                        Mortal73(::core::primitive::u8),
                        #[codec(index = 74)]
                        Mortal74(::core::primitive::u8),
                        #[codec(index = 75)]
                        Mortal75(::core::primitive::u8),
                        #[codec(index = 76)]
                        Mortal76(::core::primitive::u8),
                        #[codec(index = 77)]
                        Mortal77(::core::primitive::u8),
                        #[codec(index = 78)]
                        Mortal78(::core::primitive::u8),
                        #[codec(index = 79)]
                        Mortal79(::core::primitive::u8),
                        #[codec(index = 80)]
                        Mortal80(::core::primitive::u8),
                        #[codec(index = 81)]
                        Mortal81(::core::primitive::u8),
                        #[codec(index = 82)]
                        Mortal82(::core::primitive::u8),
                        #[codec(index = 83)]
                        Mortal83(::core::primitive::u8),
                        #[codec(index = 84)]
                        Mortal84(::core::primitive::u8),
                        #[codec(index = 85)]
                        Mortal85(::core::primitive::u8),
                        #[codec(index = 86)]
                        Mortal86(::core::primitive::u8),
                        #[codec(index = 87)]
                        Mortal87(::core::primitive::u8),
                        #[codec(index = 88)]
                        Mortal88(::core::primitive::u8),
                        #[codec(index = 89)]
                        Mortal89(::core::primitive::u8),
                        #[codec(index = 90)]
                        Mortal90(::core::primitive::u8),
                        #[codec(index = 91)]
                        Mortal91(::core::primitive::u8),
                        #[codec(index = 92)]
                        Mortal92(::core::primitive::u8),
                        #[codec(index = 93)]
                        Mortal93(::core::primitive::u8),
                        #[codec(index = 94)]
                        Mortal94(::core::primitive::u8),
                        #[codec(index = 95)]
                        Mortal95(::core::primitive::u8),
                        #[codec(index = 96)]
                        Mortal96(::core::primitive::u8),
                        #[codec(index = 97)]
                        Mortal97(::core::primitive::u8),
                        #[codec(index = 98)]
                        Mortal98(::core::primitive::u8),
                        #[codec(index = 99)]
                        Mortal99(::core::primitive::u8),
                        #[codec(index = 100)]
                        Mortal100(::core::primitive::u8),
                        #[codec(index = 101)]
                        Mortal101(::core::primitive::u8),
                        #[codec(index = 102)]
                        Mortal102(::core::primitive::u8),
                        #[codec(index = 103)]
                        Mortal103(::core::primitive::u8),
                        #[codec(index = 104)]
                        Mortal104(::core::primitive::u8),
                        #[codec(index = 105)]
                        Mortal105(::core::primitive::u8),
                        #[codec(index = 106)]
                        Mortal106(::core::primitive::u8),
                        #[codec(index = 107)]
                        Mortal107(::core::primitive::u8),
                        #[codec(index = 108)]
                        Mortal108(::core::primitive::u8),
                        #[codec(index = 109)]
                        Mortal109(::core::primitive::u8),
                        #[codec(index = 110)]
                        Mortal110(::core::primitive::u8),
                        #[codec(index = 111)]
                        Mortal111(::core::primitive::u8),
                        #[codec(index = 112)]
                        Mortal112(::core::primitive::u8),
                        #[codec(index = 113)]
                        Mortal113(::core::primitive::u8),
                        #[codec(index = 114)]
                        Mortal114(::core::primitive::u8),
                        #[codec(index = 115)]
                        Mortal115(::core::primitive::u8),
                        #[codec(index = 116)]
                        Mortal116(::core::primitive::u8),
                        #[codec(index = 117)]
                        Mortal117(::core::primitive::u8),
                        #[codec(index = 118)]
                        Mortal118(::core::primitive::u8),
                        #[codec(index = 119)]
                        Mortal119(::core::primitive::u8),
                        #[codec(index = 120)]
                        Mortal120(::core::primitive::u8),
                        #[codec(index = 121)]
                        Mortal121(::core::primitive::u8),
                        #[codec(index = 122)]
                        Mortal122(::core::primitive::u8),
                        #[codec(index = 123)]
                        Mortal123(::core::primitive::u8),
                        #[codec(index = 124)]
                        Mortal124(::core::primitive::u8),
                        #[codec(index = 125)]
                        Mortal125(::core::primitive::u8),
                        #[codec(index = 126)]
                        Mortal126(::core::primitive::u8),
                        #[codec(index = 127)]
                        Mortal127(::core::primitive::u8),
                        #[codec(index = 128)]
                        Mortal128(::core::primitive::u8),
                        #[codec(index = 129)]
                        Mortal129(::core::primitive::u8),
                        #[codec(index = 130)]
                        Mortal130(::core::primitive::u8),
                        #[codec(index = 131)]
                        Mortal131(::core::primitive::u8),
                        #[codec(index = 132)]
                        Mortal132(::core::primitive::u8),
                        #[codec(index = 133)]
                        Mortal133(::core::primitive::u8),
                        #[codec(index = 134)]
                        Mortal134(::core::primitive::u8),
                        #[codec(index = 135)]
                        Mortal135(::core::primitive::u8),
                        #[codec(index = 136)]
                        Mortal136(::core::primitive::u8),
                        #[codec(index = 137)]
                        Mortal137(::core::primitive::u8),
                        #[codec(index = 138)]
                        Mortal138(::core::primitive::u8),
                        #[codec(index = 139)]
                        Mortal139(::core::primitive::u8),
                        #[codec(index = 140)]
                        Mortal140(::core::primitive::u8),
                        #[codec(index = 141)]
                        Mortal141(::core::primitive::u8),
                        #[codec(index = 142)]
                        Mortal142(::core::primitive::u8),
                        #[codec(index = 143)]
                        Mortal143(::core::primitive::u8),
                        #[codec(index = 144)]
                        Mortal144(::core::primitive::u8),
                        #[codec(index = 145)]
                        Mortal145(::core::primitive::u8),
                        #[codec(index = 146)]
                        Mortal146(::core::primitive::u8),
                        #[codec(index = 147)]
                        Mortal147(::core::primitive::u8),
                        #[codec(index = 148)]
                        Mortal148(::core::primitive::u8),
                        #[codec(index = 149)]
                        Mortal149(::core::primitive::u8),
                        #[codec(index = 150)]
                        Mortal150(::core::primitive::u8),
                        #[codec(index = 151)]
                        Mortal151(::core::primitive::u8),
                        #[codec(index = 152)]
                        Mortal152(::core::primitive::u8),
                        #[codec(index = 153)]
                        Mortal153(::core::primitive::u8),
                        #[codec(index = 154)]
                        Mortal154(::core::primitive::u8),
                        #[codec(index = 155)]
                        Mortal155(::core::primitive::u8),
                        #[codec(index = 156)]
                        Mortal156(::core::primitive::u8),
                        #[codec(index = 157)]
                        Mortal157(::core::primitive::u8),
                        #[codec(index = 158)]
                        Mortal158(::core::primitive::u8),
                        #[codec(index = 159)]
                        Mortal159(::core::primitive::u8),
                        #[codec(index = 160)]
                        Mortal160(::core::primitive::u8),
                        #[codec(index = 161)]
                        Mortal161(::core::primitive::u8),
                        #[codec(index = 162)]
                        Mortal162(::core::primitive::u8),
                        #[codec(index = 163)]
                        Mortal163(::core::primitive::u8),
                        #[codec(index = 164)]
                        Mortal164(::core::primitive::u8),
                        #[codec(index = 165)]
                        Mortal165(::core::primitive::u8),
                        #[codec(index = 166)]
                        Mortal166(::core::primitive::u8),
                        #[codec(index = 167)]
                        Mortal167(::core::primitive::u8),
                        #[codec(index = 168)]
                        Mortal168(::core::primitive::u8),
                        #[codec(index = 169)]
                        Mortal169(::core::primitive::u8),
                        #[codec(index = 170)]
                        Mortal170(::core::primitive::u8),
                        #[codec(index = 171)]
                        Mortal171(::core::primitive::u8),
                        #[codec(index = 172)]
                        Mortal172(::core::primitive::u8),
                        #[codec(index = 173)]
                        Mortal173(::core::primitive::u8),
                        #[codec(index = 174)]
                        Mortal174(::core::primitive::u8),
                        #[codec(index = 175)]
                        Mortal175(::core::primitive::u8),
                        #[codec(index = 176)]
                        Mortal176(::core::primitive::u8),
                        #[codec(index = 177)]
                        Mortal177(::core::primitive::u8),
                        #[codec(index = 178)]
                        Mortal178(::core::primitive::u8),
                        #[codec(index = 179)]
                        Mortal179(::core::primitive::u8),
                        #[codec(index = 180)]
                        Mortal180(::core::primitive::u8),
                        #[codec(index = 181)]
                        Mortal181(::core::primitive::u8),
                        #[codec(index = 182)]
                        Mortal182(::core::primitive::u8),
                        #[codec(index = 183)]
                        Mortal183(::core::primitive::u8),
                        #[codec(index = 184)]
                        Mortal184(::core::primitive::u8),
                        #[codec(index = 185)]
                        Mortal185(::core::primitive::u8),
                        #[codec(index = 186)]
                        Mortal186(::core::primitive::u8),
                        #[codec(index = 187)]
                        Mortal187(::core::primitive::u8),
                        #[codec(index = 188)]
                        Mortal188(::core::primitive::u8),
                        #[codec(index = 189)]
                        Mortal189(::core::primitive::u8),
                        #[codec(index = 190)]
                        Mortal190(::core::primitive::u8),
                        #[codec(index = 191)]
                        Mortal191(::core::primitive::u8),
                        #[codec(index = 192)]
                        Mortal192(::core::primitive::u8),
                        #[codec(index = 193)]
                        Mortal193(::core::primitive::u8),
                        #[codec(index = 194)]
                        Mortal194(::core::primitive::u8),
                        #[codec(index = 195)]
                        Mortal195(::core::primitive::u8),
                        #[codec(index = 196)]
                        Mortal196(::core::primitive::u8),
                        #[codec(index = 197)]
                        Mortal197(::core::primitive::u8),
                        #[codec(index = 198)]
                        Mortal198(::core::primitive::u8),
                        #[codec(index = 199)]
                        Mortal199(::core::primitive::u8),
                        #[codec(index = 200)]
                        Mortal200(::core::primitive::u8),
                        #[codec(index = 201)]
                        Mortal201(::core::primitive::u8),
                        #[codec(index = 202)]
                        Mortal202(::core::primitive::u8),
                        #[codec(index = 203)]
                        Mortal203(::core::primitive::u8),
                        #[codec(index = 204)]
                        Mortal204(::core::primitive::u8),
                        #[codec(index = 205)]
                        Mortal205(::core::primitive::u8),
                        #[codec(index = 206)]
                        Mortal206(::core::primitive::u8),
                        #[codec(index = 207)]
                        Mortal207(::core::primitive::u8),
                        #[codec(index = 208)]
                        Mortal208(::core::primitive::u8),
                        #[codec(index = 209)]
                        Mortal209(::core::primitive::u8),
                        #[codec(index = 210)]
                        Mortal210(::core::primitive::u8),
                        #[codec(index = 211)]
                        Mortal211(::core::primitive::u8),
                        #[codec(index = 212)]
                        Mortal212(::core::primitive::u8),
                        #[codec(index = 213)]
                        Mortal213(::core::primitive::u8),
                        #[codec(index = 214)]
                        Mortal214(::core::primitive::u8),
                        #[codec(index = 215)]
                        Mortal215(::core::primitive::u8),
                        #[codec(index = 216)]
                        Mortal216(::core::primitive::u8),
                        #[codec(index = 217)]
                        Mortal217(::core::primitive::u8),
                        #[codec(index = 218)]
                        Mortal218(::core::primitive::u8),
                        #[codec(index = 219)]
                        Mortal219(::core::primitive::u8),
                        #[codec(index = 220)]
                        Mortal220(::core::primitive::u8),
                        #[codec(index = 221)]
                        Mortal221(::core::primitive::u8),
                        #[codec(index = 222)]
                        Mortal222(::core::primitive::u8),
                        #[codec(index = 223)]
                        Mortal223(::core::primitive::u8),
                        #[codec(index = 224)]
                        Mortal224(::core::primitive::u8),
                        #[codec(index = 225)]
                        Mortal225(::core::primitive::u8),
                        #[codec(index = 226)]
                        Mortal226(::core::primitive::u8),
                        #[codec(index = 227)]
                        Mortal227(::core::primitive::u8),
                        #[codec(index = 228)]
                        Mortal228(::core::primitive::u8),
                        #[codec(index = 229)]
                        Mortal229(::core::primitive::u8),
                        #[codec(index = 230)]
                        Mortal230(::core::primitive::u8),
                        #[codec(index = 231)]
                        Mortal231(::core::primitive::u8),
                        #[codec(index = 232)]
                        Mortal232(::core::primitive::u8),
                        #[codec(index = 233)]
                        Mortal233(::core::primitive::u8),
                        #[codec(index = 234)]
                        Mortal234(::core::primitive::u8),
                        #[codec(index = 235)]
                        Mortal235(::core::primitive::u8),
                        #[codec(index = 236)]
                        Mortal236(::core::primitive::u8),
                        #[codec(index = 237)]
                        Mortal237(::core::primitive::u8),
                        #[codec(index = 238)]
                        Mortal238(::core::primitive::u8),
                        #[codec(index = 239)]
                        Mortal239(::core::primitive::u8),
                        #[codec(index = 240)]
                        Mortal240(::core::primitive::u8),
                        #[codec(index = 241)]
                        Mortal241(::core::primitive::u8),
                        #[codec(index = 242)]
                        Mortal242(::core::primitive::u8),
                        #[codec(index = 243)]
                        Mortal243(::core::primitive::u8),
                        #[codec(index = 244)]
                        Mortal244(::core::primitive::u8),
                        #[codec(index = 245)]
                        Mortal245(::core::primitive::u8),
                        #[codec(index = 246)]
                        Mortal246(::core::primitive::u8),
                        #[codec(index = 247)]
                        Mortal247(::core::primitive::u8),
                        #[codec(index = 248)]
                        Mortal248(::core::primitive::u8),
                        #[codec(index = 249)]
                        Mortal249(::core::primitive::u8),
                        #[codec(index = 250)]
                        Mortal250(::core::primitive::u8),
                        #[codec(index = 251)]
                        Mortal251(::core::primitive::u8),
                        #[codec(index = 252)]
                        Mortal252(::core::primitive::u8),
                        #[codec(index = 253)]
                        Mortal253(::core::primitive::u8),
                        #[codec(index = 254)]
                        Mortal254(::core::primitive::u8),
                        #[codec(index = 255)]
                        Mortal255(::core::primitive::u8),
                    }
                }
                pub mod header {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct Header<_0> {
                        pub parent_hash: ::subxt::ext::subxt_core::utils::H256,
                        #[codec(compact)]
                        pub number: _0,
                        pub state_root: ::subxt::ext::subxt_core::utils::H256,
                        pub extrinsics_root: ::subxt::ext::subxt_core::utils::H256,
                        pub digest: runtime_types::sp_runtime::generic::digest::Digest,
                    }
                }
            }
            pub mod transaction_validity {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum InvalidTransaction {
                    #[codec(index = 0)]
                    Call,
                    #[codec(index = 1)]
                    Payment,
                    #[codec(index = 2)]
                    Future,
                    #[codec(index = 3)]
                    Stale,
                    #[codec(index = 4)]
                    BadProof,
                    #[codec(index = 5)]
                    AncientBirthBlock,
                    #[codec(index = 6)]
                    ExhaustsResources,
                    #[codec(index = 7)]
                    Custom(::core::primitive::u8),
                    #[codec(index = 8)]
                    BadMandatory,
                    #[codec(index = 9)]
                    MandatoryValidation,
                    #[codec(index = 10)]
                    BadSigner,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum TransactionSource {
                    #[codec(index = 0)]
                    InBlock,
                    #[codec(index = 1)]
                    Local,
                    #[codec(index = 2)]
                    External,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum TransactionValidityError {
                    #[codec(index = 0)]
                    Invalid(runtime_types::sp_runtime::transaction_validity::InvalidTransaction),
                    #[codec(index = 1)]
                    Unknown(runtime_types::sp_runtime::transaction_validity::UnknownTransaction),
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum UnknownTransaction {
                    #[codec(index = 0)]
                    CannotLookup,
                    #[codec(index = 1)]
                    NoUnsignedValidator,
                    #[codec(index = 2)]
                    Custom(::core::primitive::u8),
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ValidTransaction {
                    pub priority: ::core::primitive::u64,
                    pub requires: ::subxt::ext::subxt_core::alloc::vec::Vec<
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    >,
                    pub provides: ::subxt::ext::subxt_core::alloc::vec::Vec<
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    >,
                    pub longevity: ::core::primitive::u64,
                    pub propagate: ::core::primitive::bool,
                }
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum DispatchError {
                #[codec(index = 0)]
                Other,
                #[codec(index = 1)]
                CannotLookup,
                #[codec(index = 2)]
                BadOrigin,
                #[codec(index = 3)]
                Module(runtime_types::sp_runtime::ModuleError),
                #[codec(index = 4)]
                ConsumerRemaining,
                #[codec(index = 5)]
                NoProviders,
                #[codec(index = 6)]
                TooManyConsumers,
                #[codec(index = 7)]
                Token(runtime_types::sp_runtime::TokenError),
                #[codec(index = 8)]
                Arithmetic(runtime_types::sp_arithmetic::ArithmeticError),
                #[codec(index = 9)]
                Transactional(runtime_types::sp_runtime::TransactionalError),
                #[codec(index = 10)]
                Exhausted,
                #[codec(index = 11)]
                Corruption,
                #[codec(index = 12)]
                Unavailable,
                #[codec(index = 13)]
                RootNotAllowed,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum ExtrinsicInclusionMode {
                #[codec(index = 0)]
                AllExtrinsics,
                #[codec(index = 1)]
                OnlyInherents,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct ModuleError {
                pub index: ::core::primitive::u8,
                pub error: [::core::primitive::u8; 4usize],
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum MultiSignature {
                #[codec(index = 0)]
                Ed25519(runtime_types::sp_core::ed25519::Signature),
                #[codec(index = 1)]
                Sr25519(runtime_types::sp_core::sr25519::Signature),
                #[codec(index = 2)]
                Ecdsa(runtime_types::sp_core::ecdsa::Signature),
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum TokenError {
                #[codec(index = 0)]
                FundsUnavailable,
                #[codec(index = 1)]
                OnlyProvider,
                #[codec(index = 2)]
                BelowMinimum,
                #[codec(index = 3)]
                CannotCreate,
                #[codec(index = 4)]
                UnknownAsset,
                #[codec(index = 5)]
                Frozen,
                #[codec(index = 6)]
                Unsupported,
                #[codec(index = 7)]
                CannotCreateHold,
                #[codec(index = 8)]
                NotExpendable,
                #[codec(index = 9)]
                Blocked,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum TransactionalError {
                #[codec(index = 0)]
                LimitReached,
                #[codec(index = 1)]
                NoLayer,
            }
        }
        pub mod sp_version {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct RuntimeVersion {
                pub spec_name: ::subxt::ext::subxt_core::alloc::string::String,
                pub impl_name: ::subxt::ext::subxt_core::alloc::string::String,
                pub authoring_version: ::core::primitive::u32,
                pub spec_version: ::core::primitive::u32,
                pub impl_version: ::core::primitive::u32,
                pub apis: ::subxt::ext::subxt_core::alloc::vec::Vec<(
                    [::core::primitive::u8; 8usize],
                    ::core::primitive::u32,
                )>,
                pub transaction_version: ::core::primitive::u32,
                pub state_version: ::core::primitive::u8,
            }
        }
        pub mod sp_weights {
            use super::runtime_types;
            pub mod weight_v2 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Weight {
                    #[codec(compact)]
                    pub ref_time: ::core::primitive::u64,
                    #[codec(compact)]
                    pub proof_size: ::core::primitive::u64,
                }
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct RuntimeDbWeight {
                pub read: ::core::primitive::u64,
                pub write: ::core::primitive::u64,
            }
        }
    }
}
