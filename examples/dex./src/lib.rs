use borsh::{BorshDeserialize, BorshSerialize};
use wasmlanche_sdk::{program::Program, public, state_keys, types::Address};

/// The program state keys.
#[state_keys]
enum StateKey {
    /// The total supply of the token. Key prefix 0x0.
    Token0,
    Token1,
    Reserve0,
    Reserve1,
    /// The name of the token. Key prefix 0x1.
    Name,
    /// The symbol of the token. Key prefix 0x2.
    Symbol,
    /// The balance of the token by address. Key prefix 0x3 + address.
    Balance(Address),
}

/// Initializes the program with a name, symbol, and total supply.
#[public]
pub fn init(program: Program,token0:Address,token1:Address) -> bool {
    program
        .state()
        .store(StateKey::Token0.to_vec(), &token0)
        .expect("failed to store total supply");
        program
        .state()
        .store(StateKey::Token1.to_vec(), &token1)
        .expect("failed to store total supply");
  
    true
}
fn get_pair_state()->(Address,Address,i64,i64){
   (program
        .state()
        .get(StateKey::Token0.to_vec())
        .expect("failed to store total supply"),
        program
        .state()
        .get(StateKey::Token0.to_vec())
        .expect("failed to store total supply"),
        program
        .state()
        .get(StateKey::Reserve0.to_vec())
        .expect("failed to store total supply"),
        program
        .state()
        .get(StateKey::Token0.to_vec())
        .expect("failed to store total supply"))

}
fn calculate_out(amount_in:i64,reserve_in:i64,reserve_out:i64)->i64{
    ///uint amountInWithFee = amountIn.mul(997);
    ///uint numerator = amountInWithFee.mul(reserveOut);
    ///uint denominator = reserveIn.mul(1000).add(amountInWithFee);
    ///amountOut = numerator / denominator;
    let amt_in=amount_in*1000;
    let numerator=amt_in*reserve_out;
    let denominator=reserve_in*1000+amt_in;
    numerator/denominator
}   
// given an output amount of an asset and pair reserves, returns a required input amount of the other asset
fn calculate_in(amount_out:i64,reserve_in:i64,reserve_out:i64)->i64{
    /// uint numerator = reserveIn.mul(amountOut).mul(1000);
    ///    uint denominator = reserveOut.sub(amountOut).mul(997);
    ///    amountIn = (numerator / denominator).add(1);
    let numerator=reserve_in*amount_out*1000;
    let denominator=(reserve_out-amount_out)*1000;
    (numerator/denominator)+1
}
#[public]
pub fn swap_exact_out(program: Program,side:bool,amount_out:i64) -> bool {
    let (token0,token1,reserve0,reserve1)=get_pair_state();
    let transfer_in = match side{
        calculate_in(amount_out,reserve0,reserve1),
        calculate_in(amount_out,reserve1,reserve0)
    };
    
    /// transfer from caller transfer_in
    /// transfer to caller transter_out
    true
}
#[public]
pub fn swap_exact_in(program: Program,side:bool,amount_in:i64) -> bool {
    let (token0,token1,reserve0,reserve1)=get_pair_state();
    let transfer_out = match side{
        calculate_out(amount_in,reserve0,reserve1),
        calculate_out(amount_in,reserve1,reserve0)
    };
    true
}
/// Transfers balance from the token owner to the recipient.

pub fn transfer(program: Program,from:Address, to: Address, amount: i64,token:bool) -> bool {   

    true
}

#[public]
pub fn mint_to(program: Program, recipient: Address, amount: i64) -> bool {   

    true
}



#[cfg(test)]
mod tests {
    use serial_test::serial;
    use std::env;
    use wasmlanche_sdk::simulator::{
        self, id_from_step, Key, Operator, PlanResponse, Require, ResultAssertion,
    };

   
    #[test]
    #[serial]
    #[ignore = "requires SIMULATOR_PATH and PROGRAM_PATH to be set"]
    fn test_create_program() {
        let s_path = env::var(simulator::PATH_KEY).expect("SIMULATOR_PATH not set");
        let simulator = simulator::Client::new(s_path);

        let owner_key = "owner";
        // create owner key in single step
        let resp = simulator
            .key_create::<PlanResponse>(owner_key, Key::Ed25519)
            .unwrap();
        assert_eq!(resp.error, None);

        let p_path = env::var("PROGRAM_PATH").expect("PROGRAM_PATH not set");
        // create a new program on chain.
        let resp = simulator
            .program_create::<PlanResponse>("owner", p_path.as_ref())
            .unwrap();
        assert_eq!(resp.error, None);
        assert_eq!(resp.result.id.is_some(), true);
    }
}
