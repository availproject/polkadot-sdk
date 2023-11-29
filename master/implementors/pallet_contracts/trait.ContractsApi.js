(function() {var implementors = {
"contracts_rococo_runtime":[["impl&lt;__SrApiBlock__: <a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">BlockT</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>, RuntimeApiImplCall: <a class=\"trait\" href=\"sp_api/trait.CallApiAt.html\" title=\"trait sp_api::CallApiAt\">CallApiAt</a>&lt;__SrApiBlock__&gt; + 'static&gt; <a class=\"trait\" href=\"pallet_contracts/trait.ContractsApi.html\" title=\"trait pallet_contracts::ContractsApi\">ContractsApi</a>&lt;__SrApiBlock__, &lt;&lt;<a class=\"enum\" href=\"sp_runtime/enum.MultiSignature.html\" title=\"enum sp_runtime::MultiSignature\">MultiSignature</a> as <a class=\"trait\" href=\"sp_runtime/traits/trait.Verify.html\" title=\"trait sp_runtime::traits::Verify\">Verify</a>&gt;::<a class=\"associatedtype\" href=\"sp_runtime/traits/trait.Verify.html#associatedtype.Signer\" title=\"type sp_runtime::traits::Verify::Signer\">Signer</a> as <a class=\"trait\" href=\"sp_runtime/traits/trait.IdentifyAccount.html\" title=\"trait sp_runtime::traits::IdentifyAccount\">IdentifyAccount</a>&gt;::<a class=\"associatedtype\" href=\"sp_runtime/traits/trait.IdentifyAccount.html#associatedtype.AccountId\" title=\"type sp_runtime::traits::IdentifyAccount::AccountId\">AccountId</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.73.0/std/primitive.u128.html\">u128</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.73.0/std/primitive.u32.html\">u32</a>, H256, <a class=\"struct\" href=\"frame_system/struct.EventRecord.html\" title=\"struct frame_system::EventRecord\">EventRecord</a>&lt;&lt;<a class=\"struct\" href=\"contracts_rococo_runtime/struct.Runtime.html\" title=\"struct contracts_rococo_runtime::Runtime\">Runtime</a> as <a class=\"trait\" href=\"frame_system/pallet/trait.Config.html\" title=\"trait frame_system::pallet::Config\">Config</a>&gt;::<a class=\"associatedtype\" href=\"frame_system/pallet/trait.Config.html#associatedtype.RuntimeEvent\" title=\"type frame_system::pallet::Config::RuntimeEvent\">RuntimeEvent</a>, &lt;<a class=\"struct\" href=\"contracts_rococo_runtime/struct.Runtime.html\" title=\"struct contracts_rococo_runtime::Runtime\">Runtime</a> as <a class=\"trait\" href=\"frame_system/pallet/trait.Config.html\" title=\"trait frame_system::pallet::Config\">Config</a>&gt;::<a class=\"associatedtype\" href=\"frame_system/pallet/trait.Config.html#associatedtype.Hash\" title=\"type frame_system::pallet::Config::Hash\">Hash</a>&gt;&gt; for <a class=\"struct\" href=\"contracts_rococo_runtime/struct.RuntimeApiImpl.html\" title=\"struct contracts_rococo_runtime::RuntimeApiImpl\">RuntimeApiImpl</a>&lt;__SrApiBlock__, RuntimeApiImplCall&gt;<span class=\"where fmt-newline\">where\n    RuntimeApiImplCall::<a class=\"associatedtype\" href=\"sp_api/trait.CallApiAt.html#associatedtype.StateBackend\" title=\"type sp_api::CallApiAt::StateBackend\">StateBackend</a>: <a class=\"trait\" href=\"sp_state_machine/backend/trait.Backend.html\" title=\"trait sp_state_machine::backend::Backend\">StateBackend</a>&lt;<a class=\"type\" href=\"sp_runtime/traits/type.HashingFor.html\" title=\"type sp_runtime::traits::HashingFor\">HashingFor</a>&lt;__SrApiBlock__&gt;&gt;,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.73.0/std/primitive.reference.html\">&amp;'static RuntimeApiImplCall</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,\n    <a class=\"type\" href=\"parachains_common/types/type.AccountId.html\" title=\"type parachains_common::types::AccountId\">AccountId</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    <a class=\"type\" href=\"contracts_rococo_runtime/type.Balance.html\" title=\"type contracts_rococo_runtime::Balance\">Balance</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    <a class=\"enum\" href=\"https://doc.rust-lang.org/1.73.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"struct\" href=\"sp_weights/weight_v2/struct.Weight.html\" title=\"struct sp_weights::weight_v2::Weight\">Weight</a>&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    <a class=\"enum\" href=\"https://doc.rust-lang.org/1.73.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"type\" href=\"contracts_rococo_runtime/type.Balance.html\" title=\"type contracts_rococo_runtime::Balance\">Balance</a>&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    <a class=\"struct\" href=\"https://doc.rust-lang.org/1.73.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.73.0/std/primitive.u8.html\">u8</a>&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    <a class=\"type\" href=\"pallet_contracts/primitives/type.ContractExecResult.html\" title=\"type pallet_contracts::primitives::ContractExecResult\">ContractExecResult</a>&lt;<a class=\"type\" href=\"contracts_rococo_runtime/type.Balance.html\" title=\"type contracts_rococo_runtime::Balance\">Balance</a>, <a class=\"struct\" href=\"frame_system/struct.EventRecord.html\" title=\"struct frame_system::EventRecord\">EventRecord</a>&lt;&lt;<a class=\"struct\" href=\"contracts_rococo_runtime/struct.Runtime.html\" title=\"struct contracts_rococo_runtime::Runtime\">Runtime</a> as <a class=\"trait\" href=\"frame_system/pallet/trait.Config.html\" title=\"trait frame_system::pallet::Config\">Config</a>&gt;::<a class=\"associatedtype\" href=\"frame_system/pallet/trait.Config.html#associatedtype.RuntimeEvent\" title=\"type frame_system::pallet::Config::RuntimeEvent\">RuntimeEvent</a>, &lt;<a class=\"struct\" href=\"contracts_rococo_runtime/struct.Runtime.html\" title=\"struct contracts_rococo_runtime::Runtime\">Runtime</a> as <a class=\"trait\" href=\"frame_system/pallet/trait.Config.html\" title=\"trait frame_system::pallet::Config\">Config</a>&gt;::<a class=\"associatedtype\" href=\"frame_system/pallet/trait.Config.html#associatedtype.Hash\" title=\"type frame_system::pallet::Config::Hash\">Hash</a>&gt;&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    <a class=\"enum\" href=\"pallet_contracts/primitives/enum.Code.html\" title=\"enum pallet_contracts::primitives::Code\">Code</a>&lt;<a class=\"type\" href=\"parachains_common/types/type.Hash.html\" title=\"type parachains_common::types::Hash\">Hash</a>&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    <a class=\"type\" href=\"pallet_contracts/primitives/type.ContractInstantiateResult.html\" title=\"type pallet_contracts::primitives::ContractInstantiateResult\">ContractInstantiateResult</a>&lt;<a class=\"type\" href=\"parachains_common/types/type.AccountId.html\" title=\"type parachains_common::types::AccountId\">AccountId</a>, <a class=\"type\" href=\"contracts_rococo_runtime/type.Balance.html\" title=\"type contracts_rococo_runtime::Balance\">Balance</a>, <a class=\"struct\" href=\"frame_system/struct.EventRecord.html\" title=\"struct frame_system::EventRecord\">EventRecord</a>&lt;&lt;<a class=\"struct\" href=\"contracts_rococo_runtime/struct.Runtime.html\" title=\"struct contracts_rococo_runtime::Runtime\">Runtime</a> as <a class=\"trait\" href=\"frame_system/pallet/trait.Config.html\" title=\"trait frame_system::pallet::Config\">Config</a>&gt;::<a class=\"associatedtype\" href=\"frame_system/pallet/trait.Config.html#associatedtype.RuntimeEvent\" title=\"type frame_system::pallet::Config::RuntimeEvent\">RuntimeEvent</a>, &lt;<a class=\"struct\" href=\"contracts_rococo_runtime/struct.Runtime.html\" title=\"struct contracts_rococo_runtime::Runtime\">Runtime</a> as <a class=\"trait\" href=\"frame_system/pallet/trait.Config.html\" title=\"trait frame_system::pallet::Config\">Config</a>&gt;::<a class=\"associatedtype\" href=\"frame_system/pallet/trait.Config.html#associatedtype.Hash\" title=\"type frame_system::pallet::Config::Hash\">Hash</a>&gt;&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    <a class=\"enum\" href=\"pallet_contracts/wasm/enum.Determinism.html\" title=\"enum pallet_contracts::wasm::Determinism\">Determinism</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    <a class=\"type\" href=\"pallet_contracts/primitives/type.CodeUploadResult.html\" title=\"type pallet_contracts::primitives::CodeUploadResult\">CodeUploadResult</a>&lt;<a class=\"type\" href=\"parachains_common/types/type.Hash.html\" title=\"type parachains_common::types::Hash\">Hash</a>, <a class=\"type\" href=\"contracts_rococo_runtime/type.Balance.html\" title=\"type contracts_rococo_runtime::Balance\">Balance</a>&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    <a class=\"type\" href=\"pallet_contracts/primitives/type.GetStorageResult.html\" title=\"type pallet_contracts::primitives::GetStorageResult\">GetStorageResult</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    __SrApiBlock__::<a class=\"associatedtype\" href=\"sp_runtime/traits/trait.Block.html#associatedtype.Header\" title=\"type sp_runtime::traits::Block::Header\">Header</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,</span>"]],
"kitchensink_runtime":[["impl&lt;__SrApiBlock__: <a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">BlockT</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>, RuntimeApiImplCall: <a class=\"trait\" href=\"sp_api/trait.CallApiAt.html\" title=\"trait sp_api::CallApiAt\">CallApiAt</a>&lt;__SrApiBlock__&gt; + 'static&gt; <a class=\"trait\" href=\"pallet_contracts/trait.ContractsApi.html\" title=\"trait pallet_contracts::ContractsApi\">ContractsApi</a>&lt;__SrApiBlock__, &lt;&lt;<a class=\"enum\" href=\"sp_runtime/enum.MultiSignature.html\" title=\"enum sp_runtime::MultiSignature\">MultiSignature</a> as <a class=\"trait\" href=\"sp_runtime/traits/trait.Verify.html\" title=\"trait sp_runtime::traits::Verify\">Verify</a>&gt;::<a class=\"associatedtype\" href=\"sp_runtime/traits/trait.Verify.html#associatedtype.Signer\" title=\"type sp_runtime::traits::Verify::Signer\">Signer</a> as <a class=\"trait\" href=\"sp_runtime/traits/trait.IdentifyAccount.html\" title=\"trait sp_runtime::traits::IdentifyAccount\">IdentifyAccount</a>&gt;::<a class=\"associatedtype\" href=\"sp_runtime/traits/trait.IdentifyAccount.html#associatedtype.AccountId\" title=\"type sp_runtime::traits::IdentifyAccount::AccountId\">AccountId</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.73.0/std/primitive.u128.html\">u128</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.73.0/std/primitive.u32.html\">u32</a>, H256, <a class=\"struct\" href=\"frame_system/struct.EventRecord.html\" title=\"struct frame_system::EventRecord\">EventRecord</a>&lt;&lt;<a class=\"struct\" href=\"kitchensink_runtime/struct.Runtime.html\" title=\"struct kitchensink_runtime::Runtime\">Runtime</a> as <a class=\"trait\" href=\"frame_system/pallet/trait.Config.html\" title=\"trait frame_system::pallet::Config\">Config</a>&gt;::<a class=\"associatedtype\" href=\"frame_system/pallet/trait.Config.html#associatedtype.RuntimeEvent\" title=\"type frame_system::pallet::Config::RuntimeEvent\">RuntimeEvent</a>, &lt;<a class=\"struct\" href=\"kitchensink_runtime/struct.Runtime.html\" title=\"struct kitchensink_runtime::Runtime\">Runtime</a> as <a class=\"trait\" href=\"frame_system/pallet/trait.Config.html\" title=\"trait frame_system::pallet::Config\">Config</a>&gt;::<a class=\"associatedtype\" href=\"frame_system/pallet/trait.Config.html#associatedtype.Hash\" title=\"type frame_system::pallet::Config::Hash\">Hash</a>&gt;&gt; for <a class=\"struct\" href=\"kitchensink_runtime/struct.RuntimeApiImpl.html\" title=\"struct kitchensink_runtime::RuntimeApiImpl\">RuntimeApiImpl</a>&lt;__SrApiBlock__, RuntimeApiImplCall&gt;<span class=\"where fmt-newline\">where\n    RuntimeApiImplCall::<a class=\"associatedtype\" href=\"sp_api/trait.CallApiAt.html#associatedtype.StateBackend\" title=\"type sp_api::CallApiAt::StateBackend\">StateBackend</a>: <a class=\"trait\" href=\"sp_state_machine/backend/trait.Backend.html\" title=\"trait sp_state_machine::backend::Backend\">StateBackend</a>&lt;<a class=\"type\" href=\"sp_runtime/traits/type.HashingFor.html\" title=\"type sp_runtime::traits::HashingFor\">HashingFor</a>&lt;__SrApiBlock__&gt;&gt;,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.73.0/std/primitive.reference.html\">&amp;'static RuntimeApiImplCall</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,\n    <a class=\"type\" href=\"kitchensink_runtime/type.AccountId.html\" title=\"type kitchensink_runtime::AccountId\">AccountId</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    <a class=\"type\" href=\"node_primitives/type.Balance.html\" title=\"type node_primitives::Balance\">Balance</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    <a class=\"enum\" href=\"https://doc.rust-lang.org/1.73.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"struct\" href=\"sp_weights/weight_v2/struct.Weight.html\" title=\"struct sp_weights::weight_v2::Weight\">Weight</a>&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    <a class=\"enum\" href=\"https://doc.rust-lang.org/1.73.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"type\" href=\"node_primitives/type.Balance.html\" title=\"type node_primitives::Balance\">Balance</a>&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    <a class=\"struct\" href=\"https://doc.rust-lang.org/1.73.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.73.0/std/primitive.u8.html\">u8</a>&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    <a class=\"type\" href=\"pallet_contracts/primitives/type.ContractExecResult.html\" title=\"type pallet_contracts::primitives::ContractExecResult\">ContractExecResult</a>&lt;<a class=\"type\" href=\"node_primitives/type.Balance.html\" title=\"type node_primitives::Balance\">Balance</a>, <a class=\"struct\" href=\"frame_system/struct.EventRecord.html\" title=\"struct frame_system::EventRecord\">EventRecord</a>&lt;&lt;<a class=\"struct\" href=\"kitchensink_runtime/struct.Runtime.html\" title=\"struct kitchensink_runtime::Runtime\">Runtime</a> as <a class=\"trait\" href=\"frame_system/pallet/trait.Config.html\" title=\"trait frame_system::pallet::Config\">Config</a>&gt;::<a class=\"associatedtype\" href=\"frame_system/pallet/trait.Config.html#associatedtype.RuntimeEvent\" title=\"type frame_system::pallet::Config::RuntimeEvent\">RuntimeEvent</a>, &lt;<a class=\"struct\" href=\"kitchensink_runtime/struct.Runtime.html\" title=\"struct kitchensink_runtime::Runtime\">Runtime</a> as <a class=\"trait\" href=\"frame_system/pallet/trait.Config.html\" title=\"trait frame_system::pallet::Config\">Config</a>&gt;::<a class=\"associatedtype\" href=\"frame_system/pallet/trait.Config.html#associatedtype.Hash\" title=\"type frame_system::pallet::Config::Hash\">Hash</a>&gt;&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    <a class=\"enum\" href=\"pallet_contracts/primitives/enum.Code.html\" title=\"enum pallet_contracts::primitives::Code\">Code</a>&lt;<a class=\"type\" href=\"node_primitives/type.Hash.html\" title=\"type node_primitives::Hash\">Hash</a>&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    <a class=\"type\" href=\"pallet_contracts/primitives/type.ContractInstantiateResult.html\" title=\"type pallet_contracts::primitives::ContractInstantiateResult\">ContractInstantiateResult</a>&lt;<a class=\"type\" href=\"kitchensink_runtime/type.AccountId.html\" title=\"type kitchensink_runtime::AccountId\">AccountId</a>, <a class=\"type\" href=\"node_primitives/type.Balance.html\" title=\"type node_primitives::Balance\">Balance</a>, <a class=\"struct\" href=\"frame_system/struct.EventRecord.html\" title=\"struct frame_system::EventRecord\">EventRecord</a>&lt;&lt;<a class=\"struct\" href=\"kitchensink_runtime/struct.Runtime.html\" title=\"struct kitchensink_runtime::Runtime\">Runtime</a> as <a class=\"trait\" href=\"frame_system/pallet/trait.Config.html\" title=\"trait frame_system::pallet::Config\">Config</a>&gt;::<a class=\"associatedtype\" href=\"frame_system/pallet/trait.Config.html#associatedtype.RuntimeEvent\" title=\"type frame_system::pallet::Config::RuntimeEvent\">RuntimeEvent</a>, &lt;<a class=\"struct\" href=\"kitchensink_runtime/struct.Runtime.html\" title=\"struct kitchensink_runtime::Runtime\">Runtime</a> as <a class=\"trait\" href=\"frame_system/pallet/trait.Config.html\" title=\"trait frame_system::pallet::Config\">Config</a>&gt;::<a class=\"associatedtype\" href=\"frame_system/pallet/trait.Config.html#associatedtype.Hash\" title=\"type frame_system::pallet::Config::Hash\">Hash</a>&gt;&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    <a class=\"enum\" href=\"pallet_contracts/wasm/enum.Determinism.html\" title=\"enum pallet_contracts::wasm::Determinism\">Determinism</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    <a class=\"type\" href=\"pallet_contracts/primitives/type.CodeUploadResult.html\" title=\"type pallet_contracts::primitives::CodeUploadResult\">CodeUploadResult</a>&lt;<a class=\"type\" href=\"node_primitives/type.Hash.html\" title=\"type node_primitives::Hash\">Hash</a>, <a class=\"type\" href=\"node_primitives/type.Balance.html\" title=\"type node_primitives::Balance\">Balance</a>&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    <a class=\"type\" href=\"pallet_contracts/primitives/type.GetStorageResult.html\" title=\"type pallet_contracts::primitives::GetStorageResult\">GetStorageResult</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    __SrApiBlock__::<a class=\"associatedtype\" href=\"sp_runtime/traits/trait.Block.html#associatedtype.Header\" title=\"type sp_runtime::traits::Block::Header\">Header</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,</span>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()