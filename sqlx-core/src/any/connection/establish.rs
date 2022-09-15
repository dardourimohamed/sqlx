use crate::any::options::AnyConnectOptions;
use crate::any::AnyConnection;
use crate::connection::Connection;
use crate::error::Error;

impl AnyConnection {
    pub(crate) async fn establish(options: &AnyConnectOptions) -> Result<Self, Error> {
        let driver = crate::any::driver::choose_driver(&options.database_url)?;
        (driver.connect)(options).await
    }
}
