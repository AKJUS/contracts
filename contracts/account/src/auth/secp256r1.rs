use crate::error::ContractResult;
use cosmwasm_std::Binary;
use p256::ecdsa::{signature::Verifier, Signature, VerifyingKey};

pub fn verify(tx_hash: &[u8], sig_bytes: &[u8], pubkey_bytes: &Binary) -> ContractResult<bool> {
    let verifying_key: VerifyingKey = VerifyingKey::from_sec1_bytes(pubkey_bytes.as_slice())?;

    let signature: Signature = Signature::from_bytes(sig_bytes.into())?;
    verifying_key.verify(tx_hash, &signature)?;

    Ok(true)
}

#[cfg(test)]
mod tests {
    use crate::auth::secp256r1::verify;
    use cosmwasm_std::Binary;
    use p256::ecdsa::{signature::Signer, Signature, SigningKey, VerifyingKey};

    #[test]
    fn test_verify_signature() {
        let key_serialized = "3ee21644150adb50dc4c20e330184fabf12e75ecbf31fe167885587e6ebf2255";
        let key_bytes = hex::decode(key_serialized).unwrap();
        let signing_key = SigningKey::from_bytes(key_bytes.as_slice().into()).unwrap();
        println!("signing key: {}", hex::encode(signing_key.to_bytes()));

        let test_value = "test_value".as_bytes();
        let signature: Signature = signing_key.sign(test_value);
        let signature_bytes = signature.to_bytes();
        println!("signature: {}", hex::encode(signature_bytes));

        let verifying_key = VerifyingKey::from(&signing_key);
        let verifying_key_bytes = verifying_key.to_sec1_bytes();
        let verifying_key_binary = Binary::from(verifying_key_bytes.to_vec());
        println!("verifying key: {}", hex::encode(verifying_key_bytes));

        assert!(verify(
            test_value,
            signature_bytes.as_slice(),
            &verifying_key_binary,
        )
        .unwrap());

        // test with invalid msg
        let bad_value = "invalid starting msg".as_bytes();
        let result = verify(bad_value, signature_bytes.as_slice(), &verifying_key_binary);
        assert!(result.is_err())
    }
}
