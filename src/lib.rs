#![cfg_attr(not(feature = "std"), no_std)]

//! # A Concordium V1 smart contract
use concordium_std::*;
use core::fmt::Debug;

/// Your smart contract state.
#[derive(Serialize, SchemaType)]
pub struct State {
    // Your state
    counter: i8, 
}

/// Your smart contract errors.
#[derive(Debug, PartialEq, Eq, Reject, Serialize, SchemaType)]
pub enum Error {
    /// Failed parsing the parameter.
    #[from(ParseError)]
    ParseParams,
    /// Your error
    OwnerError,
    IncrementError,
    DecrementError,
}

/// Init function that creates a new smart contract.
#[init(contract = "counter_contract")]
fn init(_ctx: &InitContext, _state_builder: &mut StateBuilder) -> InitResult<State> {
    Ok(State { counter: 0 }) // Set initial counter value to 0
}

type IncrementVal = i8;
/// Receive function. The input parameter is the increment value `i8`.
///  If the account owner does not match the contract owner, the receive function will throw [`Error::OwnerError`].
///  If the number to increment by is not positive or is zero, the receive function will throw [`Error::IncrementError`].
#[receive(
    contract = "counter_contract",
    name = "increment",
    parameter = "i8",
    error = "Error",
    mutable
)]
fn increment<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State, StateApiType = S>,
) -> Result<(), Error> {
    // Your code
    let param: IncrementVal = ctx.parameter_cursor().get()?;
    let state = host.state_mut();
    ensure!(
        ctx.sender().matches_account(&ctx.owner()),
        Error::OwnerError
    );

    ensure!(param > 0, Error::IncrementError);
    state.counter += param;
    Ok(())
}

// Decrement function
#[receive(
    contract = "counter_contract",
    name = "decrement",
    parameter = "i8",
    error = "Error",
    mutable
)]
fn decrement<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State, StateApiType = S>,
) -> Result<(), Error> {
    // Your code

    let param: IncrementVal = ctx.parameter_cursor().get()?;
    let state = host.state_mut();
    ensure!(
        ctx.sender().matches_account(&ctx.owner()),
        Error::OwnerError
    );

    ensure!(param < 0, Error::DecrementError);
    state.counter += param;
    Ok(())
}

/// View function that returns the counter value.
#[receive(contract = "counter_contract", name = "view", return_value = "i8")]
fn view<'a, 'b, S: HasStateApi>(
    _ctx: &'a impl HasReceiveContext,
    host: &'b impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<i8> {
    Ok(host.state().counter)
}
