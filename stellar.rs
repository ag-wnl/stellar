#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, Env, String, log, Address, Symbol};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NotOwner = 1,
    NoOwner = 2,
}

#[contract]
pub struct stellarindiaEvent;

#[contractimpl]
impl stellarindiaEvent {
    pub fn initilize (env: Env, admin: Address, name: String, symbol: Symbol){
        log!(&env, "admin: {}, name: {}, symbol: {}", admin, name, symbol);
    }

    ///mint a seat ticket for the event
    pub fn mint(env: Env, owner: Address, seat: u32) -> u32{
        //check if the seat is taken
        if env.storage().persistent().has(&seat){
            panic!("this seat is already taken");
        }

        env.storage().persistent().set(&seat, &owner);
        seat
    }

    ///owner of the NFT ticket
    pub fn owner_of(env: Env, seat: u32) -> Option<Address> {
        env.storage().persistent().get(&seat)
    }

    ///transfer address from to address 'to'
    pub fn transfer(env: Env, seat: u32, from: Address, to: Address) -> Result<u32, Error>{
        let current_owner: Option<Address> = env.storage().persistent().get(&seat);
        log!(&env, "check current_owner: {}", current_owner);

        match current_owner {
            Some(current_owner) => log!(&env, ""),
            None => return Err(Error::NotOwner),
        }

        env.storage().persistent().set(&seat, &to);
        Ok(seat)
    }
    
}

// Deployed at : 
// https://futurenet.stellarchain.io/transactions/9293eb75200ffc867aa1d68db7b6f96b4d94bee994e685cbc58b0788e6999d6a
// https://testnet.stellarchain.io/transactions/24a025db94ea6e815a92fc565efeda3594040c066f0807cd421655d8194a2bb0





