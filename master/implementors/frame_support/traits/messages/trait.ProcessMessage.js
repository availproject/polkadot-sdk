(function() {var implementors = {
"pallet_message_queue":[["impl&lt;Origin, const REQUIRED_WEIGHT: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.70.0/std/primitive.u64.html\">u64</a>&gt; <a class=\"trait\" href=\"frame_support/traits/messages/trait.ProcessMessage.html\" title=\"trait frame_support::traits::messages::ProcessMessage\">ProcessMessage</a> for <a class=\"struct\" href=\"pallet_message_queue/mock_helpers/struct.NoopMessageProcessor.html\" title=\"struct pallet_message_queue::mock_helpers::NoopMessageProcessor\">NoopMessageProcessor</a>&lt;Origin, REQUIRED_WEIGHT&gt;<span class=\"where fmt-newline\">where\n    Origin: FullCodec + MaxEncodedLen + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> + TypeInfo + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,</span>"]],
"rococo_runtime":[["impl <a class=\"trait\" href=\"frame_support/traits/messages/trait.ProcessMessage.html\" title=\"trait frame_support::traits::messages::ProcessMessage\">ProcessMessage</a> for <a class=\"struct\" href=\"rococo_runtime/struct.MessageProcessor.html\" title=\"struct rococo_runtime::MessageProcessor\">MessageProcessor</a>"]],
"staging_xcm_builder":[["impl&lt;MessageOrigin: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"struct\" href=\"staging_xcm_builder/test_utils/struct.MultiLocation.html\" title=\"struct staging_xcm_builder::test_utils::MultiLocation\">MultiLocation</a>&gt; + FullCodec + MaxEncodedLen + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> + TypeInfo + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>, XcmExecutor: <a class=\"trait\" href=\"staging_xcm_builder/test_utils/trait.ExecuteXcm.html\" title=\"trait staging_xcm_builder::test_utils::ExecuteXcm\">ExecuteXcm</a>&lt;Call&gt;, Call&gt; <a class=\"trait\" href=\"frame_support/traits/messages/trait.ProcessMessage.html\" title=\"trait frame_support::traits::messages::ProcessMessage\">ProcessMessage</a> for <a class=\"struct\" href=\"staging_xcm_builder/struct.ProcessXcmMessage.html\" title=\"struct staging_xcm_builder::ProcessXcmMessage\">ProcessXcmMessage</a>&lt;MessageOrigin, XcmExecutor, Call&gt;"]],
"westend_runtime":[["impl <a class=\"trait\" href=\"frame_support/traits/messages/trait.ProcessMessage.html\" title=\"trait frame_support::traits::messages::ProcessMessage\">ProcessMessage</a> for <a class=\"struct\" href=\"westend_runtime/struct.MessageProcessor.html\" title=\"struct westend_runtime::MessageProcessor\">MessageProcessor</a>"]],
"xcm_emulator":[["impl&lt;T&gt; <a class=\"trait\" href=\"xcm_emulator/trait.ProcessMessage.html\" title=\"trait xcm_emulator::ProcessMessage\">ProcessMessage</a> for <a class=\"struct\" href=\"xcm_emulator/struct.DefaultMessageProcessor.html\" title=\"struct xcm_emulator::DefaultMessageProcessor\">DefaultMessageProcessor</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"xcm_emulator/trait.Chain.html\" title=\"trait xcm_emulator::Chain\">Chain</a> + <a class=\"trait\" href=\"xcm_emulator/trait.RelayChain.html\" title=\"trait xcm_emulator::RelayChain\">RelayChain</a>,\n    T::<a class=\"associatedtype\" href=\"xcm_emulator/trait.Chain.html#associatedtype.Runtime\" title=\"type xcm_emulator::Chain::Runtime\">Runtime</a>: <a class=\"trait\" href=\"xcm_emulator/trait.MessageQueueConfig.html\" title=\"trait xcm_emulator::MessageQueueConfig\">MessageQueueConfig</a>,\n    &lt;&lt;T::<a class=\"associatedtype\" href=\"xcm_emulator/trait.Chain.html#associatedtype.Runtime\" title=\"type xcm_emulator::Chain::Runtime\">Runtime</a> as <a class=\"trait\" href=\"xcm_emulator/trait.MessageQueueConfig.html\" title=\"trait xcm_emulator::MessageQueueConfig\">MessageQueueConfig</a>&gt;::<a class=\"associatedtype\" href=\"xcm_emulator/trait.MessageQueueConfig.html#associatedtype.MessageProcessor\" title=\"type xcm_emulator::MessageQueueConfig::MessageProcessor\">MessageProcessor</a> as <a class=\"trait\" href=\"xcm_emulator/trait.ProcessMessage.html\" title=\"trait xcm_emulator::ProcessMessage\">ProcessMessage</a>&gt;::<a class=\"associatedtype\" href=\"xcm_emulator/trait.ProcessMessage.html#associatedtype.Origin\" title=\"type xcm_emulator::ProcessMessage::Origin\">Origin</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a>&lt;<a class=\"enum\" href=\"xcm_emulator/enum.AggregateMessageOrigin.html\" title=\"enum xcm_emulator::AggregateMessageOrigin\">AggregateMessageOrigin</a>&gt;,\n    <a class=\"struct\" href=\"xcm_emulator/struct.MessageQueuePallet.html\" title=\"struct xcm_emulator::MessageQueuePallet\">MessageQueuePallet</a>&lt;T::<a class=\"associatedtype\" href=\"xcm_emulator/trait.Chain.html#associatedtype.Runtime\" title=\"type xcm_emulator::Chain::Runtime\">Runtime</a>&gt;: <a class=\"trait\" href=\"xcm_emulator/trait.EnqueueMessage.html\" title=\"trait xcm_emulator::EnqueueMessage\">EnqueueMessage</a>&lt;<a class=\"enum\" href=\"xcm_emulator/enum.AggregateMessageOrigin.html\" title=\"enum xcm_emulator::AggregateMessageOrigin\">AggregateMessageOrigin</a>&gt; + <a class=\"trait\" href=\"xcm_emulator/trait.ServiceQueues.html\" title=\"trait xcm_emulator::ServiceQueues\">ServiceQueues</a>,</span>"]],
"xcm_fuzzer":[["impl <a class=\"trait\" href=\"frame_support/traits/messages/trait.ProcessMessage.html\" title=\"trait frame_support::traits::messages::ProcessMessage\">ProcessMessage</a> for <a class=\"struct\" href=\"xcm_fuzzer/struct.Relay.html\" title=\"struct xcm_fuzzer::Relay\">Relay</a>"],["impl <a class=\"trait\" href=\"frame_support/traits/messages/trait.ProcessMessage.html\" title=\"trait frame_support::traits::messages::ProcessMessage\">ProcessMessage</a> for <a class=\"struct\" href=\"xcm_fuzzer/relay_chain/struct.MessageProcessor.html\" title=\"struct xcm_fuzzer::relay_chain::MessageProcessor\">MessageProcessor</a>"]],
"xcm_simulator":[],
"xcm_simulator_example":[["impl <a class=\"trait\" href=\"frame_support/traits/messages/trait.ProcessMessage.html\" title=\"trait frame_support::traits::messages::ProcessMessage\">ProcessMessage</a> for <a class=\"struct\" href=\"xcm_simulator_example/struct.Relay.html\" title=\"struct xcm_simulator_example::Relay\">Relay</a>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()