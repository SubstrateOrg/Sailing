use support::{decl_module, decl_storage};
//use sr_primitives::traits::Member;
use codec::{Encode, Decode};

pub trait Trait: system::Trait {}

type Uint256 = u32;

#[cfg_attr(feature = "std", derive(Debug, PartialEq, Eq))]
#[derive(Encode, Decode)]
pub struct PRC721Metadata {
	name: Option<String>,
	symbol: Option<String>,
	tokenUrl: Option<String>,
}

//impl PRC721Metadata {
//    pub fn tokenURI(owner: T::AccountId) -> String {
//        return Self.tokenUrl;
//    }
//    pub fn setTokenURI(tokenId: Uint256, uri: String) {}
//}

decl_storage! {
	trait Store for Module<T: Trait> as PRC721 {
		// Mapping from token ID to owner
        //mapping (Uint256 => address) private _tokenOwner;
		TokenOwner get(owned_token): map (Uint256) => Option<T::AccountId>;

		// Mapping from token ID to approved address
        //mapping (Uint256 => address) private _tokenApprovals;
		TokenApprovals get(approval_token): map(Uint256) => Option<T::AccountId>;

		// Mapping from owner to number of owned token
		//mapping (address => Counters.Counter) private _ownedTokensCount;
		OwnedTokensCount get(owned_token_count): map(T::AccountId) => Uint256;

		// Mapping from owner to operator approvals
		//mapping (address => mapping (address => bool)) private _operatorApprovals;
		OperatorApprovals get(approval_operator): map(T::AccountId) => (T::AccountId, bool);
	}
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

fn transfer_from(origin, from: T::AccountId, to: T::AccountId, token_id: T::Hash) -> Result {
            let sender = ensure_signed(origin)?;
            ensure!(Self::_is_approved_or_owner(sender, token_id), "You do not own this token");

            Self::_transfer_from(from, to, token_id)?;

            Ok(())
        }


fn safe_transfer_from(origin, from: T::AccountId, to: T::AccountId, token_id: T::Hash) -> Result {
            let to_balance = <balances::Module<T>>::free_balance(&to);
            ensure!(!to_balance.is_zero(), "'to' account does not satisfy the `ExistentialDeposit` requirement");

            Self::transfer_from(origin, from, to, token_id)?;

            Ok(())



    }

}

impl<T: Trait> Module<T> {


	 fn _transfer_from(from: T::AccountId, to: T::AccountId, token_id: T::Hash) -> Result {
        let owner = match Self::owner_of(token_id) {
            Some(c) => c,
            None => return Err("No owner for this token"),
        };

        ensure!(owner == from, "'from' account does not own this token");

        let balance_of_from = Self::balance_of(&from);
        let balance_of_to = Self::balance_of(&to);

        let new_balance_of_from = match balance_of_from.checked_sub(1) {
            Some (c) => c,
            None => return Err("Transfer causes underflow of 'from' token balance"),
        };

        let new_balance_of_to = match balance_of_to.checked_add(1) {
            Some(c) => c,
            None => return Err("Transfer causes overflow of 'to' token balance"),
        };

        // Writing to storage begins here
        Self::_remove_token_from_owner_enumeration(from.clone(), token_id)?;
        Self::_add_token_to_owner_enumeration(to.clone(), token_id)?;
        
        Self::_clear_approval(token_id)?;
        <OwnedTokensCount<T>>::insert(&from, new_balance_of_from);
        <OwnedTokensCount<T>>::insert(&to, new_balance_of_to);
        <TokenOwner<T>>::insert(&token_id, &to);

        Self::deposit_event(RawEvent::Transfer(Some(from), Some(to), token_id));
        
        Ok(())
    }





	//event Transfer(from: AccountId, to: AccountId, tokenId: Uint256);
    //event Approval(owner: AccountId, approved: AccountId, tokenId: Uint256);
    //event ApprovalForAll(owner: AccountId, operator: AccountId, approved: bool);

    //pub fn balanceOf(owner: AccountId) -> Uint256 {};
    //pub fn ownerOf(tokenId: Uint256) -> AccountId {};

    //pub fn safeTransferFrom(from: AccountId, to: AccountId, tokenId: Uint256) {};

	fn transfer_from(origin, from: T::AccountId, to: T::AccountId, token_id: T::Hash) -> Result {
            let sender = ensure_signed(origin)?;
            ensure!(Self::_is_approved_or_owner(sender, token_id), "You do not own this token");

            Self::_transfer_from(from, to, token_id)?;

            Ok(())
        }


	 fn _add_token_to_owner_enumeration(to: T::AccountId, token_id: T::Hash) -> Result {
        let new_token_index = Self::balance_of(&to);

        <OwnedTokensIndex<T>>::insert(token_id, new_token_index);
        <OwnedTokens<T>>::insert((to, new_token_index), token_id);

        Ok(())
    }	


	 fn _remove_token_from_owner_enumeration(from: T::AccountId, token_id: T::Hash) -> Result {
        let balance_of_from = Self::balance_of(&from);

        // Should never fail because same check happens before this call is made
        let last_token_index = match balance_of_from.checked_sub(1) {
            Some (c) => c,
            None => return Err("Transfer causes underflow of 'from' token balance"),
        };
        
        let token_index = <OwnedTokensIndex<T>>::get(token_id);

        if token_index != last_token_index {
            let last_token_id = <OwnedTokens<T>>::get((from.clone(), last_token_index));
            <OwnedTokens<T>>::insert((from.clone(), token_index), last_token_id);
            <OwnedTokensIndex<T>>::insert(last_token_id, token_index);
        }

        <OwnedTokens<T>>::remove((from, last_token_index));
        // OpenZeppelin does not do this... should I?
        <OwnedTokensIndex<T>>::remove(token_id);

        Ok(())
	 }
    //pub fn transferFrom(from: AccountId, to: AccountId, tokenId: Uint256) {};
    //pub fn approve(to: AccountId, tokenId: Uint256) {};
    //pub fn getApproved(tokenId: Uint256) -> AccountId {};

    //pub fn setApprovalForAll(operator: AccountId, _approved: bool) {};
    //pub fn isApprovedForAll(owner: AccountId, operator: AccountId) -> bool {};

    //pub fn safeTransferFrom(from: AccountId, to: AccountId, tokenId: Uint256, data: bytes){};
}
