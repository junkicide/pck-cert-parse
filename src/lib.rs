use x509_parser::{
    certificate::X509Certificate, error::X509Error, pem, prelude::FromDer, public_key::PublicKey,
};

/// Convert a pem encoded certificate to a der encoded certificate
pub fn pem_to_der(input: &[u8]) -> Result<Vec<u8>, X509Error> {
    let (_data, pem) = pem::parse_x509_pem(input).map_err(|_| X509Error::Generic)?;
    Ok(pem.contents)
}

/// Parse a der encoded certificate to an X509Certificate struct
pub fn parse_der(input: &[u8]) -> Result<X509Certificate, X509Error> {
    let (_remaining, cert) = X509Certificate::from_der(input)?;
    Ok(cert)
}

/// Given an X509Certificate, get the subject public key, assuming it is ECDSA, and encoded it to
/// bytes
pub fn x509_to_subject_public_key(input: X509Certificate) -> Result<Vec<u8>, X509Error> {
    let public_key = input.tbs_certificate.subject_pki.parsed()?;
    match public_key {
        PublicKey::EC(ec_point) => Ok(ec_point.data().to_vec()),
        _ => Err(X509Error::Generic),
    }
}

/// Check that a given PCK certificate is signed with the public key from a given PCS
/// certificate which in our case should be from Intel
pub fn verify_pck(pck: &X509Certificate, pcs: &X509Certificate) -> bool {
    let issuer_public_key = pcs.public_key();
    pck.verify_signature(Some(issuer_public_key)).is_ok()
}

/// Verify a certificate chain: The signature of each certificate is verified against the public key of the previous certificate
pub fn verify_cert_chain(v: Vec<X509Certificate>) -> Result<(), X509Error> {
    let mut keys: Vec<usize> = (1..v.len()).collect();
        keys.push(v.len() - 1);
        let certs: Vec<usize> = (0..v.len()).collect();
        for (k, c) in core::iter::zip(keys, certs) {
            let _verify =  v[c].verify_signature(Some(v[k].public_key()))?;
        }
    Ok(())
}


