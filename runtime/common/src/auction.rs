use frame_support::pallet_prelude::Get;
use frame_support::traits::{Currency, OriginTrait};
use orml_traits::{BasicCurrency, MultiCurrency as MultiCurrencyTrait};
use pallet_evm::{
	AddressMapping, ExitRevert, ExitSucceed, Precompile, PrecompileFailure, PrecompileHandle, PrecompileOutput,
	PrecompileResult, PrecompileSet,
};
use sp_core::{H160, U256};
use sp_runtime::traits::Dispatchable;
use sp_std::{marker::PhantomData, prelude::*};

use precompile_utils::data::{Address, EvmData, EvmDataWriter};
use precompile_utils::handle::PrecompileHandleExt;
use precompile_utils::modifier::FunctionModifier;
use precompile_utils::prelude::RuntimeHelper;
use precompile_utils::{succeed, EvmResult};
use primitives::evm::{Erc20Mapping, Output};
use primitives::{evm, Balance, FungibleTokenId, ItemId, AuctionId};

#[precompile_utils_macro::generate_function_selector]
#[derive(Debug, PartialEq)]
pub enum Action {
    ListingItem = "getListingItem(uint256)",
    ListingMetaverse = "getListingMetaverse(uint256)",
    ListingPrice = "getListingEnd(uint256)",
	ListingEnd = "getListingEnd(uint256)",
    ListingHighestBidder = "getListingHighestBidder(uint256)",
    CreateNftAuction = "createNftAuction(account,uint256,uint256,uint256,uint256,uint256,uint256)",
    Bid = "bid(account,uint256,uint256)",
    FinalizeAuction = "finalizeAuction(uint256)",
    CreateNftBuyNow = "createNftBuyNow(account,uint256,uint256,uint256,uint256,uint256,uint256)",
    BuyNow = "buyNow(account,uint256,uint256)",
    CancelListing = "cancelListing(account,uint256)",
    MakeOffer = "makeOffer(account,uint256,uint256,uint256)",
    AcceptOffer = "acceptOffer(account,account,uint256,uint256)",
    WithdrawOffer = "withrawOffer(account,uint256,uint256)",
}

/// Alias for the Balance type for the provided Runtime and Instance.
pub type BalanceOf<Runtime> = <<Runtime as auction::Config>::FungibleTokenCurrency as MultiCurrencyTrait<
	<Runtime as frame_system::Config>::AccountId,
>>::Balance;

/// The `Auction` impl precompile.
///
///
/// Actions
/// - Query listing's NFT. Rest `input` bytes: `listing_id`.
/// - Query listing's metaverse. Rest `input` bytes: `listing_id`.
/// - Query listing's currency and, current price. Rest `input` bytes: `listing_id`.
/// - Query listing's current end block. Rest `input` bytes: `listing_id`.
/// - Query listing's current highest bidder. Rest `input` bytes: `listing_id`.
/// - Create auction for an NFT. Rest `input` bytes: `class_id`, `token_id`, `value`, `end_time`, `metaverse_id`, `currency_id`.
/// - Bid on auction. Rest `input` bytes: `listing_id`, `value`.
/// - Finalize auction. Rest `input` bytes: `listing_id`.
/// - Create buy now for an NFT. Rest `input` bytes: `class_id`, `token_id`, `value`, `end_time`, `metaverse_id`, `currency_id`.
/// - Buy a buy now listing. Rest `input` bytes: `listing_id`, `value`.
/// - Cancel auction or buy now listing. Rest `input` bytes: `listing_id`.
/// - Make offer for an NFT. Rest `input` bytes: `class_id`, `token_id`, `value`.
/// - Accept offer for an NFT. Rest `input` bytes: `class_id`, `token_id`, `account_id`.
/// - Withdraw offer for an NFT. Rest `input` bytes: `class_id`, `token_id`.
pub struct AuctionPrecompile<Runtime>(PhantomData<Runtime>);

impl<Runtime> Precompile for AuctionPrecompile<Runtime>
where
	Runtime: auction::Config + pallet_evm::Config + frame_system::Config,
	Runtime: Erc20Maping,
	U256: From<
		<<Runtime as auction::Config>::FungibleTokenCurrency as MultiCurrencyTrait<
            <Runtime as frame_system::Config>::AccountId,
        >>::Balance,
	>,
	//BalanceOf<Runtime>: TryFrom<U256> + Into<U256> + EvmData,
	<<Runtime as frame_system::Config>::Call as Dispatchable>::Origin: OriginTrait,
{
	fn execute(handle: &mut impl PrecompileHandle) -> PrecompileResult {
		let result = {
			let selector = match handle.read_selector() {
				Ok(selector) => selector,
				Err(e) => return Err(e),
			};

			if let Err(err) = handle.check_function_modifier(match selector {
				Action::CreateNftAuction | Action::CreateNftBuyNow | Action::Bid | Action::BuyNow | Action::CancelListing | Action::FinalizeAuction | Action::MakeOffer | Action::AcceptOffer | Action::WithdrawOffer => FunctionModifier::NonPayable,
				_ => FunctionModifier::View,
			}) {
				return Err(err);
			}

			match selector {
				// Local and Foreign common
                ListingItem => Self::auction_info(handle),
                ListingMetaverse => Self::auction_info(handle),
                ListingPrice => Self::auction_info(handle),
                ListingEnd => Self::auction_info(handle),
                ListingHighestBidder => Self::auction_info(handle),
                CreateNftAuction => Self::create_auction(handle),
                Bid => Self::bid(handle),
                FinalizeAuction => Self::finalize_auction(handle),
                CreateNftBuyNow => Self::create_buy_now(handle),
                BuyNow => Self::buy_now(handle),
                CancelListing => Self::cancel_listing(handle),
                MakeOffer => Self::make_offer(handle),
                AcceptOffer => Self::accept_offer(handle),
                WithdrawOffer => Self::withdraw_offer(handle),
			}
		};
		Err(PrecompileFailure::Revert {
			exit_status: ExitRevert::Reverted,
			output: "invalid marketplace action".into(),
		})
	}
}