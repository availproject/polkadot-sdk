use crate::*;

// A trait for Fusion operations with a generic `AccountId` and `Balance` and `PoolId`.
pub trait FusionExt<AccountId, Balance, PoolId> {
	/// Handles the change of an era, which includes operations like distributing rewards and cleaning up old data.
	fn handle_end_era(era: EraIndex, era_duration: u64) -> ();

	/// Set the exposure for each pool for reward computation
	/// Exposure is set at the beginning of the era N for era N using stake from era N-1
	fn set_fusion_exposures() -> ();

	/// Return the fusion voters to add to the staking pallet
	fn get_fusion_voters() -> Vec<(AccountId, u64, Vec<AccountId>)>;

	/// Return the fusion voters count
	fn get_active_pool_count() -> usize;

	/// Returns the pool id if the account is a pool funds account
	fn get_pool_id_from_funds_account(account: &AccountId) -> Option<PoolId>;

	/// Updates the Fusion exposure with election data result
	fn update_pool_exposure(
		maybe_pool_account: &AccountId,
		validator: &AccountId,
		value: Balance,
		era: EraIndex,
	) -> ();

	/// In the staking pallet, if a pool was slashed, we record an unapplied slash
	fn add_fusion_slash(
		era: EraIndex,
		validator: &AccountId,
		nominators: &Vec<(AccountId, Balance)>,
	) -> Weight;

	/// If a slash was cancelled and it concerned a Fusion pool, we need to cancel it there too
	fn cancel_fusion_slash(era: EraIndex, slash_indices: Vec<u32>) -> ();

	/// If a slash is applied, we need to intercept it and take the corresponding fusion currencies
	/// Returns true if the nominator is a fusion pool (regardless if it succeed to get slashed)
	/// In this function we will give 100% of the slash amount to the treasury,
	/// the rewards for validator are going to get minted in the staking pallet like before
	fn apply_fusion_slash(
		slash_era: EraIndex,
		validator: &AccountId,
		funds_account: &AccountId,
	) -> bool;
}
impl<AccountId, Balance, PoolId> FusionExt<AccountId, Balance, PoolId> for () {
	fn handle_end_era(_era: EraIndex, _era_duration: u64) {
		()
	}

	fn set_fusion_exposures() {
		()
	}

	fn get_fusion_voters() -> Vec<(AccountId, u64, Vec<AccountId>)> {
		Vec::default()
	}

	fn get_active_pool_count() -> usize {
		0
	}

	fn get_pool_id_from_funds_account(_account: &AccountId) -> Option<PoolId> {
		None
	}

	fn update_pool_exposure(
		_maybe_pool_account: &AccountId,
		_validator: &AccountId,
		_value: Balance,
		_era: EraIndex,
	) {
		()
	}

	fn add_fusion_slash(
		_era: EraIndex,
		_validator: &AccountId,
		_nominators: &Vec<(AccountId, Balance)>,
	) -> Weight {
		Weight::from_parts(0, 0)
	}

	fn cancel_fusion_slash(_era: EraIndex, _slash_indices: Vec<u32>) -> () {
		()
	}

	fn apply_fusion_slash(
		_slash_era: EraIndex,
		_validator: &AccountId,
		_funds_account: &AccountId,
	) -> bool {
		false
	}
}
