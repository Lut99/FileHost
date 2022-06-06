/* CONFIG.rs
 *   by Lut99
 *
 * Created:
 *   06 Jun 2022, 10:06:15
 * Last edited:
 *   06 Jun 2022, 11:49:21
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains the configuration definition of the server side, shared
 *   between the daemon and the CTL.
**/

use std::error;
use std::fmt::{Display, Formatter, Result as FResult};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

use log::LevelFilter;
use serde::{Deserialize, Serialize};


/***** ERRORS *****/
/// Defines errors that relate to parsing the Config.
#[derive(Debug)]
pub enum Error {
    /// Failed to parse the given reader's contents.
    ReaderParseError{ err: serde_json::Error },
    /// Failed to serialize the Config to the given writer.
    WriterWriteError{ err: serde_json::Error },

    /// Failed to read the config from a series of raw bytes.
    BytesParseError{ err: Box<Self> },
    /// Failed to serialzie the Config as a series of raw bytes.
    BytesWriteError{ err: Box<Self> },

    /// Failed to read the config from a String.
    StringParseError{ err: Box<Self> },
    /// Failed to convert the given bytes to a String.
    UnicodeError{ err: std::string::FromUtf8Error },
    /// Failed to write the config to a String.
    StringWriteError{ err: Box<Self> },

    /// Failed to create a given file.
    FileCreateError{ path: PathBuf, err: std::io::Error },
    /// Failed to open a given file.
    FileOpenError{ path: PathBuf, err: std::io::Error },
    /// Failed to read the config from a given file handle.
    FileParseError{ path: PathBuf, err: Box<Self> },
    /// Failed to write the config to the given file handle.
    FileWriteError{ path: PathBuf, err: Box<Self> },

    /// Non-absolute paths were found in the already-parsed config.
    RelativePathFound{ path: PathBuf, delinquint: PathBuf },
}

impl Display for Error {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use Error::*;
        match self {
            ReaderParseError{ err } => write!(f, "{}", err),
            WriterWriteError{ err } => write!(f, "{}", err),

            BytesParseError{ err }  => write!(f, "Could not parse a Config from the given bytes: {}", err),
            BytesWriteError{ err }  => write!(f, "Could not write the Config to raw bytes: {}", err),

            StringParseError{ err } => write!(f, "Could not parse a Config from the given string: {}", err),
            UnicodeError{ err }     => write!(f, "Could not decode bytes as UTF-8: {}", err),
            StringWriteError{ err } => write!(f, "Could not write the Config to a string: {}", err),

            FileCreateError{ path, err } => write!(f, "Could not create the given file '{}': {}", path.display(), err),
            FileOpenError{ path, err }   => write!(f, "Could not open the given file '{}': {}", path.display(), err),
            FileParseError{ path, err }  => write!(f, "Could not parse a Config from the given file '{}': {}", path.display(), err),
            FileWriteError{ path, err }  => write!(f, "Could not write the Config to the given file '{}': {}", path.display(), err),

            RelativePathFound{ path, delinquint } => write!(f, "Path '{}' in config file '{}' is not absolute", delinquint.display(), path.display()),
        }
    }
}

impl error::Error for Error {}





/***** LIBRARY *****/
/// Defines the parsed configuration file of the server, which is shared between the daemon and the CTL.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    /// The log level to apply.
    pub log_level   : LevelFilter,

    /// The socket path to listen for.
    pub socket_path : PathBuf,
    /// The address:port to listen on.
    pub listen_addr : String,
}

impl Config {
    /// Parse the Config from a given reader.
    /// 
    /// # General arguments
    /// - `R`: The type of the Read-implementing type given as the `reader`.
    /// 
    /// # Arguments
    ///  - `reader`: The Read-capable reader to parse.
    /// 
    /// # Returns
    /// A new instance of the Config struct, populated with values parsed from the given reader.
    /// 
    /// # Errors
    /// This function may error if the given reader could not be read from, contains invalid JSON, is missing certain fields or fields have illegal values.
    #[inline]
    pub fn from_reader<R: Read>(reader: R) -> Result<Self, Error> {
        // Simply call the serde parser
        match serde_json::from_reader(reader) {
            Ok(res)  => Ok(res),
            Err(err) => Err(Error::ReaderParseError{ err }),
        }
    }

    /// Parse the Config from raw bytes.
    /// 
    /// # General arguments
    /// - `B`: The type of the Bytes-like given as `value`.
    /// 
    /// # Arguments
    ///  - `value`: The Bytes(-like) to parse.
    /// 
    /// # Returns
    /// A new instance of the Config struct, populated with values parsed from the given array of Bytes.
    /// 
    /// # Errors
    /// This function may error if the given bytes does not parse as unicode, contains invalid JSON, is missing certain fields or fields have illegal values.
    #[inline]
    pub fn from_bytes<B: AsRef<[u8]>>(value: B) -> Result<Self, Error> {
        // Simply call the serde parser (by reading it as a reader)
        match Self::from_reader(value.as_ref()) {
            Ok(res)  => Ok(res),
            Err(err) => Err(Error::BytesParseError{ err: Box::new(err) }),
        }
    }

    /// Parse the Config from a string.
    /// 
    /// # General arguments
    /// - `S`: The type of the String-like given as `value`.
    /// 
    /// # Arguments
    ///  - `value`: The String(-like) value to parse.
    /// 
    /// # Returns
    /// A new instance of the Config struct, populated with values parsed from the given string.
    /// 
    /// # Errors
    /// This function may error if the given string is invalid JSON, is missing certain fields or fields have illegal values.
    #[inline]
    pub fn from_string<S: AsRef<str>>(value: S) -> Result<Self, Error> {
        // Simply call the serde parser (by reading it as bytes)
        match Self::from_bytes(value.as_ref()) {
            Ok(res)  => Ok(res),
            Err(err) => Err(Error::StringParseError{ err: Box::new(err) }),
        }
    }

    /// Parse the Config from a given file.
    /// 
    /// # General arguments
    /// - `P`: The type of the Path-like given as `path`.
    /// 
    /// # Arguments
    ///  - `path`: The Path(-like) of the file to parse.
    /// 
    /// # Returns
    /// A new instance of the Config struct, populated with values parsed from the given file.
    /// 
    /// # Errors
    /// This function may error if the given file could not be read, is invalid JSON, is missing certain fields or fields have illegal values.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        // Convert the path-like into a Path.
        let path: &Path = path.as_ref();

        // Open the file first
        let handle = match File::open(path) {
            Ok(handle) => handle,
            Err(err)   => { return Err(Error::FileOpenError{ path: path.into(), err }); }
        };

        // Wrap it in a buffered reader
        let handle = BufReader::new(handle);

        // Now we can simply call the serde parser
        match Self::from_reader(handle) {
            Ok(res)  => Ok(res),
            Err(err) => Err(Error::FileParseError{ path: path.into(), err: Box::new(err) }),
        }
    }



    /// Serializes the Config to the given writer.
    /// 
    /// # General arguments
    /// - `W`: The type of the Write-implementing type given as the `writer`.
    /// 
    /// # Arguments
    /// - `writer`: The Write-capable write to serialize to.
    /// - `pretty`: Whether or not to write in a human-readable fashion (`true`) or in a compact fashion (`false`).
    /// 
    /// # Returns
    /// Nothing directly, but does obviously cause the write to be updated with the serialized Config.
    /// 
    /// # Errors
    /// This function may error if the writer could not be written to or if the underlying Serde backend crashes.
    #[inline]
    pub fn to_writer<W: Write>(&self, writer: W, pretty: bool) -> Result<(), Error> {
        // Simply call serde, either pretty or no
        if pretty {
            match serde_json::to_writer_pretty(writer, self) {
                Ok(_)    => Ok(()),
                Err(err) => Err(Error::WriterWriteError{ err }),
            }
        } else {
            match serde_json::to_writer(writer, self) {
                Ok(_)    => Ok(()),
                Err(err) => Err(Error::WriterWriteError{ err }),
            }
        }
    }

    /// Writes the Config to an array of Bytes.
    /// 
    /// # Arguments
    /// - `pretty`: Whether or not to write in a human-readable fashion (`true`) or in a compact fashion (`false`).
    /// 
    /// # Returns
    /// A Vector of bytes, representing a serialized Config file.
    /// 
    /// # Errors
    /// This function may error if the underlying serde backend does.
    pub fn to_bytes(&self, pretty: bool) -> Result<Vec<u8>, Error> {
        // Spawn the array of bytes.
        let mut res: Vec<u8> = Vec::new();

        // Simply call the serde serializer.
        match self.to_writer(&mut res, pretty) {
            Ok(_)    => Ok(res),
            Err(err) => Err(Error::BytesWriteError{ err: Box::new(err) }),
        }
    }

    /// Writes the Config to a String.
    /// 
    /// Note that the resulting string will be quite compact; check `Config::to_string_pretty()` for a more human-readable result.
    /// 
    /// # Arguments
    /// - `pretty`: Whether or not to write in a human-readable fashion (`true`) or in a compact fashion (`false`).
    /// 
    /// # Returns
    /// A String, representing a serialized Config file.
    /// 
    /// # Errors
    /// This function may error if the underlying serde backend does.
    #[inline]
    pub fn to_string(&self, pretty: bool) -> Result<String, Error> {
        // Simply call the serde serializer.
        match self.to_bytes(pretty) {
            Ok(res)  => match String::from_utf8(res) {
                Ok(res)  => Ok(res),
                Err(err) => Err(Error::UnicodeError{ err }),
            },
            Err(err) => Err(Error::StringWriteError{ err: Box::new(err) }),
        }
    }

    /// Writes the Config to a file.
    /// 
    /// # Arguments
    /// - `path`: The Path(-like) location of where the file will be created.
    /// - `pretty`: Whether or not to write in a human-readable fashion (`true`) or in a compact fashion (`false`).
    /// 
    /// # Returns
    /// Nothing, but will generate a file that contains the Config.
    /// 
    /// # Errors
    /// This function may error if the file could not be written to or if the underlying serde backend does.
    #[inline]
    pub fn to_file<P: AsRef<Path>>(&self, path: P, pretty: bool) -> Result<(), Error> {
        // Convert the Path-like to a Path
        let path: &Path = path.as_ref();

        // Create the file
        let handle = match File::create(path) {
            Ok(handle) => handle,
            Err(err)   => { return Err(Error::FileCreateError{ path: path.into(), err }); }
        };

        // Wrap it in a buffered writer
        let handle = BufWriter::new(handle);

        // Write to it using the to_writer() function.
        match self.to_writer(handle, pretty) {
            Ok(res)  => Ok(res),
            Err(err) => Err(Error::FileWriteError{ path: path.into(), err: Box::new(err) }),
        }
    }



    /// Resolves the Config with information read from the CLI.
    /// 
    /// # Arguments
    /// - 
    /// 
    /// # Returns
    /// Nothing, but might alter the internal struct to the resolved values.
    /// 
    /// # Errors
    /// May error if any of the Paths already in the Config are relative.
    pub fn resolve(&mut self) -> Result<(), Error> {
        // Nothing for now
        Ok(())
    }
}

impl Default for Config {
    #[inline]
    fn default() -> Self {
        Self {
            log_level : LevelFilter::Debug,

            socket_path : PathBuf::from("/run/filehost/ctl.sock"),
            listen_addr : String::from("127.0.0.1:8719"),
        }
    }
}
