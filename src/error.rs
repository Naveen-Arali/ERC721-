use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.


    #[error("Name is not in the expected format (3-30 UTF-8 bytes)")]
    NameWrongFormat {},

    #[error("Ticker symbol is not in expected format [A-Z]{{3,6}}")]
    TickerWrongSymbolFormat {},

    #[error("Decimals must not exceed 18")]
    DecimalsExceeded {},

    #[error("Insufficient allowance (allowance {allowance}, required={required})")]
    InsufficientAllowance { allowance: u128, required: u128 },

    #[error("Insufficient funds (balance {balance}, required={required})")]
    InsufficientFunds { balance: u128, required: u128 },

    #[error("Corrupted data found (16 byte expected)")]
    CorruptedDataFound {},
}
