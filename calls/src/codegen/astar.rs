#[allow(dead_code, unused_imports, non_camel_case_types)]
pub mod api {
    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
    pub enum Event {
        #[codec(index = 0)]
        System(system::Event),
        #[codec(index = 5)]
        Grandpa(grandpa::Event),
        #[codec(index = 6)]
        Balances(balances::Event),
        #[codec(index = 8)]
        OctopusAppchain(octopus_appchain::Event),
        #[codec(index = 9)]
        OctopusLpos(octopus_lpos::Event),
        #[codec(index = 10)]
        OctopusUpwardMessages(octopus_upward_messages::Event),
        #[codec(index = 11)]
        Session(session::Event),
        #[codec(index = 12)]
        ImOnline(im_online::Event),
        #[codec(index = 14)]
        Assets(assets::Event),
        #[codec(index = 18)]
        Sudo(sudo::Event),
        #[codec(index = 19)]
        Ibc(ibc::Event),
        #[codec(index = 20)]
        TemplateModule(template_module::Event),
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
                        runtime_types::node_template_runtime::Event,
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
                            runtime_types::node_template_runtime::Event,
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
    pub mod randomness_collective_flip {
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct RandomMaterial;
            impl ::subxt::StorageEntry for RandomMaterial {
                const PALLET: &'static str = "RandomnessCollectiveFlip";
                const STORAGE: &'static str = "RandomMaterial";
                type Value = ::std::vec::Vec<::subxt::sp_core::H256>;
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
                pub async fn random_material(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::std::vec::Vec<::subxt::sp_core::H256>, ::subxt::Error>
                {
                    let entry = RandomMaterial;
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
    pub mod babe {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ReportEquivocation {
                pub equivocation_proof: runtime_types::sp_consensus_slots::EquivocationProof<
                    runtime_types::sp_runtime::generic::header::Header<
                        ::core::primitive::u32,
                        runtime_types::sp_runtime::traits::BlakeTwo256,
                    >,
                    runtime_types::sp_consensus_babe::app::Public,
                >,
                pub key_owner_proof: runtime_types::sp_session::MembershipProof,
            }
            impl ::subxt::Call for ReportEquivocation {
                const PALLET: &'static str = "Babe";
                const FUNCTION: &'static str = "report_equivocation";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ReportEquivocationUnsigned {
                pub equivocation_proof: runtime_types::sp_consensus_slots::EquivocationProof<
                    runtime_types::sp_runtime::generic::header::Header<
                        ::core::primitive::u32,
                        runtime_types::sp_runtime::traits::BlakeTwo256,
                    >,
                    runtime_types::sp_consensus_babe::app::Public,
                >,
                pub key_owner_proof: runtime_types::sp_session::MembershipProof,
            }
            impl ::subxt::Call for ReportEquivocationUnsigned {
                const PALLET: &'static str = "Babe";
                const FUNCTION: &'static str = "report_equivocation_unsigned";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct PlanConfigChange {
                pub config: runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
            }
            impl ::subxt::Call for PlanConfigChange {
                const PALLET: &'static str = "Babe";
                const FUNCTION: &'static str = "plan_config_change";
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
                pub fn report_equivocation(
                    &self,
                    equivocation_proof: runtime_types::sp_consensus_slots::EquivocationProof<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                        runtime_types::sp_consensus_babe::app::Public,
                    >,
                    key_owner_proof: runtime_types::sp_session::MembershipProof,
                ) -> ::subxt::SubmittableExtrinsic<T, ReportEquivocation> {
                    let call = ReportEquivocation {
                        equivocation_proof,
                        key_owner_proof,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn report_equivocation_unsigned(
                    &self,
                    equivocation_proof: runtime_types::sp_consensus_slots::EquivocationProof<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                        runtime_types::sp_consensus_babe::app::Public,
                    >,
                    key_owner_proof: runtime_types::sp_session::MembershipProof,
                ) -> ::subxt::SubmittableExtrinsic<T, ReportEquivocationUnsigned> {
                    let call = ReportEquivocationUnsigned {
                        equivocation_proof,
                        key_owner_proof,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn plan_config_change(
                    &self,
                    config: runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
                ) -> ::subxt::SubmittableExtrinsic<T, PlanConfigChange> {
                    let call = PlanConfigChange { config };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct EpochIndex;
            impl ::subxt::StorageEntry for EpochIndex {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "EpochIndex";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Authorities;
            impl ::subxt::StorageEntry for Authorities {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "Authorities";
                type Value =
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_consensus_babe::app::Public,
                        ::core::primitive::u64,
                    )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct GenesisSlot;
            impl ::subxt::StorageEntry for GenesisSlot {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "GenesisSlot";
                type Value = runtime_types::sp_consensus_slots::Slot;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentSlot;
            impl ::subxt::StorageEntry for CurrentSlot {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "CurrentSlot";
                type Value = runtime_types::sp_consensus_slots::Slot;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Randomness;
            impl ::subxt::StorageEntry for Randomness {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "Randomness";
                type Value = [::core::primitive::u8; 32usize];
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PendingEpochConfigChange;
            impl ::subxt::StorageEntry for PendingEpochConfigChange {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "PendingEpochConfigChange";
                type Value = runtime_types::sp_consensus_babe::digests::NextConfigDescriptor;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextRandomness;
            impl ::subxt::StorageEntry for NextRandomness {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "NextRandomness";
                type Value = [::core::primitive::u8; 32usize];
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextAuthorities;
            impl ::subxt::StorageEntry for NextAuthorities {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "NextAuthorities";
                type Value =
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_consensus_babe::app::Public,
                        ::core::primitive::u64,
                    )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SegmentIndex;
            impl ::subxt::StorageEntry for SegmentIndex {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "SegmentIndex";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UnderConstruction(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for UnderConstruction {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "UnderConstruction";
                type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    [::core::primitive::u8; 32usize],
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Initialized;
            impl ::subxt::StorageEntry for Initialized {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "Initialized";
                type Value = ::core::option::Option<[::core::primitive::u8; 32usize]>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AuthorVrfRandomness;
            impl ::subxt::StorageEntry for AuthorVrfRandomness {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "AuthorVrfRandomness";
                type Value = ::core::option::Option<[::core::primitive::u8; 32usize]>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EpochStart;
            impl ::subxt::StorageEntry for EpochStart {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "EpochStart";
                type Value = (::core::primitive::u32, ::core::primitive::u32);
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Lateness;
            impl ::subxt::StorageEntry for Lateness {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "Lateness";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EpochConfig;
            impl ::subxt::StorageEntry for EpochConfig {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "EpochConfig";
                type Value = runtime_types::sp_consensus_babe::BabeEpochConfiguration;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextEpochConfig;
            impl ::subxt::StorageEntry for NextEpochConfig {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "NextEpochConfig";
                type Value = runtime_types::sp_consensus_babe::BabeEpochConfiguration;
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
                pub async fn epoch_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::Error>
                {
                    let entry = EpochIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn authorities(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_consensus_babe::app::Public,
                        ::core::primitive::u64,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = Authorities;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn genesis_slot(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<runtime_types::sp_consensus_slots::Slot, ::subxt::Error>
                {
                    let entry = GenesisSlot;
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
                pub async fn randomness(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<[::core::primitive::u8; 32usize], ::subxt::Error>
                {
                    let entry = Randomness;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn pending_epoch_config_change(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
                    >,
                    ::subxt::Error,
                > {
                    let entry = PendingEpochConfigChange;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn next_randomness(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<[::core::primitive::u8; 32usize], ::subxt::Error>
                {
                    let entry = NextRandomness;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_authorities(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_consensus_babe::app::Public,
                        ::core::primitive::u64,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = NextAuthorities;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn segment_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = SegmentIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn under_construction(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        [::core::primitive::u8; 32usize],
                    >,
                    ::subxt::Error,
                > {
                    let entry = UnderConstruction(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn under_construction_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, UnderConstruction>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn initialized(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        ::core::option::Option<[::core::primitive::u8; 32usize]>,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Initialized;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn author_vrf_randomness(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<[::core::primitive::u8; 32usize]>,
                    ::subxt::Error,
                > {
                    let entry = AuthorVrfRandomness;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn epoch_start(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    (::core::primitive::u32, ::core::primitive::u32),
                    ::subxt::Error,
                > {
                    let entry = EpochStart;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn lateness(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = Lateness;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn epoch_config(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::sp_consensus_babe::BabeEpochConfiguration,
                    >,
                    ::subxt::Error,
                > {
                    let entry = EpochConfig;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn next_epoch_config(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::sp_consensus_babe::BabeEpochConfiguration,
                    >,
                    ::subxt::Error,
                > {
                    let entry = NextEpochConfig;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod grandpa {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ReportEquivocation {
                pub equivocation_proof: runtime_types::sp_finality_grandpa::EquivocationProof<
                    ::subxt::sp_core::H256,
                    ::core::primitive::u32,
                >,
                pub key_owner_proof: runtime_types::sp_session::MembershipProof,
            }
            impl ::subxt::Call for ReportEquivocation {
                const PALLET: &'static str = "Grandpa";
                const FUNCTION: &'static str = "report_equivocation";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ReportEquivocationUnsigned {
                pub equivocation_proof: runtime_types::sp_finality_grandpa::EquivocationProof<
                    ::subxt::sp_core::H256,
                    ::core::primitive::u32,
                >,
                pub key_owner_proof: runtime_types::sp_session::MembershipProof,
            }
            impl ::subxt::Call for ReportEquivocationUnsigned {
                const PALLET: &'static str = "Grandpa";
                const FUNCTION: &'static str = "report_equivocation_unsigned";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NoteStalled {
                pub delay: ::core::primitive::u32,
                pub best_finalized_block_number: ::core::primitive::u32,
            }
            impl ::subxt::Call for NoteStalled {
                const PALLET: &'static str = "Grandpa";
                const FUNCTION: &'static str = "note_stalled";
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
                pub fn report_equivocation(
                    &self,
                    equivocation_proof: runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                    key_owner_proof: runtime_types::sp_session::MembershipProof,
                ) -> ::subxt::SubmittableExtrinsic<T, ReportEquivocation> {
                    let call = ReportEquivocation {
                        equivocation_proof,
                        key_owner_proof,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn report_equivocation_unsigned(
                    &self,
                    equivocation_proof: runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                    key_owner_proof: runtime_types::sp_session::MembershipProof,
                ) -> ::subxt::SubmittableExtrinsic<T, ReportEquivocationUnsigned> {
                    let call = ReportEquivocationUnsigned {
                        equivocation_proof,
                        key_owner_proof,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn note_stalled(
                    &self,
                    delay: ::core::primitive::u32,
                    best_finalized_block_number: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, NoteStalled> {
                    let call = NoteStalled {
                        delay,
                        best_finalized_block_number,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_grandpa::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NewAuthorities(
                pub  ::std::vec::Vec<(
                    runtime_types::sp_finality_grandpa::app::Public,
                    ::core::primitive::u64,
                )>,
            );
            impl ::subxt::Event for NewAuthorities {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "NewAuthorities";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Paused {}
            impl ::subxt::Event for Paused {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "Paused";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Resumed {}
            impl ::subxt::Event for Resumed {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "Resumed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct State;
            impl ::subxt::StorageEntry for State {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "State";
                type Value = runtime_types::pallet_grandpa::StoredState<::core::primitive::u32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PendingChange;
            impl ::subxt::StorageEntry for PendingChange {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "PendingChange";
                type Value =
                    runtime_types::pallet_grandpa::StoredPendingChange<::core::primitive::u32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextForced;
            impl ::subxt::StorageEntry for NextForced {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "NextForced";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Stalled;
            impl ::subxt::StorageEntry for Stalled {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "Stalled";
                type Value = (::core::primitive::u32, ::core::primitive::u32);
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentSetId;
            impl ::subxt::StorageEntry for CurrentSetId {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "CurrentSetId";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SetIdSession(pub ::core::primitive::u64);
            impl ::subxt::StorageEntry for SetIdSession {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "SetIdSession";
                type Value = ::core::primitive::u32;
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
                pub async fn state(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_grandpa::StoredState<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = State;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn pending_change(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_grandpa::StoredPendingChange<::core::primitive::u32>,
                    >,
                    ::subxt::Error,
                > {
                    let entry = PendingChange;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn next_forced(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = NextForced;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn stalled(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                    ::subxt::Error,
                > {
                    let entry = Stalled;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn current_set_id(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::Error>
                {
                    let entry = CurrentSetId;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn set_id_session(
                    &self,
                    _0: ::core::primitive::u64,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = SetIdSession(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn set_id_session_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, SetIdSession>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
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
    pub mod octopus_appchain {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SubmitObservations {
                pub payload: runtime_types::pallet_octopus_appchain::ObservationsPayload<
                    runtime_types::sp_runtime::MultiSigner,
                    ::core::primitive::u32,
                    ::subxt::sp_core::crypto::AccountId32,
                >,
                pub signature: runtime_types::sp_runtime::MultiSignature,
            }
            impl ::subxt::Call for SubmitObservations {
                const PALLET: &'static str = "OctopusAppchain";
                const FUNCTION: &'static str = "submit_observations";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceSetPlannedValidators {
                pub validators: ::std::vec::Vec<(
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                )>,
            }
            impl ::subxt::Call for ForceSetPlannedValidators {
                const PALLET: &'static str = "OctopusAppchain";
                const FUNCTION: &'static str = "force_set_planned_validators";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Lock {
                pub receiver_id: ::std::vec::Vec<::core::primitive::u8>,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for Lock {
                const PALLET: &'static str = "OctopusAppchain";
                const FUNCTION: &'static str = "lock";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct MintAsset {
                pub asset_id: ::core::primitive::u32,
                pub sender_id: ::std::vec::Vec<::core::primitive::u8>,
                pub receiver:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for MintAsset {
                const PALLET: &'static str = "OctopusAppchain";
                const FUNCTION: &'static str = "mint_asset";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BurnAsset {
                pub asset_id: ::core::primitive::u32,
                pub receiver_id: ::std::vec::Vec<::core::primitive::u8>,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for BurnAsset {
                const PALLET: &'static str = "OctopusAppchain";
                const FUNCTION: &'static str = "burn_asset";
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
                pub fn submit_observations(
                    &self,
                    payload: runtime_types::pallet_octopus_appchain::ObservationsPayload<
                        runtime_types::sp_runtime::MultiSigner,
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    signature: runtime_types::sp_runtime::MultiSignature,
                ) -> ::subxt::SubmittableExtrinsic<T, SubmitObservations> {
                    let call = SubmitObservations { payload, signature };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_set_planned_validators(
                    &self,
                    validators: ::std::vec::Vec<(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    )>,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceSetPlannedValidators> {
                    let call = ForceSetPlannedValidators { validators };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn lock(
                    &self,
                    receiver_id: ::std::vec::Vec<::core::primitive::u8>,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, Lock> {
                    let call = Lock {
                        receiver_id,
                        amount,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn mint_asset(
                    &self,
                    asset_id: ::core::primitive::u32,
                    sender_id: ::std::vec::Vec<::core::primitive::u8>,
                    receiver: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, MintAsset> {
                    let call = MintAsset {
                        asset_id,
                        sender_id,
                        receiver,
                        amount,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn burn_asset(
                    &self,
                    asset_id: ::core::primitive::u32,
                    receiver_id: ::std::vec::Vec<::core::primitive::u8>,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, BurnAsset> {
                    let call = BurnAsset {
                        asset_id,
                        receiver_id,
                        amount,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_octopus_appchain::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Locked(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Locked {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "Locked";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Unlocked(
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Unlocked {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "Unlocked";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AssetMinted(
                pub ::core::primitive::u32,
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for AssetMinted {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "AssetMinted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AssetBurned(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for AssetBurned {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "AssetBurned";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct AnchorContract;
            impl ::subxt::StorageEntry for AnchorContract {
                const PALLET: &'static str = "OctopusAppchain";
                const STORAGE: &'static str = "AnchorContract";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PlannedValidators;
            impl ::subxt::StorageEntry for PlannedValidators {
                const PALLET: &'static str = "OctopusAppchain";
                const STORAGE: &'static str = "PlannedValidators";
                type Value = ::std::vec::Vec<(
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextFactSequence;
            impl ::subxt::StorageEntry for NextFactSequence {
                const PALLET: &'static str = "OctopusAppchain";
                const STORAGE: &'static str = "NextFactSequence";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Observations(
                runtime_types::pallet_octopus_appchain::ObservationType,
                ::core::primitive::u32,
            );
            impl ::subxt::StorageEntry for Observations {
                const PALLET: &'static str = "OctopusAppchain";
                const STORAGE: &'static str = "Observations";
                type Value = ::std::vec::Vec<
                    runtime_types::pallet_octopus_appchain::Observation<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
                    ])
                }
            }
            pub struct Observing(
                pub  runtime_types::pallet_octopus_appchain::Observation<
                    ::subxt::sp_core::crypto::AccountId32,
                >,
            );
            impl ::subxt::StorageEntry for Observing {
                const PALLET: &'static str = "OctopusAppchain";
                const STORAGE: &'static str = "Observing";
                type Value = ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct AssetIdByName(pub ::std::vec::Vec<::core::primitive::u8>);
            impl ::subxt::StorageEntry for AssetIdByName {
                const PALLET: &'static str = "OctopusAppchain";
                const STORAGE: &'static str = "AssetIdByName";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct PreminedAmount;
            impl ::subxt::StorageEntry for PreminedAmount {
                const PALLET: &'static str = "OctopusAppchain";
                const STORAGE: &'static str = "PreminedAmount";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PalletAccount;
            impl ::subxt::StorageEntry for PalletAccount {
                const PALLET: &'static str = "OctopusAppchain";
                const STORAGE: &'static str = "PalletAccount";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextSubmitObsIndex;
            impl ::subxt::StorageEntry for NextSubmitObsIndex {
                const PALLET: &'static str = "OctopusAppchain";
                const STORAGE: &'static str = "NextSubmitObsIndex";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SubmitSequenceNumber;
            impl ::subxt::StorageEntry for SubmitSequenceNumber {
                const PALLET: &'static str = "OctopusAppchain";
                const STORAGE: &'static str = "SubmitSequenceNumber";
                type Value = ::core::primitive::u32;
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
                pub async fn anchor_contract(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::std::vec::Vec<::core::primitive::u8>, ::subxt::Error>
                {
                    let entry = AnchorContract;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn planned_validators(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = PlannedValidators;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_fact_sequence(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = NextFactSequence;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn observations(
                    &self,
                    _0: runtime_types::pallet_octopus_appchain::ObservationType,
                    _1: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::pallet_octopus_appchain::Observation<
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Observations(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn observations_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Observations>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn observing(
                    &self,
                    _0: runtime_types::pallet_octopus_appchain::Observation<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::Error,
                > {
                    let entry = Observing(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn observing_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Observing>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn asset_id_by_name(
                    &self,
                    _0: ::std::vec::Vec<::core::primitive::u8>,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = AssetIdByName(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn asset_id_by_name_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, AssetIdByName>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn premined_amount(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::Error>
                {
                    let entry = PreminedAmount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn pallet_account(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::crypto::AccountId32, ::subxt::Error>
                {
                    let entry = PalletAccount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_submit_obs_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = NextSubmitObsIndex;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn submit_sequence_number(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = SubmitSequenceNumber;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod octopus_lpos {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetHistoryDepth {
                #[codec(compact)]
                pub new_history_depth: ::core::primitive::u32,
                #[codec(compact)]
                pub era_items_deleted: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetHistoryDepth {
                const PALLET: &'static str = "OctopusLpos";
                const FUNCTION: &'static str = "set_history_depth";
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
                pub fn set_history_depth(
                    &self,
                    new_history_depth: ::core::primitive::u32,
                    era_items_deleted: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetHistoryDepth> {
                    let call = SetHistoryDepth {
                        new_history_depth,
                        era_items_deleted,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_octopus_lpos::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct EraPayout(pub ::core::primitive::u32, pub ::core::primitive::u128);
            impl ::subxt::Event for EraPayout {
                const PALLET: &'static str = "OctopusLpos";
                const EVENT: &'static str = "EraPayout";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Reward(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Reward {
                const PALLET: &'static str = "OctopusLpos";
                const EVENT: &'static str = "Reward";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Slash(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Slash {
                const PALLET: &'static str = "OctopusLpos";
                const EVENT: &'static str = "Slash";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct OldSlashingReportDiscarded(pub ::core::primitive::u32);
            impl ::subxt::Event for OldSlashingReportDiscarded {
                const PALLET: &'static str = "OctopusLpos";
                const EVENT: &'static str = "OldSlashingReportDiscarded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct StakingElection {}
            impl ::subxt::Event for StakingElection {
                const PALLET: &'static str = "OctopusLpos";
                const EVENT: &'static str = "StakingElection";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Bonded(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Bonded {
                const PALLET: &'static str = "OctopusLpos";
                const EVENT: &'static str = "Bonded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Unbonded(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Unbonded {
                const PALLET: &'static str = "OctopusLpos";
                const EVENT: &'static str = "Unbonded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Withdrawn(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Withdrawn {
                const PALLET: &'static str = "OctopusLpos";
                const EVENT: &'static str = "Withdrawn";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Kicked(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for Kicked {
                const PALLET: &'static str = "OctopusLpos";
                const EVENT: &'static str = "Kicked";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct StakingElectionFailed {}
            impl ::subxt::Event for StakingElectionFailed {
                const PALLET: &'static str = "OctopusLpos";
                const EVENT: &'static str = "StakingElectionFailed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct HistoryDepth;
            impl ::subxt::StorageEntry for HistoryDepth {
                const PALLET: &'static str = "OctopusLpos";
                const STORAGE: &'static str = "HistoryDepth";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentEra;
            impl ::subxt::StorageEntry for CurrentEra {
                const PALLET: &'static str = "OctopusLpos";
                const STORAGE: &'static str = "CurrentEra";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ActiveEra;
            impl ::subxt::StorageEntry for ActiveEra {
                const PALLET: &'static str = "OctopusLpos";
                const STORAGE: &'static str = "ActiveEra";
                type Value = runtime_types::pallet_octopus_lpos::ActiveEraInfo;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ErasStartSessionIndex(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ErasStartSessionIndex {
                const PALLET: &'static str = "OctopusLpos";
                const STORAGE: &'static str = "ErasStartSessionIndex";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct ErasStakers(
                ::core::primitive::u32,
                ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for ErasStakers {
                const PALLET: &'static str = "OctopusLpos";
                const STORAGE: &'static str = "ErasStakers";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
                    ])
                }
            }
            pub struct ErasValidatorReward(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ErasValidatorReward {
                const PALLET: &'static str = "OctopusLpos";
                const STORAGE: &'static str = "ErasValidatorReward";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct ErasRewardPoints(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ErasRewardPoints {
                const PALLET: &'static str = "OctopusLpos";
                const STORAGE: &'static str = "ErasRewardPoints";
                type Value = runtime_types::pallet_octopus_lpos::EraRewardPoints<
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct ErasTotalStake(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ErasTotalStake {
                const PALLET: &'static str = "OctopusLpos";
                const STORAGE: &'static str = "ErasTotalStake";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct BondedEras;
            impl ::subxt::StorageEntry for BondedEras {
                const PALLET: &'static str = "OctopusLpos";
                const STORAGE: &'static str = "BondedEras";
                type Value = ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentPlannedSession;
            impl ::subxt::StorageEntry for CurrentPlannedSession {
                const PALLET: &'static str = "OctopusLpos";
                const STORAGE: &'static str = "CurrentPlannedSession";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EraPayout;
            impl ::subxt::StorageEntry for EraPayout {
                const PALLET: &'static str = "OctopusLpos";
                const STORAGE: &'static str = "EraPayout";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageVersion;
            impl ::subxt::StorageEntry for StorageVersion {
                const PALLET: &'static str = "OctopusLpos";
                const STORAGE: &'static str = "StorageVersion";
                type Value = runtime_types::pallet_octopus_lpos::Releases;
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
                pub async fn history_depth(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = HistoryDepth;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn current_era(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = CurrentEra;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn active_era(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::pallet_octopus_lpos::ActiveEraInfo>,
                    ::subxt::Error,
                > {
                    let entry = ActiveEra;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn eras_start_session_index(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = ErasStartSessionIndex(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn eras_start_session_index_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasStartSessionIndex>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_stakers(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::Error>
                {
                    let entry = ErasStakers(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn eras_stakers_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, ErasStakers>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_validator_reward(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u128>,
                    ::subxt::Error,
                > {
                    let entry = ErasValidatorReward(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn eras_validator_reward_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasValidatorReward>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_reward_points(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_octopus_lpos::EraRewardPoints<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    ::subxt::Error,
                > {
                    let entry = ErasRewardPoints(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn eras_reward_points_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, ErasRewardPoints>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_total_stake(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::Error>
                {
                    let entry = ErasTotalStake(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn eras_total_stake_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, ErasTotalStake>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn bonded_eras(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
                    ::subxt::Error,
                > {
                    let entry = BondedEras;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn current_planned_session(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = CurrentPlannedSession;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn era_payout(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::Error>
                {
                    let entry = EraPayout;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn storage_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_octopus_lpos::Releases,
                    ::subxt::Error,
                > {
                    let entry = StorageVersion;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod octopus_upward_messages {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
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
            }
        }
        pub type Event = runtime_types::pallet_octopus_upward_messages::pallet::Event;
        pub mod events {
            use super::runtime_types;
        }
        pub mod storage {
            use super::runtime_types;
            pub struct MessageQueue;
            impl ::subxt::StorageEntry for MessageQueue {
                const PALLET: &'static str = "OctopusUpwardMessages";
                const STORAGE: &'static str = "MessageQueue";
                type Value =
                    ::std::vec::Vec<runtime_types::pallet_octopus_upward_messages::Message>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Nonce;
            impl ::subxt::StorageEntry for Nonce {
                const PALLET: &'static str = "OctopusUpwardMessages";
                const STORAGE: &'static str = "Nonce";
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
                pub async fn message_queue(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<runtime_types::pallet_octopus_upward_messages::Message>,
                    ::subxt::Error,
                > {
                    let entry = MessageQueue;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn nonce(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::Error>
                {
                    let entry = Nonce;
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
                pub keys: runtime_types::node_template_runtime::opaque::SessionKeys,
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
                    keys: runtime_types::node_template_runtime::opaque::SessionKeys,
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
                    runtime_types::node_template_runtime::opaque::SessionKeys,
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
                type Value = runtime_types::node_template_runtime::opaque::SessionKeys;
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
                        runtime_types::node_template_runtime::opaque::SessionKeys,
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
                    ::core::option::Option<
                        runtime_types::node_template_runtime::opaque::SessionKeys,
                    >,
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
    pub mod im_online {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Heartbeat {
                pub heartbeat: runtime_types::pallet_im_online::Heartbeat<::core::primitive::u32>,
                pub signature: runtime_types::pallet_im_online::sr25519::app_sr25519::Signature,
            }
            impl ::subxt::Call for Heartbeat {
                const PALLET: &'static str = "ImOnline";
                const FUNCTION: &'static str = "heartbeat";
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
                pub fn heartbeat(
                    &self,
                    heartbeat: runtime_types::pallet_im_online::Heartbeat<::core::primitive::u32>,
                    signature: runtime_types::pallet_im_online::sr25519::app_sr25519::Signature,
                ) -> ::subxt::SubmittableExtrinsic<T, Heartbeat> {
                    let call = Heartbeat {
                        heartbeat,
                        signature,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_im_online::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct HeartbeatReceived(
                pub runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
            );
            impl ::subxt::Event for HeartbeatReceived {
                const PALLET: &'static str = "ImOnline";
                const EVENT: &'static str = "HeartbeatReceived";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AllGood {}
            impl ::subxt::Event for AllGood {
                const PALLET: &'static str = "ImOnline";
                const EVENT: &'static str = "AllGood";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SomeOffline(
                pub  ::std::vec::Vec<(
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                )>,
            );
            impl ::subxt::Event for SomeOffline {
                const PALLET: &'static str = "ImOnline";
                const EVENT: &'static str = "SomeOffline";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct HeartbeatAfter;
            impl ::subxt::StorageEntry for HeartbeatAfter {
                const PALLET: &'static str = "ImOnline";
                const STORAGE: &'static str = "HeartbeatAfter";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Keys;
            impl ::subxt::StorageEntry for Keys {
                const PALLET: &'static str = "ImOnline";
                const STORAGE: &'static str = "Keys";
                type Value =
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ReceivedHeartbeats(::core::primitive::u32, ::core::primitive::u32);
            impl ::subxt::StorageEntry for ReceivedHeartbeats {
                const PALLET: &'static str = "ImOnline";
                const STORAGE: &'static str = "ReceivedHeartbeats";
                type Value = runtime_types::frame_support::traits::misc::WrapperOpaque<
                    runtime_types::pallet_im_online::BoundedOpaqueNetworkState,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
                    ])
                }
            }
            pub struct AuthoredBlocks(
                ::core::primitive::u32,
                ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for AuthoredBlocks {
                const PALLET: &'static str = "ImOnline";
                const STORAGE: &'static str = "AuthoredBlocks";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
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
                pub async fn heartbeat_after(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = HeartbeatAfter;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn keys(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Keys;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn received_heartbeats(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::frame_support::traits::misc::WrapperOpaque<
                            runtime_types::pallet_im_online::BoundedOpaqueNetworkState,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = ReceivedHeartbeats(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn received_heartbeats_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ReceivedHeartbeats>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn authored_blocks(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = AuthoredBlocks(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn authored_blocks_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, AuthoredBlocks>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod historical {
        use super::runtime_types;
    }
    pub mod assets {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Create {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub admin:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub min_balance: ::core::primitive::u128,
            }
            impl ::subxt::Call for Create {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "create";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceCreate {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub owner:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub is_sufficient: ::core::primitive::bool,
                #[codec(compact)]
                pub min_balance: ::core::primitive::u128,
            }
            impl ::subxt::Call for ForceCreate {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "force_create";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Destroy {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub witness: runtime_types::pallet_assets::types::DestroyWitness,
            }
            impl ::subxt::Call for Destroy {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "destroy";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Mint {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub beneficiary:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for Mint {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "mint";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Burn {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub who:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for Burn {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "burn";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Transfer {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub target:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for Transfer {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct TransferKeepAlive {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub target:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for TransferKeepAlive {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "transfer_keep_alive";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceTransfer {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub source:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for ForceTransfer {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "force_transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Freeze {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub who:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for Freeze {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "freeze";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Thaw {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub who:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for Thaw {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "thaw";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct FreezeAsset {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
            }
            impl ::subxt::Call for FreezeAsset {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "freeze_asset";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ThawAsset {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
            }
            impl ::subxt::Call for ThawAsset {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "thaw_asset";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct TransferOwnership {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub owner:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for TransferOwnership {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "transfer_ownership";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetTeam {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub issuer:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub admin:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub freezer:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for SetTeam {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "set_team";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetMetadata {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub name: ::std::vec::Vec<::core::primitive::u8>,
                pub symbol: ::std::vec::Vec<::core::primitive::u8>,
                pub decimals: ::core::primitive::u8,
            }
            impl ::subxt::Call for SetMetadata {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "set_metadata";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ClearMetadata {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
            }
            impl ::subxt::Call for ClearMetadata {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "clear_metadata";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceSetMetadata {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub name: ::std::vec::Vec<::core::primitive::u8>,
                pub symbol: ::std::vec::Vec<::core::primitive::u8>,
                pub decimals: ::core::primitive::u8,
                pub is_frozen: ::core::primitive::bool,
            }
            impl ::subxt::Call for ForceSetMetadata {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "force_set_metadata";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceClearMetadata {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
            }
            impl ::subxt::Call for ForceClearMetadata {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "force_clear_metadata";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceAssetStatus {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub owner:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub issuer:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub admin:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub freezer:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub min_balance: ::core::primitive::u128,
                pub is_sufficient: ::core::primitive::bool,
                pub is_frozen: ::core::primitive::bool,
            }
            impl ::subxt::Call for ForceAssetStatus {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "force_asset_status";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ApproveTransfer {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub delegate:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for ApproveTransfer {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "approve_transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CancelApproval {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub delegate:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for CancelApproval {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "cancel_approval";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceCancelApproval {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub owner:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub delegate:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for ForceCancelApproval {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "force_cancel_approval";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct TransferApproved {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub owner:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub destination:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for TransferApproved {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "transfer_approved";
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
                pub fn create(
                    &self,
                    id: ::core::primitive::u32,
                    admin: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    min_balance: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, Create> {
                    let call = Create {
                        id,
                        admin,
                        min_balance,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_create(
                    &self,
                    id: ::core::primitive::u32,
                    owner: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    is_sufficient: ::core::primitive::bool,
                    min_balance: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceCreate> {
                    let call = ForceCreate {
                        id,
                        owner,
                        is_sufficient,
                        min_balance,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn destroy(
                    &self,
                    id: ::core::primitive::u32,
                    witness: runtime_types::pallet_assets::types::DestroyWitness,
                ) -> ::subxt::SubmittableExtrinsic<T, Destroy> {
                    let call = Destroy { id, witness };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn mint(
                    &self,
                    id: ::core::primitive::u32,
                    beneficiary: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, Mint> {
                    let call = Mint {
                        id,
                        beneficiary,
                        amount,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn burn(
                    &self,
                    id: ::core::primitive::u32,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, Burn> {
                    let call = Burn { id, who, amount };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer(
                    &self,
                    id: ::core::primitive::u32,
                    target: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, Transfer> {
                    let call = Transfer { id, target, amount };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_keep_alive(
                    &self,
                    id: ::core::primitive::u32,
                    target: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, TransferKeepAlive> {
                    let call = TransferKeepAlive { id, target, amount };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_transfer(
                    &self,
                    id: ::core::primitive::u32,
                    source: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceTransfer> {
                    let call = ForceTransfer {
                        id,
                        source,
                        dest,
                        amount,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn freeze(
                    &self,
                    id: ::core::primitive::u32,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, Freeze> {
                    let call = Freeze { id, who };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn thaw(
                    &self,
                    id: ::core::primitive::u32,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, Thaw> {
                    let call = Thaw { id, who };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn freeze_asset(
                    &self,
                    id: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, FreezeAsset> {
                    let call = FreezeAsset { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn thaw_asset(
                    &self,
                    id: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, ThawAsset> {
                    let call = ThawAsset { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_ownership(
                    &self,
                    id: ::core::primitive::u32,
                    owner: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, TransferOwnership> {
                    let call = TransferOwnership { id, owner };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_team(
                    &self,
                    id: ::core::primitive::u32,
                    issuer: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    admin: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    freezer: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, SetTeam> {
                    let call = SetTeam {
                        id,
                        issuer,
                        admin,
                        freezer,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_metadata(
                    &self,
                    id: ::core::primitive::u32,
                    name: ::std::vec::Vec<::core::primitive::u8>,
                    symbol: ::std::vec::Vec<::core::primitive::u8>,
                    decimals: ::core::primitive::u8,
                ) -> ::subxt::SubmittableExtrinsic<T, SetMetadata> {
                    let call = SetMetadata {
                        id,
                        name,
                        symbol,
                        decimals,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn clear_metadata(
                    &self,
                    id: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, ClearMetadata> {
                    let call = ClearMetadata { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_set_metadata(
                    &self,
                    id: ::core::primitive::u32,
                    name: ::std::vec::Vec<::core::primitive::u8>,
                    symbol: ::std::vec::Vec<::core::primitive::u8>,
                    decimals: ::core::primitive::u8,
                    is_frozen: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceSetMetadata> {
                    let call = ForceSetMetadata {
                        id,
                        name,
                        symbol,
                        decimals,
                        is_frozen,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_clear_metadata(
                    &self,
                    id: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceClearMetadata> {
                    let call = ForceClearMetadata { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_asset_status(
                    &self,
                    id: ::core::primitive::u32,
                    owner: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    issuer: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    admin: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    freezer: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    min_balance: ::core::primitive::u128,
                    is_sufficient: ::core::primitive::bool,
                    is_frozen: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceAssetStatus> {
                    let call = ForceAssetStatus {
                        id,
                        owner,
                        issuer,
                        admin,
                        freezer,
                        min_balance,
                        is_sufficient,
                        is_frozen,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn approve_transfer(
                    &self,
                    id: ::core::primitive::u32,
                    delegate: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, ApproveTransfer> {
                    let call = ApproveTransfer {
                        id,
                        delegate,
                        amount,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_approval(
                    &self,
                    id: ::core::primitive::u32,
                    delegate: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, CancelApproval> {
                    let call = CancelApproval { id, delegate };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_cancel_approval(
                    &self,
                    id: ::core::primitive::u32,
                    owner: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    delegate: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceCancelApproval> {
                    let call = ForceCancelApproval {
                        id,
                        owner,
                        delegate,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_approved(
                    &self,
                    id: ::core::primitive::u32,
                    owner: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    destination: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, TransferApproved> {
                    let call = TransferApproved {
                        id,
                        owner,
                        destination,
                        amount,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_assets::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Created(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for Created {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Created";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Issued(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Issued {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Issued";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Transferred(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Transferred {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Transferred";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Burned(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Burned {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Burned";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct TeamChanged(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for TeamChanged {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "TeamChanged";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct OwnerChanged(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for OwnerChanged {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "OwnerChanged";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Frozen(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for Frozen {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Frozen";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Thawed(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for Thawed {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Thawed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AssetFrozen(pub ::core::primitive::u32);
            impl ::subxt::Event for AssetFrozen {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "AssetFrozen";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AssetThawed(pub ::core::primitive::u32);
            impl ::subxt::Event for AssetThawed {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "AssetThawed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Destroyed(pub ::core::primitive::u32);
            impl ::subxt::Event for Destroyed {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Destroyed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceCreated(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for ForceCreated {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "ForceCreated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct MetadataSet(
                pub ::core::primitive::u32,
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::core::primitive::u8,
                pub ::core::primitive::bool,
            );
            impl ::subxt::Event for MetadataSet {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "MetadataSet";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct MetadataCleared(pub ::core::primitive::u32);
            impl ::subxt::Event for MetadataCleared {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "MetadataCleared";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ApprovedTransfer(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for ApprovedTransfer {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "ApprovedTransfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ApprovalCancelled(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for ApprovalCancelled {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "ApprovalCancelled";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct TransferredApproved(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for TransferredApproved {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "TransferredApproved";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AssetStatusChanged(pub ::core::primitive::u32);
            impl ::subxt::Event for AssetStatusChanged {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "AssetStatusChanged";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Asset(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Asset {
                const PALLET: &'static str = "Assets";
                const STORAGE: &'static str = "Asset";
                type Value = runtime_types::pallet_assets::types::AssetDetails<
                    ::core::primitive::u128,
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct Account(
                ::core::primitive::u32,
                ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for Account {
                const PALLET: &'static str = "Assets";
                const STORAGE: &'static str = "Account";
                type Value =
                    runtime_types::pallet_assets::types::AssetBalance<::core::primitive::u128, ()>;
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
            pub struct Approvals(
                ::core::primitive::u32,
                ::subxt::sp_core::crypto::AccountId32,
                ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for Approvals {
                const PALLET: &'static str = "Assets";
                const STORAGE: &'static str = "Approvals";
                type Value = runtime_types::pallet_assets::types::Approval<
                    ::core::primitive::u128,
                    ::core::primitive::u128,
                >;
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
                        ::subxt::StorageMapKey::new(
                            &self.2,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct Metadata(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Metadata {
                const PALLET: &'static str = "Assets";
                const STORAGE: &'static str = "Metadata";
                type Value = runtime_types::pallet_assets::types::AssetMetadata<
                    ::core::primitive::u128,
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
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
                pub async fn asset(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_assets::types::AssetDetails<
                            ::core::primitive::u128,
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Asset(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn asset_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Asset>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn account(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_assets::types::AssetBalance<::core::primitive::u128, ()>,
                    ::subxt::Error,
                > {
                    let entry = Account(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Account>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn approvals(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    _2: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_assets::types::Approval<
                            ::core::primitive::u128,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Approvals(_0, _1, _2);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn approvals_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Approvals>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn metadata(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_assets::types::AssetMetadata<
                        ::core::primitive::u128,
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Metadata(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn metadata_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Metadata>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod mmr {
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct RootHash;
            impl ::subxt::StorageEntry for RootHash {
                const PALLET: &'static str = "Mmr";
                const STORAGE: &'static str = "RootHash";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NumberOfLeaves;
            impl ::subxt::StorageEntry for NumberOfLeaves {
                const PALLET: &'static str = "Mmr";
                const STORAGE: &'static str = "NumberOfLeaves";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Nodes(pub ::core::primitive::u64);
            impl ::subxt::StorageEntry for Nodes {
                const PALLET: &'static str = "Mmr";
                const STORAGE: &'static str = "Nodes";
                type Value = ::subxt::sp_core::H256;
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
                pub async fn root_hash(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::Error>
                {
                    let entry = RootHash;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn number_of_leaves(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::Error>
                {
                    let entry = NumberOfLeaves;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn nodes(
                    &self,
                    _0: ::core::primitive::u64,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::H256>,
                    ::subxt::Error,
                > {
                    let entry = Nodes(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn nodes_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Nodes>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod beefy {
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct Authorities;
            impl ::subxt::StorageEntry for Authorities {
                const PALLET: &'static str = "Beefy";
                const STORAGE: &'static str = "Authorities";
                type Value = ::std::vec::Vec<runtime_types::beefy_primitives::crypto::Public>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ValidatorSetId;
            impl ::subxt::StorageEntry for ValidatorSetId {
                const PALLET: &'static str = "Beefy";
                const STORAGE: &'static str = "ValidatorSetId";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextAuthorities;
            impl ::subxt::StorageEntry for NextAuthorities {
                const PALLET: &'static str = "Beefy";
                const STORAGE: &'static str = "NextAuthorities";
                type Value = ::std::vec::Vec<runtime_types::beefy_primitives::crypto::Public>;
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
                    ::std::vec::Vec<runtime_types::beefy_primitives::crypto::Public>,
                    ::subxt::Error,
                > {
                    let entry = Authorities;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn validator_set_id(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::Error>
                {
                    let entry = ValidatorSetId;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_authorities(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<runtime_types::beefy_primitives::crypto::Public>,
                    ::subxt::Error,
                > {
                    let entry = NextAuthorities;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod mmr_leaf {
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct BeefyNextAuthorities;
            impl ::subxt::StorageEntry for BeefyNextAuthorities {
                const PALLET: &'static str = "MmrLeaf";
                const STORAGE: &'static str = "BeefyNextAuthorities";
                type Value = runtime_types::beefy_primitives::mmr::BeefyNextAuthoritySet<
                    ::subxt::sp_core::H256,
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
                pub async fn beefy_next_authorities(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::beefy_primitives::mmr::BeefyNextAuthoritySet<
                        ::subxt::sp_core::H256,
                    >,
                    ::subxt::Error,
                > {
                    let entry = BeefyNextAuthorities;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod sudo {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Sudo {
                pub call: runtime_types::node_template_runtime::Call,
            }
            impl ::subxt::Call for Sudo {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SudoUncheckedWeight {
                pub call: runtime_types::node_template_runtime::Call,
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
                pub call: runtime_types::node_template_runtime::Call,
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
                    call: runtime_types::node_template_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<T, Sudo> {
                    let call = Sudo { call };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn sudo_unchecked_weight(
                    &self,
                    call: runtime_types::node_template_runtime::Call,
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
                    call: runtime_types::node_template_runtime::Call,
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
                pub fn deliver(
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
                pub runtime_types::pallet_ibc::event::primitive::ClientType,
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
                pub runtime_types::pallet_ibc::event::primitive::ClientType,
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
                pub runtime_types::pallet_ibc::event::primitive::ClientType,
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
                pub runtime_types::pallet_ibc::event::primitive::ClientType,
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
            pub struct ClientStatesKeys;
            impl ::subxt::StorageEntry for ClientStatesKeys {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "ClientStatesKeys";
                type Value = ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
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
            pub struct ConnectionsKeys;
            impl ::subxt::StorageEntry for ConnectionsKeys {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "ConnectionsKeys";
                type Value = ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
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
            pub struct ChannelsKeys;
            impl ::subxt::StorageEntry for ChannelsKeys {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "ChannelsKeys";
                type Value = ::std::vec::Vec<(
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::std::vec::Vec<::core::primitive::u8>,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
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
            pub struct NextSequenceRecv(
                ::std::vec::Vec<::core::primitive::u8>,
                ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::StorageEntry for NextSequenceRecv {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "NextSequenceRecv";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
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
            pub struct NextSequenceAck(
                ::std::vec::Vec<::core::primitive::u8>,
                ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::StorageEntry for NextSequenceAck {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "NextSequenceAck";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
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
            pub struct AcknowledgementsKeys;
            impl ::subxt::StorageEntry for AcknowledgementsKeys {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "AcknowledgementsKeys";
                type Value = ::std::vec::Vec<(
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::std::vec::Vec<::core::primitive::u8>,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
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
            pub struct PacketCommitmentKeys;
            impl ::subxt::StorageEntry for PacketCommitmentKeys {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "PacketCommitmentKeys";
                type Value = ::std::vec::Vec<(
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::std::vec::Vec<::core::primitive::u8>,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
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
            pub struct WriteAckPacketEvent(
                ::std::vec::Vec<::core::primitive::u8>,
                ::std::vec::Vec<::core::primitive::u8>,
                ::core::primitive::u64,
            );
            impl ::subxt::StorageEntry for WriteAckPacketEvent {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "WriteAckPacketEvent";
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
                pub async fn client_states_keys(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::Error,
                > {
                    let entry = ClientStatesKeys;
                    self.client.storage().fetch_or_default(&entry, hash).await
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
                pub async fn connections_keys(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::Error,
                > {
                    let entry = ConnectionsKeys;
                    self.client.storage().fetch_or_default(&entry, hash).await
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
                pub async fn channels_keys(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = ChannelsKeys;
                    self.client.storage().fetch_or_default(&entry, hash).await
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
                pub async fn acknowledgements_keys(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = AcknowledgementsKeys;
                    self.client.storage().fetch_or_default(&entry, hash).await
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
                pub async fn packet_commitment_keys(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = PacketCommitmentKeys;
                    self.client.storage().fetch_or_default(&entry, hash).await
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
                pub async fn write_ack_packet_event(
                    &self,
                    _0: ::std::vec::Vec<::core::primitive::u8>,
                    _1: ::std::vec::Vec<::core::primitive::u8>,
                    _2: ::core::primitive::u64,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::std::vec::Vec<::core::primitive::u8>, ::subxt::Error>
                {
                    let entry = WriteAckPacketEvent(_0, _1, _2);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn write_ack_packet_event_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, WriteAckPacketEvent>,
                    ::subxt::Error,
                > {
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
    pub mod template_module {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct DoSomething {
                pub something: ::core::primitive::u32,
            }
            impl ::subxt::Call for DoSomething {
                const PALLET: &'static str = "TemplateModule";
                const FUNCTION: &'static str = "do_something";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CauseError {}
            impl ::subxt::Call for CauseError {
                const PALLET: &'static str = "TemplateModule";
                const FUNCTION: &'static str = "cause_error";
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
                pub fn do_something(
                    &self,
                    something: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, DoSomething> {
                    let call = DoSomething { something };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cause_error(&self) -> ::subxt::SubmittableExtrinsic<T, CauseError> {
                    let call = CauseError {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_template::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SomethingStored(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for SomethingStored {
                const PALLET: &'static str = "TemplateModule";
                const EVENT: &'static str = "SomethingStored";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Something;
            impl ::subxt::StorageEntry for Something {
                const PALLET: &'static str = "TemplateModule";
                const STORAGE: &'static str = "Something";
                type Value = ::core::primitive::u32;
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
                pub async fn something(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = Something;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod runtime_types {
        use super::runtime_types;
        pub mod beefy_primitives {
            use super::runtime_types;
            pub mod crypto {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Public(pub runtime_types::sp_core::ecdsa::Public);
            }
            pub mod mmr {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct BeefyNextAuthoritySet<_0> {
                    pub id: ::core::primitive::u64,
                    pub len: ::core::primitive::u32,
                    pub root: _0,
                }
            }
        }
        pub mod finality_grandpa {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Equivocation<_0, _1, _2> {
                pub round_number: ::core::primitive::u64,
                pub identity: _0,
                pub first: (_1, _2),
                pub second: (_1, _2),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Precommit<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Prevote<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
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
                pub mod misc {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct WrapperOpaque<_0>(#[codec(compact)] ::core::primitive::u32, pub _0);
                }
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
        pub mod node_template_runtime {
            use super::runtime_types;
            pub mod opaque {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct SessionKeys {
                    pub babe: runtime_types::sp_consensus_babe::app::Public,
                    pub grandpa: runtime_types::sp_finality_grandpa::app::Public,
                    pub im_online: runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
                    pub beefy: runtime_types::beefy_primitives::crypto::Public,
                    pub octopus: runtime_types::pallet_octopus_appchain::crypto::Public,
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Call {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Call),
                #[codec(index = 2)]
                Timestamp(runtime_types::pallet_timestamp::pallet::Call),
                #[codec(index = 3)]
                Authorship(runtime_types::pallet_authorship::pallet::Call),
                #[codec(index = 4)]
                Babe(runtime_types::pallet_babe::pallet::Call),
                #[codec(index = 5)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Call),
                #[codec(index = 6)]
                Balances(runtime_types::pallet_balances::pallet::Call),
                #[codec(index = 8)]
                OctopusAppchain(runtime_types::pallet_octopus_appchain::pallet::Call),
                #[codec(index = 9)]
                OctopusLpos(runtime_types::pallet_octopus_lpos::pallet::Call),
                #[codec(index = 10)]
                OctopusUpwardMessages(runtime_types::pallet_octopus_upward_messages::pallet::Call),
                #[codec(index = 11)]
                Session(runtime_types::pallet_session::pallet::Call),
                #[codec(index = 12)]
                ImOnline(runtime_types::pallet_im_online::pallet::Call),
                #[codec(index = 14)]
                Assets(runtime_types::pallet_assets::pallet::Call),
                #[codec(index = 18)]
                Sudo(runtime_types::pallet_sudo::pallet::Call),
                #[codec(index = 19)]
                Ibc(runtime_types::pallet_ibc::pallet::Call),
                #[codec(index = 20)]
                TemplateModule(runtime_types::pallet_template::pallet::Call),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Event {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Event),
                #[codec(index = 5)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Event),
                #[codec(index = 6)]
                Balances(runtime_types::pallet_balances::pallet::Event),
                #[codec(index = 8)]
                OctopusAppchain(runtime_types::pallet_octopus_appchain::pallet::Event),
                #[codec(index = 9)]
                OctopusLpos(runtime_types::pallet_octopus_lpos::pallet::Event),
                #[codec(index = 10)]
                OctopusUpwardMessages(runtime_types::pallet_octopus_upward_messages::pallet::Event),
                #[codec(index = 11)]
                Session(runtime_types::pallet_session::pallet::Event),
                #[codec(index = 12)]
                ImOnline(runtime_types::pallet_im_online::pallet::Event),
                #[codec(index = 14)]
                Assets(runtime_types::pallet_assets::pallet::Event),
                #[codec(index = 18)]
                Sudo(runtime_types::pallet_sudo::pallet::Event),
                #[codec(index = 19)]
                Ibc(runtime_types::pallet_ibc::pallet::Event),
                #[codec(index = 20)]
                TemplateModule(runtime_types::pallet_template::pallet::Event),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Runtime {}
        }
        pub mod pallet_assets {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    create {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        admin: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        min_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    force_create {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        owner: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        is_sufficient: ::core::primitive::bool,
                        #[codec(compact)]
                        min_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    destroy {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        witness: runtime_types::pallet_assets::types::DestroyWitness,
                    },
                    #[codec(index = 3)]
                    mint {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        beneficiary: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    burn {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    transfer {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        target: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    transfer_keep_alive {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        target: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 7)]
                    force_transfer {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        source: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    freeze {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 9)]
                    thaw {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 10)]
                    freeze_asset {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 11)]
                    thaw_asset {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 12)]
                    transfer_ownership {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        owner: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 13)]
                    set_team {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        issuer: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        admin: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        freezer: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 14)]
                    set_metadata {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        name: ::std::vec::Vec<::core::primitive::u8>,
                        symbol: ::std::vec::Vec<::core::primitive::u8>,
                        decimals: ::core::primitive::u8,
                    },
                    #[codec(index = 15)]
                    clear_metadata {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 16)]
                    force_set_metadata {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        name: ::std::vec::Vec<::core::primitive::u8>,
                        symbol: ::std::vec::Vec<::core::primitive::u8>,
                        decimals: ::core::primitive::u8,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 17)]
                    force_clear_metadata {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 18)]
                    force_asset_status {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        owner: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        issuer: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        admin: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        freezer: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        min_balance: ::core::primitive::u128,
                        is_sufficient: ::core::primitive::bool,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 19)]
                    approve_transfer {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        delegate: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 20)]
                    cancel_approval {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        delegate: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 21)]
                    force_cancel_approval {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        owner: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        delegate: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 22)]
                    transfer_approved {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        owner: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        destination: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    BalanceLow,
                    #[codec(index = 1)]
                    BalanceZero,
                    #[codec(index = 2)]
                    NoPermission,
                    #[codec(index = 3)]
                    Unknown,
                    #[codec(index = 4)]
                    Frozen,
                    #[codec(index = 5)]
                    InUse,
                    #[codec(index = 6)]
                    BadWitness,
                    #[codec(index = 7)]
                    MinBalanceZero,
                    #[codec(index = 8)]
                    NoProvider,
                    #[codec(index = 9)]
                    BadMetadata,
                    #[codec(index = 10)]
                    Unapproved,
                    #[codec(index = 11)]
                    WouldDie,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    Created(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 1)]
                    Issued(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 2)]
                    Transferred(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 3)]
                    Burned(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 4)]
                    TeamChanged(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 5)]
                    OwnerChanged(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 6)]
                    Frozen(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 7)]
                    Thawed(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 8)]
                    AssetFrozen(::core::primitive::u32),
                    #[codec(index = 9)]
                    AssetThawed(::core::primitive::u32),
                    #[codec(index = 10)]
                    Destroyed(::core::primitive::u32),
                    #[codec(index = 11)]
                    ForceCreated(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 12)]
                    MetadataSet(
                        ::core::primitive::u32,
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::core::primitive::u8,
                        ::core::primitive::bool,
                    ),
                    #[codec(index = 13)]
                    MetadataCleared(::core::primitive::u32),
                    #[codec(index = 14)]
                    ApprovedTransfer(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 15)]
                    ApprovalCancelled(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 16)]
                    TransferredApproved(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 17)]
                    AssetStatusChanged(::core::primitive::u32),
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Approval<_0, _1> {
                    pub amount: _0,
                    pub deposit: _0,
                    #[codec(skip)]
                    pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct AssetBalance<_0, _1> {
                    pub balance: _0,
                    pub is_frozen: ::core::primitive::bool,
                    pub sufficient: ::core::primitive::bool,
                    pub extra: _1,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct AssetDetails<_0, _1, _2> {
                    pub owner: _1,
                    pub issuer: _1,
                    pub admin: _1,
                    pub freezer: _1,
                    pub supply: _0,
                    pub deposit: _0,
                    pub min_balance: _0,
                    pub is_sufficient: ::core::primitive::bool,
                    pub accounts: ::core::primitive::u32,
                    pub sufficients: ::core::primitive::u32,
                    pub approvals: ::core::primitive::u32,
                    pub is_frozen: ::core::primitive::bool,
                    #[codec(skip)]
                    pub __subxt_unused_type_params: ::core::marker::PhantomData<_2>,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct AssetMetadata<_0, _1> {
                    pub deposit: _0,
                    pub name: _1,
                    pub symbol: _1,
                    pub decimals: ::core::primitive::u8,
                    pub is_frozen: ::core::primitive::bool,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct DestroyWitness {
                    #[codec(compact)]
                    pub accounts: ::core::primitive::u32,
                    #[codec(compact)]
                    pub sufficients: ::core::primitive::u32,
                    #[codec(compact)]
                    pub approvals: ::core::primitive::u32,
                }
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
        pub mod pallet_babe {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    report_equivocation {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_consensus_slots::EquivocationProof<
                                runtime_types::sp_runtime::generic::header::Header<
                                    ::core::primitive::u32,
                                    runtime_types::sp_runtime::traits::BlakeTwo256,
                                >,
                                runtime_types::sp_consensus_babe::app::Public,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_session::MembershipProof,
                    },
                    #[codec(index = 1)]
                    report_equivocation_unsigned {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_consensus_slots::EquivocationProof<
                                runtime_types::sp_runtime::generic::header::Header<
                                    ::core::primitive::u32,
                                    runtime_types::sp_runtime::traits::BlakeTwo256,
                                >,
                                runtime_types::sp_consensus_babe::app::Public,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_session::MembershipProof,
                    },
                    #[codec(index = 2)]
                    plan_config_change {
                        config: runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidEquivocationProof,
                    #[codec(index = 1)]
                    InvalidKeyOwnershipProof,
                    #[codec(index = 2)]
                    DuplicateOffenceReport,
                }
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
        pub mod pallet_grandpa {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    report_equivocation {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_finality_grandpa::EquivocationProof<
                                ::subxt::sp_core::H256,
                                ::core::primitive::u32,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_session::MembershipProof,
                    },
                    #[codec(index = 1)]
                    report_equivocation_unsigned {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_finality_grandpa::EquivocationProof<
                                ::subxt::sp_core::H256,
                                ::core::primitive::u32,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_session::MembershipProof,
                    },
                    #[codec(index = 2)]
                    note_stalled {
                        delay: ::core::primitive::u32,
                        best_finalized_block_number: ::core::primitive::u32,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    PauseFailed,
                    #[codec(index = 1)]
                    ResumeFailed,
                    #[codec(index = 2)]
                    ChangePending,
                    #[codec(index = 3)]
                    TooSoon,
                    #[codec(index = 4)]
                    InvalidKeyOwnershipProof,
                    #[codec(index = 5)]
                    InvalidEquivocationProof,
                    #[codec(index = 6)]
                    DuplicateOffenceReport,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    NewAuthorities(
                        ::std::vec::Vec<(
                            runtime_types::sp_finality_grandpa::app::Public,
                            ::core::primitive::u64,
                        )>,
                    ),
                    #[codec(index = 1)]
                    Paused,
                    #[codec(index = 2)]
                    Resumed,
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct StoredPendingChange<_0> {
                pub scheduled_at: _0,
                pub delay: _0,
                pub next_authorities:
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_finality_grandpa::app::Public,
                        ::core::primitive::u64,
                    )>,
                pub forced: ::core::option::Option<_0>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
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
                    pub enum ClientType {
                        #[codec(index = 0)]
                        Tendermint,
                        #[codec(index = 1)]
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
                    pub struct Timestamp {
                        pub time: ::std::vec::Vec<::core::primitive::u8>,
                    }
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
                        runtime_types::pallet_ibc::event::primitive::ClientType,
                        runtime_types::pallet_ibc::event::primitive::Height,
                    ),
                    #[codec(index = 2)]
                    UpdateClient(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::ClientId,
                        runtime_types::pallet_ibc::event::primitive::ClientType,
                        runtime_types::pallet_ibc::event::primitive::Height,
                    ),
                    #[codec(index = 3)]
                    UpgradeClient(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::ClientId,
                        runtime_types::pallet_ibc::event::primitive::ClientType,
                        runtime_types::pallet_ibc::event::primitive::Height,
                    ),
                    #[codec(index = 4)]
                    ClientMisbehaviour(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::ClientId,
                        runtime_types::pallet_ibc::event::primitive::ClientType,
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
        pub mod pallet_im_online {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    heartbeat {
                        heartbeat:
                            runtime_types::pallet_im_online::Heartbeat<::core::primitive::u32>,
                        signature: runtime_types::pallet_im_online::sr25519::app_sr25519::Signature,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidKey,
                    #[codec(index = 1)]
                    DuplicatedHeartbeat,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    HeartbeatReceived(
                        runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
                    ),
                    #[codec(index = 1)]
                    AllGood,
                    #[codec(index = 2)]
                    SomeOffline(
                        ::std::vec::Vec<(
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        )>,
                    ),
                }
            }
            pub mod sr25519 {
                use super::runtime_types;
                pub mod app_sr25519 {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct Public(pub runtime_types::sp_core::sr25519::Public);
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct Signature(pub runtime_types::sp_core::sr25519::Signature);
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BoundedOpaqueNetworkState {
                pub peer_id:
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        ::core::primitive::u8,
                    >,
                pub external_addresses:
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                            ::core::primitive::u8,
                        >,
                    >,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Heartbeat<_0> {
                pub block_number: _0,
                pub network_state: runtime_types::sp_core::offchain::OpaqueNetworkState,
                pub session_index: _0,
                pub authority_index: _0,
                pub validators_len: _0,
            }
        }
        pub mod pallet_octopus_appchain {
            use super::runtime_types;
            pub mod crypto {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Public(pub runtime_types::sp_core::sr25519::Public);
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    submit_observations {
                        payload: runtime_types::pallet_octopus_appchain::ObservationsPayload<
                            runtime_types::sp_runtime::MultiSigner,
                            ::core::primitive::u32,
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                        signature: runtime_types::sp_runtime::MultiSignature,
                    },
                    #[codec(index = 1)]
                    force_set_planned_validators {
                        validators: ::std::vec::Vec<(
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        )>,
                    },
                    #[codec(index = 2)]
                    lock {
                        receiver_id: ::std::vec::Vec<::core::primitive::u8>,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    mint_asset {
                        asset_id: ::core::primitive::u32,
                        sender_id: ::std::vec::Vec<::core::primitive::u8>,
                        receiver: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    burn_asset {
                        asset_id: ::core::primitive::u32,
                        receiver_id: ::std::vec::Vec<::core::primitive::u8>,
                        amount: ::core::primitive::u128,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    WrongSetId,
                    #[codec(index = 1)]
                    NotValidator,
                    #[codec(index = 2)]
                    AmountOverflow,
                    #[codec(index = 3)]
                    NextFactSequenceOverflow,
                    #[codec(index = 4)]
                    WrongAssetId,
                    #[codec(index = 5)]
                    InvalidActiveTotalStake,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    Locked(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 1)]
                    Unlocked(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 2)]
                    AssetMinted(
                        ::core::primitive::u32,
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 3)]
                    AssetBurned(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::core::primitive::u128,
                    ),
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BurnEvent<_0> {
                pub index: ::core::primitive::u32,
                pub sender_id: ::std::vec::Vec<::core::primitive::u8>,
                pub receiver: _0,
                pub amount: ::core::primitive::u128,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct LockAssetEvent<_0> {
                pub index: ::core::primitive::u32,
                pub token_id: ::std::vec::Vec<::core::primitive::u8>,
                pub sender_id: ::std::vec::Vec<::core::primitive::u8>,
                pub receiver: _0,
                pub amount: ::core::primitive::u128,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Observation<_0> {
                #[codec(index = 0)]
                UpdateValidatorSet(runtime_types::pallet_octopus_appchain::ValidatorSet<_0>),
                #[codec(index = 1)]
                LockAsset(runtime_types::pallet_octopus_appchain::LockAssetEvent<_0>),
                #[codec(index = 2)]
                Burn(runtime_types::pallet_octopus_appchain::BurnEvent<_0>),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum ObservationType {
                #[codec(index = 0)]
                UpdateValidatorSet,
                #[codec(index = 1)]
                Burn,
                #[codec(index = 2)]
                LockAsset,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ObservationsPayload<_0, _1, _2> {
                pub public: _0,
                pub block_number: _1,
                pub next_fact_sequence: _1,
                pub observations:
                    ::std::vec::Vec<runtime_types::pallet_octopus_appchain::Observation<_2>>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Validator<_0> {
                pub validator_id_in_appchain: _0,
                pub total_stake: ::core::primitive::u128,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ValidatorSet<_0> {
                pub era: ::core::primitive::u32,
                pub validators:
                    ::std::vec::Vec<runtime_types::pallet_octopus_appchain::Validator<_0>>,
            }
        }
        pub mod pallet_octopus_lpos {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    set_history_depth {
                        #[codec(compact)]
                        new_history_depth: ::core::primitive::u32,
                        #[codec(compact)]
                        era_items_deleted: ::core::primitive::u32,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    NotController,
                    #[codec(index = 1)]
                    NotStash,
                    #[codec(index = 2)]
                    AlreadyBonded,
                    #[codec(index = 3)]
                    AlreadyPaired,
                    #[codec(index = 4)]
                    EmptyTargets,
                    #[codec(index = 5)]
                    DuplicateIndex,
                    #[codec(index = 6)]
                    InvalidSlashIndex,
                    #[codec(index = 7)]
                    InsufficientBond,
                    #[codec(index = 8)]
                    NoMoreChunks,
                    #[codec(index = 9)]
                    NoUnlockChunk,
                    #[codec(index = 10)]
                    FundedTarget,
                    #[codec(index = 11)]
                    InvalidEraToReward,
                    #[codec(index = 12)]
                    InvalidNumberOfNominations,
                    #[codec(index = 13)]
                    NotSortedAndUnique,
                    #[codec(index = 14)]
                    AlreadyClaimed,
                    #[codec(index = 15)]
                    IncorrectHistoryDepth,
                    #[codec(index = 16)]
                    IncorrectSlashingSpans,
                    #[codec(index = 17)]
                    BadState,
                    #[codec(index = 18)]
                    TooManyTargets,
                    #[codec(index = 19)]
                    BadTarget,
                    #[codec(index = 20)]
                    CannotChillOther,
                    #[codec(index = 21)]
                    TooManyNominators,
                    #[codec(index = 22)]
                    TooManyValidators,
                    #[codec(index = 23)]
                    NoClaimedRewards,
                    #[codec(index = 24)]
                    AmountOverflow,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    EraPayout(::core::primitive::u32, ::core::primitive::u128),
                    #[codec(index = 1)]
                    Reward(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 2)]
                    Slash(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 3)]
                    OldSlashingReportDiscarded(::core::primitive::u32),
                    #[codec(index = 4)]
                    StakingElection,
                    #[codec(index = 5)]
                    Bonded(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 6)]
                    Unbonded(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 7)]
                    Withdrawn(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 8)]
                    Kicked(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 9)]
                    StakingElectionFailed,
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ActiveEraInfo {
                pub index: ::core::primitive::u32,
                pub start: ::core::option::Option<::core::primitive::u64>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct EraRewardPoints<_0> {
                pub total: ::core::primitive::u32,
                pub individual: ::std::collections::BTreeMap<_0, ::core::primitive::u32>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Releases {
                #[codec(index = 0)]
                V1_0_0,
            }
        }
        pub mod pallet_octopus_support {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum PayloadType {
                    #[codec(index = 0)]
                    Lock,
                    #[codec(index = 1)]
                    BurnAsset,
                    #[codec(index = 2)]
                    PlanNewEra,
                    #[codec(index = 3)]
                    EraPayout,
                }
            }
        }
        pub mod pallet_octopus_upward_messages {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {}
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    NonceOverflow,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {}
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Message {
                pub nonce: ::core::primitive::u64,
                pub payload_type: runtime_types::pallet_octopus_support::types::PayloadType,
                pub payload: ::std::vec::Vec<::core::primitive::u8>,
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
                        keys: runtime_types::node_template_runtime::opaque::SessionKeys,
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
                        call: ::std::boxed::Box<runtime_types::node_template_runtime::Call>,
                    },
                    #[codec(index = 1)]
                    sudo_unchecked_weight {
                        call: ::std::boxed::Box<runtime_types::node_template_runtime::Call>,
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
                        call: ::std::boxed::Box<runtime_types::node_template_runtime::Call>,
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
        pub mod pallet_template {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    do_something { something: ::core::primitive::u32 },
                    #[codec(index = 1)]
                    cause_error,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    NoneValue,
                    #[codec(index = 1)]
                    StorageOverflow,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    SomethingStored(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
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
        pub mod primitive_types {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct H256(pub [::core::primitive::u8; 32usize]);
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
        pub mod sp_consensus_babe {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Public(pub runtime_types::sp_core::sr25519::Public);
            }
            pub mod digests {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum NextConfigDescriptor {
                    #[codec(index = 1)]
                    V1 {
                        c: (::core::primitive::u64, ::core::primitive::u64),
                        allowed_slots: runtime_types::sp_consensus_babe::AllowedSlots,
                    },
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum AllowedSlots {
                #[codec(index = 0)]
                PrimarySlots,
                #[codec(index = 1)]
                PrimaryAndSecondaryPlainSlots,
                #[codec(index = 2)]
                PrimaryAndSecondaryVRFSlots,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BabeEpochConfiguration {
                pub c: (::core::primitive::u64, ::core::primitive::u64),
                pub allowed_slots: runtime_types::sp_consensus_babe::AllowedSlots,
            }
        }
        pub mod sp_consensus_slots {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct EquivocationProof<_0, _1> {
                pub offender: _1,
                pub slot: runtime_types::sp_consensus_slots::Slot,
                pub first_header: _0,
                pub second_header: _0,
            }
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
                pub struct Public(pub [::core::primitive::u8; 33usize]);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Signature(pub [::core::primitive::u8; 65usize]);
            }
            pub mod ed25519 {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            pub mod offchain {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct OpaqueMultiaddr(pub ::std::vec::Vec<::core::primitive::u8>);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct OpaqueNetworkState {
                    pub peer_id: runtime_types::sp_core::OpaquePeerId,
                    pub external_addresses:
                        ::std::vec::Vec<runtime_types::sp_core::offchain::OpaqueMultiaddr>,
                }
            }
            pub mod sr25519 {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct OpaquePeerId(pub ::std::vec::Vec<::core::primitive::u8>);
        }
        pub mod sp_finality_grandpa {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Public(pub runtime_types::sp_core::ed25519::Public);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Signature(pub runtime_types::sp_core::ed25519::Signature);
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Equivocation<_0, _1> {
                #[codec(index = 0)]
                Prevote(
                    runtime_types::finality_grandpa::Equivocation<
                        runtime_types::sp_finality_grandpa::app::Public,
                        runtime_types::finality_grandpa::Prevote<_0, _1>,
                        runtime_types::sp_finality_grandpa::app::Signature,
                    >,
                ),
                #[codec(index = 1)]
                Precommit(
                    runtime_types::finality_grandpa::Equivocation<
                        runtime_types::sp_finality_grandpa::app::Public,
                        runtime_types::finality_grandpa::Precommit<_0, _1>,
                        runtime_types::sp_finality_grandpa::app::Signature,
                    >,
                ),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct EquivocationProof<_0, _1> {
                pub set_id: ::core::primitive::u64,
                pub equivocation: runtime_types::sp_finality_grandpa::Equivocation<_0, _1>,
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
                        #[codec(skip)] pub ::core::marker::PhantomData<(_0, _2, _1, _3)>,
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
            pub enum MultiSigner {
                #[codec(index = 0)]
                Ed25519(runtime_types::sp_core::ed25519::Public),
                #[codec(index = 1)]
                Sr25519(runtime_types::sp_core::sr25519::Public),
                #[codec(index = 2)]
                Ecdsa(runtime_types::sp_core::ecdsa::Public),
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
        pub mod sp_session {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct MembershipProof {
                pub session: ::core::primitive::u32,
                pub trie_nodes: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                pub validator_count: ::core::primitive::u32,
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
        pub fn randomness_collective_flip(
            &self,
        ) -> randomness_collective_flip::storage::StorageApi<'a, T> {
            randomness_collective_flip::storage::StorageApi::new(self.client)
        }
        pub fn timestamp(&self) -> timestamp::storage::StorageApi<'a, T> {
            timestamp::storage::StorageApi::new(self.client)
        }
        pub fn authorship(&self) -> authorship::storage::StorageApi<'a, T> {
            authorship::storage::StorageApi::new(self.client)
        }
        pub fn babe(&self) -> babe::storage::StorageApi<'a, T> {
            babe::storage::StorageApi::new(self.client)
        }
        pub fn grandpa(&self) -> grandpa::storage::StorageApi<'a, T> {
            grandpa::storage::StorageApi::new(self.client)
        }
        pub fn balances(&self) -> balances::storage::StorageApi<'a, T> {
            balances::storage::StorageApi::new(self.client)
        }
        pub fn transaction_payment(&self) -> transaction_payment::storage::StorageApi<'a, T> {
            transaction_payment::storage::StorageApi::new(self.client)
        }
        pub fn octopus_appchain(&self) -> octopus_appchain::storage::StorageApi<'a, T> {
            octopus_appchain::storage::StorageApi::new(self.client)
        }
        pub fn octopus_lpos(&self) -> octopus_lpos::storage::StorageApi<'a, T> {
            octopus_lpos::storage::StorageApi::new(self.client)
        }
        pub fn octopus_upward_messages(
            &self,
        ) -> octopus_upward_messages::storage::StorageApi<'a, T> {
            octopus_upward_messages::storage::StorageApi::new(self.client)
        }
        pub fn session(&self) -> session::storage::StorageApi<'a, T> {
            session::storage::StorageApi::new(self.client)
        }
        pub fn im_online(&self) -> im_online::storage::StorageApi<'a, T> {
            im_online::storage::StorageApi::new(self.client)
        }
        pub fn assets(&self) -> assets::storage::StorageApi<'a, T> {
            assets::storage::StorageApi::new(self.client)
        }
        pub fn mmr(&self) -> mmr::storage::StorageApi<'a, T> {
            mmr::storage::StorageApi::new(self.client)
        }
        pub fn beefy(&self) -> beefy::storage::StorageApi<'a, T> {
            beefy::storage::StorageApi::new(self.client)
        }
        pub fn mmr_leaf(&self) -> mmr_leaf::storage::StorageApi<'a, T> {
            mmr_leaf::storage::StorageApi::new(self.client)
        }
        pub fn sudo(&self) -> sudo::storage::StorageApi<'a, T> {
            sudo::storage::StorageApi::new(self.client)
        }
        pub fn ibc(&self) -> ibc::storage::StorageApi<'a, T> {
            ibc::storage::StorageApi::new(self.client)
        }
        pub fn template_module(&self) -> template_module::storage::StorageApi<'a, T> {
            template_module::storage::StorageApi::new(self.client)
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
        pub fn timestamp(&self) -> timestamp::calls::TransactionApi<'a, T> {
            timestamp::calls::TransactionApi::new(self.client)
        }
        pub fn authorship(&self) -> authorship::calls::TransactionApi<'a, T> {
            authorship::calls::TransactionApi::new(self.client)
        }
        pub fn babe(&self) -> babe::calls::TransactionApi<'a, T> {
            babe::calls::TransactionApi::new(self.client)
        }
        pub fn grandpa(&self) -> grandpa::calls::TransactionApi<'a, T> {
            grandpa::calls::TransactionApi::new(self.client)
        }
        pub fn balances(&self) -> balances::calls::TransactionApi<'a, T> {
            balances::calls::TransactionApi::new(self.client)
        }
        pub fn octopus_appchain(&self) -> octopus_appchain::calls::TransactionApi<'a, T> {
            octopus_appchain::calls::TransactionApi::new(self.client)
        }
        pub fn octopus_lpos(&self) -> octopus_lpos::calls::TransactionApi<'a, T> {
            octopus_lpos::calls::TransactionApi::new(self.client)
        }
        pub fn octopus_upward_messages(
            &self,
        ) -> octopus_upward_messages::calls::TransactionApi<'a, T> {
            octopus_upward_messages::calls::TransactionApi::new(self.client)
        }
        pub fn session(&self) -> session::calls::TransactionApi<'a, T> {
            session::calls::TransactionApi::new(self.client)
        }
        pub fn im_online(&self) -> im_online::calls::TransactionApi<'a, T> {
            im_online::calls::TransactionApi::new(self.client)
        }
        pub fn assets(&self) -> assets::calls::TransactionApi<'a, T> {
            assets::calls::TransactionApi::new(self.client)
        }
        pub fn sudo(&self) -> sudo::calls::TransactionApi<'a, T> {
            sudo::calls::TransactionApi::new(self.client)
        }
        pub fn ibc(&self) -> ibc::calls::TransactionApi<'a, T> {
            ibc::calls::TransactionApi::new(self.client)
        }
        pub fn template_module(&self) -> template_module::calls::TransactionApi<'a, T> {
            template_module::calls::TransactionApi::new(self.client)
        }
    }
}
