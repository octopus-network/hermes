use ic_agent::identity::PemError;
use std::path::PathBuf;

pub(crate) fn create_identity(pem_file: &PathBuf) -> Result<impl ic_agent::Identity, PemError> {
    ic_agent::identity::BasicIdentity::from_pem_file(pem_file)
}
