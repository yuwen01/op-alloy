#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/alloy.jpg",
    html_favicon_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/favicon.ico"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod receipt;
pub use receipt::{OpDepositReceipt, OpDepositReceiptWithBloom, OpReceiptEnvelope, OpTxReceipt};

mod transaction;
pub use transaction::{
    DepositSourceDomain, DepositSourceDomainIdentifier, DepositTransaction, L1InfoDepositSource,
    OpTxEnvelope, OpTxType, OpTypedTransaction, TxDeposit, UpgradeDepositSource, UserDepositSource,
    DEPOSIT_TX_TYPE_ID,
};

mod eip1559;
pub use eip1559::{decode_eip_1559_params, decode_holocene_extra_data, EIP1559ParamError};

mod hardforks;
pub use hardforks::{
    Hardforks, EIP4788_FROM, FJORD_GAS_PRICE_ORACLE, GAS_PRICE_ORACLE, GAS_PRICE_ORACLE_DEPLOYER,
    GAS_PRICE_ORACLE_FJORD_DEPLOYER, L1_BLOCK_DEPLOYER, L1_INFO_DEPOSITER, NEW_L1_BLOCK,
};

mod block;
pub use block::OpBlock;

#[cfg(feature = "serde")]
pub use transaction::serde_deposit_tx_rpc;

/// Bincode-compatible serde implementations for consensus types.
///
/// `bincode` crate doesn't work well with optionally serializable serde fields, but some of the
/// consensus types require optional serialization for RPC compatibility. This module makes so that
/// all fields are serialized.
///
/// Read more: <https://github.com/bincode-org/bincode/issues/326>
#[cfg(all(feature = "serde", feature = "serde-bincode-compat"))]
pub mod serde_bincode_compat {
    pub use super::transaction::serde_bincode_compat::TxDeposit;
}
