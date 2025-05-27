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
	Config, Error, Event, Pallet as Contracts, StakeInfoMap, DelegateInfoMap
};
use frame_system::pallet_prelude::BlockNumberFor;
use codec::{ Encode, Decode, MaxEncodedLen };
use scale_info::TypeInfo;
use sp_runtime::{
    traits::Hash, DispatchError
}; 


/// The minimum reputation required to participate in staking contracts.
/// 
pub const MIN_REPUTATION: u32 = 3; 

/// The fixed unit used for incrementing reputation and initializing it during instantiation.
/// 
pub const REPUTATION_FACTOR: u32 = 1;

/// The initial stake score, set to zero for contract constructor purposes.
/// 
pub const INITIAL_STAKE_SCORE: u128 = 0;


/// Represents the delegation details of a deployed contract.
/// 
/// It includes:
/// `owner` - The owner of the contract.
/// `delegate_to` - The validator account i.e., contract to which the contract is delegated.
/// `delegate_at` - The block number when the delegation was set.
/// 
#[derive(Encode, Decode, Clone, TypeInfo, PartialEq, Eq, MaxEncodedLen)]
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

    /// Creates a new `DelegateInfo` instance where the deployer is both the owner and delegate.
    /// 
    fn new(owner: &T::AccountId) -> Self {
        Self {
            owner: owner.clone(),
            delegate_to: owner.clone(),
            delegate_at: frame_system::Pallet::<T>::block_number(),
        }
    }


}
/// Tracks the gas usage metrics of a contract for staking purposes.
/// 
/// It includes:
/// `reputation` - The reputation score of the contract.
/// `blockheight` - The block height of its most recent usage.
/// `stake_score` - The stake score associated with the contract.
/// 
#[derive(Encode, Decode, Clone, TypeInfo,  PartialEq, Eq, MaxEncodedLen)]
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

    /// Creates a new `StakeInfo` instance using predefined constants for instantiation. 
    /// 
	fn new() -> Self {
		Self{
			reputation: REPUTATION_FACTOR,
			blockheight: <frame_system::Pallet<T>>::block_number(),
			stake_score: INITIAL_STAKE_SCORE,
		}
	}

    /// Updates the stake score based on gas usage provided and adjusts reputation if the block height has changed.
    /// 
    fn update(&self, gas: &u64) -> Self {
        let current_block_height = <frame_system::Pallet<T>>::block_number();
        let current_reputation = self.reputation;
        let gas_cast = *gas as u128;
        if current_block_height > self.blockheight {
            let new_stake_score =  gas_cast
                                        .saturating_mul(current_reputation.into())
                                        .saturating_add(self.stake_score);
            Self {
                reputation: current_reputation
                            .saturating_add(REPUTATION_FACTOR),
                blockheight: current_block_height,
                stake_score: new_stake_score,
            }
        } else {
            let new_stake_score = gas_cast
                                        .saturating_add(self.stake_score);
            Self {
                reputation: current_reputation,
                blockheight: current_block_height,
                stake_score: new_stake_score,
            }
        }
    }

}


/// Represents a stake request for each contract invocation or execution.
///
/// It includes
/// - `contract` - The account ID of the contract being invoked.
/// - `caller` - The account ID of the caller (transaction origin or another contract).
/// - `gas` - The total gas expenditure for this invocation of a single stack frame.
/// 
#[derive(Encode, Decode, Clone, PartialEq, TypeInfo, Eq, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct StakeRequest<T: Config> {
	contract: T::AccountId,
    caller: T::AccountId,
    gas: u64
}

impl<T: Config> StakeRequest<T>{

    /// Processes a stake request for a given contract.
    ///
    /// This function serves as the entry point for handling stake requests.  
    /// It first checks whether stake information already exists for the specified contract.  
    /// If it does, a new stake request is processed; otherwise, an empty stake info entry is created.
    /// 
    pub fn stake(
        origin: &T::AccountId, 
        contract_addr: &T::AccountId, 
        gas: &u64) -> Result<(),DispatchError>
    {
        if StakeInfoMap::<T>::contains_key(contract_addr){
            Self::new(contract_addr, gas)?;
        } else {
            Self::empty(origin, contract_addr);
        }
        Ok(())
    }

    /// Initializes an empty stake and delegate entry for a contract.
    ///
    /// This helper function creates a new `StakeInfo` and `DelegateInfo` entry  
    /// for the specified contract and stores them in their respective maps.
    /// 
    fn empty(origin: &T::AccountId, contract_addr: &T::AccountId) {
		let stake_info: StakeInfo<T> = StakeInfo::<T>::new();
		StakeInfoMap::<T>::insert(contract_addr, stake_info.clone());
        let delegate_info = <DelegateInfo<T>>::new(origin);
        DelegateInfoMap::<T>::insert(contract_addr, delegate_info.clone());
    }

    /// Processes new stake request and updates stake info for the given contract.
    ///
    /// This helper function retrieves the existing delegate and stake information  
    /// for the specified contract and updates the stake score based on  
    /// delegation conditions. It also triggers appropriate events  
    /// and decides whether bonding actions are necessary.
    /// 
    fn new(contract_addr: &T::AccountId, gas: &u64) -> Result<(),DispatchError>{
        let delegate_info = <DelegateInfo<T>>::get(contract_addr)?;
        let stake_info = <StakeInfo<T>>::get(contract_addr)?;

        // Provide zero gas if contract isn't matured i.e., haven't delegated at all.
        let gas = if delegate_info.owner != delegate_info.delegate_to { gas } else { &0 };

        let new_stake_info = <StakeInfo<T>>::update(&stake_info, gas);
        StakeInfoMap::<T>::insert(contract_addr, new_stake_info.clone());

        // No Stake Update due to zero gas, hence no stake event emission
        if delegate_info.owner != delegate_info.delegate_to {
            Contracts::<T>::deposit_event(
                Event::Staked {
                    contract: contract_addr.clone(),
                    stake_score: new_stake_info.stake_score.clone(),
                },
            );
        } 

        // If contract passes criteria notify ready for staking!
        if new_stake_info.reputation == MIN_REPUTATION {
            Contracts::<T>::deposit_event(
                Event::ReadyToStake {
                    contract: contract_addr.clone(),
                },
            );
        }

        Ok(())
    }


}