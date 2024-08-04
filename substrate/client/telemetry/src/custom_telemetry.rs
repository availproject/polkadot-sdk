// Execute the following command to run the tests:
//
// cd polkadot-sdk/substrate/client/telemetry/src
// cargo test -- --test-threads 1
//

use std::{
	sync::Mutex,
	time::{Duration, SystemTimeError},
};

use serde::Serialize;
use wasm_timer::{SystemTime, UNIX_EPOCH};

use crate::custom_telemetry::external::BlockIntervalFromNode;
use crate::{telemetry, TelemetryHandle, SUBSTRATE_INFO};

///
#[repr(u8)]
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub enum IntervalKind {
	/// Tells us how long it took us to produce a block. Basically it's all about
	/// taking transactions from the mem pool and executing them.
	Proposal = 0,
	/// Tells us how long it took to get a block from someone.
	Sync = 1,
	/// Tells us how long it took to import a block.
	/// Import is measured for the node that produced the block as well as for the
	/// node that requested that block.
	Import = 2,
}

/// Interval information bundled together with block information.
#[derive(Serialize, Clone, Debug)]
pub struct IntervalWithBlockInformation {
	///
	pub kind: IntervalKind,
	///
	pub block_number: u64,
	///
	pub block_hash: String,
	///
	pub start_timestamp: u64,
	///
	pub end_timestamp: u64,
}

///
#[derive(Serialize, Debug)]
pub struct BlockRequestsDetail {
	///
	pub current_queue_size: u32,
	///
	pub requests_handled: u32,
	///
	pub time_frame: u64,
}

///
#[derive(Default, Debug)]
pub struct BlockMetrics {
	///
	intervals: Vec<IntervalWithBlockInformation>,
	///
	partial_intervals: Vec<IntervalWithBlockInformation>,
	///
	block_requests: Vec<BlockRequestsDetail>,
	///
	max_interval_buffer_size: usize,
	///
	max_block_request_buffer_size: usize,
}

impl BlockMetrics {
	///
	pub const fn new() -> Self {
		Self {
			intervals: Vec::new(),
			partial_intervals: Vec::new(),
			block_requests: Vec::new(),
			max_interval_buffer_size: 0,
			max_block_request_buffer_size: 0,
		}
	}
}

static BLOCK_METRICS: Mutex<BlockMetrics> = Mutex::new(BlockMetrics::new());

impl BlockMetrics {
	///
	pub fn observe_interval(value: IntervalWithBlockInformation) {
		println!(
			"Observing new Interval. BlockHash={:?}, BlockNumber={:?}, Kind={:?}",
			value.block_hash, value.block_number, value.kind
		);
		let Ok(mut lock) = BLOCK_METRICS.lock() else {
			return;
		};

		lock.intervals.push(value);

		if lock.intervals.len() > lock.max_interval_buffer_size {
			lock.intervals.remove(0);
		}
	}

	///
	pub fn observe_interval_partial(
		kind: IntervalKind,
		block_number: u64,
		block_hash: String,
		timestamp: u64,
		is_start: bool,
	) {
		println!(
			"Observing Partial Interval. BlockHash={:?}, BlockNumber={:?}, Kind={:?}",
			block_hash, block_number, kind
		);

		let mut entry = {
			let Ok(mut lock) = BLOCK_METRICS.lock() else {
				return;
			};

			if is_start {
				let value = IntervalWithBlockInformation {
					kind,
					block_number,
					block_hash,
					start_timestamp: timestamp,
					end_timestamp: 0,
				};

				lock.partial_intervals.push(value);

				if lock.partial_intervals.len() > lock.max_interval_buffer_size {
					lock.partial_intervals.remove(0);
				}

				return;
			}

			let existing_entry_pos = lock.partial_intervals.iter_mut().position(|v| {
				v.block_hash == block_hash && v.block_number == block_number && v.kind == kind
			});

			let Some(pos) = existing_entry_pos else {
				return;
			};

			lock.partial_intervals.remove(pos)
		};

		entry.end_timestamp = timestamp;

		Self::observe_interval(entry);
	}

	///
	pub fn observe_block_request(value: BlockRequestsDetail) {
		println!(
			"Observing Block Request. RH={:?}, CQS={:?}, TM={:?}",
			value.requests_handled, value.current_queue_size, value.time_frame
		);

		let Ok(mut lock) = BLOCK_METRICS.lock() else {
			return;
		};

		lock.block_requests.push(value);

		if lock.block_requests.len() > lock.max_block_request_buffer_size {
			lock.block_requests.remove(0);
		}
	}

	///
	pub fn take_metrics() -> Option<BlockMetrics> {
		let Ok(mut lock) = BLOCK_METRICS.lock() else {
			return None;
		};

		let metrics = std::mem::take(&mut *lock);
		lock.max_interval_buffer_size = metrics.max_interval_buffer_size;
		lock.max_block_request_buffer_size = metrics.max_block_request_buffer_size;

		Some(metrics)
	}

	///
	pub fn get_current_timestamp_in_ms_or_default() -> u64 {
		Self::get_current_timestamp_in_ms().map(|v| v as u64).unwrap_or(0u64)
	}

	fn get_current_timestamp_in_ms() -> Result<u128, SystemTimeError> {
		let start = SystemTime::now();
		start.duration_since(UNIX_EPOCH).map(|f| f.as_millis())
	}
}

/// This will be send to the telemetry
pub mod external {
	use super::*;

	///
	#[derive(Debug, Serialize, Clone)]
	pub struct IntervalFromNode {
		///
		pub kind: IntervalKind,
		///
		pub start_timestamp: u64,
		///
		pub end_timestamp: u64,
	}

	///
	#[derive(Debug, Default, Serialize, Clone)]
	pub struct BlockIntervalFromNode {
		///
		pub block_number: u64,
		///
		pub block_hash: String,
		///
		pub intervals: Vec<IntervalFromNode>,
	}

	///
	pub fn prepare_data(
		mut value: Vec<IntervalWithBlockInformation>,
	) -> Vec<BlockIntervalFromNode> {
		let mut output = Vec::with_capacity(value.len() / 2);
		value.sort_by(|l, r| {
			if l.block_number == r.block_number {
				l.block_hash.cmp(&r.block_hash)
			} else {
				l.block_number.cmp(&r.block_number)
			}
		});

		let mut block = BlockIntervalFromNode::default();
		for v in value {
			let interval = IntervalFromNode {
				kind: v.kind,
				start_timestamp: v.start_timestamp,
				end_timestamp: v.end_timestamp,
			};

			if (v.block_number != block.block_number || v.block_hash != block.block_hash)
				&& block.block_number != u64::default()
			{
				output.push(std::mem::take(&mut block));
			}

			block.block_number = v.block_number;
			block.block_hash = v.block_hash;
			block.intervals.push(interval);
		}

		if block.block_number != u64::default() {
			output.push(block);
		}

		output
	}
}

///
pub struct CustomTelemetryWorker {
	///
	pub handle: Option<TelemetryHandle>,
	///
	pub sampling_interval_ms: u128,
	///
	pub max_interval_buffer_size: usize,
	///
	pub max_block_request_buffer_size: usize,
}

impl CustomTelemetryWorker {
	///
	pub async fn run(
		self,
		filter_intervals: Option<fn(Vec<BlockIntervalFromNode>) -> Vec<BlockIntervalFromNode>>,
		filter_block_requests: Option<fn(Vec<BlockRequestsDetail>) -> Vec<BlockRequestsDetail>>,
	) {
		const SLEEP_DURATION: Duration = Duration::from_millis(250);

		if let Ok(mut lock) = BLOCK_METRICS.lock() {
			lock.max_interval_buffer_size = self.max_interval_buffer_size;
			lock.max_block_request_buffer_size = self.max_block_request_buffer_size;
		}

		let mut start = std::time::Instant::now();
		loop {
			if start.elapsed().as_millis() >= self.sampling_interval_ms {
				self.send_telemetry(filter_intervals, filter_block_requests).await;
				start = std::time::Instant::now();
			}

			tokio::time::sleep(SLEEP_DURATION).await;
		}
	}

	///
	pub async fn send_telemetry(
		&self,
		filter_intervals: Option<fn(Vec<BlockIntervalFromNode>) -> Vec<BlockIntervalFromNode>>,
		filter_block_requests: Option<fn(Vec<BlockRequestsDetail>) -> Vec<BlockRequestsDetail>>,
	) {
		let (block_intervals, block_requests) =
			Self::get_and_filter_data(filter_intervals, filter_block_requests);

		dbg!(&block_intervals);
		dbg!(&block_requests);
		println!("Done");

		if block_intervals.len() > 0 || block_requests.len() > 0 {
			telemetry!(
				self.handle;
				SUBSTRATE_INFO;
				"block.metrics";
				"block_intervals" => block_intervals,
				"block_requests" => block_requests,
			);
		}
	}

	pub(crate) fn get_and_filter_data(
		filter_intervals: Option<fn(Vec<BlockIntervalFromNode>) -> Vec<BlockIntervalFromNode>>,
		filter_block_requests: Option<fn(Vec<BlockRequestsDetail>) -> Vec<BlockRequestsDetail>>,
	) -> (Vec<BlockIntervalFromNode>, Vec<BlockRequestsDetail>) {
		let metrics = BlockMetrics::take_metrics().unwrap_or_default();
		dbg!(&metrics);

		let block_intervals = external::prepare_data(metrics.intervals);
		let block_intervals = match filter_intervals {
			Some(f) => f(block_intervals),
			_ => block_intervals,
		};

		let block_requests = metrics.block_requests;
		let block_requests = match filter_block_requests {
			Some(f) => f(block_requests),
			_ => block_requests,
		};

		(block_intervals, block_requests)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn dummy_interval(block_number: Option<u64>) -> IntervalWithBlockInformation {
		IntervalWithBlockInformation {
			kind: IntervalKind::Import,
			block_number: block_number.unwrap_or(0),
			block_hash: "".to_string(),
			start_timestamp: 0,
			end_timestamp: 0,
		}
	}

	fn observe_block_request(time_frame: u64) {
		let value = BlockRequestsDetail { current_queue_size: 0, requests_handled: 0, time_frame };
		BlockMetrics::observe_block_request(value);
	}

	fn reset_global_variable(buffer_size: usize) {
		{
			let mut lock = BLOCK_METRICS.lock().unwrap();
			lock.max_interval_buffer_size = buffer_size;
			lock.max_block_request_buffer_size = buffer_size;
		}

		_ = BlockMetrics::take_metrics().unwrap();
	}

	#[test]
	fn buffer_interval_0_buffer_size() {
		reset_global_variable(0);

		BlockMetrics::observe_interval(dummy_interval(None));
		let metrics = BlockMetrics::take_metrics().unwrap();
		assert_eq!(metrics.intervals.len(), 0);
	}

	#[test]
	fn buffer_interval_1_buffer_size() {
		reset_global_variable(1);

		BlockMetrics::observe_interval(dummy_interval(Some(0)));
		let metrics = BlockMetrics::take_metrics().unwrap();
		assert_eq!(metrics.intervals.len(), 1);
		assert_eq!(metrics.intervals[0].block_number, 0);

		BlockMetrics::observe_interval(dummy_interval(Some(1)));
		let metrics = BlockMetrics::take_metrics().unwrap();
		assert_eq!(metrics.intervals.len(), 1);
		assert_eq!(metrics.intervals[0].block_number, 1);
	}

	#[test]
	fn mem_take_works() {
		const BUFFER_SIZE: usize = 10;
		reset_global_variable(BUFFER_SIZE);

		BlockMetrics::observe_interval(dummy_interval(Some(0)));
		BlockMetrics::observe_interval_partial(IntervalKind::Sync, 0, "".to_string(), 0, true);
		BlockMetrics::observe_interval_partial(IntervalKind::Sync, 10, "".to_string(), 0, true);
		BlockMetrics::observe_interval_partial(IntervalKind::Sync, 10, "".to_string(), 0, false);

		{
			let lock = BLOCK_METRICS.lock().unwrap();
			assert_eq!(lock.max_interval_buffer_size, BUFFER_SIZE);
			assert_eq!(lock.intervals.len(), 2);
			assert_eq!(lock.partial_intervals.len(), 1);
			assert_eq!(lock.block_requests.len(), 0);
		}

		let old_metrics = BlockMetrics::take_metrics().unwrap();
		assert_eq!(old_metrics.max_interval_buffer_size, BUFFER_SIZE);
		assert_eq!(old_metrics.intervals.len(), 2);
		assert_eq!(old_metrics.partial_intervals.len(), 1);
		assert_eq!(old_metrics.block_requests.len(), 0);

		let lock = BLOCK_METRICS.lock().unwrap();
		assert_eq!(lock.max_interval_buffer_size, BUFFER_SIZE);
		assert_eq!(lock.intervals.len(), 0);
		assert_eq!(lock.partial_intervals.len(), 0);
		assert_eq!(lock.block_requests.len(), 0);
	}

	#[test]
	fn buffer_partial_interval_0_buffer_size() {
		reset_global_variable(0);

		BlockMetrics::observe_interval_partial(IntervalKind::Sync, 0, "".to_string(), 0, true);
		let metrics = BlockMetrics::take_metrics().unwrap();
		assert_eq!(metrics.partial_intervals.len(), 0);
	}

	#[test]
	fn buffer_partial_interval_1_buffer_size() {
		reset_global_variable(1);

		BlockMetrics::observe_interval_partial(IntervalKind::Sync, 0, "".to_string(), 0, true);
		let metrics = BlockMetrics::take_metrics().unwrap();
		assert_eq!(metrics.partial_intervals.len(), 1);
		assert_eq!(metrics.partial_intervals[0].block_number, 0);

		BlockMetrics::observe_interval_partial(IntervalKind::Sync, 1, "".to_string(), 0, true);
		let metrics = BlockMetrics::take_metrics().unwrap();
		assert_eq!(metrics.partial_intervals.len(), 1);
		assert_eq!(metrics.partial_intervals[0].block_number, 1);
	}

	#[test]
	fn buffer_partial_interval_works() {
		reset_global_variable(1);

		BlockMetrics::observe_interval_partial(IntervalKind::Sync, 25, "".to_string(), 0, true);
		{
			let lock = BLOCK_METRICS.lock().unwrap();
			assert_eq!(lock.partial_intervals.len(), 1);
			assert_eq!(lock.partial_intervals[0].block_number, 25);
		}

		BlockMetrics::observe_interval_partial(IntervalKind::Sync, 25, "".to_string(), 0, false);
		{
			let lock = BLOCK_METRICS.lock().unwrap();
			assert_eq!(lock.partial_intervals.len(), 0);
			assert_eq!(lock.intervals.len(), 1);
			assert_eq!(lock.intervals[0].block_number, 25);
		}
	}

	#[test]
	fn get_and_filter_data_works() {
		reset_global_variable(10);
		use IntervalKind::*;

		let setup_scenartion = || {
			BlockMetrics::observe_interval_partial(Import, 1, "".to_string(), 0, true);
			BlockMetrics::observe_interval_partial(Import, 1, "".to_string(), 0, false);
			BlockMetrics::observe_interval_partial(Sync, 1, "".to_string(), 0, true);
			BlockMetrics::observe_interval_partial(Sync, 1, "".to_string(), 0, false);
			BlockMetrics::observe_interval_partial(Proposal, 2, "".to_string(), 0, true);
			BlockMetrics::observe_interval_partial(Proposal, 2, "".to_string(), 0, false);
			BlockMetrics::observe_interval_partial(Proposal, 3, "".to_string(), 0, true);
			observe_block_request(1);
			observe_block_request(2);
			observe_block_request(3);
		};

		setup_scenartion();

		let (block_intervals, block_requests) =
			CustomTelemetryWorker::get_and_filter_data(None, None);

		assert_eq!(block_intervals.len(), 2);
		assert_eq!(block_intervals[0].block_number, 1);
		assert_eq!(block_intervals[0].block_number, 1);
		assert_eq!(block_intervals[0].intervals.len(), 2);
		assert_eq!(block_intervals[1].intervals.len(), 1);

		assert_eq!(block_requests.len(), 3);
		assert_eq!(block_requests[0].time_frame, 1);
		assert_eq!(block_requests[1].time_frame, 2);
		assert_eq!(block_requests[2].time_frame, 3);

		// Second test. Filter Interval data 1
		setup_scenartion();

		let (block_intervals, block_requests) = CustomTelemetryWorker::get_and_filter_data(
			Some(no_data_interval),
			Some(no_data_request),
		);
		assert_eq!(block_intervals.len(), 0);
		assert_eq!(block_requests.len(), 0);

		// Third test. Filter Interval data 2
		setup_scenartion();

		let (block_intervals, block_requests) =
			CustomTelemetryWorker::get_and_filter_data(Some(one_interval), Some(one_request));
		assert_eq!(block_intervals.len(), 1);
		assert_eq!(block_intervals[0].block_number, 1);
		assert_eq!(block_requests.len(), 1);
		assert_eq!(block_requests[0].time_frame, 1);
	}

	fn no_data_interval(_: Vec<BlockIntervalFromNode>) -> Vec<BlockIntervalFromNode> {
		vec![]
	}

	fn no_data_request(_: Vec<BlockRequestsDetail>) -> Vec<BlockRequestsDetail> {
		vec![]
	}

	fn one_interval(mut value: Vec<BlockIntervalFromNode>) -> Vec<BlockIntervalFromNode> {
		while value.len() > 1 {
			value.pop();
		}

		value
	}

	fn one_request(mut value: Vec<BlockRequestsDetail>) -> Vec<BlockRequestsDetail> {
		while value.len() > 1 {
			value.pop();
		}

		value
	}
}
