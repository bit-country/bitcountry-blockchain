#![cfg(feature = "runtime-benchmarks")]

use frame_benchmarking::{account, whitelisted_caller};
use frame_support::traits::{Currency, Get, OnFinalize, OnInitialize};
use frame_system::RawOrigin;
use orml_benchmarking::runtime_benchmarks;
use primitives::{AccountId, Balance, BlockNumber, FungibleTokenId};

use super::utils::{dollar, set_balance, set_metaverse_treasury_initial_balance};

use crate::{
	Call, Event, MinimumCampaignCoolingOffPeriod, MinimumCampaignDuration, MinimumRewardPool, Reward, Runtime, System,
};

const CURRENCY_ID: FungibleTokenId = FungibleTokenId::NativeToken(0);

fn next_block() {
	Reward::on_finalize(System::block_number());
	System::set_block_number(System::block_number() + 1);
	Reward::on_initialize(System::block_number());
}

pub fn run_to_block(n: u32) {
	while System::block_number() < n {
		next_block();
	}
}

runtime_benchmarks! {
	{ Runtime, reward }

	// create campaign
	create_campaign {
		System::set_block_number(1u32.into());
		let origin: AccountId = whitelisted_caller();
		set_balance(CURRENCY_ID, &origin, dollar(1000));
		let campaign_end  = System::block_number() + MinimumCampaignDuration::get();
	}: _(RawOrigin::Signed(origin.clone()), origin.clone(), MinimumRewardPool::get(), campaign_end.clone(), MinimumCampaignCoolingOffPeriod::get(), vec![1])

	//  claim reward
	claim_reward{
		System::set_block_number(1u32.into());
		let origin: AccountId = whitelisted_caller();
		set_balance(CURRENCY_ID, &origin, dollar(1000));
		let claiming_account: AccountId = whitelisted_caller();
		set_balance(CURRENCY_ID, &claiming_account, dollar(10));

		let campaign_end  = System::block_number() + MinimumCampaignDuration::get();
		Reward::create_campaign(RawOrigin::Signed(origin.clone()).into(), origin.clone(), MinimumRewardPool::get(), campaign_end.clone(), MinimumCampaignCoolingOffPeriod::get(), vec![1]);
		Reward::set_reward(RawOrigin::Root.into(), 0u32.into(), claiming_account.clone(), 5u32.into());
		run_to_block(campaign_end);
	}: _(RawOrigin::Signed(claiming_account.clone()), 0u32.into())

	// set reward
	set_reward{
		System::set_block_number(1u32.into());
		let origin: AccountId = whitelisted_caller();
		set_balance(CURRENCY_ID, &origin, dollar(1000));
		let claiming_account: AccountId = whitelisted_caller();
		set_balance(CURRENCY_ID, &claiming_account, dollar(10));

		let campaign_end  = System::block_number() + MinimumCampaignDuration::get();
		Reward::create_campaign(RawOrigin::Signed(origin.clone()).into(), origin.clone(), MinimumRewardPool::get(), campaign_end.clone(), MinimumCampaignCoolingOffPeriod::get(), vec![1]);
	}: _(RawOrigin::Root, 0u32.into(), claiming_account.clone(), 5u32.into())

	// close_campaign
	close_campaign{
		System::set_block_number(1u32.into());
		let origin: AccountId = whitelisted_caller();
		set_balance(CURRENCY_ID, &origin, dollar(1000));

		let campaign_end  = System::block_number() + MinimumCampaignDuration::get();
		Reward::create_campaign(RawOrigin::Signed(origin.clone()).into(), origin.clone(), MinimumRewardPool::get(), campaign_end.clone(), MinimumCampaignCoolingOffPeriod::get(), vec![1]);
		run_to_block(2*campaign_end);
	}: _(RawOrigin::Signed(origin.clone()), 0u32.into())

	// on finalize
	on_finalize {
		System::set_block_number(1u32.into());
		let origin: AccountId = whitelisted_caller();
		set_balance(CURRENCY_ID, &origin, dollar(1000));
		let claiming_account: AccountId = whitelisted_caller();
		set_balance(CURRENCY_ID, &claiming_account, dollar(10));

		let campaign_end  = System::block_number() + MinimumCampaignDuration::get();
		Reward::create_campaign(RawOrigin::Signed(origin.clone()).into(), origin.clone(), MinimumRewardPool::get(), campaign_end.clone(), MinimumCampaignCoolingOffPeriod::get(), vec![1]);
		Reward::set_reward(RawOrigin::Root.into(), 0u32.into(), claiming_account.clone(), 5u32.into());
	}: {
		Reward::on_finalize(System::block_number() + MinimumCampaignDuration::get());
	}
}

#[cfg(test)]
mod tests {
	use orml_benchmarking::impl_benchmark_test_suite;

	use crate::benchmarking::utils::tests::new_test_ext;

	use super::*;

	impl_benchmark_test_suite!(new_test_ext(),);
}
