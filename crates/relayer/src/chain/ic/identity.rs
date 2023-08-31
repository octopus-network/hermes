use ic_agent::identity::PemError;
use ic_agent::identity::Secp256k1Identity;
use std::path::PathBuf;

pub(crate) fn create_identity(pem_file: &PathBuf) -> Result<Secp256k1Identity, PemError> {
    Secp256k1Identity::from_pem_file(pem_file)
}

#[test]
fn test_create_identity() {
    let pem_file = PathBuf::from("/Users/davirain/.config/dfx/identity/default/identity.pem");
    let ret = create_identity(&pem_file);
    println!("ret = {:?}", ret);
}
