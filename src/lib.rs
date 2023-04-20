#![allow(clippy::drop_non_drop)]
#![deny(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]

pub use db::{
    Builder, Database, MultimapTableDefinition, MultimapTableHandle, TableDefinition, TableHandle,
    UntypedMultimapTableHandle, UntypedTableHandle,
};
pub use error::Error;
pub use multimap_table::{
    MultimapRange, MultimapTable, MultimapValue, ReadOnlyMultimapTable, ReadableMultimapTable,
};
pub use table::{Drain, DrainFilter, Range, ReadOnlyTable, ReadableTable, Table};
pub use transactions::{DatabaseStats, Durability, ReadTransaction, WriteTransaction};
pub use tree_store::{AccessGuard, AccessGuardMut, Savepoint};
pub use types::{RedbKey, RedbValue, TypeName};

type Result<T = (), E = Error> = std::result::Result<T, E>;

#[cfg(feature = "python")]
pub use crate::python::redb;

mod db;
mod error;
mod multimap_table;
#[cfg(feature = "python")]
mod python;
mod sealed;
mod table;
mod transaction_tracker;
mod transactions;
mod tree_store;
mod tuple_types;
mod types;
