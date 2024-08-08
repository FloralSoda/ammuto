//!Ammuto Core is the central library used to host Ammuto technologies.
//!It will accept objects that implement traits for the different adapters. This will allow a multitude of unique implementations of Ammuto to share the same components.
//!
//!The implementations provided here only include officially supported features. For custom features, please fork or make your own version, or consider requesting that feature to be included in the official implementation!

use std::{
    future::Future,
    sync::mpsc::{RecvError, SendError},
};

use query::{DatabaseQuery, DatabaseResult, QueryError};
use uuid::Uuid;

pub mod data;
pub mod query;

//Components:
//Core <--(Poll/Push Adapter)--> UI
//Core <--(Connection Adapter)--> Database
//Database <--(Compression & Encryption Adapter)--> File System

///The core structure for Ammuto. Contains all the objects Ammuto requires to operate, and provides the required functions for communicating with all the adapters
pub struct Core<Frontend, Database>
where
    Database: DatabaseAdapter,
    Frontend: FrontendAdapter<Database>,
{
    pub frontend: Frontend,
    pub database: Option<Database>,
}
impl<Frontend, Database> Core<Frontend, Database>
where
    Database: DatabaseAdapter,
    Frontend: FrontendAdapter<Database>,
{
    ///Sends a query to the connected DatabaseAdapter.\
    ///If there is no DatabaseAdapter, the database_message event on the frontend will be called immediately with [`DatabaseResult::NoDatabase`]
    pub async fn send_query(
        &self,
        query: DatabaseQuery,
    ) -> Result<DatabaseResult, QueryError<Database::ErrorType>> {
        if let Some(database) = &self.database {
            database.send_query(query)
        } else {
            Err(QueryError::NoDatabase)
        }
    }
}

///Describes the functions required for Ammuto to control and handle any frontend
pub trait FrontendAdapter<Database>
where
    Database: DatabaseAdapter,
{
    ///Serves as the entry point for the frontend. This is expected to block the thread
    fn run();
}
///Describes the functions a Database should provide to accept data and requests from Ammuto
pub trait DatabaseAdapter {
    ///The type used for a custom error
    type ErrorType;
    //TODO: CRUD and saving
    ///Synchronously sends a query to the Database. See [`Core::send_query`] for an asynchronous version
    fn send_query(
        &self,
        query: DatabaseQuery,
    ) -> Result<DatabaseResult, QueryError<Self::ErrorType>>;
}
///Describes the functions needed to get and put resources on the target medium.
pub trait ResourceAdapter {
    //TODO: Filesystem Access that also encompasses HTTPS or FTP request format.
    fn read_media();

    ///Sets the adapter used for compression. If [`None`], compression must be either handled by the filesystem or there to be no compression at all.
    fn set_compression_adapter(&mut self, adapter: Option<impl CompressionAdapter>);
    ///Sets the adapter used for encryption. If [`None`], encryption must be either handled by the filesystem or there to be no encryption at all.
    fn set_encryption_adapter(&mut self, adapter: Option<impl EncryptionAdapter>);
}
///Describes the functions needed to compress and decompress a resource coming from the parent [`ResourceAdapter`]
pub trait CompressionAdapter {
    ///Takes in any amount of uncompressed byte data, and outputs the data in a lossless compressed format
    fn compress(&self, data: &[u8]) -> Vec<u8>;
    ///Takes in any amount of compressed byte data, and outputs the original data
    fn decompress(&self, compressed_data: &[u8]) -> Vec<u8>;
}
///Describes the functions needed to encrypt and decrypt a resource coming from the parent [`ResourceAdapter`]
pub trait EncryptionAdapter {
    ///Takes in any amount of unencrypted data, and outputs the data in an encrypted format
    fn encrypt(&self, data: &[u8]) -> Vec<u8>;
    ///Takes in any amount of encrypted data, and outputs the data in an unencrypted format
    fn decrypt(&self, encrypted_data: &[u8]) -> Vec<u8>;
}
