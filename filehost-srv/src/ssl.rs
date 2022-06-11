/* SSL.rs
 *   by Lut99
 *
 * Created:
 *   11 Jun 2022, 13:30:22
 * Last edited:
 *   11 Jun 2022, 15:32:53
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Implements the part of the server that does SSL.
**/

use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;

use log::warn;
use rustls::{Certificate, KeyLogFile, PrivateKey, RootCertStore, ServerConfig};
use rustls::server::AllowAnyAnonymousOrAuthenticatedClient;

use filehost_spc::login::GUEST_ID;

pub use crate::errors::SSLError as Error;
use crate::users::Users;


/***** LIBRARY *****/
/// A struct that contains the SSL state configuration.
pub struct SSLConfig {
    /// The SSL server configuration.
    pub config : Arc<ServerConfig>,
}

impl SSLConfig {
    /// Constructor for the SSLConfig.
    /// 
    /// # Arguments
    /// - `users`: List of users to load keys for.
    pub fn new<P1: AsRef<Path>, P2: AsRef<Path>>(server_cert: P1, server_key: P2, users: &Users) -> Result<Self, Error> {
        // Convert the Path-likea into Paths
        let server_cert: &Path = server_cert.as_ref();
        let server_key: &Path  = server_key.as_ref();

        // Load the server certificate(s)
        let handle = match File::open(server_cert) {
            Ok(handle) => BufReader::new(handle),
            Err(err)   => { return Err(Error::CertOpenError{ path: server_cert.into(), err }); }
        };
        let server_certs: Vec<Certificate> = match rustls_pemfile::certs(&mut BufReader::new(handle)) {
            Ok(certs) => certs.into_iter().map(|v| Certificate(v)).collect(),
            Err(err)  => { return Err(Error::CertParseError{ path: server_cert.display().to_string(), err }); }
        };
        if server_certs.is_empty() { warn!("Server certificate file '{}' is empty", server_cert.display()); }

        // Load the server private key
        let mut handle = match File::open(server_key) {
            Ok(handle) => BufReader::new(handle),
            Err(err)   => { return Err(Error::KeyOpenError{ path: server_key.into(), err }); }
        };
        let server_key: PrivateKey = loop { match rustls_pemfile::read_one(&mut handle) {
            Ok(key)  => match key {
                Some(rustls_pemfile::Item::RSAKey(key))   => { break PrivateKey(key); },
                Some(rustls_pemfile::Item::PKCS8Key(key)) => { break PrivateKey(key); },
                Some(rustls_pemfile::Item::ECKey(key))    => { break PrivateKey(key); },
                None                                      => { return Err(Error::NoKeysFound{ path: server_key.into() }); },
                _                                         => { continue; },
            },
            Err(err) => { return Err(Error::KeyParseError{ path: server_key.into(), err }); }
        } };

        // Now, load the client public keys / certificates
        let mut user_roots: RootCertStore = RootCertStore::empty();
        for (_, user) in &users.users {
            // Skip if the guest user (no certificate)
            if user.id == GUEST_ID { continue; }

            // Open the file referenced
            let mut handle = match File::open(&user.certs) {
                Ok(handle) => BufReader::new(handle),
                Err(err)   => { return Err(Error::CertOpenError{ path: user.certs.clone(), err }); }
            };

            // Try to load the certificates for this user
            let certs: Vec<Certificate> = match rustls_pemfile::certs(&mut handle) {
                Ok(certs) => certs.into_iter().map(|v| Certificate(v)).collect(),
                Err(err)  => { return Err(Error::CertParseError{ path: user.certs.display().to_string(), err }); }
            };

            // Add them all to the store, then move to the next
            for cert in certs {
                if let Err(err) = user_roots.add(&cert) { return Err(Error::CertAddError{ err }); };
            }
        }
        let user_roots = AllowAnyAnonymousOrAuthenticatedClient::new(user_roots);

        // Create the config
        let mut config: ServerConfig = match ServerConfig::builder()
            .with_safe_default_cipher_suites()
            .with_safe_default_kx_groups()
            .with_safe_default_protocol_versions()
            .expect("Inconsistent default cipher-suites & versions; this should never happen!")
            .with_client_cert_verifier(user_roots)
            .with_single_cert(server_certs, server_key)
        {
            Ok(config) => config,
            Err(err)   => { return Err(Error::ConfigError{ err }); }
        };
        // Assign a keylogfile if it all goes nicely
        config.key_log = Arc::new(KeyLogFile::new());

        // Done! Wrap that in ourselves
        Ok(Self {
            config: Arc::new(config),
        })
    }
}
