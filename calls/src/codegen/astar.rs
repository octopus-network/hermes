#[allow(dead_code, unused_imports, non_camel_case_types)]
pub mod api {
    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
    pub enum Event {
        #[codec(index = 10)]
        System(system::Event),
        #[codec(index = 11)]
        Utility(utility::Event),
        #[codec(index = 12)]
        Identity(identity::Event),
        #[codec(index = 14)]
        Multisig(multisig::Event),
        #[codec(index = 20)]
        ParachainSystem(parachain_system::Event),
        #[codec(index = 31)]
        Balances(balances::Event),
        #[codec(index = 32)]
        Vesting(vesting::Event),
        #[codec(index = 34)]
        DappsStaking(dapps_staking::Event),
        #[codec(index = 41)]
        CollatorSelection(collator_selection::Event),
        #[codec(index = 42)]
        Session(session::Event),
        #[codec(index = 60)]
        EVM(evm::Event),
        #[codec(index = 61)]
        Ethereum(ethereum::Event),
        #[codec(index = 62)]
        EthCall(eth_call::Event),
        #[codec(index = 99)]
        Sudo(sudo::Event),
        #[codec(index = 110)]
        Ibc(ibc::Event),
    }
    pub mod system {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct FillBlock {
                pub ratio: runtime_types::sp_arithmetic::per_things::Perbill,
            }
            impl ::subxt::Call for FillBlock {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "fill_block";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Remark {
                pub remark: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for Remark {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "remark";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetHeapPages {
                pub pages: ::core::primitive::u64,
            }
            impl ::subxt::Call for SetHeapPages {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_heap_pages";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetCode {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for SetCode {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_code";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetCodeWithoutChecks {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for SetCodeWithoutChecks {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_code_without_checks";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetChangesTrieConfig {
                pub changes_trie_config: ::core::option::Option<
                    runtime_types::sp_core::changes_trie::ChangesTrieConfiguration,
                >,
            }
            impl ::subxt::Call for SetChangesTrieConfig {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_changes_trie_config";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetStorage {
                pub items: ::std::vec::Vec<(
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::std::vec::Vec<::core::primitive::u8>,
                )>,
            }
            impl ::subxt::Call for SetStorage {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_storage";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct KillStorage {
                pub keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
            }
            impl ::subxt::Call for KillStorage {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "kill_storage";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct KillPrefix {
                pub prefix: ::std::vec::Vec<::core::primitive::u8>,
                pub subkeys: ::core::primitive::u32,
            }
            impl ::subxt::Call for KillPrefix {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "kill_prefix";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RemarkWithEvent {
                pub remark: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for RemarkWithEvent {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "remark_with_event";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn fill_block(
                    &self,
                    ratio: runtime_types::sp_arithmetic::per_things::Perbill,
                ) -> ::subxt::SubmittableExtrinsic<T, FillBlock> {
                    let call = FillBlock { ratio };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remark(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, Remark> {
                    let call = Remark { remark };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_heap_pages(
                    &self,
                    pages: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<T, SetHeapPages> {
                    let call = SetHeapPages { pages };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_code(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, SetCode> {
                    let call = SetCode { code };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_code_without_checks(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, SetCodeWithoutChecks> {
                    let call = SetCodeWithoutChecks { code };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_changes_trie_config(
                    &self,
                    changes_trie_config: ::core::option::Option<
                        runtime_types::sp_core::changes_trie::ChangesTrieConfiguration,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, SetChangesTrieConfig> {
                    let call = SetChangesTrieConfig {
                        changes_trie_config,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_storage(
                    &self,
                    items: ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                ) -> ::subxt::SubmittableExtrinsic<T, SetStorage> {
                    let call = SetStorage { items };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn kill_storage(
                    &self,
                    keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                ) -> ::subxt::SubmittableExtrinsic<T, KillStorage> {
                    let call = KillStorage { keys };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn kill_prefix(
                    &self,
                    prefix: ::std::vec::Vec<::core::primitive::u8>,
                    subkeys: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, KillPrefix> {
                    let call = KillPrefix { prefix, subkeys };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remark_with_event(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, RemarkWithEvent> {
                    let call = RemarkWithEvent { remark };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::frame_system::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ExtrinsicSuccess(pub runtime_types::frame_support::weights::DispatchInfo);
            impl ::subxt::Event for ExtrinsicSuccess {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicSuccess";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ExtrinsicFailed(
                pub runtime_types::sp_runtime::DispatchError,
                pub runtime_types::frame_support::weights::DispatchInfo,
            );
            impl ::subxt::Event for ExtrinsicFailed {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicFailed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CodeUpdated {}
            impl ::subxt::Event for CodeUpdated {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "CodeUpdated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NewAccount(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for NewAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "NewAccount";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct KilledAccount(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for KilledAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "KilledAccount";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Remarked(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::H256,
            );
            impl ::subxt::Event for Remarked {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "Remarked";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Account(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Account {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Account";
                type Value = runtime_types::frame_system::AccountInfo<
                    ::core::primitive::u32,
                    runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct ExtrinsicCount;
            impl ::subxt::StorageEntry for ExtrinsicCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ExtrinsicCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct BlockWeight;
            impl ::subxt::StorageEntry for BlockWeight {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "BlockWeight";
                type Value =
                    runtime_types::frame_support::weights::PerDispatchClass<::core::primitive::u64>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AllExtrinsicsLen;
            impl ::subxt::StorageEntry for AllExtrinsicsLen {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "AllExtrinsicsLen";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct BlockHash(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for BlockHash {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "BlockHash";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct ExtrinsicData(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ExtrinsicData {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ExtrinsicData";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Number;
            impl ::subxt::StorageEntry for Number {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Number";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ParentHash;
            impl ::subxt::StorageEntry for ParentHash {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ParentHash";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Digest;
            impl ::subxt::StorageEntry for Digest {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Digest";
                type Value =
                    runtime_types::sp_runtime::generic::digest::Digest<::subxt::sp_core::H256>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Events;
            impl ::subxt::StorageEntry for Events {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Events";
                type Value = ::std::vec::Vec<
                    runtime_types::frame_system::EventRecord<
                        runtime_types::shiden_runtime::Event,
                        ::subxt::sp_core::H256,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EventCount;
            impl ::subxt::StorageEntry for EventCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "EventCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EventTopics(pub ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for EventTopics {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "EventTopics";
                type Value = ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct LastRuntimeUpgrade;
            impl ::subxt::StorageEntry for LastRuntimeUpgrade {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "LastRuntimeUpgrade";
                type Value = runtime_types::frame_system::LastRuntimeUpgradeInfo;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UpgradedToU32RefCount;
            impl ::subxt::StorageEntry for UpgradedToU32RefCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "UpgradedToU32RefCount";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UpgradedToTripleRefCount;
            impl ::subxt::StorageEntry for UpgradedToTripleRefCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "UpgradedToTripleRefCount";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ExecutionPhase;
            impl ::subxt::StorageEntry for ExecutionPhase {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ExecutionPhase";
                type Value = runtime_types::frame_system::Phase;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn account(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_system::AccountInfo<
                        ::core::primitive::u32,
                        runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Account(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Account>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn extrinsic_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = ExtrinsicCount;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn block_weight(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::weights::PerDispatchClass<::core::primitive::u64>,
                    ::subxt::Error,
                > {
                    let entry = BlockWeight;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn all_extrinsics_len(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = AllExtrinsicsLen;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn block_hash(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::Error>
                {
                    let entry = BlockHash(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn block_hash_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, BlockHash>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn extrinsic_data(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::std::vec::Vec<::core::primitive::u8>, ::subxt::Error>
                {
                    let entry = ExtrinsicData(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn extrinsic_data_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, ExtrinsicData>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn number(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = Number;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn parent_hash(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::Error>
                {
                    let entry = ParentHash;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn digest(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_runtime::generic::digest::Digest<::subxt::sp_core::H256>,
                    ::subxt::Error,
                > {
                    let entry = Digest;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn events(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::frame_system::EventRecord<
                            runtime_types::shiden_runtime::Event,
                            ::subxt::sp_core::H256,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Events;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn event_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = EventCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn event_topics(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
                    ::subxt::Error,
                > {
                    let entry = EventTopics(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn event_topics_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, EventTopics>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn last_runtime_upgrade(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::frame_system::LastRuntimeUpgradeInfo>,
                    ::subxt::Error,
                > {
                    let entry = LastRuntimeUpgrade;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn upgraded_to_u32_ref_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::Error>
                {
                    let entry = UpgradedToU32RefCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn upgraded_to_triple_ref_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::Error>
                {
                    let entry = UpgradedToTripleRefCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn execution_phase(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::frame_system::Phase>,
                    ::subxt::Error,
                > {
                    let entry = ExecutionPhase;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod utility {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Batch {
                pub calls: ::std::vec::Vec<runtime_types::shiden_runtime::Call>,
            }
            impl ::subxt::Call for Batch {
                const PALLET: &'static str = "Utility";
                const FUNCTION: &'static str = "batch";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AsDerivative {
                pub index: ::core::primitive::u16,
                pub call: runtime_types::shiden_runtime::Call,
            }
            impl ::subxt::Call for AsDerivative {
                const PALLET: &'static str = "Utility";
                const FUNCTION: &'static str = "as_derivative";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BatchAll {
                pub calls: ::std::vec::Vec<runtime_types::shiden_runtime::Call>,
            }
            impl ::subxt::Call for BatchAll {
                const PALLET: &'static str = "Utility";
                const FUNCTION: &'static str = "batch_all";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn batch(
                    &self,
                    calls: ::std::vec::Vec<runtime_types::shiden_runtime::Call>,
                ) -> ::subxt::SubmittableExtrinsic<T, Batch> {
                    let call = Batch { calls };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn as_derivative(
                    &self,
                    index: ::core::primitive::u16,
                    call: runtime_types::shiden_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<T, AsDerivative> {
                    let call = AsDerivative { index, call };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn batch_all(
                    &self,
                    calls: ::std::vec::Vec<runtime_types::shiden_runtime::Call>,
                ) -> ::subxt::SubmittableExtrinsic<T, BatchAll> {
                    let call = BatchAll { calls };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_utility::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BatchInterrupted(
                pub ::core::primitive::u32,
                pub runtime_types::sp_runtime::DispatchError,
            );
            impl ::subxt::Event for BatchInterrupted {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "BatchInterrupted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BatchCompleted {}
            impl ::subxt::Event for BatchCompleted {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "BatchCompleted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ItemCompleted {}
            impl ::subxt::Event for ItemCompleted {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "ItemCompleted";
            }
        }
    }
    pub mod identity {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AddRegistrar {
                pub account: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for AddRegistrar {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "add_registrar";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetIdentity {
                pub info: runtime_types::pallet_identity::types::IdentityInfo,
            }
            impl ::subxt::Call for SetIdentity {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "set_identity";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetSubs {
                pub subs: ::std::vec::Vec<(
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::pallet_identity::types::Data,
                )>,
            }
            impl ::subxt::Call for SetSubs {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "set_subs";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ClearIdentity {}
            impl ::subxt::Call for ClearIdentity {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "clear_identity";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RequestJudgement {
                #[codec(compact)]
                pub reg_index: ::core::primitive::u32,
                #[codec(compact)]
                pub max_fee: ::core::primitive::u128,
            }
            impl ::subxt::Call for RequestJudgement {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "request_judgement";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CancelRequest {
                pub reg_index: ::core::primitive::u32,
            }
            impl ::subxt::Call for CancelRequest {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "cancel_request";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetFee {
                #[codec(compact)]
                pub index: ::core::primitive::u32,
                #[codec(compact)]
                pub fee: ::core::primitive::u128,
            }
            impl ::subxt::Call for SetFee {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "set_fee";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetAccountId {
                #[codec(compact)]
                pub index: ::core::primitive::u32,
                pub new: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for SetAccountId {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "set_account_id";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetFields {
                #[codec(compact)]
                pub index: ::core::primitive::u32,
                pub fields: runtime_types::pallet_identity::types::BitFlags<
                    runtime_types::pallet_identity::types::IdentityField,
                >,
            }
            impl ::subxt::Call for SetFields {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "set_fields";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ProvideJudgement {
                #[codec(compact)]
                pub reg_index: ::core::primitive::u32,
                pub target:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub judgement:
                    runtime_types::pallet_identity::types::Judgement<::core::primitive::u128>,
            }
            impl ::subxt::Call for ProvideJudgement {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "provide_judgement";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct KillIdentity {
                pub target:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for KillIdentity {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "kill_identity";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AddSub {
                pub sub:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub data: runtime_types::pallet_identity::types::Data,
            }
            impl ::subxt::Call for AddSub {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "add_sub";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RenameSub {
                pub sub:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub data: runtime_types::pallet_identity::types::Data,
            }
            impl ::subxt::Call for RenameSub {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "rename_sub";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RemoveSub {
                pub sub:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for RemoveSub {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "remove_sub";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct QuitSub {}
            impl ::subxt::Call for QuitSub {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "quit_sub";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn add_registrar(
                    &self,
                    account: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<T, AddRegistrar> {
                    let call = AddRegistrar { account };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_identity(
                    &self,
                    info: runtime_types::pallet_identity::types::IdentityInfo,
                ) -> ::subxt::SubmittableExtrinsic<T, SetIdentity> {
                    let call = SetIdentity { info };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_subs(
                    &self,
                    subs: ::std::vec::Vec<(
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::pallet_identity::types::Data,
                    )>,
                ) -> ::subxt::SubmittableExtrinsic<T, SetSubs> {
                    let call = SetSubs { subs };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn clear_identity(&self) -> ::subxt::SubmittableExtrinsic<T, ClearIdentity> {
                    let call = ClearIdentity {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn request_judgement(
                    &self,
                    reg_index: ::core::primitive::u32,
                    max_fee: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, RequestJudgement> {
                    let call = RequestJudgement { reg_index, max_fee };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_request(
                    &self,
                    reg_index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, CancelRequest> {
                    let call = CancelRequest { reg_index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_fee(
                    &self,
                    index: ::core::primitive::u32,
                    fee: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, SetFee> {
                    let call = SetFee { index, fee };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_account_id(
                    &self,
                    index: ::core::primitive::u32,
                    new: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetAccountId> {
                    let call = SetAccountId { index, new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_fields(
                    &self,
                    index: ::core::primitive::u32,
                    fields: runtime_types::pallet_identity::types::BitFlags<
                        runtime_types::pallet_identity::types::IdentityField,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, SetFields> {
                    let call = SetFields { index, fields };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn provide_judgement(
                    &self,
                    reg_index: ::core::primitive::u32,
                    target: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    judgement: runtime_types::pallet_identity::types::Judgement<
                        ::core::primitive::u128,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, ProvideJudgement> {
                    let call = ProvideJudgement {
                        reg_index,
                        target,
                        judgement,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn kill_identity(
                    &self,
                    target: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, KillIdentity> {
                    let call = KillIdentity { target };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn add_sub(
                    &self,
                    sub: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    data: runtime_types::pallet_identity::types::Data,
                ) -> ::subxt::SubmittableExtrinsic<T, AddSub> {
                    let call = AddSub { sub, data };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn rename_sub(
                    &self,
                    sub: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    data: runtime_types::pallet_identity::types::Data,
                ) -> ::subxt::SubmittableExtrinsic<T, RenameSub> {
                    let call = RenameSub { sub, data };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remove_sub(
                    &self,
                    sub: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, RemoveSub> {
                    let call = RemoveSub { sub };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn quit_sub(&self) -> ::subxt::SubmittableExtrinsic<T, QuitSub> {
                    let call = QuitSub {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_identity::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct IdentitySet(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for IdentitySet {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "IdentitySet";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct IdentityCleared(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for IdentityCleared {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "IdentityCleared";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct IdentityKilled(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for IdentityKilled {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "IdentityKilled";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct JudgementRequested(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for JudgementRequested {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "JudgementRequested";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct JudgementUnrequested(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for JudgementUnrequested {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "JudgementUnrequested";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct JudgementGiven(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for JudgementGiven {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "JudgementGiven";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RegistrarAdded(pub ::core::primitive::u32);
            impl ::subxt::Event for RegistrarAdded {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "RegistrarAdded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SubIdentityAdded(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for SubIdentityAdded {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "SubIdentityAdded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SubIdentityRemoved(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for SubIdentityRemoved {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "SubIdentityRemoved";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SubIdentityRevoked(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for SubIdentityRevoked {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "SubIdentityRevoked";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct IdentityOf(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for IdentityOf {
                const PALLET: &'static str = "Identity";
                const STORAGE: &'static str = "IdentityOf";
                type Value =
                    runtime_types::pallet_identity::types::Registration<::core::primitive::u128>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct SuperOf(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for SuperOf {
                const PALLET: &'static str = "Identity";
                const STORAGE: &'static str = "SuperOf";
                type Value = (
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::pallet_identity::types::Data,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct SubsOf(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for SubsOf {
                const PALLET: &'static str = "Identity";
                const STORAGE: &'static str = "SubsOf";
                type Value = (
                    ::core::primitive::u128,
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Registrars;
            impl ::subxt::StorageEntry for Registrars {
                const PALLET: &'static str = "Identity";
                const STORAGE: &'static str = "Registrars";
                type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::option::Option<
                        runtime_types::pallet_identity::types::RegistrarInfo<
                            ::core::primitive::u128,
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn identity_of(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_identity::types::Registration<
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = IdentityOf(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn identity_of_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, IdentityOf>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn super_of(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::pallet_identity::types::Data,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = SuperOf(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn super_of_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, SuperOf>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn subs_of(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    (
                        ::core::primitive::u128,
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    ),
                    ::subxt::Error,
                > {
                    let entry = SubsOf(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn subs_of_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, SubsOf>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn registrars(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::option::Option<
                            runtime_types::pallet_identity::types::RegistrarInfo<
                                ::core::primitive::u128,
                                ::subxt::sp_core::crypto::AccountId32,
                            >,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Registrars;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod timestamp {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Set {
                #[codec(compact)]
                pub now: ::core::primitive::u64,
            }
            impl ::subxt::Call for Set {
                const PALLET: &'static str = "Timestamp";
                const FUNCTION: &'static str = "set";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn set(
                    &self,
                    now: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<T, Set> {
                    let call = Set { now };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Now;
            impl ::subxt::StorageEntry for Now {
                const PALLET: &'static str = "Timestamp";
                const STORAGE: &'static str = "Now";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct DidUpdate;
            impl ::subxt::StorageEntry for DidUpdate {
                const PALLET: &'static str = "Timestamp";
                const STORAGE: &'static str = "DidUpdate";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn now(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::Error>
                {
                    let entry = Now;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn did_update(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::Error>
                {
                    let entry = DidUpdate;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod multisig {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AsMultiThreshold1 {
                pub other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                pub call: runtime_types::shiden_runtime::Call,
            }
            impl ::subxt::Call for AsMultiThreshold1 {
                const PALLET: &'static str = "Multisig";
                const FUNCTION: &'static str = "as_multi_threshold1";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AsMulti {
                pub threshold: ::core::primitive::u16,
                pub other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                pub maybe_timepoint: ::core::option::Option<
                    runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                >,
                pub call: ::std::vec::Vec<::core::primitive::u8>,
                pub store_call: ::core::primitive::bool,
                pub max_weight: ::core::primitive::u64,
            }
            impl ::subxt::Call for AsMulti {
                const PALLET: &'static str = "Multisig";
                const FUNCTION: &'static str = "as_multi";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ApproveAsMulti {
                pub threshold: ::core::primitive::u16,
                pub other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                pub maybe_timepoint: ::core::option::Option<
                    runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                >,
                pub call_hash: [::core::primitive::u8; 32usize],
                pub max_weight: ::core::primitive::u64,
            }
            impl ::subxt::Call for ApproveAsMulti {
                const PALLET: &'static str = "Multisig";
                const FUNCTION: &'static str = "approve_as_multi";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CancelAsMulti {
                pub threshold: ::core::primitive::u16,
                pub other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                pub call_hash: [::core::primitive::u8; 32usize],
            }
            impl ::subxt::Call for CancelAsMulti {
                const PALLET: &'static str = "Multisig";
                const FUNCTION: &'static str = "cancel_as_multi";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn as_multi_threshold1(
                    &self,
                    other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    call: runtime_types::shiden_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<T, AsMultiThreshold1> {
                    let call = AsMultiThreshold1 {
                        other_signatories,
                        call,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn as_multi(
                    &self,
                    threshold: ::core::primitive::u16,
                    other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    maybe_timepoint: ::core::option::Option<
                        runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                    >,
                    call: ::std::vec::Vec<::core::primitive::u8>,
                    store_call: ::core::primitive::bool,
                    max_weight: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<T, AsMulti> {
                    let call = AsMulti {
                        threshold,
                        other_signatories,
                        maybe_timepoint,
                        call,
                        store_call,
                        max_weight,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn approve_as_multi(
                    &self,
                    threshold: ::core::primitive::u16,
                    other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    maybe_timepoint: ::core::option::Option<
                        runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                    >,
                    call_hash: [::core::primitive::u8; 32usize],
                    max_weight: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<T, ApproveAsMulti> {
                    let call = ApproveAsMulti {
                        threshold,
                        other_signatories,
                        maybe_timepoint,
                        call_hash,
                        max_weight,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_as_multi(
                    &self,
                    threshold: ::core::primitive::u16,
                    other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                    call_hash: [::core::primitive::u8; 32usize],
                ) -> ::subxt::SubmittableExtrinsic<T, CancelAsMulti> {
                    let call = CancelAsMulti {
                        threshold,
                        other_signatories,
                        timepoint,
                        call_hash,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_multisig::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NewMultisig(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub [::core::primitive::u8; 32usize],
            );
            impl ::subxt::Event for NewMultisig {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "NewMultisig";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct MultisigApproval(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub [::core::primitive::u8; 32usize],
            );
            impl ::subxt::Event for MultisigApproval {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "MultisigApproval";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct MultisigExecuted(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub [::core::primitive::u8; 32usize],
                pub ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            );
            impl ::subxt::Event for MultisigExecuted {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "MultisigExecuted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct MultisigCancelled(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub [::core::primitive::u8; 32usize],
            );
            impl ::subxt::Event for MultisigCancelled {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "MultisigCancelled";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Multisigs(
                ::subxt::sp_core::crypto::AccountId32,
                [::core::primitive::u8; 32usize],
            );
            impl ::subxt::StorageEntry for Multisigs {
                const PALLET: &'static str = "Multisig";
                const STORAGE: &'static str = "Multisigs";
                type Value = runtime_types::pallet_multisig::Multisig<
                    ::core::primitive::u32,
                    ::core::primitive::u128,
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct Calls(pub [::core::primitive::u8; 32usize]);
            impl ::subxt::StorageEntry for Calls {
                const PALLET: &'static str = "Multisig";
                const STORAGE: &'static str = "Calls";
                type Value = (
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn multisigs(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    _1: [::core::primitive::u8; 32usize],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_multisig::Multisig<
                            ::core::primitive::u32,
                            ::core::primitive::u128,
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Multisigs(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn multisigs_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Multisigs>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn calls(
                    &self,
                    _0: [::core::primitive::u8; 32usize],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = Calls(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn calls_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Calls>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod parachain_system {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetValidationData {
                pub data:
                    runtime_types::cumulus_primitives_parachain_inherent::ParachainInherentData,
            }
            impl ::subxt::Call for SetValidationData {
                const PALLET: &'static str = "ParachainSystem";
                const FUNCTION: &'static str = "set_validation_data";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SudoSendUpwardMessage {
                pub message: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for SudoSendUpwardMessage {
                const PALLET: &'static str = "ParachainSystem";
                const FUNCTION: &'static str = "sudo_send_upward_message";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AuthorizeUpgrade {
                pub code_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for AuthorizeUpgrade {
                const PALLET: &'static str = "ParachainSystem";
                const FUNCTION: &'static str = "authorize_upgrade";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct EnactAuthorizedUpgrade {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for EnactAuthorizedUpgrade {
                const PALLET: &'static str = "ParachainSystem";
                const FUNCTION: &'static str = "enact_authorized_upgrade";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn set_validation_data(
                    &self,
                    data : runtime_types :: cumulus_primitives_parachain_inherent :: ParachainInherentData,
                ) -> ::subxt::SubmittableExtrinsic<T, SetValidationData> {
                    let call = SetValidationData { data };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn sudo_send_upward_message(
                    &self,
                    message: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, SudoSendUpwardMessage> {
                    let call = SudoSendUpwardMessage { message };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn authorize_upgrade(
                    &self,
                    code_hash: ::subxt::sp_core::H256,
                ) -> ::subxt::SubmittableExtrinsic<T, AuthorizeUpgrade> {
                    let call = AuthorizeUpgrade { code_hash };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn enact_authorized_upgrade(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, EnactAuthorizedUpgrade> {
                    let call = EnactAuthorizedUpgrade { code };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::cumulus_pallet_parachain_system::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ValidationFunctionStored {}
            impl ::subxt::Event for ValidationFunctionStored {
                const PALLET: &'static str = "ParachainSystem";
                const EVENT: &'static str = "ValidationFunctionStored";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ValidationFunctionApplied(pub ::core::primitive::u32);
            impl ::subxt::Event for ValidationFunctionApplied {
                const PALLET: &'static str = "ParachainSystem";
                const EVENT: &'static str = "ValidationFunctionApplied";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ValidationFunctionDiscarded {}
            impl ::subxt::Event for ValidationFunctionDiscarded {
                const PALLET: &'static str = "ParachainSystem";
                const EVENT: &'static str = "ValidationFunctionDiscarded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct UpgradeAuthorized(pub ::subxt::sp_core::H256);
            impl ::subxt::Event for UpgradeAuthorized {
                const PALLET: &'static str = "ParachainSystem";
                const EVENT: &'static str = "UpgradeAuthorized";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct DownwardMessagesReceived(pub ::core::primitive::u32);
            impl ::subxt::Event for DownwardMessagesReceived {
                const PALLET: &'static str = "ParachainSystem";
                const EVENT: &'static str = "DownwardMessagesReceived";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct DownwardMessagesProcessed(
                pub ::core::primitive::u64,
                pub ::subxt::sp_core::H256,
            );
            impl ::subxt::Event for DownwardMessagesProcessed {
                const PALLET: &'static str = "ParachainSystem";
                const EVENT: &'static str = "DownwardMessagesProcessed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct PendingValidationCode;
            impl ::subxt::StorageEntry for PendingValidationCode {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "PendingValidationCode";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NewValidationCode;
            impl ::subxt::StorageEntry for NewValidationCode {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "NewValidationCode";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ValidationData;
            impl ::subxt::StorageEntry for ValidationData {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "ValidationData";
                type Value = runtime_types::polkadot_primitives::v1::PersistedValidationData<
                    ::subxt::sp_core::H256,
                    ::core::primitive::u32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct DidSetValidationCode;
            impl ::subxt::StorageEntry for DidSetValidationCode {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "DidSetValidationCode";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UpgradeRestrictionSignal;
            impl ::subxt::StorageEntry for UpgradeRestrictionSignal {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "UpgradeRestrictionSignal";
                type Value = ::core::option::Option<
                    runtime_types::polkadot_primitives::v1::UpgradeRestriction,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct RelevantMessagingState;
            impl ::subxt::StorageEntry for RelevantMessagingState {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "RelevantMessagingState";
                type Value = runtime_types :: cumulus_pallet_parachain_system :: relay_state_snapshot :: MessagingStateSnapshot ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct HostConfiguration;
            impl ::subxt::StorageEntry for HostConfiguration {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "HostConfiguration";
                type Value = runtime_types::polkadot_primitives::v1::AbridgedHostConfiguration;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct LastDmqMqcHead;
            impl ::subxt::StorageEntry for LastDmqMqcHead {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "LastDmqMqcHead";
                type Value = runtime_types::cumulus_pallet_parachain_system::MessageQueueChain;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct LastHrmpMqcHeads;
            impl ::subxt::StorageEntry for LastHrmpMqcHeads {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "LastHrmpMqcHeads";
                type Value = ::std::collections::BTreeMap<
                    runtime_types::polkadot_parachain::primitives::Id,
                    runtime_types::cumulus_pallet_parachain_system::MessageQueueChain,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ProcessedDownwardMessages;
            impl ::subxt::StorageEntry for ProcessedDownwardMessages {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "ProcessedDownwardMessages";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct HrmpWatermark;
            impl ::subxt::StorageEntry for HrmpWatermark {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "HrmpWatermark";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct HrmpOutboundMessages;
            impl ::subxt::StorageEntry for HrmpOutboundMessages {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "HrmpOutboundMessages";
                type Value = ::std::vec::Vec<
                    runtime_types::polkadot_core_primitives::OutboundHrmpMessage<
                        runtime_types::polkadot_parachain::primitives::Id,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UpwardMessages;
            impl ::subxt::StorageEntry for UpwardMessages {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "UpwardMessages";
                type Value = ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PendingUpwardMessages;
            impl ::subxt::StorageEntry for PendingUpwardMessages {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "PendingUpwardMessages";
                type Value = ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AnnouncedHrmpMessagesPerCandidate;
            impl ::subxt::StorageEntry for AnnouncedHrmpMessagesPerCandidate {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "AnnouncedHrmpMessagesPerCandidate";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ReservedXcmpWeightOverride;
            impl ::subxt::StorageEntry for ReservedXcmpWeightOverride {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "ReservedXcmpWeightOverride";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ReservedDmpWeightOverride;
            impl ::subxt::StorageEntry for ReservedDmpWeightOverride {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "ReservedDmpWeightOverride";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AuthorizedUpgrade;
            impl ::subxt::StorageEntry for AuthorizedUpgrade {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "AuthorizedUpgrade";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn pending_validation_code(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::std::vec::Vec<::core::primitive::u8>, ::subxt::Error>
                {
                    let entry = PendingValidationCode;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn new_validation_code(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::Error,
                > {
                    let entry = NewValidationCode;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn validation_data(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_primitives::v1::PersistedValidationData<
                            ::subxt::sp_core::H256,
                            ::core::primitive::u32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = ValidationData;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn did_set_validation_code(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::Error>
                {
                    let entry = DidSetValidationCode;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn upgrade_restriction_signal(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_primitives::v1::UpgradeRestriction,
                    >,
                    ::subxt::Error,
                > {
                    let entry = UpgradeRestrictionSignal;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn relevant_messaging_state (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: cumulus_pallet_parachain_system :: relay_state_snapshot :: MessagingStateSnapshot > , :: subxt :: Error >{
                    let entry = RelevantMessagingState;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn host_configuration(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_primitives::v1::AbridgedHostConfiguration,
                    >,
                    ::subxt::Error,
                > {
                    let entry = HostConfiguration;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn last_dmq_mqc_head(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::cumulus_pallet_parachain_system::MessageQueueChain,
                    ::subxt::Error,
                > {
                    let entry = LastDmqMqcHead;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn last_hrmp_mqc_heads(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::collections::BTreeMap<
                        runtime_types::polkadot_parachain::primitives::Id,
                        runtime_types::cumulus_pallet_parachain_system::MessageQueueChain,
                    >,
                    ::subxt::Error,
                > {
                    let entry = LastHrmpMqcHeads;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn processed_downward_messages(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = ProcessedDownwardMessages;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn hrmp_watermark(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = HrmpWatermark;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn hrmp_outbound_messages(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::polkadot_core_primitives::OutboundHrmpMessage<
                            runtime_types::polkadot_parachain::primitives::Id,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = HrmpOutboundMessages;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn upward_messages(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::Error,
                > {
                    let entry = UpwardMessages;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn pending_upward_messages(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::Error,
                > {
                    let entry = PendingUpwardMessages;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn announced_hrmp_messages_per_candidate(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = AnnouncedHrmpMessagesPerCandidate;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn reserved_xcmp_weight_override(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u64>,
                    ::subxt::Error,
                > {
                    let entry = ReservedXcmpWeightOverride;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn reserved_dmp_weight_override(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u64>,
                    ::subxt::Error,
                > {
                    let entry = ReservedDmpWeightOverride;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn authorized_upgrade(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::H256>,
                    ::subxt::Error,
                > {
                    let entry = AuthorizedUpgrade;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod parachain_info {
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct ParachainId;
            impl ::subxt::StorageEntry for ParachainId {
                const PALLET: &'static str = "ParachainInfo";
                const STORAGE: &'static str = "ParachainId";
                type Value = runtime_types::polkadot_parachain::primitives::Id;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn parachain_id(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::polkadot_parachain::primitives::Id,
                    ::subxt::Error,
                > {
                    let entry = ParachainId;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod transaction_payment {
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct NextFeeMultiplier;
            impl ::subxt::StorageEntry for NextFeeMultiplier {
                const PALLET: &'static str = "TransactionPayment";
                const STORAGE: &'static str = "NextFeeMultiplier";
                type Value = runtime_types::sp_arithmetic::fixed_point::FixedU128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageVersion;
            impl ::subxt::StorageEntry for StorageVersion {
                const PALLET: &'static str = "TransactionPayment";
                const STORAGE: &'static str = "StorageVersion";
                type Value = runtime_types::pallet_transaction_payment::Releases;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn next_fee_multiplier(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_arithmetic::fixed_point::FixedU128,
                    ::subxt::Error,
                > {
                    let entry = NextFeeMultiplier;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn storage_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_transaction_payment::Releases,
                    ::subxt::Error,
                > {
                    let entry = StorageVersion;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod balances {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Transfer {
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for Transfer {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetBalance {
                pub who:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub new_free: ::core::primitive::u128,
                #[codec(compact)]
                pub new_reserved: ::core::primitive::u128,
            }
            impl ::subxt::Call for SetBalance {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "set_balance";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceTransfer {
                pub source:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for ForceTransfer {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "force_transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct TransferKeepAlive {
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for TransferKeepAlive {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "transfer_keep_alive";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct TransferAll {
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub keep_alive: ::core::primitive::bool,
            }
            impl ::subxt::Call for TransferAll {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "transfer_all";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceUnreserve {
                pub who:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for ForceUnreserve {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "force_unreserve";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn transfer(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, Transfer> {
                    let call = Transfer { dest, value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_balance(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    new_free: ::core::primitive::u128,
                    new_reserved: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, SetBalance> {
                    let call = SetBalance {
                        who,
                        new_free,
                        new_reserved,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_transfer(
                    &self,
                    source: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceTransfer> {
                    let call = ForceTransfer {
                        source,
                        dest,
                        value,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_keep_alive(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, TransferKeepAlive> {
                    let call = TransferKeepAlive { dest, value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_all(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    keep_alive: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<T, TransferAll> {
                    let call = TransferAll { dest, keep_alive };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_unreserve(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceUnreserve> {
                    let call = ForceUnreserve { who, amount };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_balances::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Endowed(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Endowed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Endowed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct DustLost(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for DustLost {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "DustLost";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Transfer(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Transfer {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BalanceSet(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for BalanceSet {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "BalanceSet";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Reserved(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Reserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Reserved";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Unreserved(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Unreserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Unreserved";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ReserveRepatriated(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
                pub runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
            );
            impl ::subxt::Event for ReserveRepatriated {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "ReserveRepatriated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Deposit(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Deposit {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Deposit";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Withdraw(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Withdraw {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Withdraw";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Slashed(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Slashed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Slashed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct TotalIssuance;
            impl ::subxt::StorageEntry for TotalIssuance {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "TotalIssuance";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Account(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Account {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "Account";
                type Value = runtime_types::pallet_balances::AccountData<::core::primitive::u128>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct Locks(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Locks {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "Locks";
                type Value =
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::pallet_balances::BalanceLock<::core::primitive::u128>,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct Reserves(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Reserves {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "Reserves";
                type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    runtime_types::pallet_balances::ReserveData<
                        [::core::primitive::u8; 8usize],
                        ::core::primitive::u128,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct StorageVersion;
            impl ::subxt::StorageEntry for StorageVersion {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "StorageVersion";
                type Value = runtime_types::pallet_balances::Releases;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn total_issuance(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::Error>
                {
                    let entry = TotalIssuance;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
                    ::subxt::Error,
                > {
                    let entry = Account(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Account>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn locks(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::pallet_balances::BalanceLock<::core::primitive::u128>,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Locks(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn locks_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Locks>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn reserves(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::ReserveData<
                            [::core::primitive::u8; 8usize],
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Reserves(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn reserves_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Reserves>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn storage_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<runtime_types::pallet_balances::Releases, ::subxt::Error>
                {
                    let entry = StorageVersion;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod vesting {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Vest {}
            impl ::subxt::Call for Vest {
                const PALLET: &'static str = "Vesting";
                const FUNCTION: &'static str = "vest";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct VestOther {
                pub target:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for VestOther {
                const PALLET: &'static str = "Vesting";
                const FUNCTION: &'static str = "vest_other";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct VestedTransfer {
                pub target:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub schedule: runtime_types::pallet_vesting::vesting_info::VestingInfo<
                    ::core::primitive::u128,
                    ::core::primitive::u32,
                >,
            }
            impl ::subxt::Call for VestedTransfer {
                const PALLET: &'static str = "Vesting";
                const FUNCTION: &'static str = "vested_transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceVestedTransfer {
                pub source:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub target:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub schedule: runtime_types::pallet_vesting::vesting_info::VestingInfo<
                    ::core::primitive::u128,
                    ::core::primitive::u32,
                >,
            }
            impl ::subxt::Call for ForceVestedTransfer {
                const PALLET: &'static str = "Vesting";
                const FUNCTION: &'static str = "force_vested_transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct MergeSchedules {
                pub schedule1_index: ::core::primitive::u32,
                pub schedule2_index: ::core::primitive::u32,
            }
            impl ::subxt::Call for MergeSchedules {
                const PALLET: &'static str = "Vesting";
                const FUNCTION: &'static str = "merge_schedules";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn vest(&self) -> ::subxt::SubmittableExtrinsic<T, Vest> {
                    let call = Vest {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn vest_other(
                    &self,
                    target: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, VestOther> {
                    let call = VestOther { target };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn vested_transfer(
                    &self,
                    target: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    schedule: runtime_types::pallet_vesting::vesting_info::VestingInfo<
                        ::core::primitive::u128,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, VestedTransfer> {
                    let call = VestedTransfer { target, schedule };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_vested_transfer(
                    &self,
                    source: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    target: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    schedule: runtime_types::pallet_vesting::vesting_info::VestingInfo<
                        ::core::primitive::u128,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceVestedTransfer> {
                    let call = ForceVestedTransfer {
                        source,
                        target,
                        schedule,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn merge_schedules(
                    &self,
                    schedule1_index: ::core::primitive::u32,
                    schedule2_index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, MergeSchedules> {
                    let call = MergeSchedules {
                        schedule1_index,
                        schedule2_index,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_vesting::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct VestingUpdated(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for VestingUpdated {
                const PALLET: &'static str = "Vesting";
                const EVENT: &'static str = "VestingUpdated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct VestingCompleted(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for VestingCompleted {
                const PALLET: &'static str = "Vesting";
                const EVENT: &'static str = "VestingCompleted";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Vesting(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Vesting {
                const PALLET: &'static str = "Vesting";
                const STORAGE: &'static str = "Vesting";
                type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    runtime_types::pallet_vesting::vesting_info::VestingInfo<
                        ::core::primitive::u128,
                        ::core::primitive::u32,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct StorageVersion;
            impl ::subxt::StorageEntry for StorageVersion {
                const PALLET: &'static str = "Vesting";
                const STORAGE: &'static str = "StorageVersion";
                type Value = runtime_types::pallet_vesting::Releases;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn vesting(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            runtime_types::pallet_vesting::vesting_info::VestingInfo<
                                ::core::primitive::u128,
                                ::core::primitive::u32,
                            >,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Vesting(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn vesting_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Vesting>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn storage_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<runtime_types::pallet_vesting::Releases, ::subxt::Error>
                {
                    let entry = StorageVersion;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod block_reward {
        use super::runtime_types;
    }
    pub mod dapps_staking {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Register {
                pub contract_id: runtime_types::shiden_runtime::SmartContract<
                    ::subxt::sp_core::crypto::AccountId32,
                >,
            }
            impl ::subxt::Call for Register {
                const PALLET: &'static str = "DappsStaking";
                const FUNCTION: &'static str = "register";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Unregister {
                pub contract_id: runtime_types::shiden_runtime::SmartContract<
                    ::subxt::sp_core::crypto::AccountId32,
                >,
            }
            impl ::subxt::Call for Unregister {
                const PALLET: &'static str = "DappsStaking";
                const FUNCTION: &'static str = "unregister";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BondAndStake {
                pub contract_id: runtime_types::shiden_runtime::SmartContract<
                    ::subxt::sp_core::crypto::AccountId32,
                >,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for BondAndStake {
                const PALLET: &'static str = "DappsStaking";
                const FUNCTION: &'static str = "bond_and_stake";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct UnbondUnstakeAndWithdraw {
                pub contract_id: runtime_types::shiden_runtime::SmartContract<
                    ::subxt::sp_core::crypto::AccountId32,
                >,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for UnbondUnstakeAndWithdraw {
                const PALLET: &'static str = "DappsStaking";
                const FUNCTION: &'static str = "unbond_unstake_and_withdraw";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Claim {
                pub contract_id: runtime_types::shiden_runtime::SmartContract<
                    ::subxt::sp_core::crypto::AccountId32,
                >,
                pub era: ::core::primitive::u32,
            }
            impl ::subxt::Call for Claim {
                const PALLET: &'static str = "DappsStaking";
                const FUNCTION: &'static str = "claim";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceNewEra {}
            impl ::subxt::Call for ForceNewEra {
                const PALLET: &'static str = "DappsStaking";
                const FUNCTION: &'static str = "force_new_era";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct DeveloperPreApproval {
                pub developer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for DeveloperPreApproval {
                const PALLET: &'static str = "DappsStaking";
                const FUNCTION: &'static str = "developer_pre_approval";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct EnableDeveloperPreApproval {
                pub enabled: ::core::primitive::bool,
            }
            impl ::subxt::Call for EnableDeveloperPreApproval {
                const PALLET: &'static str = "DappsStaking";
                const FUNCTION: &'static str = "enable_developer_pre_approval";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn register(
                    &self,
                    contract_id: runtime_types::shiden_runtime::SmartContract<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, Register> {
                    let call = Register { contract_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn unregister(
                    &self,
                    contract_id: runtime_types::shiden_runtime::SmartContract<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, Unregister> {
                    let call = Unregister { contract_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn bond_and_stake(
                    &self,
                    contract_id: runtime_types::shiden_runtime::SmartContract<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, BondAndStake> {
                    let call = BondAndStake { contract_id, value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn unbond_unstake_and_withdraw(
                    &self,
                    contract_id: runtime_types::shiden_runtime::SmartContract<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, UnbondUnstakeAndWithdraw> {
                    let call = UnbondUnstakeAndWithdraw { contract_id, value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn claim(
                    &self,
                    contract_id: runtime_types::shiden_runtime::SmartContract<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    era: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, Claim> {
                    let call = Claim { contract_id, era };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_new_era(&self) -> ::subxt::SubmittableExtrinsic<T, ForceNewEra> {
                    let call = ForceNewEra {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn developer_pre_approval(
                    &self,
                    developer: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<T, DeveloperPreApproval> {
                    let call = DeveloperPreApproval { developer };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn enable_developer_pre_approval(
                    &self,
                    enabled: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<T, EnableDeveloperPreApproval> {
                    let call = EnableDeveloperPreApproval { enabled };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_dapps_staking::pallet::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BondAndStake(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub  runtime_types::shiden_runtime::SmartContract<
                    ::subxt::sp_core::crypto::AccountId32,
                >,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for BondAndStake {
                const PALLET: &'static str = "DappsStaking";
                const EVENT: &'static str = "BondAndStake";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct UnbondUnstakeAndWithdraw(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub  runtime_types::shiden_runtime::SmartContract<
                    ::subxt::sp_core::crypto::AccountId32,
                >,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for UnbondUnstakeAndWithdraw {
                const PALLET: &'static str = "DappsStaking";
                const EVENT: &'static str = "UnbondUnstakeAndWithdraw";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NewContract(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub  runtime_types::shiden_runtime::SmartContract<
                    ::subxt::sp_core::crypto::AccountId32,
                >,
            );
            impl ::subxt::Event for NewContract {
                const PALLET: &'static str = "DappsStaking";
                const EVENT: &'static str = "NewContract";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ContractRemoved(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub  runtime_types::shiden_runtime::SmartContract<
                    ::subxt::sp_core::crypto::AccountId32,
                >,
            );
            impl ::subxt::Event for ContractRemoved {
                const PALLET: &'static str = "DappsStaking";
                const EVENT: &'static str = "ContractRemoved";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NewDappStakingEra(pub ::core::primitive::u32);
            impl ::subxt::Event for NewDappStakingEra {
                const PALLET: &'static str = "DappsStaking";
                const EVENT: &'static str = "NewDappStakingEra";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Reward(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub  runtime_types::shiden_runtime::SmartContract<
                    ::subxt::sp_core::crypto::AccountId32,
                >,
                pub ::core::primitive::u32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Reward {
                const PALLET: &'static str = "DappsStaking";
                const EVENT: &'static str = "Reward";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Ledger(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Ledger {
                const PALLET: &'static str = "DappsStaking";
                const STORAGE: &'static str = "Ledger";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct CurrentEra;
            impl ::subxt::StorageEntry for CurrentEra {
                const PALLET: &'static str = "DappsStaking";
                const STORAGE: &'static str = "CurrentEra";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct BlockRewardAccumulator;
            impl ::subxt::StorageEntry for BlockRewardAccumulator {
                const PALLET: &'static str = "DappsStaking";
                const STORAGE: &'static str = "BlockRewardAccumulator";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ForceEra;
            impl ::subxt::StorageEntry for ForceEra {
                const PALLET: &'static str = "DappsStaking";
                const STORAGE: &'static str = "ForceEra";
                type Value = runtime_types::pallet_dapps_staking::Forcing;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct RegisteredDevelopers(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for RegisteredDevelopers {
                const PALLET: &'static str = "DappsStaking";
                const STORAGE: &'static str = "RegisteredDevelopers";
                type Value = runtime_types::shiden_runtime::SmartContract<
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct RegisteredDapps(
                pub  runtime_types::shiden_runtime::SmartContract<
                    ::subxt::sp_core::crypto::AccountId32,
                >,
            );
            impl ::subxt::StorageEntry for RegisteredDapps {
                const PALLET: &'static str = "DappsStaking";
                const STORAGE: &'static str = "RegisteredDapps";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct EraRewardsAndStakes(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for EraRewardsAndStakes {
                const PALLET: &'static str = "DappsStaking";
                const STORAGE: &'static str = "EraRewardsAndStakes";
                type Value =
                    runtime_types::pallet_dapps_staking::EraRewardAndStake<::core::primitive::u128>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct ContractEraStake(
                runtime_types::shiden_runtime::SmartContract<::subxt::sp_core::crypto::AccountId32>,
                ::core::primitive::u32,
            );
            impl ::subxt::StorageEntry for ContractEraStake {
                const PALLET: &'static str = "DappsStaking";
                const STORAGE: &'static str = "ContractEraStake";
                type Value = runtime_types::pallet_dapps_staking::EraStakingPoints<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
                    ])
                }
            }
            pub struct PreApprovalIsEnabled;
            impl ::subxt::StorageEntry for PreApprovalIsEnabled {
                const PALLET: &'static str = "DappsStaking";
                const STORAGE: &'static str = "PreApprovalIsEnabled";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PreApprovedDevelopers(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for PreApprovedDevelopers {
                const PALLET: &'static str = "DappsStaking";
                const STORAGE: &'static str = "PreApprovedDevelopers";
                type Value = ();
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn ledger(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::Error>
                {
                    let entry = Ledger(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn ledger_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Ledger>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn current_era(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = CurrentEra;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn block_reward_accumulator(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::Error>
                {
                    let entry = BlockRewardAccumulator;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn force_era(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_dapps_staking::Forcing,
                    ::subxt::Error,
                > {
                    let entry = ForceEra;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn registered_developers(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::shiden_runtime::SmartContract<
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = RegisteredDevelopers(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn registered_developers_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, RegisteredDevelopers>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn registered_dapps(
                    &self,
                    _0: runtime_types::shiden_runtime::SmartContract<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::Error,
                > {
                    let entry = RegisteredDapps(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn registered_dapps_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, RegisteredDapps>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn era_rewards_and_stakes(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_dapps_staking::EraRewardAndStake<
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = EraRewardsAndStakes(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn era_rewards_and_stakes_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, EraRewardsAndStakes>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn contract_era_stake(
                    &self,
                    _0: runtime_types::shiden_runtime::SmartContract<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    _1: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_dapps_staking::EraStakingPoints<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = ContractEraStake(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn contract_era_stake_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, ContractEraStake>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn pre_approval_is_enabled(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::Error>
                {
                    let entry = PreApprovalIsEnabled;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn pre_approved_developers(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<(), ::subxt::Error> {
                    let entry = PreApprovedDevelopers(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn pre_approved_developers_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, PreApprovedDevelopers>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod authorship {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetUncles {
                pub new_uncles: ::std::vec::Vec<
                    runtime_types::sp_runtime::generic::header::Header<
                        ::core::primitive::u32,
                        runtime_types::sp_runtime::traits::BlakeTwo256,
                    >,
                >,
            }
            impl ::subxt::Call for SetUncles {
                const PALLET: &'static str = "Authorship";
                const FUNCTION: &'static str = "set_uncles";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn set_uncles(
                    &self,
                    new_uncles: ::std::vec::Vec<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, SetUncles> {
                    let call = SetUncles { new_uncles };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Uncles;
            impl ::subxt::StorageEntry for Uncles {
                const PALLET: &'static str = "Authorship";
                const STORAGE: &'static str = "Uncles";
                type Value = ::std::vec::Vec<
                    runtime_types::pallet_authorship::UncleEntryItem<
                        ::core::primitive::u32,
                        ::subxt::sp_core::H256,
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Author;
            impl ::subxt::StorageEntry for Author {
                const PALLET: &'static str = "Authorship";
                const STORAGE: &'static str = "Author";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct DidSetUncles;
            impl ::subxt::StorageEntry for DidSetUncles {
                const PALLET: &'static str = "Authorship";
                const STORAGE: &'static str = "DidSetUncles";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn uncles(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::pallet_authorship::UncleEntryItem<
                            ::core::primitive::u32,
                            ::subxt::sp_core::H256,
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Uncles;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn author(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::Error,
                > {
                    let entry = Author;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn did_set_uncles(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::Error>
                {
                    let entry = DidSetUncles;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod collator_selection {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetInvulnerables {
                pub new: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
            }
            impl ::subxt::Call for SetInvulnerables {
                const PALLET: &'static str = "CollatorSelection";
                const FUNCTION: &'static str = "set_invulnerables";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetDesiredCandidates {
                pub max: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetDesiredCandidates {
                const PALLET: &'static str = "CollatorSelection";
                const FUNCTION: &'static str = "set_desired_candidates";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetCandidacyBond {
                pub bond: ::core::primitive::u128,
            }
            impl ::subxt::Call for SetCandidacyBond {
                const PALLET: &'static str = "CollatorSelection";
                const FUNCTION: &'static str = "set_candidacy_bond";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RegisterAsCandidate {}
            impl ::subxt::Call for RegisterAsCandidate {
                const PALLET: &'static str = "CollatorSelection";
                const FUNCTION: &'static str = "register_as_candidate";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct LeaveIntent {}
            impl ::subxt::Call for LeaveIntent {
                const PALLET: &'static str = "CollatorSelection";
                const FUNCTION: &'static str = "leave_intent";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn set_invulnerables(
                    &self,
                    new: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                ) -> ::subxt::SubmittableExtrinsic<T, SetInvulnerables> {
                    let call = SetInvulnerables { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_desired_candidates(
                    &self,
                    max: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetDesiredCandidates> {
                    let call = SetDesiredCandidates { max };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_candidacy_bond(
                    &self,
                    bond: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, SetCandidacyBond> {
                    let call = SetCandidacyBond { bond };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn register_as_candidate(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<T, RegisterAsCandidate> {
                    let call = RegisterAsCandidate {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn leave_intent(&self) -> ::subxt::SubmittableExtrinsic<T, LeaveIntent> {
                    let call = LeaveIntent {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_collator_selection::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NewInvulnerables(pub ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>);
            impl ::subxt::Event for NewInvulnerables {
                const PALLET: &'static str = "CollatorSelection";
                const EVENT: &'static str = "NewInvulnerables";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NewDesiredCandidates(pub ::core::primitive::u32);
            impl ::subxt::Event for NewDesiredCandidates {
                const PALLET: &'static str = "CollatorSelection";
                const EVENT: &'static str = "NewDesiredCandidates";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NewCandidacyBond(pub ::core::primitive::u128);
            impl ::subxt::Event for NewCandidacyBond {
                const PALLET: &'static str = "CollatorSelection";
                const EVENT: &'static str = "NewCandidacyBond";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CandidateAdded(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for CandidateAdded {
                const PALLET: &'static str = "CollatorSelection";
                const EVENT: &'static str = "CandidateAdded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CandidateRemoved(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for CandidateRemoved {
                const PALLET: &'static str = "CollatorSelection";
                const EVENT: &'static str = "CandidateRemoved";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Invulnerables;
            impl ::subxt::StorageEntry for Invulnerables {
                const PALLET: &'static str = "CollatorSelection";
                const STORAGE: &'static str = "Invulnerables";
                type Value = ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Candidates;
            impl ::subxt::StorageEntry for Candidates {
                const PALLET: &'static str = "CollatorSelection";
                const STORAGE: &'static str = "Candidates";
                type Value = ::std::vec::Vec<
                    runtime_types::pallet_collator_selection::pallet::CandidateInfo<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct LastAuthoredBlock(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for LastAuthoredBlock {
                const PALLET: &'static str = "CollatorSelection";
                const STORAGE: &'static str = "LastAuthoredBlock";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct DesiredCandidates;
            impl ::subxt::StorageEntry for DesiredCandidates {
                const PALLET: &'static str = "CollatorSelection";
                const STORAGE: &'static str = "DesiredCandidates";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CandidacyBond;
            impl ::subxt::StorageEntry for CandidacyBond {
                const PALLET: &'static str = "CollatorSelection";
                const STORAGE: &'static str = "CandidacyBond";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn invulnerables(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::Error,
                > {
                    let entry = Invulnerables;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn candidates(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::pallet_collator_selection::pallet::CandidateInfo<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Candidates;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn last_authored_block(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = LastAuthoredBlock(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn last_authored_block_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, LastAuthoredBlock>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn desired_candidates(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = DesiredCandidates;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn candidacy_bond(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::Error>
                {
                    let entry = CandidacyBond;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod session {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetKeys {
                pub keys: runtime_types::shiden_runtime::SessionKeys,
                pub proof: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for SetKeys {
                const PALLET: &'static str = "Session";
                const FUNCTION: &'static str = "set_keys";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct PurgeKeys {}
            impl ::subxt::Call for PurgeKeys {
                const PALLET: &'static str = "Session";
                const FUNCTION: &'static str = "purge_keys";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn set_keys(
                    &self,
                    keys: runtime_types::shiden_runtime::SessionKeys,
                    proof: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, SetKeys> {
                    let call = SetKeys { keys, proof };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn purge_keys(&self) -> ::subxt::SubmittableExtrinsic<T, PurgeKeys> {
                    let call = PurgeKeys {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_session::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NewSession(pub ::core::primitive::u32);
            impl ::subxt::Event for NewSession {
                const PALLET: &'static str = "Session";
                const EVENT: &'static str = "NewSession";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Validators;
            impl ::subxt::StorageEntry for Validators {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "Validators";
                type Value = ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentIndex;
            impl ::subxt::StorageEntry for CurrentIndex {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "CurrentIndex";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct QueuedChanged;
            impl ::subxt::StorageEntry for QueuedChanged {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "QueuedChanged";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct QueuedKeys;
            impl ::subxt::StorageEntry for QueuedKeys {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "QueuedKeys";
                type Value = ::std::vec::Vec<(
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::shiden_runtime::SessionKeys,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct DisabledValidators;
            impl ::subxt::StorageEntry for DisabledValidators {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "DisabledValidators";
                type Value = ::std::vec::Vec<::core::primitive::u32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextKeys(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for NextKeys {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "NextKeys";
                type Value = runtime_types::shiden_runtime::SessionKeys;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct KeyOwner(
                runtime_types::sp_core::crypto::KeyTypeId,
                ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::StorageEntry for KeyOwner {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "KeyOwner";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn validators(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::Error,
                > {
                    let entry = Validators;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn current_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = CurrentIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn queued_changed(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::Error>
                {
                    let entry = QueuedChanged;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn queued_keys(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::shiden_runtime::SessionKeys,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = QueuedKeys;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn disabled_validators(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::std::vec::Vec<::core::primitive::u32>, ::subxt::Error>
                {
                    let entry = DisabledValidators;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_keys(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::shiden_runtime::SessionKeys>,
                    ::subxt::Error,
                > {
                    let entry = NextKeys(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn next_keys_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, NextKeys>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn key_owner(
                    &self,
                    _0: runtime_types::sp_core::crypto::KeyTypeId,
                    _1: ::std::vec::Vec<::core::primitive::u8>,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::Error,
                > {
                    let entry = KeyOwner(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn key_owner_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, KeyOwner>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod aura {
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct Authorities;
            impl ::subxt::StorageEntry for Authorities {
                const PALLET: &'static str = "Aura";
                const STORAGE: &'static str = "Authorities";
                type Value =
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentSlot;
            impl ::subxt::StorageEntry for CurrentSlot {
                const PALLET: &'static str = "Aura";
                const STORAGE: &'static str = "CurrentSlot";
                type Value = runtime_types::sp_consensus_slots::Slot;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn authorities(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Authorities;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn current_slot(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<runtime_types::sp_consensus_slots::Slot, ::subxt::Error>
                {
                    let entry = CurrentSlot;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod aura_ext {
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct Authorities;
            impl ::subxt::StorageEntry for Authorities {
                const PALLET: &'static str = "AuraExt";
                const STORAGE: &'static str = "Authorities";
                type Value =
                    ::std::vec::Vec<runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn authorities(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public>,
                    ::subxt::Error,
                > {
                    let entry = Authorities;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod evm {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Withdraw {
                pub address: runtime_types::primitive_types::H160,
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for Withdraw {
                const PALLET: &'static str = "EVM";
                const FUNCTION: &'static str = "withdraw";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Call {
                pub source: runtime_types::primitive_types::H160,
                pub target: runtime_types::primitive_types::H160,
                pub input: ::std::vec::Vec<::core::primitive::u8>,
                pub value: runtime_types::primitive_types::U256,
                pub gas_limit: ::core::primitive::u64,
                pub gas_price: runtime_types::primitive_types::U256,
                pub nonce: ::core::option::Option<runtime_types::primitive_types::U256>,
            }
            impl ::subxt::Call for Call {
                const PALLET: &'static str = "EVM";
                const FUNCTION: &'static str = "call";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Create {
                pub source: runtime_types::primitive_types::H160,
                pub init: ::std::vec::Vec<::core::primitive::u8>,
                pub value: runtime_types::primitive_types::U256,
                pub gas_limit: ::core::primitive::u64,
                pub gas_price: runtime_types::primitive_types::U256,
                pub nonce: ::core::option::Option<runtime_types::primitive_types::U256>,
            }
            impl ::subxt::Call for Create {
                const PALLET: &'static str = "EVM";
                const FUNCTION: &'static str = "create";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Create2 {
                pub source: runtime_types::primitive_types::H160,
                pub init: ::std::vec::Vec<::core::primitive::u8>,
                pub salt: ::subxt::sp_core::H256,
                pub value: runtime_types::primitive_types::U256,
                pub gas_limit: ::core::primitive::u64,
                pub gas_price: runtime_types::primitive_types::U256,
                pub nonce: ::core::option::Option<runtime_types::primitive_types::U256>,
            }
            impl ::subxt::Call for Create2 {
                const PALLET: &'static str = "EVM";
                const FUNCTION: &'static str = "create2";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn withdraw(
                    &self,
                    address: runtime_types::primitive_types::H160,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, Withdraw> {
                    let call = Withdraw { address, value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn call(
                    &self,
                    source: runtime_types::primitive_types::H160,
                    target: runtime_types::primitive_types::H160,
                    input: ::std::vec::Vec<::core::primitive::u8>,
                    value: runtime_types::primitive_types::U256,
                    gas_limit: ::core::primitive::u64,
                    gas_price: runtime_types::primitive_types::U256,
                    nonce: ::core::option::Option<runtime_types::primitive_types::U256>,
                ) -> ::subxt::SubmittableExtrinsic<T, Call> {
                    let call = Call {
                        source,
                        target,
                        input,
                        value,
                        gas_limit,
                        gas_price,
                        nonce,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn create(
                    &self,
                    source: runtime_types::primitive_types::H160,
                    init: ::std::vec::Vec<::core::primitive::u8>,
                    value: runtime_types::primitive_types::U256,
                    gas_limit: ::core::primitive::u64,
                    gas_price: runtime_types::primitive_types::U256,
                    nonce: ::core::option::Option<runtime_types::primitive_types::U256>,
                ) -> ::subxt::SubmittableExtrinsic<T, Create> {
                    let call = Create {
                        source,
                        init,
                        value,
                        gas_limit,
                        gas_price,
                        nonce,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn create2(
                    &self,
                    source: runtime_types::primitive_types::H160,
                    init: ::std::vec::Vec<::core::primitive::u8>,
                    salt: ::subxt::sp_core::H256,
                    value: runtime_types::primitive_types::U256,
                    gas_limit: ::core::primitive::u64,
                    gas_price: runtime_types::primitive_types::U256,
                    nonce: ::core::option::Option<runtime_types::primitive_types::U256>,
                ) -> ::subxt::SubmittableExtrinsic<T, Create2> {
                    let call = Create2 {
                        source,
                        init,
                        salt,
                        value,
                        gas_limit,
                        gas_price,
                        nonce,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_evm::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Log(pub runtime_types::ethereum::log::Log);
            impl ::subxt::Event for Log {
                const PALLET: &'static str = "EVM";
                const EVENT: &'static str = "Log";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Created(pub runtime_types::primitive_types::H160);
            impl ::subxt::Event for Created {
                const PALLET: &'static str = "EVM";
                const EVENT: &'static str = "Created";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CreatedFailed(pub runtime_types::primitive_types::H160);
            impl ::subxt::Event for CreatedFailed {
                const PALLET: &'static str = "EVM";
                const EVENT: &'static str = "CreatedFailed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Executed(pub runtime_types::primitive_types::H160);
            impl ::subxt::Event for Executed {
                const PALLET: &'static str = "EVM";
                const EVENT: &'static str = "Executed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ExecutedFailed(pub runtime_types::primitive_types::H160);
            impl ::subxt::Event for ExecutedFailed {
                const PALLET: &'static str = "EVM";
                const EVENT: &'static str = "ExecutedFailed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BalanceDeposit(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::primitive_types::H160,
                pub runtime_types::primitive_types::U256,
            );
            impl ::subxt::Event for BalanceDeposit {
                const PALLET: &'static str = "EVM";
                const EVENT: &'static str = "BalanceDeposit";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BalanceWithdraw(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::primitive_types::H160,
                pub runtime_types::primitive_types::U256,
            );
            impl ::subxt::Event for BalanceWithdraw {
                const PALLET: &'static str = "EVM";
                const EVENT: &'static str = "BalanceWithdraw";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct AccountCodes(pub runtime_types::primitive_types::H160);
            impl ::subxt::StorageEntry for AccountCodes {
                const PALLET: &'static str = "EVM";
                const STORAGE: &'static str = "AccountCodes";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct AccountStorages(
                runtime_types::primitive_types::H160,
                ::subxt::sp_core::H256,
            );
            impl ::subxt::StorageEntry for AccountStorages {
                const PALLET: &'static str = "EVM";
                const STORAGE: &'static str = "AccountStorages";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn account_codes(
                    &self,
                    _0: runtime_types::primitive_types::H160,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::std::vec::Vec<::core::primitive::u8>, ::subxt::Error>
                {
                    let entry = AccountCodes(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account_codes_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, AccountCodes>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn account_storages(
                    &self,
                    _0: runtime_types::primitive_types::H160,
                    _1: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::Error>
                {
                    let entry = AccountStorages(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account_storages_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, AccountStorages>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod ethereum {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Transact {
                pub transaction: runtime_types::ethereum::transaction::LegacyTransaction,
            }
            impl ::subxt::Call for Transact {
                const PALLET: &'static str = "Ethereum";
                const FUNCTION: &'static str = "transact";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn transact(
                    &self,
                    transaction: runtime_types::ethereum::transaction::LegacyTransaction,
                ) -> ::subxt::SubmittableExtrinsic<T, Transact> {
                    let call = Transact { transaction };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_ethereum::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Executed(
                pub runtime_types::primitive_types::H160,
                pub runtime_types::primitive_types::H160,
                pub ::subxt::sp_core::H256,
                pub runtime_types::evm_core::error::ExitReason,
            );
            impl ::subxt::Event for Executed {
                const PALLET: &'static str = "Ethereum";
                const EVENT: &'static str = "Executed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Pending;
            impl ::subxt::StorageEntry for Pending {
                const PALLET: &'static str = "Ethereum";
                const STORAGE: &'static str = "Pending";
                type Value = ::std::vec::Vec<(
                    runtime_types::ethereum::transaction::LegacyTransaction,
                    runtime_types::fp_rpc::TransactionStatus,
                    runtime_types::ethereum::receipt::Receipt,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentBlock;
            impl ::subxt::StorageEntry for CurrentBlock {
                const PALLET: &'static str = "Ethereum";
                const STORAGE: &'static str = "CurrentBlock";
                type Value = runtime_types::ethereum::block::Block<
                    runtime_types::ethereum::transaction::LegacyTransaction,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentReceipts;
            impl ::subxt::StorageEntry for CurrentReceipts {
                const PALLET: &'static str = "Ethereum";
                const STORAGE: &'static str = "CurrentReceipts";
                type Value = ::std::vec::Vec<runtime_types::ethereum::receipt::Receipt>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentTransactionStatuses;
            impl ::subxt::StorageEntry for CurrentTransactionStatuses {
                const PALLET: &'static str = "Ethereum";
                const STORAGE: &'static str = "CurrentTransactionStatuses";
                type Value = ::std::vec::Vec<runtime_types::fp_rpc::TransactionStatus>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct BlockHash(pub runtime_types::primitive_types::U256);
            impl ::subxt::StorageEntry for BlockHash {
                const PALLET: &'static str = "Ethereum";
                const STORAGE: &'static str = "BlockHash";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn pending(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        runtime_types::ethereum::transaction::LegacyTransaction,
                        runtime_types::fp_rpc::TransactionStatus,
                        runtime_types::ethereum::receipt::Receipt,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = Pending;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn current_block(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::ethereum::block::Block<
                            runtime_types::ethereum::transaction::LegacyTransaction,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = CurrentBlock;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn current_receipts(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        ::std::vec::Vec<runtime_types::ethereum::receipt::Receipt>,
                    >,
                    ::subxt::Error,
                > {
                    let entry = CurrentReceipts;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn current_transaction_statuses(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        ::std::vec::Vec<runtime_types::fp_rpc::TransactionStatus>,
                    >,
                    ::subxt::Error,
                > {
                    let entry = CurrentTransactionStatuses;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn block_hash(
                    &self,
                    _0: runtime_types::primitive_types::U256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::Error>
                {
                    let entry = BlockHash(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn block_hash_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, BlockHash>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod eth_call {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Call {
                pub call: runtime_types::shiden_runtime::Call,
                pub signer: ::subxt::sp_core::crypto::AccountId32,
                pub signature: ::std::vec::Vec<::core::primitive::u8>,
                #[codec(compact)]
                pub nonce: ::core::primitive::u32,
            }
            impl ::subxt::Call for Call {
                const PALLET: &'static str = "EthCall";
                const FUNCTION: &'static str = "call";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn call(
                    &self,
                    call: runtime_types::shiden_runtime::Call,
                    signer: ::subxt::sp_core::crypto::AccountId32,
                    signature: ::std::vec::Vec<::core::primitive::u8>,
                    nonce: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, Call> {
                    let call = Call {
                        call,
                        signer,
                        signature,
                        nonce,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_custom_signatures::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Executed(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            );
            impl ::subxt::Event for Executed {
                const PALLET: &'static str = "EthCall";
                const EVENT: &'static str = "Executed";
            }
        }
    }
    pub mod sudo {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Sudo {
                pub call: runtime_types::shiden_runtime::Call,
            }
            impl ::subxt::Call for Sudo {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SudoUncheckedWeight {
                pub call: runtime_types::shiden_runtime::Call,
                pub weight: ::core::primitive::u64,
            }
            impl ::subxt::Call for SudoUncheckedWeight {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo_unchecked_weight";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetKey {
                pub new:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for SetKey {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "set_key";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SudoAs {
                pub who:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub call: runtime_types::shiden_runtime::Call,
            }
            impl ::subxt::Call for SudoAs {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo_as";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn sudo(
                    &self,
                    call: runtime_types::shiden_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<T, Sudo> {
                    let call = Sudo { call };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn sudo_unchecked_weight(
                    &self,
                    call: runtime_types::shiden_runtime::Call,
                    weight: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<T, SudoUncheckedWeight> {
                    let call = SudoUncheckedWeight { call, weight };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_key(
                    &self,
                    new: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, SetKey> {
                    let call = SetKey { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn sudo_as(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    call: runtime_types::shiden_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<T, SudoAs> {
                    let call = SudoAs { who, call };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_sudo::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Sudid(
                pub ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            );
            impl ::subxt::Event for Sudid {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "Sudid";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct KeyChanged(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for KeyChanged {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "KeyChanged";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SudoAsDone(
                pub ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            );
            impl ::subxt::Event for SudoAsDone {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "SudoAsDone";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Key;
            impl ::subxt::StorageEntry for Key {
                const PALLET: &'static str = "Sudo";
                const STORAGE: &'static str = "Key";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn key(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::crypto::AccountId32, ::subxt::Error>
                {
                    let entry = Key;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod ibc {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Deliver {
                pub messages: ::std::vec::Vec<runtime_types::pallet_ibc::Any>,
                pub tmp: ::core::primitive::u8,
            }
            impl ::subxt::Call for Deliver {
                const PALLET: &'static str = "Ibc";
                const FUNCTION: &'static str = "deliver";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn SubmittableExtrinsic(
                    &self,
                    messages: ::std::vec::Vec<runtime_types::pallet_ibc::Any>,
                    tmp: ::core::primitive::u8,
                ) -> ::subxt::SubmittableExtrinsic<T, Deliver> {
                    let call = Deliver { messages, tmp };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_ibc::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NewBlock(pub runtime_types::pallet_ibc::event::primitive::Height);
            impl ::subxt::Event for NewBlock {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "NewBlock";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CreateClient(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub runtime_types::pallet_ibc::event::primitive::ClientId,
                pub runtime_types::pallet_ibc::event::primitive::ClientTy,
                pub runtime_types::pallet_ibc::event::primitive::Height,
            );
            impl ::subxt::Event for CreateClient {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "CreateClient";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct UpdateClient(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub runtime_types::pallet_ibc::event::primitive::ClientId,
                pub runtime_types::pallet_ibc::event::primitive::ClientTy,
                pub runtime_types::pallet_ibc::event::primitive::Height,
            );
            impl ::subxt::Event for UpdateClient {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "UpdateClient";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct UpgradeClient(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub runtime_types::pallet_ibc::event::primitive::ClientId,
                pub runtime_types::pallet_ibc::event::primitive::ClientTy,
                pub runtime_types::pallet_ibc::event::primitive::Height,
            );
            impl ::subxt::Event for UpgradeClient {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "UpgradeClient";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ClientMisbehaviour(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub runtime_types::pallet_ibc::event::primitive::ClientId,
                pub runtime_types::pallet_ibc::event::primitive::ClientTy,
                pub runtime_types::pallet_ibc::event::primitive::Height,
            );
            impl ::subxt::Event for ClientMisbehaviour {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "ClientMisbehaviour";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct OpenInitConnection(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub  ::core::option::Option<
                    runtime_types::pallet_ibc::event::primitive::ConnectionId,
                >,
                pub runtime_types::pallet_ibc::event::primitive::ClientId,
                pub  ::core::option::Option<
                    runtime_types::pallet_ibc::event::primitive::ConnectionId,
                >,
                pub runtime_types::pallet_ibc::event::primitive::ClientId,
            );
            impl ::subxt::Event for OpenInitConnection {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "OpenInitConnection";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct OpenTryConnection(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub  ::core::option::Option<
                    runtime_types::pallet_ibc::event::primitive::ConnectionId,
                >,
                pub runtime_types::pallet_ibc::event::primitive::ClientId,
                pub  ::core::option::Option<
                    runtime_types::pallet_ibc::event::primitive::ConnectionId,
                >,
                pub runtime_types::pallet_ibc::event::primitive::ClientId,
            );
            impl ::subxt::Event for OpenTryConnection {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "OpenTryConnection";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct OpenAckConnection(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub  ::core::option::Option<
                    runtime_types::pallet_ibc::event::primitive::ConnectionId,
                >,
                pub runtime_types::pallet_ibc::event::primitive::ClientId,
                pub  ::core::option::Option<
                    runtime_types::pallet_ibc::event::primitive::ConnectionId,
                >,
                pub runtime_types::pallet_ibc::event::primitive::ClientId,
            );
            impl ::subxt::Event for OpenAckConnection {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "OpenAckConnection";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct OpenConfirmConnection(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub  ::core::option::Option<
                    runtime_types::pallet_ibc::event::primitive::ConnectionId,
                >,
                pub runtime_types::pallet_ibc::event::primitive::ClientId,
                pub  ::core::option::Option<
                    runtime_types::pallet_ibc::event::primitive::ConnectionId,
                >,
                pub runtime_types::pallet_ibc::event::primitive::ClientId,
            );
            impl ::subxt::Event for OpenConfirmConnection {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "OpenConfirmConnection";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct OpenInitChannel(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub runtime_types::pallet_ibc::event::primitive::PortId,
                pub ::core::option::Option<runtime_types::pallet_ibc::event::primitive::ChannelId>,
                pub runtime_types::pallet_ibc::event::primitive::ConnectionId,
                pub runtime_types::pallet_ibc::event::primitive::PortId,
                pub ::core::option::Option<runtime_types::pallet_ibc::event::primitive::ChannelId>,
            );
            impl ::subxt::Event for OpenInitChannel {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "OpenInitChannel";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct OpenTryChannel(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub runtime_types::pallet_ibc::event::primitive::PortId,
                pub ::core::option::Option<runtime_types::pallet_ibc::event::primitive::ChannelId>,
                pub runtime_types::pallet_ibc::event::primitive::ConnectionId,
                pub runtime_types::pallet_ibc::event::primitive::PortId,
                pub ::core::option::Option<runtime_types::pallet_ibc::event::primitive::ChannelId>,
            );
            impl ::subxt::Event for OpenTryChannel {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "OpenTryChannel";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct OpenAckChannel(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub runtime_types::pallet_ibc::event::primitive::PortId,
                pub ::core::option::Option<runtime_types::pallet_ibc::event::primitive::ChannelId>,
                pub runtime_types::pallet_ibc::event::primitive::ConnectionId,
                pub runtime_types::pallet_ibc::event::primitive::PortId,
                pub ::core::option::Option<runtime_types::pallet_ibc::event::primitive::ChannelId>,
            );
            impl ::subxt::Event for OpenAckChannel {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "OpenAckChannel";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct OpenConfirmChannel(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub runtime_types::pallet_ibc::event::primitive::PortId,
                pub ::core::option::Option<runtime_types::pallet_ibc::event::primitive::ChannelId>,
                pub runtime_types::pallet_ibc::event::primitive::ConnectionId,
                pub runtime_types::pallet_ibc::event::primitive::PortId,
                pub ::core::option::Option<runtime_types::pallet_ibc::event::primitive::ChannelId>,
            );
            impl ::subxt::Event for OpenConfirmChannel {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "OpenConfirmChannel";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CloseInitChannel(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub runtime_types::pallet_ibc::event::primitive::PortId,
                pub ::core::option::Option<runtime_types::pallet_ibc::event::primitive::ChannelId>,
                pub runtime_types::pallet_ibc::event::primitive::ConnectionId,
                pub runtime_types::pallet_ibc::event::primitive::PortId,
                pub ::core::option::Option<runtime_types::pallet_ibc::event::primitive::ChannelId>,
            );
            impl ::subxt::Event for CloseInitChannel {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "CloseInitChannel";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CloseConfirmChannel(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub runtime_types::pallet_ibc::event::primitive::PortId,
                pub ::core::option::Option<runtime_types::pallet_ibc::event::primitive::ChannelId>,
                pub runtime_types::pallet_ibc::event::primitive::ConnectionId,
                pub runtime_types::pallet_ibc::event::primitive::PortId,
                pub ::core::option::Option<runtime_types::pallet_ibc::event::primitive::ChannelId>,
            );
            impl ::subxt::Event for CloseConfirmChannel {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "CloseConfirmChannel";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SendPacket(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub runtime_types::pallet_ibc::event::primitive::Packet,
            );
            impl ::subxt::Event for SendPacket {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "SendPacket";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ReceivePacket(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub runtime_types::pallet_ibc::event::primitive::Packet,
            );
            impl ::subxt::Event for ReceivePacket {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "ReceivePacket";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct WriteAcknowledgement(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub runtime_types::pallet_ibc::event::primitive::Packet,
                pub ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::Event for WriteAcknowledgement {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "WriteAcknowledgement";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AcknowledgePacket(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub runtime_types::pallet_ibc::event::primitive::Packet,
            );
            impl ::subxt::Event for AcknowledgePacket {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "AcknowledgePacket";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct TimeoutPacket(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub runtime_types::pallet_ibc::event::primitive::Packet,
            );
            impl ::subxt::Event for TimeoutPacket {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "TimeoutPacket";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct TimeoutOnClosePacket(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub runtime_types::pallet_ibc::event::primitive::Packet,
            );
            impl ::subxt::Event for TimeoutOnClosePacket {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "TimeoutOnClosePacket";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Empty(pub ::std::vec::Vec<::core::primitive::u8>);
            impl ::subxt::Event for Empty {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "Empty";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ChainError(pub ::std::vec::Vec<::core::primitive::u8>);
            impl ::subxt::Event for ChainError {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "ChainError";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct ClientStates(pub ::std::vec::Vec<::core::primitive::u8>);
            impl ::subxt::StorageEntry for ClientStates {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "ClientStates";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct ConsensusStates(pub ::std::vec::Vec<::core::primitive::u8>);
            impl ::subxt::StorageEntry for ConsensusStates {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "ConsensusStates";
                type Value = ::std::vec::Vec<(
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::std::vec::Vec<::core::primitive::u8>,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct Connections(pub ::std::vec::Vec<::core::primitive::u8>);
            impl ::subxt::StorageEntry for Connections {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "Connections";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct Channels(
                ::std::vec::Vec<::core::primitive::u8>,
                ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::StorageEntry for Channels {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "Channels";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct ChannelsConnection(pub ::std::vec::Vec<::core::primitive::u8>);
            impl ::subxt::StorageEntry for ChannelsConnection {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "ChannelsConnection";
                type Value = ::std::vec::Vec<(
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::std::vec::Vec<::core::primitive::u8>,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct NextSequenceSend(
                ::std::vec::Vec<::core::primitive::u8>,
                ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::StorageEntry for NextSequenceSend {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "NextSequenceSend";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct NextSequenceRecv(
                ::std::vec::Vec<::core::primitive::u8>,
                ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::StorageEntry for NextSequenceRecv {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "NextSequenceRecv";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct NextSequenceAck(
                ::std::vec::Vec<::core::primitive::u8>,
                ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::StorageEntry for NextSequenceAck {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "NextSequenceAck";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct Acknowledgements(
                ::std::vec::Vec<::core::primitive::u8>,
                ::std::vec::Vec<::core::primitive::u8>,
                ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::StorageEntry for Acknowledgements {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "Acknowledgements";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct Clients(pub ::std::vec::Vec<::core::primitive::u8>);
            impl ::subxt::StorageEntry for Clients {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "Clients";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct ClientCounter;
            impl ::subxt::StorageEntry for ClientCounter {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "ClientCounter";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ConnectionCounter;
            impl ::subxt::StorageEntry for ConnectionCounter {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "ConnectionCounter";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ChannelCounter;
            impl ::subxt::StorageEntry for ChannelCounter {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "ChannelCounter";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ConnectionClient(pub ::std::vec::Vec<::core::primitive::u8>);
            impl ::subxt::StorageEntry for ConnectionClient {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "ConnectionClient";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct PacketReceipt(
                ::std::vec::Vec<::core::primitive::u8>,
                ::std::vec::Vec<::core::primitive::u8>,
                ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::StorageEntry for PacketReceipt {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "PacketReceipt";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct PacketCommitment(
                ::std::vec::Vec<::core::primitive::u8>,
                ::std::vec::Vec<::core::primitive::u8>,
                ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::StorageEntry for PacketCommitment {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "PacketCommitment";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct SendPacketEvent(
                ::std::vec::Vec<::core::primitive::u8>,
                ::std::vec::Vec<::core::primitive::u8>,
                ::core::primitive::u64,
            );
            impl ::subxt::StorageEntry for SendPacketEvent {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "SendPacketEvent";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct LatestHeight;
            impl ::subxt::StorageEntry for LatestHeight {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "LatestHeight";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct OldHeight;
            impl ::subxt::StorageEntry for OldHeight {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "OldHeight";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn client_states(
                    &self,
                    _0: ::std::vec::Vec<::core::primitive::u8>,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::std::vec::Vec<::core::primitive::u8>, ::subxt::Error>
                {
                    let entry = ClientStates(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn client_states_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, ClientStates>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn consensus_states(
                    &self,
                    _0: ::std::vec::Vec<::core::primitive::u8>,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = ConsensusStates(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn consensus_states_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, ConsensusStates>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn connections(
                    &self,
                    _0: ::std::vec::Vec<::core::primitive::u8>,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::std::vec::Vec<::core::primitive::u8>, ::subxt::Error>
                {
                    let entry = Connections(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn connections_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Connections>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn channels(
                    &self,
                    _0: ::std::vec::Vec<::core::primitive::u8>,
                    _1: ::std::vec::Vec<::core::primitive::u8>,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::std::vec::Vec<::core::primitive::u8>, ::subxt::Error>
                {
                    let entry = Channels(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn channels_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Channels>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn channels_connection(
                    &self,
                    _0: ::std::vec::Vec<::core::primitive::u8>,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = ChannelsConnection(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn channels_connection_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ChannelsConnection>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn next_sequence_send(
                    &self,
                    _0: ::std::vec::Vec<::core::primitive::u8>,
                    _1: ::std::vec::Vec<::core::primitive::u8>,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::std::vec::Vec<::core::primitive::u8>, ::subxt::Error>
                {
                    let entry = NextSequenceSend(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_sequence_send_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, NextSequenceSend>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn next_sequence_recv(
                    &self,
                    _0: ::std::vec::Vec<::core::primitive::u8>,
                    _1: ::std::vec::Vec<::core::primitive::u8>,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::std::vec::Vec<::core::primitive::u8>, ::subxt::Error>
                {
                    let entry = NextSequenceRecv(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_sequence_recv_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, NextSequenceRecv>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn next_sequence_ack(
                    &self,
                    _0: ::std::vec::Vec<::core::primitive::u8>,
                    _1: ::std::vec::Vec<::core::primitive::u8>,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::std::vec::Vec<::core::primitive::u8>, ::subxt::Error>
                {
                    let entry = NextSequenceAck(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_sequence_ack_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, NextSequenceAck>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn acknowledgements(
                    &self,
                    _0: ::std::vec::Vec<::core::primitive::u8>,
                    _1: ::std::vec::Vec<::core::primitive::u8>,
                    _2: ::std::vec::Vec<::core::primitive::u8>,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::std::vec::Vec<::core::primitive::u8>, ::subxt::Error>
                {
                    let entry = Acknowledgements(_0, _1, _2);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn acknowledgements_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Acknowledgements>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn clients(
                    &self,
                    _0: ::std::vec::Vec<::core::primitive::u8>,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::std::vec::Vec<::core::primitive::u8>, ::subxt::Error>
                {
                    let entry = Clients(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn clients_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Clients>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn client_counter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::Error>
                {
                    let entry = ClientCounter;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn connection_counter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::Error>
                {
                    let entry = ConnectionCounter;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn channel_counter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::Error>
                {
                    let entry = ChannelCounter;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn connection_client(
                    &self,
                    _0: ::std::vec::Vec<::core::primitive::u8>,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::std::vec::Vec<::core::primitive::u8>, ::subxt::Error>
                {
                    let entry = ConnectionClient(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn connection_client_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, ConnectionClient>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn packet_receipt(
                    &self,
                    _0: ::std::vec::Vec<::core::primitive::u8>,
                    _1: ::std::vec::Vec<::core::primitive::u8>,
                    _2: ::std::vec::Vec<::core::primitive::u8>,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::std::vec::Vec<::core::primitive::u8>, ::subxt::Error>
                {
                    let entry = PacketReceipt(_0, _1, _2);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn packet_receipt_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, PacketReceipt>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn packet_commitment(
                    &self,
                    _0: ::std::vec::Vec<::core::primitive::u8>,
                    _1: ::std::vec::Vec<::core::primitive::u8>,
                    _2: ::std::vec::Vec<::core::primitive::u8>,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::std::vec::Vec<::core::primitive::u8>, ::subxt::Error>
                {
                    let entry = PacketCommitment(_0, _1, _2);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn packet_commitment_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, PacketCommitment>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn send_packet_event(
                    &self,
                    _0: ::std::vec::Vec<::core::primitive::u8>,
                    _1: ::std::vec::Vec<::core::primitive::u8>,
                    _2: ::core::primitive::u64,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::std::vec::Vec<::core::primitive::u8>, ::subxt::Error>
                {
                    let entry = SendPacketEvent(_0, _1, _2);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn send_packet_event_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, SendPacketEvent>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn latest_height(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::std::vec::Vec<::core::primitive::u8>, ::subxt::Error>
                {
                    let entry = LatestHeight;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn old_height(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::Error>
                {
                    let entry = OldHeight;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod runtime_types {
        use super::runtime_types;
        pub mod cumulus_pallet_parachain_system {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    # [codec (index = 0)] set_validation_data { data : runtime_types :: cumulus_primitives_parachain_inherent :: ParachainInherentData , } , # [codec (index = 1)] sudo_send_upward_message { message : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , # [codec (index = 2)] authorize_upgrade { code_hash : :: subxt :: sp_core :: H256 , } , # [codec (index = 3)] enact_authorized_upgrade { code : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    OverlappingUpgrades,
                    #[codec(index = 1)]
                    ProhibitedByPolkadot,
                    #[codec(index = 2)]
                    TooBig,
                    #[codec(index = 3)]
                    ValidationDataNotAvailable,
                    #[codec(index = 4)]
                    HostConfigurationNotAvailable,
                    #[codec(index = 5)]
                    NotScheduled,
                    #[codec(index = 6)]
                    NothingAuthorized,
                    #[codec(index = 7)]
                    Unauthorized,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    ValidationFunctionStored,
                    #[codec(index = 1)]
                    ValidationFunctionApplied(::core::primitive::u32),
                    #[codec(index = 2)]
                    ValidationFunctionDiscarded,
                    #[codec(index = 3)]
                    UpgradeAuthorized(::subxt::sp_core::H256),
                    #[codec(index = 4)]
                    DownwardMessagesReceived(::core::primitive::u32),
                    #[codec(index = 5)]
                    DownwardMessagesProcessed(::core::primitive::u64, ::subxt::sp_core::H256),
                }
            }
            pub mod relay_state_snapshot {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct MessagingStateSnapshot {
                    pub dmq_mqc_head: ::subxt::sp_core::H256,
                    pub relay_dispatch_queue_size: (::core::primitive::u32, ::core::primitive::u32),
                    pub ingress_channels: ::std::vec::Vec<(
                        runtime_types::polkadot_parachain::primitives::Id,
                        runtime_types::polkadot_primitives::v1::AbridgedHrmpChannel,
                    )>,
                    pub egress_channels: ::std::vec::Vec<(
                        runtime_types::polkadot_parachain::primitives::Id,
                        runtime_types::polkadot_primitives::v1::AbridgedHrmpChannel,
                    )>,
                }
            }
        }
        pub mod cumulus_primitives_parachain_inherent {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ParachainInherentData {
                pub validation_data:
                    runtime_types::polkadot_primitives::v1::PersistedValidationData<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                pub relay_chain_state: runtime_types::sp_trie::storage_proof::StorageProof,
                pub downward_messages: ::std::vec::Vec<
                    runtime_types::polkadot_core_primitives::InboundDownwardMessage<
                        ::core::primitive::u32,
                    >,
                >,
                pub horizontal_messages: ::std::collections::BTreeMap<
                    runtime_types::polkadot_parachain::primitives::Id,
                    ::std::vec::Vec<
                        runtime_types::polkadot_core_primitives::InboundHrmpMessage<
                            ::core::primitive::u32,
                        >,
                    >,
                >,
            }
        }
        pub mod ethbloom {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Bloom(pub [::core::primitive::u8; 256usize]);
        }
        pub mod ethereum {
            use super::runtime_types;
            pub mod block {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Block<_0> {
                    pub header: runtime_types::ethereum::header::Header,
                    pub transactions: ::std::vec::Vec<_0>,
                    pub ommers: ::std::vec::Vec<runtime_types::ethereum::header::Header>,
                }
            }
            pub mod header {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Header {
                    pub parent_hash: ::subxt::sp_core::H256,
                    pub ommers_hash: ::subxt::sp_core::H256,
                    pub beneficiary: runtime_types::primitive_types::H160,
                    pub state_root: ::subxt::sp_core::H256,
                    pub transactions_root: ::subxt::sp_core::H256,
                    pub receipts_root: ::subxt::sp_core::H256,
                    pub logs_bloom: runtime_types::ethbloom::Bloom,
                    pub difficulty: runtime_types::primitive_types::U256,
                    pub number: runtime_types::primitive_types::U256,
                    pub gas_limit: runtime_types::primitive_types::U256,
                    pub gas_used: runtime_types::primitive_types::U256,
                    pub timestamp: ::core::primitive::u64,
                    pub extra_data: ::std::vec::Vec<::core::primitive::u8>,
                    pub mix_hash: ::subxt::sp_core::H256,
                    pub nonce: runtime_types::ethereum_types::hash::H64,
                }
            }
            pub mod log {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Log {
                    pub address: runtime_types::primitive_types::H160,
                    pub topics: ::std::vec::Vec<::subxt::sp_core::H256>,
                    pub data: ::std::vec::Vec<::core::primitive::u8>,
                }
            }
            pub mod receipt {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Receipt {
                    pub state_root: ::subxt::sp_core::H256,
                    pub used_gas: runtime_types::primitive_types::U256,
                    pub logs_bloom: runtime_types::ethbloom::Bloom,
                    pub logs: ::std::vec::Vec<runtime_types::ethereum::log::Log>,
                }
            }
            pub mod transaction {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct LegacyTransaction {
                    pub nonce: runtime_types::primitive_types::U256,
                    pub gas_price: runtime_types::primitive_types::U256,
                    pub gas_limit: runtime_types::primitive_types::U256,
                    pub action: runtime_types::ethereum::transaction::TransactionAction,
                    pub value: runtime_types::primitive_types::U256,
                    pub input: ::std::vec::Vec<::core::primitive::u8>,
                    pub signature: runtime_types::ethereum::transaction::TransactionSignature,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum TransactionAction {
                    #[codec(index = 0)]
                    Call(runtime_types::primitive_types::H160),
                    #[codec(index = 1)]
                    Create,
                }
                #[derive(
                    :: subxt :: codec :: CompactAs,
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                )]
                pub struct TransactionRecoveryId(pub ::core::primitive::u64);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct TransactionSignature {
                    pub v: runtime_types::ethereum::transaction::TransactionRecoveryId,
                    pub r: ::subxt::sp_core::H256,
                    pub s: ::subxt::sp_core::H256,
                }
            }
        }
        pub mod ethereum_types {
            use super::runtime_types;
            pub mod hash {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct H64(pub [::core::primitive::u8; 8usize]);
            }
        }
        pub mod evm_core {
            use super::runtime_types;
            pub mod error {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum ExitError {
                    #[codec(index = 0)]
                    StackUnderflow,
                    #[codec(index = 1)]
                    StackOverflow,
                    #[codec(index = 2)]
                    InvalidJump,
                    #[codec(index = 3)]
                    InvalidRange,
                    #[codec(index = 4)]
                    DesignatedInvalid,
                    #[codec(index = 5)]
                    CallTooDeep,
                    #[codec(index = 6)]
                    CreateCollision,
                    #[codec(index = 7)]
                    CreateContractLimit,
                    #[codec(index = 8)]
                    OutOfOffset,
                    #[codec(index = 9)]
                    OutOfGas,
                    #[codec(index = 10)]
                    OutOfFund,
                    #[codec(index = 11)]
                    PCUnderflow,
                    #[codec(index = 12)]
                    CreateEmpty,
                    #[codec(index = 13)]
                    Other(::std::string::String),
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum ExitFatal {
                    #[codec(index = 0)]
                    NotSupported,
                    #[codec(index = 1)]
                    UnhandledInterrupt,
                    #[codec(index = 2)]
                    CallErrorAsFatal(runtime_types::evm_core::error::ExitError),
                    #[codec(index = 3)]
                    Other(::std::string::String),
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum ExitReason {
                    #[codec(index = 0)]
                    Succeed(runtime_types::evm_core::error::ExitSucceed),
                    #[codec(index = 1)]
                    Error(runtime_types::evm_core::error::ExitError),
                    #[codec(index = 2)]
                    Revert(runtime_types::evm_core::error::ExitRevert),
                    #[codec(index = 3)]
                    Fatal(runtime_types::evm_core::error::ExitFatal),
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum ExitRevert {
                    #[codec(index = 0)]
                    Reverted,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum ExitSucceed {
                    #[codec(index = 0)]
                    Stopped,
                    #[codec(index = 1)]
                    Returned,
                    #[codec(index = 2)]
                    Suicided,
                }
            }
        }
        pub mod fp_rpc {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct TransactionStatus {
                pub transaction_hash: ::subxt::sp_core::H256,
                pub transaction_index: ::core::primitive::u32,
                pub from: runtime_types::primitive_types::H160,
                pub to: ::core::option::Option<runtime_types::primitive_types::H160>,
                pub contract_address: ::core::option::Option<runtime_types::primitive_types::H160>,
                pub logs: ::std::vec::Vec<runtime_types::ethereum::log::Log>,
                pub logs_bloom: runtime_types::ethbloom::Bloom,
            }
        }
        pub mod frame_support {
            use super::runtime_types;
            pub mod storage {
                use super::runtime_types;
                pub mod bounded_vec {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct BoundedVec<_0>(pub ::std::vec::Vec<_0>);
                }
                pub mod weak_bounded_vec {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct WeakBoundedVec<_0>(pub ::std::vec::Vec<_0>);
                }
            }
            pub mod traits {
                use super::runtime_types;
                pub mod tokens {
                    use super::runtime_types;
                    pub mod misc {
                        use super::runtime_types;
                        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                        pub enum BalanceStatus {
                            #[codec(index = 0)]
                            Free,
                            #[codec(index = 1)]
                            Reserved,
                        }
                    }
                }
            }
            pub mod weights {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum DispatchClass {
                    #[codec(index = 0)]
                    Normal,
                    #[codec(index = 1)]
                    Operational,
                    #[codec(index = 2)]
                    Mandatory,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct DispatchInfo {
                    pub weight: ::core::primitive::u64,
                    pub class: runtime_types::frame_support::weights::DispatchClass,
                    pub pays_fee: runtime_types::frame_support::weights::Pays,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Pays {
                    #[codec(index = 0)]
                    Yes,
                    #[codec(index = 1)]
                    No,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct PerDispatchClass<_0> {
                    pub normal: _0,
                    pub operational: _0,
                    pub mandatory: _0,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct RuntimeDbWeight {
                    pub read: ::core::primitive::u64,
                    pub write: ::core::primitive::u64,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct WeightToFeeCoefficient<_0> {
                    pub coeff_integer: _0,
                    pub coeff_frac: runtime_types::sp_arithmetic::per_things::Perbill,
                    pub negative: ::core::primitive::bool,
                    pub degree: ::core::primitive::u8,
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct PalletId(pub [::core::primitive::u8; 8usize]);
        }
        pub mod frame_system {
            use super::runtime_types;
            pub mod extensions {
                use super::runtime_types;
                pub mod check_genesis {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct CheckGenesis {}
                }
                pub mod check_mortality {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
                }
                pub mod check_nonce {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct CheckNonce(#[codec(compact)] pub ::core::primitive::u32);
                }
                pub mod check_spec_version {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct CheckSpecVersion {}
                }
                pub mod check_tx_version {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct CheckTxVersion {}
                }
                pub mod check_weight {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct CheckWeight {}
                }
            }
            pub mod limits {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct BlockLength {
                    pub max: runtime_types::frame_support::weights::PerDispatchClass<
                        ::core::primitive::u32,
                    >,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct BlockWeights {
                    pub base_block: ::core::primitive::u64,
                    pub max_block: ::core::primitive::u64,
                    pub per_class: runtime_types::frame_support::weights::PerDispatchClass<
                        runtime_types::frame_system::limits::WeightsPerClass,
                    >,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct WeightsPerClass {
                    pub base_extrinsic: ::core::primitive::u64,
                    pub max_extrinsic: ::core::option::Option<::core::primitive::u64>,
                    pub max_total: ::core::option::Option<::core::primitive::u64>,
                    pub reserved: ::core::option::Option<::core::primitive::u64>,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    fill_block {
                        ratio: runtime_types::sp_arithmetic::per_things::Perbill,
                    },
                    #[codec(index = 1)]
                    remark {
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 2)]
                    set_heap_pages { pages: ::core::primitive::u64 },
                    #[codec(index = 3)]
                    set_code {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    set_code_without_checks {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 5)]
                    set_changes_trie_config {
                        changes_trie_config: ::core::option::Option<
                            runtime_types::sp_core::changes_trie::ChangesTrieConfiguration,
                        >,
                    },
                    #[codec(index = 6)]
                    set_storage {
                        items: ::std::vec::Vec<(
                            ::std::vec::Vec<::core::primitive::u8>,
                            ::std::vec::Vec<::core::primitive::u8>,
                        )>,
                    },
                    #[codec(index = 7)]
                    kill_storage {
                        keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    },
                    #[codec(index = 8)]
                    kill_prefix {
                        prefix: ::std::vec::Vec<::core::primitive::u8>,
                        subkeys: ::core::primitive::u32,
                    },
                    #[codec(index = 9)]
                    remark_with_event {
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidSpecName,
                    #[codec(index = 1)]
                    SpecVersionNeedsToIncrease,
                    #[codec(index = 2)]
                    FailedToExtractRuntimeVersion,
                    #[codec(index = 3)]
                    NonDefaultComposite,
                    #[codec(index = 4)]
                    NonZeroRefCount,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    ExtrinsicSuccess(runtime_types::frame_support::weights::DispatchInfo),
                    #[codec(index = 1)]
                    ExtrinsicFailed(
                        runtime_types::sp_runtime::DispatchError,
                        runtime_types::frame_support::weights::DispatchInfo,
                    ),
                    #[codec(index = 2)]
                    CodeUpdated,
                    #[codec(index = 3)]
                    NewAccount(::subxt::sp_core::crypto::AccountId32),
                    #[codec(index = 4)]
                    KilledAccount(::subxt::sp_core::crypto::AccountId32),
                    #[codec(index = 5)]
                    Remarked(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::H256,
                    ),
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AccountInfo<_0, _1> {
                pub nonce: _0,
                pub consumers: _0,
                pub providers: _0,
                pub sufficients: _0,
                pub data: _1,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct EventRecord<_0, _1> {
                pub phase: runtime_types::frame_system::Phase,
                pub event: _0,
                pub topics: ::std::vec::Vec<_1>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct LastRuntimeUpgradeInfo {
                #[codec(compact)]
                pub spec_version: ::core::primitive::u32,
                pub spec_name: ::std::string::String,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Phase {
                #[codec(index = 0)]
                ApplyExtrinsic(::core::primitive::u32),
                #[codec(index = 1)]
                Finalization,
                #[codec(index = 2)]
                Initialization,
            }
        }
        pub mod pallet_authorship {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    set_uncles {
                        new_uncles: ::std::vec::Vec<
                            runtime_types::sp_runtime::generic::header::Header<
                                ::core::primitive::u32,
                                runtime_types::sp_runtime::traits::BlakeTwo256,
                            >,
                        >,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidUncleParent,
                    #[codec(index = 1)]
                    UnclesAlreadySet,
                    #[codec(index = 2)]
                    TooManyUncles,
                    #[codec(index = 3)]
                    GenesisUncle,
                    #[codec(index = 4)]
                    TooHighUncle,
                    #[codec(index = 5)]
                    UncleAlreadyIncluded,
                    #[codec(index = 6)]
                    OldUncle,
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum UncleEntryItem<_0, _1, _2> {
                #[codec(index = 0)]
                InclusionHeight(_0),
                #[codec(index = 1)]
                Uncle(_1, ::core::option::Option<_2>),
            }
        }
        pub mod pallet_balances {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    transfer {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    set_balance {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        new_free: ::core::primitive::u128,
                        #[codec(compact)]
                        new_reserved: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    force_transfer {
                        source: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    transfer_keep_alive {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    transfer_all {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        keep_alive: ::core::primitive::bool,
                    },
                    #[codec(index = 5)]
                    force_unreserve {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        amount: ::core::primitive::u128,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    VestingBalance,
                    #[codec(index = 1)]
                    LiquidityRestrictions,
                    #[codec(index = 2)]
                    InsufficientBalance,
                    #[codec(index = 3)]
                    ExistentialDeposit,
                    #[codec(index = 4)]
                    KeepAlive,
                    #[codec(index = 5)]
                    ExistingVestingSchedule,
                    #[codec(index = 6)]
                    DeadAccount,
                    #[codec(index = 7)]
                    TooManyReserves,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    Endowed(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 1)]
                    DustLost(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 2)]
                    Transfer(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 3)]
                    BalanceSet(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 4)]
                    Reserved(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 5)]
                    Unreserved(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 6)]
                    ReserveRepatriated(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                        runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
                    ),
                    #[codec(index = 7)]
                    Deposit(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 8)]
                    Withdraw(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 9)]
                    Slashed(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AccountData<_0> {
                pub free: _0,
                pub reserved: _0,
                pub misc_frozen: _0,
                pub fee_frozen: _0,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BalanceLock<_0> {
                pub id: [::core::primitive::u8; 8usize],
                pub amount: _0,
                pub reasons: runtime_types::pallet_balances::Reasons,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Reasons {
                #[codec(index = 0)]
                Fee,
                #[codec(index = 1)]
                Misc,
                #[codec(index = 2)]
                All,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Releases {
                #[codec(index = 0)]
                V1_0_0,
                #[codec(index = 1)]
                V2_0_0,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ReserveData<_0, _1> {
                pub id: _0,
                pub amount: _1,
            }
        }
        pub mod pallet_collator_selection {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    set_invulnerables {
                        new: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    },
                    #[codec(index = 1)]
                    set_desired_candidates { max: ::core::primitive::u32 },
                    #[codec(index = 2)]
                    set_candidacy_bond { bond: ::core::primitive::u128 },
                    #[codec(index = 3)]
                    register_as_candidate,
                    #[codec(index = 4)]
                    leave_intent,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct CandidateInfo<_0, _1> {
                    pub who: _0,
                    pub deposit: _1,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    TooManyCandidates,
                    #[codec(index = 1)]
                    TooFewCandidates,
                    #[codec(index = 2)]
                    Unknown,
                    #[codec(index = 3)]
                    Permission,
                    #[codec(index = 4)]
                    AlreadyCandidate,
                    #[codec(index = 5)]
                    NotCandidate,
                    #[codec(index = 6)]
                    AlreadyInvulnerable,
                    #[codec(index = 7)]
                    NoAssociatedValidatorId,
                    #[codec(index = 8)]
                    ValidatorNotRegistered,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    NewInvulnerables(::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>),
                    #[codec(index = 1)]
                    NewDesiredCandidates(::core::primitive::u32),
                    #[codec(index = 2)]
                    NewCandidacyBond(::core::primitive::u128),
                    #[codec(index = 3)]
                    CandidateAdded(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 4)]
                    CandidateRemoved(::subxt::sp_core::crypto::AccountId32),
                }
            }
        }
        pub mod pallet_custom_signatures {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    call {
                        call: ::std::boxed::Box<runtime_types::shiden_runtime::Call>,
                        signer: ::subxt::sp_core::crypto::AccountId32,
                        signature: ::std::vec::Vec<::core::primitive::u8>,
                        #[codec(compact)]
                        nonce: ::core::primitive::u32,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    DecodeFailure,
                    #[codec(index = 1)]
                    InvalidSignature,
                    #[codec(index = 2)]
                    BadNonce,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    Executed(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    ),
                }
            }
        }
        pub mod pallet_dapps_staking {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Call {
                        #[codec(index = 0)]
                        register {
                            contract_id: runtime_types::shiden_runtime::SmartContract<
                                ::subxt::sp_core::crypto::AccountId32,
                            >,
                        },
                        #[codec(index = 1)]
                        unregister {
                            contract_id: runtime_types::shiden_runtime::SmartContract<
                                ::subxt::sp_core::crypto::AccountId32,
                            >,
                        },
                        #[codec(index = 2)]
                        bond_and_stake {
                            contract_id: runtime_types::shiden_runtime::SmartContract<
                                ::subxt::sp_core::crypto::AccountId32,
                            >,
                            #[codec(compact)]
                            value: ::core::primitive::u128,
                        },
                        #[codec(index = 3)]
                        unbond_unstake_and_withdraw {
                            contract_id: runtime_types::shiden_runtime::SmartContract<
                                ::subxt::sp_core::crypto::AccountId32,
                            >,
                            #[codec(compact)]
                            value: ::core::primitive::u128,
                        },
                        #[codec(index = 4)]
                        claim {
                            contract_id: runtime_types::shiden_runtime::SmartContract<
                                ::subxt::sp_core::crypto::AccountId32,
                            >,
                            era: ::core::primitive::u32,
                        },
                        #[codec(index = 5)]
                        force_new_era,
                        #[codec(index = 6)]
                        developer_pre_approval {
                            developer: ::subxt::sp_core::crypto::AccountId32,
                        },
                        #[codec(index = 7)]
                        enable_developer_pre_approval { enabled: ::core::primitive::bool },
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Error {
                        #[codec(index = 0)]
                        StakingWithNoValue,
                        #[codec(index = 1)]
                        InsufficientValue,
                        #[codec(index = 2)]
                        MaxNumberOfStakersExceeded,
                        #[codec(index = 3)]
                        NotOperatedContract,
                        #[codec(index = 4)]
                        NotStakedContract,
                        #[codec(index = 5)]
                        UnstakingWithNoValue,
                        #[codec(index = 6)]
                        AlreadyRegisteredContract,
                        #[codec(index = 7)]
                        ContractIsNotValid,
                        #[codec(index = 8)]
                        AlreadyUsedDeveloperAccount,
                        #[codec(index = 9)]
                        NotOwnedContract,
                        #[codec(index = 10)]
                        UnknownEraReward,
                        #[codec(index = 11)]
                        NotStaked,
                        #[codec(index = 12)]
                        AlreadyClaimedInThisEra,
                        #[codec(index = 13)]
                        EraOutOfBounds,
                        #[codec(index = 14)]
                        RequiredContractPreApproval,
                        #[codec(index = 15)]
                        AlreadyPreApprovedDeveloper,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Event {
                        #[codec(index = 0)]
                        BondAndStake(
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::shiden_runtime::SmartContract<
                                ::subxt::sp_core::crypto::AccountId32,
                            >,
                            ::core::primitive::u128,
                        ),
                        #[codec(index = 1)]
                        UnbondUnstakeAndWithdraw(
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::shiden_runtime::SmartContract<
                                ::subxt::sp_core::crypto::AccountId32,
                            >,
                            ::core::primitive::u128,
                        ),
                        #[codec(index = 2)]
                        NewContract(
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::shiden_runtime::SmartContract<
                                ::subxt::sp_core::crypto::AccountId32,
                            >,
                        ),
                        #[codec(index = 3)]
                        ContractRemoved(
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::shiden_runtime::SmartContract<
                                ::subxt::sp_core::crypto::AccountId32,
                            >,
                        ),
                        #[codec(index = 4)]
                        NewDappStakingEra(::core::primitive::u32),
                        #[codec(index = 5)]
                        Reward(
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::shiden_runtime::SmartContract<
                                ::subxt::sp_core::crypto::AccountId32,
                            >,
                            ::core::primitive::u32,
                            ::core::primitive::u128,
                        ),
                    }
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct EraRewardAndStake<_0> {
                pub rewards: _0,
                pub staked: _0,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct EraStakingPoints<_0, _1> {
                pub total: _1,
                pub stakers: ::std::collections::BTreeMap<_0, _1>,
                pub _former_staked_era: ::core::primitive::u32,
                pub claimed_rewards: _1,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Forcing {
                #[codec(index = 0)]
                NotForcing,
                #[codec(index = 1)]
                ForceNew,
                #[codec(index = 2)]
                ForceNone,
                #[codec(index = 3)]
                ForceAlways,
            }
        }
        pub mod pallet_ethereum {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    transact {
                        transaction: runtime_types::ethereum::transaction::LegacyTransaction,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidSignature,
                    #[codec(index = 1)]
                    PreLogExists,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    Executed(
                        runtime_types::primitive_types::H160,
                        runtime_types::primitive_types::H160,
                        ::subxt::sp_core::H256,
                        runtime_types::evm_core::error::ExitReason,
                    ),
                }
            }
        }
        pub mod pallet_evm {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    withdraw {
                        address: runtime_types::primitive_types::H160,
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    call {
                        source: runtime_types::primitive_types::H160,
                        target: runtime_types::primitive_types::H160,
                        input: ::std::vec::Vec<::core::primitive::u8>,
                        value: runtime_types::primitive_types::U256,
                        gas_limit: ::core::primitive::u64,
                        gas_price: runtime_types::primitive_types::U256,
                        nonce: ::core::option::Option<runtime_types::primitive_types::U256>,
                    },
                    #[codec(index = 2)]
                    create {
                        source: runtime_types::primitive_types::H160,
                        init: ::std::vec::Vec<::core::primitive::u8>,
                        value: runtime_types::primitive_types::U256,
                        gas_limit: ::core::primitive::u64,
                        gas_price: runtime_types::primitive_types::U256,
                        nonce: ::core::option::Option<runtime_types::primitive_types::U256>,
                    },
                    #[codec(index = 3)]
                    create2 {
                        source: runtime_types::primitive_types::H160,
                        init: ::std::vec::Vec<::core::primitive::u8>,
                        salt: ::subxt::sp_core::H256,
                        value: runtime_types::primitive_types::U256,
                        gas_limit: ::core::primitive::u64,
                        gas_price: runtime_types::primitive_types::U256,
                        nonce: ::core::option::Option<runtime_types::primitive_types::U256>,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    BalanceLow,
                    #[codec(index = 1)]
                    FeeOverflow,
                    #[codec(index = 2)]
                    PaymentOverflow,
                    #[codec(index = 3)]
                    WithdrawFailed,
                    #[codec(index = 4)]
                    GasPriceTooLow,
                    #[codec(index = 5)]
                    InvalidNonce,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    Log(runtime_types::ethereum::log::Log),
                    #[codec(index = 1)]
                    Created(runtime_types::primitive_types::H160),
                    #[codec(index = 2)]
                    CreatedFailed(runtime_types::primitive_types::H160),
                    #[codec(index = 3)]
                    Executed(runtime_types::primitive_types::H160),
                    #[codec(index = 4)]
                    ExecutedFailed(runtime_types::primitive_types::H160),
                    #[codec(index = 5)]
                    BalanceDeposit(
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                        runtime_types::primitive_types::U256,
                    ),
                    #[codec(index = 6)]
                    BalanceWithdraw(
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                        runtime_types::primitive_types::U256,
                    ),
                }
            }
        }
        pub mod pallet_ibc {
            use super::runtime_types;
            pub mod event {
                use super::runtime_types;
                pub mod primitive {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct ChannelId(pub ::std::vec::Vec<::core::primitive::u8>);
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct ClientId(pub ::std::vec::Vec<::core::primitive::u8>);
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum ClientTy {
                        #[codec(index = 1)]
                        Tendermint,
                        #[codec(index = 2)]
                        Grandpa,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct ConnectionId(pub ::std::vec::Vec<::core::primitive::u8>);
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct Height {
                        pub revision_number: ::core::primitive::u64,
                        pub revision_height: ::core::primitive::u64,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct Packet {
                        pub sequence: runtime_types::pallet_ibc::event::primitive::Sequence,
                        pub source_port: runtime_types::pallet_ibc::event::primitive::PortId,
                        pub source_channel: runtime_types::pallet_ibc::event::primitive::ChannelId,
                        pub destination_port: runtime_types::pallet_ibc::event::primitive::PortId,
                        pub destination_channel:
                            runtime_types::pallet_ibc::event::primitive::ChannelId,
                        pub data: ::std::vec::Vec<::core::primitive::u8>,
                        pub timeout_height: runtime_types::pallet_ibc::event::primitive::Height,
                        pub timeout_timestamp:
                            runtime_types::pallet_ibc::event::primitive::Timestamp,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct PortId(pub ::std::vec::Vec<::core::primitive::u8>);
                    #[derive(
                        :: subxt :: codec :: CompactAs,
                        :: subxt :: codec :: Encode,
                        :: subxt :: codec :: Decode,
                    )]
                    pub struct Sequence(pub ::core::primitive::u64);
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct Timestamp(pub ::std::vec::Vec<::core::primitive::u8>);
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    deliver {
                        messages: ::std::vec::Vec<runtime_types::pallet_ibc::Any>,
                        tmp: ::core::primitive::u8,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {}
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    NewBlock(runtime_types::pallet_ibc::event::primitive::Height),
                    #[codec(index = 1)]
                    CreateClient(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::ClientId,
                        runtime_types::pallet_ibc::event::primitive::ClientTy,
                        runtime_types::pallet_ibc::event::primitive::Height,
                    ),
                    #[codec(index = 2)]
                    UpdateClient(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::ClientId,
                        runtime_types::pallet_ibc::event::primitive::ClientTy,
                        runtime_types::pallet_ibc::event::primitive::Height,
                    ),
                    #[codec(index = 3)]
                    UpgradeClient(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::ClientId,
                        runtime_types::pallet_ibc::event::primitive::ClientTy,
                        runtime_types::pallet_ibc::event::primitive::Height,
                    ),
                    #[codec(index = 4)]
                    ClientMisbehaviour(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::ClientId,
                        runtime_types::pallet_ibc::event::primitive::ClientTy,
                        runtime_types::pallet_ibc::event::primitive::Height,
                    ),
                    #[codec(index = 5)]
                    OpenInitConnection(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        ::core::option::Option<
                            runtime_types::pallet_ibc::event::primitive::ConnectionId,
                        >,
                        runtime_types::pallet_ibc::event::primitive::ClientId,
                        ::core::option::Option<
                            runtime_types::pallet_ibc::event::primitive::ConnectionId,
                        >,
                        runtime_types::pallet_ibc::event::primitive::ClientId,
                    ),
                    #[codec(index = 6)]
                    OpenTryConnection(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        ::core::option::Option<
                            runtime_types::pallet_ibc::event::primitive::ConnectionId,
                        >,
                        runtime_types::pallet_ibc::event::primitive::ClientId,
                        ::core::option::Option<
                            runtime_types::pallet_ibc::event::primitive::ConnectionId,
                        >,
                        runtime_types::pallet_ibc::event::primitive::ClientId,
                    ),
                    #[codec(index = 7)]
                    OpenAckConnection(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        ::core::option::Option<
                            runtime_types::pallet_ibc::event::primitive::ConnectionId,
                        >,
                        runtime_types::pallet_ibc::event::primitive::ClientId,
                        ::core::option::Option<
                            runtime_types::pallet_ibc::event::primitive::ConnectionId,
                        >,
                        runtime_types::pallet_ibc::event::primitive::ClientId,
                    ),
                    #[codec(index = 8)]
                    OpenConfirmConnection(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        ::core::option::Option<
                            runtime_types::pallet_ibc::event::primitive::ConnectionId,
                        >,
                        runtime_types::pallet_ibc::event::primitive::ClientId,
                        ::core::option::Option<
                            runtime_types::pallet_ibc::event::primitive::ConnectionId,
                        >,
                        runtime_types::pallet_ibc::event::primitive::ClientId,
                    ),
                    #[codec(index = 9)]
                    OpenInitChannel(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::PortId,
                        ::core::option::Option<
                            runtime_types::pallet_ibc::event::primitive::ChannelId,
                        >,
                        runtime_types::pallet_ibc::event::primitive::ConnectionId,
                        runtime_types::pallet_ibc::event::primitive::PortId,
                        ::core::option::Option<
                            runtime_types::pallet_ibc::event::primitive::ChannelId,
                        >,
                    ),
                    #[codec(index = 10)]
                    OpenTryChannel(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::PortId,
                        ::core::option::Option<
                            runtime_types::pallet_ibc::event::primitive::ChannelId,
                        >,
                        runtime_types::pallet_ibc::event::primitive::ConnectionId,
                        runtime_types::pallet_ibc::event::primitive::PortId,
                        ::core::option::Option<
                            runtime_types::pallet_ibc::event::primitive::ChannelId,
                        >,
                    ),
                    #[codec(index = 11)]
                    OpenAckChannel(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::PortId,
                        ::core::option::Option<
                            runtime_types::pallet_ibc::event::primitive::ChannelId,
                        >,
                        runtime_types::pallet_ibc::event::primitive::ConnectionId,
                        runtime_types::pallet_ibc::event::primitive::PortId,
                        ::core::option::Option<
                            runtime_types::pallet_ibc::event::primitive::ChannelId,
                        >,
                    ),
                    #[codec(index = 12)]
                    OpenConfirmChannel(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::PortId,
                        ::core::option::Option<
                            runtime_types::pallet_ibc::event::primitive::ChannelId,
                        >,
                        runtime_types::pallet_ibc::event::primitive::ConnectionId,
                        runtime_types::pallet_ibc::event::primitive::PortId,
                        ::core::option::Option<
                            runtime_types::pallet_ibc::event::primitive::ChannelId,
                        >,
                    ),
                    #[codec(index = 13)]
                    CloseInitChannel(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::PortId,
                        ::core::option::Option<
                            runtime_types::pallet_ibc::event::primitive::ChannelId,
                        >,
                        runtime_types::pallet_ibc::event::primitive::ConnectionId,
                        runtime_types::pallet_ibc::event::primitive::PortId,
                        ::core::option::Option<
                            runtime_types::pallet_ibc::event::primitive::ChannelId,
                        >,
                    ),
                    #[codec(index = 14)]
                    CloseConfirmChannel(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::PortId,
                        ::core::option::Option<
                            runtime_types::pallet_ibc::event::primitive::ChannelId,
                        >,
                        runtime_types::pallet_ibc::event::primitive::ConnectionId,
                        runtime_types::pallet_ibc::event::primitive::PortId,
                        ::core::option::Option<
                            runtime_types::pallet_ibc::event::primitive::ChannelId,
                        >,
                    ),
                    #[codec(index = 15)]
                    SendPacket(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::Packet,
                    ),
                    #[codec(index = 16)]
                    ReceivePacket(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::Packet,
                    ),
                    #[codec(index = 17)]
                    WriteAcknowledgement(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::Packet,
                        ::std::vec::Vec<::core::primitive::u8>,
                    ),
                    #[codec(index = 18)]
                    AcknowledgePacket(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::Packet,
                    ),
                    #[codec(index = 19)]
                    TimeoutPacket(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::Packet,
                    ),
                    #[codec(index = 20)]
                    TimeoutOnClosePacket(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::Packet,
                    ),
                    #[codec(index = 21)]
                    Empty(::std::vec::Vec<::core::primitive::u8>),
                    #[codec(index = 22)]
                    ChainError(::std::vec::Vec<::core::primitive::u8>),
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Any {
                pub type_url: ::std::vec::Vec<::core::primitive::u8>,
                pub value: ::std::vec::Vec<::core::primitive::u8>,
            }
        }
        pub mod pallet_identity {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    add_registrar {
                        account: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 1)]
                    set_identity {
                        info:
                            ::std::boxed::Box<runtime_types::pallet_identity::types::IdentityInfo>,
                    },
                    #[codec(index = 2)]
                    set_subs {
                        subs: ::std::vec::Vec<(
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::pallet_identity::types::Data,
                        )>,
                    },
                    #[codec(index = 3)]
                    clear_identity,
                    #[codec(index = 4)]
                    request_judgement {
                        #[codec(compact)]
                        reg_index: ::core::primitive::u32,
                        #[codec(compact)]
                        max_fee: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    cancel_request { reg_index: ::core::primitive::u32 },
                    #[codec(index = 6)]
                    set_fee {
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                        #[codec(compact)]
                        fee: ::core::primitive::u128,
                    },
                    #[codec(index = 7)]
                    set_account_id {
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                        new: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 8)]
                    set_fields {
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                        fields: runtime_types::pallet_identity::types::BitFlags<
                            runtime_types::pallet_identity::types::IdentityField,
                        >,
                    },
                    #[codec(index = 9)]
                    provide_judgement {
                        #[codec(compact)]
                        reg_index: ::core::primitive::u32,
                        target: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        judgement: runtime_types::pallet_identity::types::Judgement<
                            ::core::primitive::u128,
                        >,
                    },
                    #[codec(index = 10)]
                    kill_identity {
                        target: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 11)]
                    add_sub {
                        sub: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        data: runtime_types::pallet_identity::types::Data,
                    },
                    #[codec(index = 12)]
                    rename_sub {
                        sub: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        data: runtime_types::pallet_identity::types::Data,
                    },
                    #[codec(index = 13)]
                    remove_sub {
                        sub: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 14)]
                    quit_sub,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    TooManySubAccounts,
                    #[codec(index = 1)]
                    NotFound,
                    #[codec(index = 2)]
                    NotNamed,
                    #[codec(index = 3)]
                    EmptyIndex,
                    #[codec(index = 4)]
                    FeeChanged,
                    #[codec(index = 5)]
                    NoIdentity,
                    #[codec(index = 6)]
                    StickyJudgement,
                    #[codec(index = 7)]
                    JudgementGiven,
                    #[codec(index = 8)]
                    InvalidJudgement,
                    #[codec(index = 9)]
                    InvalidIndex,
                    #[codec(index = 10)]
                    InvalidTarget,
                    #[codec(index = 11)]
                    TooManyFields,
                    #[codec(index = 12)]
                    TooManyRegistrars,
                    #[codec(index = 13)]
                    AlreadyClaimed,
                    #[codec(index = 14)]
                    NotSub,
                    #[codec(index = 15)]
                    NotOwned,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    IdentitySet(::subxt::sp_core::crypto::AccountId32),
                    #[codec(index = 1)]
                    IdentityCleared(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 2)]
                    IdentityKilled(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 3)]
                    JudgementRequested(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    ),
                    #[codec(index = 4)]
                    JudgementUnrequested(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    ),
                    #[codec(index = 5)]
                    JudgementGiven(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    ),
                    #[codec(index = 6)]
                    RegistrarAdded(::core::primitive::u32),
                    #[codec(index = 7)]
                    SubIdentityAdded(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 8)]
                    SubIdentityRemoved(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 9)]
                    SubIdentityRevoked(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: CompactAs,
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                )]
                pub struct BitFlags<_0>(
                    pub ::core::primitive::u64,
                    #[codec(skip)] pub ::core::marker::PhantomData<_0>,
                );
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Data {
                    #[codec(index = 0)]
                    None,
                    #[codec(index = 1)]
                    Raw0([::core::primitive::u8; 0usize]),
                    #[codec(index = 2)]
                    Raw1([::core::primitive::u8; 1usize]),
                    #[codec(index = 3)]
                    Raw2([::core::primitive::u8; 2usize]),
                    #[codec(index = 4)]
                    Raw3([::core::primitive::u8; 3usize]),
                    #[codec(index = 5)]
                    Raw4([::core::primitive::u8; 4usize]),
                    #[codec(index = 6)]
                    Raw5([::core::primitive::u8; 5usize]),
                    #[codec(index = 7)]
                    Raw6([::core::primitive::u8; 6usize]),
                    #[codec(index = 8)]
                    Raw7([::core::primitive::u8; 7usize]),
                    #[codec(index = 9)]
                    Raw8([::core::primitive::u8; 8usize]),
                    #[codec(index = 10)]
                    Raw9([::core::primitive::u8; 9usize]),
                    #[codec(index = 11)]
                    Raw10([::core::primitive::u8; 10usize]),
                    #[codec(index = 12)]
                    Raw11([::core::primitive::u8; 11usize]),
                    #[codec(index = 13)]
                    Raw12([::core::primitive::u8; 12usize]),
                    #[codec(index = 14)]
                    Raw13([::core::primitive::u8; 13usize]),
                    #[codec(index = 15)]
                    Raw14([::core::primitive::u8; 14usize]),
                    #[codec(index = 16)]
                    Raw15([::core::primitive::u8; 15usize]),
                    #[codec(index = 17)]
                    Raw16([::core::primitive::u8; 16usize]),
                    #[codec(index = 18)]
                    Raw17([::core::primitive::u8; 17usize]),
                    #[codec(index = 19)]
                    Raw18([::core::primitive::u8; 18usize]),
                    #[codec(index = 20)]
                    Raw19([::core::primitive::u8; 19usize]),
                    #[codec(index = 21)]
                    Raw20([::core::primitive::u8; 20usize]),
                    #[codec(index = 22)]
                    Raw21([::core::primitive::u8; 21usize]),
                    #[codec(index = 23)]
                    Raw22([::core::primitive::u8; 22usize]),
                    #[codec(index = 24)]
                    Raw23([::core::primitive::u8; 23usize]),
                    #[codec(index = 25)]
                    Raw24([::core::primitive::u8; 24usize]),
                    #[codec(index = 26)]
                    Raw25([::core::primitive::u8; 25usize]),
                    #[codec(index = 27)]
                    Raw26([::core::primitive::u8; 26usize]),
                    #[codec(index = 28)]
                    Raw27([::core::primitive::u8; 27usize]),
                    #[codec(index = 29)]
                    Raw28([::core::primitive::u8; 28usize]),
                    #[codec(index = 30)]
                    Raw29([::core::primitive::u8; 29usize]),
                    #[codec(index = 31)]
                    Raw30([::core::primitive::u8; 30usize]),
                    #[codec(index = 32)]
                    Raw31([::core::primitive::u8; 31usize]),
                    #[codec(index = 33)]
                    Raw32([::core::primitive::u8; 32usize]),
                    #[codec(index = 34)]
                    BlakeTwo256([::core::primitive::u8; 32usize]),
                    #[codec(index = 35)]
                    Sha256([::core::primitive::u8; 32usize]),
                    #[codec(index = 36)]
                    Keccak256([::core::primitive::u8; 32usize]),
                    #[codec(index = 37)]
                    ShaThree256([::core::primitive::u8; 32usize]),
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum IdentityField {
                    #[codec(index = 1)]
                    Display,
                    #[codec(index = 2)]
                    Legal,
                    #[codec(index = 4)]
                    Web,
                    #[codec(index = 8)]
                    Riot,
                    #[codec(index = 16)]
                    Email,
                    #[codec(index = 32)]
                    PgpFingerprint,
                    #[codec(index = 64)]
                    Image,
                    #[codec(index = 128)]
                    Twitter,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct IdentityInfo {
                    pub additional:
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<(
                            runtime_types::pallet_identity::types::Data,
                            runtime_types::pallet_identity::types::Data,
                        )>,
                    pub display: runtime_types::pallet_identity::types::Data,
                    pub legal: runtime_types::pallet_identity::types::Data,
                    pub web: runtime_types::pallet_identity::types::Data,
                    pub riot: runtime_types::pallet_identity::types::Data,
                    pub email: runtime_types::pallet_identity::types::Data,
                    pub pgp_fingerprint: ::core::option::Option<[::core::primitive::u8; 20usize]>,
                    pub image: runtime_types::pallet_identity::types::Data,
                    pub twitter: runtime_types::pallet_identity::types::Data,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Judgement<_0> {
                    #[codec(index = 0)]
                    Unknown,
                    #[codec(index = 1)]
                    FeePaid(_0),
                    #[codec(index = 2)]
                    Reasonable,
                    #[codec(index = 3)]
                    KnownGood,
                    #[codec(index = 4)]
                    OutOfDate,
                    #[codec(index = 5)]
                    LowQuality,
                    #[codec(index = 6)]
                    Erroneous,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct RegistrarInfo<_0, _1> {
                    pub account: _1,
                    pub fee: _0,
                    pub fields: runtime_types::pallet_identity::types::BitFlags<
                        runtime_types::pallet_identity::types::IdentityField,
                    >,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Registration<_0> {
                    pub judgements:
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<(
                            ::core::primitive::u32,
                            runtime_types::pallet_identity::types::Judgement<_0>,
                        )>,
                    pub deposit: _0,
                    pub info: runtime_types::pallet_identity::types::IdentityInfo,
                }
            }
        }
        pub mod pallet_multisig {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    as_multi_threshold_1 {
                        other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                        call: ::std::boxed::Box<runtime_types::shiden_runtime::Call>,
                    },
                    #[codec(index = 1)]
                    as_multi {
                        threshold: ::core::primitive::u16,
                        other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                        maybe_timepoint: ::core::option::Option<
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        >,
                        call: ::std::vec::Vec<::core::primitive::u8>,
                        store_call: ::core::primitive::bool,
                        max_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 2)]
                    approve_as_multi {
                        threshold: ::core::primitive::u16,
                        other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                        maybe_timepoint: ::core::option::Option<
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        >,
                        call_hash: [::core::primitive::u8; 32usize],
                        max_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 3)]
                    cancel_as_multi {
                        threshold: ::core::primitive::u16,
                        other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                        timepoint:
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        call_hash: [::core::primitive::u8; 32usize],
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    MinimumThreshold,
                    #[codec(index = 1)]
                    AlreadyApproved,
                    #[codec(index = 2)]
                    NoApprovalsNeeded,
                    #[codec(index = 3)]
                    TooFewSignatories,
                    #[codec(index = 4)]
                    TooManySignatories,
                    #[codec(index = 5)]
                    SignatoriesOutOfOrder,
                    #[codec(index = 6)]
                    SenderInSignatories,
                    #[codec(index = 7)]
                    NotFound,
                    #[codec(index = 8)]
                    NotOwner,
                    #[codec(index = 9)]
                    NoTimepoint,
                    #[codec(index = 10)]
                    WrongTimepoint,
                    #[codec(index = 11)]
                    UnexpectedTimepoint,
                    #[codec(index = 12)]
                    MaxWeightTooLow,
                    #[codec(index = 13)]
                    AlreadyStored,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    NewMultisig(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        [::core::primitive::u8; 32usize],
                    ),
                    #[codec(index = 1)]
                    MultisigApproval(
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        ::subxt::sp_core::crypto::AccountId32,
                        [::core::primitive::u8; 32usize],
                    ),
                    #[codec(index = 2)]
                    MultisigExecuted(
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        ::subxt::sp_core::crypto::AccountId32,
                        [::core::primitive::u8; 32usize],
                        ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    ),
                    #[codec(index = 3)]
                    MultisigCancelled(
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        ::subxt::sp_core::crypto::AccountId32,
                        [::core::primitive::u8; 32usize],
                    ),
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Multisig<_0, _1, _2> {
                pub when: runtime_types::pallet_multisig::Timepoint<_0>,
                pub deposit: _1,
                pub depositor: _2,
                pub approvals: ::std::vec::Vec<_2>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Timepoint<_0> {
                pub height: _0,
                pub index: _0,
            }
        }
        pub mod pallet_session {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    set_keys {
                        keys: runtime_types::shiden_runtime::SessionKeys,
                        proof: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    purge_keys,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidProof,
                    #[codec(index = 1)]
                    NoAssociatedValidatorId,
                    #[codec(index = 2)]
                    DuplicatedKey,
                    #[codec(index = 3)]
                    NoKeys,
                    #[codec(index = 4)]
                    NoAccount,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    NewSession(::core::primitive::u32),
                }
            }
        }
        pub mod pallet_sudo {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    sudo {
                        call: ::std::boxed::Box<runtime_types::shiden_runtime::Call>,
                    },
                    #[codec(index = 1)]
                    sudo_unchecked_weight {
                        call: ::std::boxed::Box<runtime_types::shiden_runtime::Call>,
                        weight: ::core::primitive::u64,
                    },
                    #[codec(index = 2)]
                    set_key {
                        new: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 3)]
                    sudo_as {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        call: ::std::boxed::Box<runtime_types::shiden_runtime::Call>,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    RequireSudo,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    Sudid(::core::result::Result<(), runtime_types::sp_runtime::DispatchError>),
                    #[codec(index = 1)]
                    KeyChanged(::subxt::sp_core::crypto::AccountId32),
                    #[codec(index = 2)]
                    SudoAsDone(
                        ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    ),
                }
            }
        }
        pub mod pallet_timestamp {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    set {
                        #[codec(compact)]
                        now: ::core::primitive::u64,
                    },
                }
            }
        }
        pub mod pallet_transaction_payment {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ChargeTransactionPayment(#[codec(compact)] pub ::core::primitive::u128);
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Releases {
                #[codec(index = 0)]
                V1Ancient,
                #[codec(index = 1)]
                V2,
            }
        }
        pub mod pallet_utility {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    batch {
                        calls: ::std::vec::Vec<runtime_types::shiden_runtime::Call>,
                    },
                    #[codec(index = 1)]
                    as_derivative {
                        index: ::core::primitive::u16,
                        call: ::std::boxed::Box<runtime_types::shiden_runtime::Call>,
                    },
                    #[codec(index = 2)]
                    batch_all {
                        calls: ::std::vec::Vec<runtime_types::shiden_runtime::Call>,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    TooManyCalls,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    BatchInterrupted(
                        ::core::primitive::u32,
                        runtime_types::sp_runtime::DispatchError,
                    ),
                    #[codec(index = 1)]
                    BatchCompleted,
                    #[codec(index = 2)]
                    ItemCompleted,
                }
            }
        }
        pub mod pallet_vesting {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    vest,
                    #[codec(index = 1)]
                    vest_other {
                        target: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 2)]
                    vested_transfer {
                        target: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        schedule: runtime_types::pallet_vesting::vesting_info::VestingInfo<
                            ::core::primitive::u128,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 3)]
                    force_vested_transfer {
                        source: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        target: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        schedule: runtime_types::pallet_vesting::vesting_info::VestingInfo<
                            ::core::primitive::u128,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 4)]
                    merge_schedules {
                        schedule1_index: ::core::primitive::u32,
                        schedule2_index: ::core::primitive::u32,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    NotVesting,
                    #[codec(index = 1)]
                    AtMaxVestingSchedules,
                    #[codec(index = 2)]
                    AmountLow,
                    #[codec(index = 3)]
                    ScheduleIndexOutOfBounds,
                    #[codec(index = 4)]
                    InvalidScheduleParams,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    VestingUpdated(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 1)]
                    VestingCompleted(::subxt::sp_core::crypto::AccountId32),
                }
            }
            pub mod vesting_info {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct VestingInfo<_0, _1> {
                    pub locked: _0,
                    pub per_block: _0,
                    pub starting_block: _1,
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Releases {
                #[codec(index = 0)]
                V0,
                #[codec(index = 1)]
                V1,
            }
        }
        pub mod polkadot_core_primitives {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct InboundDownwardMessage<_0> {
                pub sent_at: _0,
                pub msg: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct InboundHrmpMessage<_0> {
                pub sent_at: _0,
                pub data: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct OutboundHrmpMessage<_0> {
                pub recipient: _0,
                pub data: ::std::vec::Vec<::core::primitive::u8>,
            }
        }
        pub mod polkadot_parachain {
            use super::runtime_types;
            pub mod primitives {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct HeadData(pub ::std::vec::Vec<::core::primitive::u8>);
                #[derive(
                    :: subxt :: codec :: CompactAs,
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                )]
                pub struct Id(pub ::core::primitive::u32);
            }
        }
        pub mod polkadot_primitives {
            use super::runtime_types;
            pub mod v1 {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct AbridgedHostConfiguration {
                    pub max_code_size: ::core::primitive::u32,
                    pub max_head_data_size: ::core::primitive::u32,
                    pub max_upward_queue_count: ::core::primitive::u32,
                    pub max_upward_queue_size: ::core::primitive::u32,
                    pub max_upward_message_size: ::core::primitive::u32,
                    pub max_upward_message_num_per_candidate: ::core::primitive::u32,
                    pub hrmp_max_message_num_per_candidate: ::core::primitive::u32,
                    pub validation_upgrade_frequency: ::core::primitive::u32,
                    pub validation_upgrade_delay: ::core::primitive::u32,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct AbridgedHrmpChannel {
                    pub max_capacity: ::core::primitive::u32,
                    pub max_total_size: ::core::primitive::u32,
                    pub max_message_size: ::core::primitive::u32,
                    pub msg_count: ::core::primitive::u32,
                    pub total_size: ::core::primitive::u32,
                    pub mqc_head: ::core::option::Option<::subxt::sp_core::H256>,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct PersistedValidationData<_0, _1> {
                    pub parent_head: runtime_types::polkadot_parachain::primitives::HeadData,
                    pub relay_parent_number: _1,
                    pub relay_parent_storage_root: _0,
                    pub max_pov_size: _1,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum UpgradeRestriction {
                    #[codec(index = 0)]
                    Present,
                }
            }
        }
        pub mod primitive_types {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct H160(pub [::core::primitive::u8; 20usize]);
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct H256(pub [::core::primitive::u8; 32usize]);
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct U256(pub [::core::primitive::u64; 4usize]);
        }
        pub mod shiden_runtime {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Call {
                #[codec(index = 10)]
                System(runtime_types::frame_system::pallet::Call),
                #[codec(index = 11)]
                Utility(runtime_types::pallet_utility::pallet::Call),
                #[codec(index = 12)]
                Identity(runtime_types::pallet_identity::pallet::Call),
                #[codec(index = 13)]
                Timestamp(runtime_types::pallet_timestamp::pallet::Call),
                #[codec(index = 14)]
                Multisig(runtime_types::pallet_multisig::pallet::Call),
                #[codec(index = 20)]
                ParachainSystem(runtime_types::cumulus_pallet_parachain_system::pallet::Call),
                #[codec(index = 31)]
                Balances(runtime_types::pallet_balances::pallet::Call),
                #[codec(index = 32)]
                Vesting(runtime_types::pallet_vesting::pallet::Call),
                #[codec(index = 34)]
                DappsStaking(runtime_types::pallet_dapps_staking::pallet::pallet::Call),
                #[codec(index = 40)]
                Authorship(runtime_types::pallet_authorship::pallet::Call),
                #[codec(index = 41)]
                CollatorSelection(runtime_types::pallet_collator_selection::pallet::Call),
                #[codec(index = 42)]
                Session(runtime_types::pallet_session::pallet::Call),
                #[codec(index = 60)]
                EVM(runtime_types::pallet_evm::pallet::Call),
                #[codec(index = 61)]
                Ethereum(runtime_types::pallet_ethereum::pallet::Call),
                #[codec(index = 62)]
                EthCall(runtime_types::pallet_custom_signatures::pallet::Call),
                #[codec(index = 99)]
                Sudo(runtime_types::pallet_sudo::pallet::Call),
                #[codec(index = 110)]
                Ibc(runtime_types::pallet_ibc::pallet::Call),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Event {
                #[codec(index = 10)]
                System(runtime_types::frame_system::pallet::Event),
                #[codec(index = 11)]
                Utility(runtime_types::pallet_utility::pallet::Event),
                #[codec(index = 12)]
                Identity(runtime_types::pallet_identity::pallet::Event),
                #[codec(index = 14)]
                Multisig(runtime_types::pallet_multisig::pallet::Event),
                #[codec(index = 20)]
                ParachainSystem(runtime_types::cumulus_pallet_parachain_system::pallet::Event),
                #[codec(index = 31)]
                Balances(runtime_types::pallet_balances::pallet::Event),
                #[codec(index = 32)]
                Vesting(runtime_types::pallet_vesting::pallet::Event),
                #[codec(index = 34)]
                DappsStaking(runtime_types::pallet_dapps_staking::pallet::pallet::Event),
                #[codec(index = 41)]
                CollatorSelection(runtime_types::pallet_collator_selection::pallet::Event),
                #[codec(index = 42)]
                Session(runtime_types::pallet_session::pallet::Event),
                #[codec(index = 60)]
                EVM(runtime_types::pallet_evm::pallet::Event),
                #[codec(index = 61)]
                Ethereum(runtime_types::pallet_ethereum::pallet::Event),
                #[codec(index = 62)]
                EthCall(runtime_types::pallet_custom_signatures::pallet::Event),
                #[codec(index = 99)]
                Sudo(runtime_types::pallet_sudo::pallet::Event),
                #[codec(index = 110)]
                Ibc(runtime_types::pallet_ibc::pallet::Event),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Runtime {}
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SessionKeys {
                pub aura: runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum SmartContract<_0> {
                #[codec(index = 0)]
                Evm(runtime_types::primitive_types::H160),
                #[codec(index = 1)]
                Wasm(_0),
            }
        }
        pub mod sp_arithmetic {
            use super::runtime_types;
            pub mod fixed_point {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: CompactAs,
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                )]
                pub struct FixedU128(pub ::core::primitive::u128);
            }
            pub mod per_things {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: CompactAs,
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                )]
                pub struct Perbill(pub ::core::primitive::u32);
            }
        }
        pub mod sp_consensus_aura {
            use super::runtime_types;
            pub mod sr25519 {
                use super::runtime_types;
                pub mod app_sr25519 {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct Public(pub runtime_types::sp_core::sr25519::Public);
                }
            }
        }
        pub mod sp_consensus_slots {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: CompactAs,
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
            )]
            pub struct Slot(pub ::core::primitive::u64);
        }
        pub mod sp_core {
            use super::runtime_types;
            pub mod changes_trie {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct ChangesTrieConfiguration {
                    pub digest_interval: ::core::primitive::u32,
                    pub digest_levels: ::core::primitive::u32,
                }
            }
            pub mod crypto {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct AccountId32(pub [::core::primitive::u8; 32usize]);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct KeyTypeId(pub [::core::primitive::u8; 4usize]);
            }
            pub mod ecdsa {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Signature(pub [::core::primitive::u8; 65usize]);
            }
            pub mod ed25519 {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            pub mod sr25519 {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
        }
        pub mod sp_runtime {
            use super::runtime_types;
            pub mod generic {
                use super::runtime_types;
                pub mod digest {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum ChangesTrieSignal {
                        #[codec(index = 0)]
                        NewConfiguration(
                            ::core::option::Option<
                                runtime_types::sp_core::changes_trie::ChangesTrieConfiguration,
                            >,
                        ),
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct Digest<_0> {
                        pub logs: ::std::vec::Vec<
                            runtime_types::sp_runtime::generic::digest::DigestItem<_0>,
                        >,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum DigestItem<_0> {
                        #[codec(index = 2)]
                        ChangesTrieRoot(_0),
                        #[codec(index = 6)]
                        PreRuntime(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 4)]
                        Consensus(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 5)]
                        Seal(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 7)]
                        ChangesTrieSignal(
                            runtime_types::sp_runtime::generic::digest::ChangesTrieSignal,
                        ),
                        #[codec(index = 0)]
                        Other(::std::vec::Vec<::core::primitive::u8>),
                        #[codec(index = 8)]
                        RuntimeEnvironmentUpdated,
                    }
                }
                pub mod era {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
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
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct Header<_0, _1> {
                        pub parent_hash: ::subxt::sp_core::H256,
                        #[codec(compact)]
                        pub number: _0,
                        pub state_root: ::subxt::sp_core::H256,
                        pub extrinsics_root: ::subxt::sp_core::H256,
                        pub digest: runtime_types::sp_runtime::generic::digest::Digest<
                            ::subxt::sp_core::H256,
                        >,
                        #[codec(skip)]
                        pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
                    }
                }
                pub mod unchecked_extrinsic {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct UncheckedExtrinsic<_0, _1, _2, _3>(
                        ::std::vec::Vec<::core::primitive::u8>,
                        #[codec(skip)] pub ::core::marker::PhantomData<(_1, _0, _2, _3)>,
                    );
                }
            }
            pub mod multiaddress {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum MultiAddress<_0, _1> {
                    #[codec(index = 0)]
                    Id(_0),
                    #[codec(index = 1)]
                    Index(#[codec(compact)] _1),
                    #[codec(index = 2)]
                    Raw(::std::vec::Vec<::core::primitive::u8>),
                    #[codec(index = 3)]
                    Address32([::core::primitive::u8; 32usize]),
                    #[codec(index = 4)]
                    Address20([::core::primitive::u8; 20usize]),
                }
            }
            pub mod traits {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct BlakeTwo256 {}
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum ArithmeticError {
                #[codec(index = 0)]
                Underflow,
                #[codec(index = 1)]
                Overflow,
                #[codec(index = 2)]
                DivisionByZero,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum DispatchError {
                #[codec(index = 0)]
                Other,
                #[codec(index = 1)]
                CannotLookup,
                #[codec(index = 2)]
                BadOrigin,
                #[codec(index = 3)]
                Module {
                    index: ::core::primitive::u8,
                    error: ::core::primitive::u8,
                },
                #[codec(index = 4)]
                ConsumerRemaining,
                #[codec(index = 5)]
                NoProviders,
                #[codec(index = 6)]
                Token(runtime_types::sp_runtime::TokenError),
                #[codec(index = 7)]
                Arithmetic(runtime_types::sp_runtime::ArithmeticError),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum MultiSignature {
                #[codec(index = 0)]
                Ed25519(runtime_types::sp_core::ed25519::Signature),
                #[codec(index = 1)]
                Sr25519(runtime_types::sp_core::sr25519::Signature),
                #[codec(index = 2)]
                Ecdsa(runtime_types::sp_core::ecdsa::Signature),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum TokenError {
                #[codec(index = 0)]
                NoFunds,
                #[codec(index = 1)]
                WouldDie,
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
            }
        }
        pub mod sp_trie {
            use super::runtime_types;
            pub mod storage_proof {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct StorageProof {
                    pub trie_nodes: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                }
            }
        }
        pub mod sp_version {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RuntimeVersion {
                pub spec_name: ::std::string::String,
                pub impl_name: ::std::string::String,
                pub authoring_version: ::core::primitive::u32,
                pub spec_version: ::core::primitive::u32,
                pub impl_version: ::core::primitive::u32,
                pub apis:
                    ::std::vec::Vec<([::core::primitive::u8; 8usize], ::core::primitive::u32)>,
                pub transaction_version: ::core::primitive::u32,
            }
        }
    }
    #[doc = r" Default configuration of common types for a target Substrate runtime."]
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct DefaultConfig;
    impl ::subxt::Config for DefaultConfig {
        type Index = u32;
        type BlockNumber = u32;
        type Hash = ::subxt::sp_core::H256;
        type Hashing = ::subxt::sp_runtime::traits::BlakeTwo256;
        type AccountId = ::subxt::sp_runtime::AccountId32;
        type Address = ::subxt::sp_runtime::MultiAddress<Self::AccountId, u32>;
        type Header = ::subxt::sp_runtime::generic::Header<
            Self::BlockNumber,
            ::subxt::sp_runtime::traits::BlakeTwo256,
        >;
        type Signature = ::subxt::sp_runtime::MultiSignature;
        type Extrinsic = ::subxt::sp_runtime::OpaqueExtrinsic;
    }
    impl ::subxt::ExtrinsicExtraData<DefaultConfig> for DefaultConfig {
        type AccountData = AccountData;
        type Extra = ::subxt::DefaultExtra<DefaultConfig>;
    }
    pub type AccountData = self::system::storage::Account;
    impl ::subxt::AccountData<DefaultConfig> for AccountData {
        fn nonce(
            result: &<Self as ::subxt::StorageEntry>::Value,
        ) -> <DefaultConfig as ::subxt::Config>::Index {
            result.nonce
        }
        fn storage_entry(account_id: <DefaultConfig as ::subxt::Config>::AccountId) -> Self {
            Self(account_id)
        }
    }
    pub struct RuntimeApi<T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
        pub client: ::subxt::Client<T>,
    }
    impl<T> ::core::convert::From<::subxt::Client<T>> for RuntimeApi<T>
    where
        T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
    {
        fn from(client: ::subxt::Client<T>) -> Self {
            Self { client }
        }
    }
    impl<'a, T> RuntimeApi<T>
    where
        T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
    {
        pub fn storage(&'a self) -> StorageApi<'a, T> {
            StorageApi {
                client: &self.client,
            }
        }
        pub fn tx(&'a self) -> TransactionApi<'a, T> {
            TransactionApi {
                client: &self.client,
            }
        }
    }
    pub struct StorageApi<'a, T>
    where
        T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
    {
        client: &'a ::subxt::Client<T>,
    }
    impl<'a, T> StorageApi<'a, T>
    where
        T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
    {
        pub fn system(&self) -> system::storage::StorageApi<'a, T> {
            system::storage::StorageApi::new(self.client)
        }
        pub fn identity(&self) -> identity::storage::StorageApi<'a, T> {
            identity::storage::StorageApi::new(self.client)
        }
        pub fn timestamp(&self) -> timestamp::storage::StorageApi<'a, T> {
            timestamp::storage::StorageApi::new(self.client)
        }
        pub fn multisig(&self) -> multisig::storage::StorageApi<'a, T> {
            multisig::storage::StorageApi::new(self.client)
        }
        pub fn parachain_system(&self) -> parachain_system::storage::StorageApi<'a, T> {
            parachain_system::storage::StorageApi::new(self.client)
        }
        pub fn parachain_info(&self) -> parachain_info::storage::StorageApi<'a, T> {
            parachain_info::storage::StorageApi::new(self.client)
        }
        pub fn transaction_payment(&self) -> transaction_payment::storage::StorageApi<'a, T> {
            transaction_payment::storage::StorageApi::new(self.client)
        }
        pub fn balances(&self) -> balances::storage::StorageApi<'a, T> {
            balances::storage::StorageApi::new(self.client)
        }
        pub fn vesting(&self) -> vesting::storage::StorageApi<'a, T> {
            vesting::storage::StorageApi::new(self.client)
        }
        pub fn dapps_staking(&self) -> dapps_staking::storage::StorageApi<'a, T> {
            dapps_staking::storage::StorageApi::new(self.client)
        }
        pub fn authorship(&self) -> authorship::storage::StorageApi<'a, T> {
            authorship::storage::StorageApi::new(self.client)
        }
        pub fn collator_selection(&self) -> collator_selection::storage::StorageApi<'a, T> {
            collator_selection::storage::StorageApi::new(self.client)
        }
        pub fn session(&self) -> session::storage::StorageApi<'a, T> {
            session::storage::StorageApi::new(self.client)
        }
        pub fn aura(&self) -> aura::storage::StorageApi<'a, T> {
            aura::storage::StorageApi::new(self.client)
        }
        pub fn aura_ext(&self) -> aura_ext::storage::StorageApi<'a, T> {
            aura_ext::storage::StorageApi::new(self.client)
        }
        pub fn evm(&self) -> evm::storage::StorageApi<'a, T> {
            evm::storage::StorageApi::new(self.client)
        }
        pub fn ethereum(&self) -> ethereum::storage::StorageApi<'a, T> {
            ethereum::storage::StorageApi::new(self.client)
        }
        pub fn sudo(&self) -> sudo::storage::StorageApi<'a, T> {
            sudo::storage::StorageApi::new(self.client)
        }
        pub fn ibc(&self) -> ibc::storage::StorageApi<'a, T> {
            ibc::storage::StorageApi::new(self.client)
        }
    }
    pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
        client: &'a ::subxt::Client<T>,
    }
    impl<'a, T> TransactionApi<'a, T>
    where
        T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
    {
        pub fn system(&self) -> system::calls::TransactionApi<'a, T> {
            system::calls::TransactionApi::new(self.client)
        }
        pub fn utility(&self) -> utility::calls::TransactionApi<'a, T> {
            utility::calls::TransactionApi::new(self.client)
        }
        pub fn identity(&self) -> identity::calls::TransactionApi<'a, T> {
            identity::calls::TransactionApi::new(self.client)
        }
        pub fn timestamp(&self) -> timestamp::calls::TransactionApi<'a, T> {
            timestamp::calls::TransactionApi::new(self.client)
        }
        pub fn multisig(&self) -> multisig::calls::TransactionApi<'a, T> {
            multisig::calls::TransactionApi::new(self.client)
        }
        pub fn parachain_system(&self) -> parachain_system::calls::TransactionApi<'a, T> {
            parachain_system::calls::TransactionApi::new(self.client)
        }
        pub fn balances(&self) -> balances::calls::TransactionApi<'a, T> {
            balances::calls::TransactionApi::new(self.client)
        }
        pub fn vesting(&self) -> vesting::calls::TransactionApi<'a, T> {
            vesting::calls::TransactionApi::new(self.client)
        }
        pub fn dapps_staking(&self) -> dapps_staking::calls::TransactionApi<'a, T> {
            dapps_staking::calls::TransactionApi::new(self.client)
        }
        pub fn authorship(&self) -> authorship::calls::TransactionApi<'a, T> {
            authorship::calls::TransactionApi::new(self.client)
        }
        pub fn collator_selection(&self) -> collator_selection::calls::TransactionApi<'a, T> {
            collator_selection::calls::TransactionApi::new(self.client)
        }
        pub fn session(&self) -> session::calls::TransactionApi<'a, T> {
            session::calls::TransactionApi::new(self.client)
        }
        pub fn evm(&self) -> evm::calls::TransactionApi<'a, T> {
            evm::calls::TransactionApi::new(self.client)
        }
        pub fn ethereum(&self) -> ethereum::calls::TransactionApi<'a, T> {
            ethereum::calls::TransactionApi::new(self.client)
        }
        pub fn eth_call(&self) -> eth_call::calls::TransactionApi<'a, T> {
            eth_call::calls::TransactionApi::new(self.client)
        }
        pub fn sudo(&self) -> sudo::calls::TransactionApi<'a, T> {
            sudo::calls::TransactionApi::new(self.client)
        }
        pub fn ibc(&self) -> ibc::calls::TransactionApi<'a, T> {
            ibc::calls::TransactionApi::new(self.client)
        }
    }
}
