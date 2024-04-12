//!Ammuto Core is the central library used to host Ammuto technologies.
//!It will accept objects that implement traits for the different adapters. This will allow a multitude of unique implementations of Ammuto to share the same components.
//! 
//!The implementations provided here only include officially supported features. For custom features, please fork or make your own version, or consider requesting that feature to be included in the official implementation!

pub mod query;
pub mod values;

//Components:
//Core <--(Poll/Push Adapter)--> UI
//Core <--(Connection Adapter)--> Database
//Database <--(Compression & Encryption Adapter)--> File System

///The core structure for Ammuto. Contains all the objects Ammuto requires to operate, and provides the required functions for communicating with all the adapters
pub struct Core<Frontend, Database>
where
	Frontend: FrontendAdapter,
	Database: DatabaseAdapter,
{
	pub frontend: Frontend,
	pub database: Database,
}
impl<Frontend, Database> Core<Frontend, Database>
where
	Frontend: FrontendAdapter,
	Database: DatabaseAdapter, {

	pub fn get_frontend(&self) -> &Frontend {
		&self.frontend
	}
	pub fn get_frontend_mut(&mut self) -> &mut Frontend {
		&mut self.frontend
	}
	pub fn get_database(&self) -> &Database {
		&self.database
	}
	pub fn get_database_mut(&mut self) -> &mut Database {
		&mut self.database
	}
}

///Describes what communication type the Frontend expects from the Core
pub enum CommunicationMode<Communicator> {
	///The Core will push updates to the Frontend without prompt. This is best for instantaneous responses but requires more resources
	Push(Communicator),
	///The Core will wait for the Frontend to request an update. This is best for performance, but the Frontend will not know if anything happens from another instance.
	Poll
}

///Describes the functions a Frontend should provide to accept data and requests from Ammuto
pub trait FrontendAdapter {
	///Requests the Frontend's preferred mode of communication
	fn preferred_flow() -> CommunicationMode<impl FrontendCommunicator>;
}
///Describes the functions required to communicate with the Frontend using [`CommunicationMode::Push`]
pub trait FrontendCommunicator {
	//TODO: Push communication style
}
///Describes the functions a Database should provide to accept data and requests from Ammuto
pub trait DatabaseAdapter {
	//TODO: CRUD and saving
}
///Describes the functions needed to get and put resources on the target medium.
pub trait ResourceAdapter {
	//TODO: Filesystem Access that also encompasses HTTPS or FTP request format.
	
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
