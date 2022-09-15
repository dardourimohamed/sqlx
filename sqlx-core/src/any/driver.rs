use crate::any::connection::AnyConnectionBackend;
use crate::any::{
    Any, AnyArguments, AnyConnectOptions, AnyConnection, AnyQueryResult, AnyRow, AnyStatement,
    AnyTypeInfo,
};
use crate::common::DebugFn;
use crate::connection::Connection;
use crate::database::Database;
use crate::describe::Describe;
use crate::transaction::Transaction;
use crate::Error;
use either::Either;
use futures_core::future::BoxFuture;
use futures_core::stream::BoxStream;
use once_cell::sync::OnceCell;
use std::fmt::Debug;
use std::marker::PhantomData;
use url::Url;

static DRIVERS: OnceCell<&'static [AnyDriver]> = OnceCell::new();

#[derive(Debug)]
#[non_exhaustive]
pub struct AnyDriver {
    pub(crate) url_schemes: &'static [&'static str],
    pub(crate) connect:
        DebugFn<fn(&AnyConnectOptions) -> BoxFuture<'_, crate::Result<AnyConnection>>>,
    pub(crate) migrate_database: Option<AnyMigrateDatabase>,
}

impl AnyDriver {
    pub const fn without_migrate<DB: Database>() -> Self
    where
        DB::Connection: AnyConnectionBackend,
        <DB::Connection as Connection>::Options: TryFrom<&AnyConnectOptions, Error = Error>,
    {
        Self {
            url_schemes: DB::URL_SCHEMES,
            connect: DebugFn(AnyConnection::connect::<DB>),
            migrate_database: None,
        }
    }

    #[cfg(not(feature = "migrate"))]
    pub fn with_migrate<DB: Database>() -> Self {
        Self::without_migrate::<DB>()
    }

    #[cfg(feature = "migrate")]
    pub fn with_migrate<DB: Database + crate::migrate::MigrateDatabase>() -> Self {
        Self {
            migrate_database: Some(AnyMigrateDatabase {
                create_database: DebugFn(DB::create_database),
                database_exists: DebugFn(DB::database_exists),
                drop_database: DebugFn(DB::drop_database),
            }),
            ..Self::without_migrate()
        }
    }
}

pub struct AnyMigrateDatabase {
    pub(crate) create_database: DebugFn<fn(&str) -> BoxFuture<'_, crate::Result<()>>>,
    pub(crate) database_exists: DebugFn<fn(&str) -> BoxFuture<'_, crate::Result<bool>>>,
    pub(crate) drop_database: DebugFn<fn(&str) -> BoxFuture<'_, crate::Result<()>>>,
}

/// Set the list of drivers that `AnyConnection` should use.
///
/// ### Panics
/// If called more than once.
pub fn install_drivers(drivers: &'static [AnyDriver]) {
    DRIVERS.set(drivers).expect("drivers already installed")
}

pub fn choose_driver(url: &Url) -> crate::Result<&'static AnyDriver> {
    let scheme = url.scheme();

    let drivers: &[AnyDriver] = DRIVERS
        .get()
        .expect("No drivers installed. Please see the documentation in `sqlx::any` for details.");

    drivers
        .iter()
        .find(|driver| driver.url_schemes.contains(&url.scheme()))
        .ok_or_else(|| {
            Error::Configuration(format!("no driver found for URL scheme {:?}", scheme).into())
        })
}
