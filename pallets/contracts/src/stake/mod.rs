// This file is part of PoCS=Substrate.
// Copyright (C) Auguth Research Foundation, India.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// This file is utilized for Proof of Contract Stake Protocol (PoCS).
//

use crate::{
	Config, Error, Pallet as Contracts
};
use frame_system::pallet_prelude::BlockNumberFor;
use codec::{ Encode, Decode, MaxEncodedLen };
use scale_info::TypeInfo;
use sp_runtime::DispatchError;

/// Represents the delegation details of a deployed contract.
/// 
/// It includes:
/// `owner` - The owner of the contract.
/// `delegate_to` - The validator account i.e., contract to which the contract is delegated.
/// `delegate_at` - The block number when the delegation was set.
/// 
#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct DelegateInfo<T: Config> {
	owner : T::AccountId,
	delegate_to: T::AccountId,
	delegate_at: BlockNumberFor<T>,
}


impl<T: Config> DelegateInfo<T> {

    /// Returns the owner `AccountId` of the contract associated with this `DelegateInfo`.
    /// 
    fn owner(&self) -> T::AccountId {
        self.owner.clone()
    }

    /// Returns the `AccountId` of the validator to whom the contract is delegated.
    /// 
    fn delegate_to(&self) -> T::AccountId {
        self.delegate_to.clone()
    }
    
    /// Returns the block number when the delegate information was last updated.
    /// 
    fn delegate_at(&self) -> BlockNumberFor<T> {
        self.delegate_at
    }

    /// Retrieves the `DelegateInfo` for a given contract address.
    /// 
    fn get(contract_addr: &T::AccountId) -> Result<DelegateInfo<T>, DispatchError> {
        Contracts::<T>::get_delegate_info(contract_addr)
            .ok_or_else(|| Error::<T>::NoStakeExists.into())
    }


}
/// Tracks the gas usage metrics of a contract for staking purposes.
/// 
/// It includes:
/// `reputation` - The reputation score of the contract.
/// `blockheight` - The block height of its most recent usage.
/// `stake_score` - The stake score associated with the contract.
/// 
#[derive(Encode, Decode, TypeInfo,  PartialEq, Eq, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct StakeInfo<T: Config> {
	reputation: u32,
	blockheight: BlockNumberFor<T>,
	stake_score: u128,
}

impl<T: Config> StakeInfo<T>{

    /// Returns the stake score of a contract's `StakeInfo`. 
    /// 
    fn stake_score(&self) -> u128 {
        self.stake_score
    }

    /// Returns the reputation score of a contract's `StakeInfo`.
    /// 
    fn reputation(&self) -> u32 {
        self.reputation
    }
    
    /// Returns the block height of the most recent interaction with the contract. 
    /// 
    fn blockheight(&self) -> BlockNumberFor<T> {
        self.blockheight
    }

    /// Retrieves the `StakeInfo` of an instantiated contract.
    /// 
    fn get(contract_addr: &T::AccountId) -> Result<StakeInfo<T>,DispatchError> {
        Contracts::<T>::get_stake_info(contract_addr)
            .ok_or_else(|| Error::<T>::NoStakeExists.into())
    }

}