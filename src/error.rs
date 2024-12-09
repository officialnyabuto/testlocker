use anchor_lang::prelude::*;

#[error_code]
pub enum LockError {
    #[msg("Lock amount is zero")]
    AmountZero,
    #[msg("Lock time is zero")]
    TimeZero,
    #[msg("Token is not valid")]
    NotValidToken,
    #[msg("Token is already locked")]
    AlreadyLocked,
    #[msg("Not reached to unlock time")]
    NotUnlockTime,
    #[msg("Token is already unlocked")]
    AlreadyUnlocked,
    #[msg("Token address is wrong")]
    SplMintError,
    #[msg("Token authorization is not valid")]
    AuthorizationErr,
    #[msg("Lock time is less than now")]
    BeforeNow,
    #[msg("Low Balance")]
    LowBalance,
    #[msg("Not Per Vesting Unlock Time")]
    NotPerVestingUnlockTime,
    #[msg("AlreadyDidFirstClaim")]
    AlreadyDidFirstClaim,
    #[msg("Not Bigger Than End Time")]
    NotBiggerThanEndTime
}
