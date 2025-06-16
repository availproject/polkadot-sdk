use sp_runtime::OpaqueExtrinsic;
use codec::{Compact, Decode, Encode, Input};
use sp_core::blake2_256;
use super::LOG_TARGET;

// const ORIGINAL_PALLET_INDEX: u8 = 0x00;
// const ORIGINAL_CALL_INDEX: u8 = 0x07;
// const LIGHT_CALL_INDEX: u8 = 0x0C;
const ORIGINAL_PALLET_INDEX: u8 = 0x1d; // DA pallet index
const ORIGINAL_CALL_INDEX: u8 = 0x05;
const LIGHT_CALL_INDEX: u8 = 0x06;

/// Convert `remark_with_event` extrinsic to `remark_with_event_light` format.
pub fn convert_da_to_light(original: &OpaqueExtrinsic) -> Option<OpaqueExtrinsic> {
    log::debug!(target: LOG_TARGET, "Converting DA to its light version");

    let encoded = original.encode();
    let mut input = &encoded[..];

    // === Decode length prefix (ignored, will recompute later) ===
    let _: Compact<u32> = Decode::decode(&mut input).ok()?;

    // === Decode version byte ===
    let version: u8 = Decode::decode(&mut input).ok()?;
    if version & 0b1000_0000 == 0 {
        // Only signed extrinsics are supported
        return None;
    }

    let mut payload = Vec::new();
    payload.push(version);

    // === Signer ===
    let signer_type = input.read_byte().ok()?;
    payload.push(signer_type);

    // AccountId32
    let signer = &input[..32];
    payload.extend_from_slice(signer);
    input = &input[32..];

    // Signature
    let sig_type = input.read_byte().ok()?;
    payload.push(sig_type);

    let signature = &input[..64];
    payload.extend_from_slice(signature);
    input = &input[64..];

    // Era
    let era_first = input.read_byte().ok()?;
    payload.push(era_first);
    if era_first != 0 {
        let era_second = input.read_byte().ok()?;
        payload.push(era_second);
    }

    // Nonce and Tip
    let nonce = Compact::<u32>::decode(&mut input).ok()?;
    nonce.encode_to(&mut payload);

    let tip = Compact::<u128>::decode(&mut input).ok()?;
    tip.encode_to(&mut payload);

    // App ID
    let app_id = Compact::<u32>::decode(&mut input).ok()?;
    app_id.encode_to(&mut payload);

    // Call index
    let pallet_index = input.read_byte().ok()?;
    let call_index = input.read_byte().ok()?;
    if (pallet_index, call_index) != (ORIGINAL_PALLET_INDEX, ORIGINAL_CALL_INDEX) {
        return None;
    }

    // === Decode call args ===
    let data: Vec<u8> = Decode::decode(&mut input).ok()?;
    let data_hash = blake2_256(&data);
    log::info!(target: LOG_TARGET, "Data hash: {:?}", data_hash);

    // === Construct new call ===
    payload.push(ORIGINAL_PALLET_INDEX); // Pallet index for DA
    payload.push(LIGHT_CALL_INDEX); // Call index for `light` DA

    // remark.encode_to(&mut payload);
    data_hash.encode_to(&mut payload);

    // === Encode length prefix ===
    let mut out = Vec::new();
    let len = payload.len() as u32;
    Compact(len).encode_to(&mut out);
    out.extend(payload);

    // Final decoding validation
    match OpaqueExtrinsic::from_bytes(&out[..]) {
        Ok(opaque) => Some(opaque),
        Err(e) => {
            log::error!(target: LOG_TARGET, "Failed to convert to light extrinsic: {:?}", e);
            None
        }
    }
}

fn skip_multiaddress(input: &mut &[u8]) -> Option<()> {
    let id = input.read_byte().ok()?; // enum tag
    match id {
        0 | 1 | 2 | 3 => {
            // AccountId32
            *input = &input[32..];
            Some(())
        }
        _ => None,
    }
}

fn skip_multisignature(input: &mut &[u8]) -> Option<()> {
    let sig_type = input.read_byte().ok()?; // enum tag
    match sig_type {
        0 | 1 | 2 => {
            // Ed25519, Sr25519, or ECDSA: 64 bytes
            *input = &input[64..];
            Some(())
        }
        _ => None,
    }
}

fn skip_era(input: &mut &[u8]) -> Option<()> {
    // Era is 1 or 2 bytes
    let first = input.read_byte().ok()?;
    if first == 0 {
        // immortal era
        Some(())
    } else {
        // mortal era is 2 bytes
        input.read_byte().ok()?;
        Some(())
    }
}

fn extract_dispatch_indices(ext: &OpaqueExtrinsic) -> Option<(u8, u8)> {
    let encoded = ext.encode();
    let mut input = &encoded[..];

    // length prefix
    let _: Compact<u32> = Decode::decode(&mut input).ok()?;

    // Version byte
    let version: u8 = Decode::decode(&mut input).ok()?;
    let is_signed = version & 0b1000_0000 != 0;

    if is_signed {
        skip_multiaddress(&mut input)?;         // signer: MultiAddress
        skip_multisignature(&mut input)?;       // signature: MultiSignature
        skip_era(&mut input)?;                  // era
        let _: Compact<u32> = Decode::decode(&mut input).ok()?;   // nonce
        let _: Compact<u128> = Decode::decode(&mut input).ok()?;  // tip
    }

    let _: Compact<u32> = Decode::decode(&mut input).ok()?; // app id

    // Decode module and call index
    let module_index = input.read_byte().ok()?;
    let call_index = input.read_byte().ok()?;

    Some((module_index, call_index))
}

/// Check if the extrinsic is a DA extrinsic.
pub fn is_da_extrinsic(ext: &OpaqueExtrinsic) -> bool {
    matches!(extract_dispatch_indices(ext), Some((ORIGINAL_PALLET_INDEX, ORIGINAL_CALL_INDEX)))
}

/// Check if the extrinsic is either DA or its light version.
pub fn is_any_da(ext: &OpaqueExtrinsic) -> bool {
    matches!(
        extract_dispatch_indices(ext),
        Some((ORIGINAL_PALLET_INDEX, ORIGINAL_CALL_INDEX))
            | Some((ORIGINAL_PALLET_INDEX, LIGHT_CALL_INDEX))
    )
}

#[test]
fn test_extract_dispatch_indices() {
    use hex_literal::hex;

    let raw = hex!("891884000897752a46ee0d36e339152b3fe17852d77e736b5bc8dfbc1ac67058e3cc0b18016c6ab84f9b7b3b9c372ba89dcc6029c82eaa3155ed3b1a99f1b603385284190c690c5869ddf2bc271ab1391fe1d62bca6d7f1472f80c0d6989cff70ec02a7d8ec401ed8200b1061d01d1160004a751a79f697510c614bcf8297e671700000000059c78da8cd2f937d40b0306f0c6161953539689cc9be5d8334c1844254b78a738a55244484d5e644bc66078c7d89becfbf2e53208a7519631d72c076da3d44d73b3d4b56764edd09846ea1ef77efbfdfb0f7ccef39ce7617aec12e381fd3986589f114cd0cf4c9265df0ae3e8cfd97770c4a0ad14f243366b75b9588342a6ad03a358dec465f30755447c8624c4c55424304dbcfda954b2fbdbebc0e03174a220956024bec1fb174c671428c6228d54e10a869165bd53741df15d7b6385a29ccf6d163772749e1121830410b46ceeeb53cfebbd2a87322a5c4cf6c2e29b60e68ad174be4169de2abc86c5850c868060bf02a7e211ee486dac074223a40afdd95c56ebc7a31917747d49ba5fe268f643c860e8afca73243922d96b6ec16d0f1de7753a406e40f7f9d36387c3822bb2a65c5564372083e120e87b8891b004f0ec7afb01719cf1c194d62fad6c6e04d1dec81363aa91a9594621d33680d94e9f3c54c70164a799e5479ec0e165fa8d88be41e48b3f1d288b367169a54d3b600408b6293d3dd4a8a76293ee8e32747d35548b5aca650d2ae3cbfdf72467740dbb684306a34070e13f083cb6647dfe399cbcc95d5db111297b47b3f5637a52e27d83dfaf4d5c870cc68060c0312736330dfebf799cabaadb65528a82aee7067beb5a96bc5fa16a85bff41a64301604fb323339f658eba91f258539f601e7f235ca081657c631e1d2a371ebea236662c8601c0822eb295d0a2fa2e3d9f2166dd3ee06590d5671e2ad4ffcb36fd59d7f0886fe8a870cc683605a55fde61987c734671dabad6c2f1b8d3b3d23e353fac293f6f04b7db3fb147646f90a7c2fbf8f0946addc36a3ea7a374eebcfee7a28b136dc673b41d632e354159d1ddd011340b03e83c487fdacaee42b94749cf37bbda12db8e2d756b73fe74eaaaa93e65caf266430090459e1882ebd5b3020f4c018d23dd1e9dc5316d5afdd9c7b6fa14df84436548b00194c06c1f95c03a98941e33cddabd5ee8dfc95fd4415e4791acfc274b5148b7f34ae8d820cfe1f040f26cbf72925d7d0570d2ea76b8f3676c478d73a8535d87d05e0fc532341c654c8602a086e094f6b9e272ddd73da706c54765ab34a7031052abe5df72f38fec6ce5caa8601194c03419421b62b3b13018b6cf7ac1576dc7a237226de2de7f80cbbc773ca618cee310a992602b66d9f74f2ce605470bd1216caaa3428b3456d38101da3942af1d743a2575a77c00c10ec68a9616fe2a50fc13cadf99d4a0ed8045ab2224ba2fa584df93c8161f26019329805826119d982edf747fd23dc1a33d426037d29bae6e1aef602ef01455271cabc5e14643007040781fb1eb2293419224278f5c346da91eec9d3f4b2535b3ab00bb8a5fae3568b90411a08e279f09332fe0d380dc77c0f7ee6b46abda18838403799b42a43bb9fa8e3222183b920d89c5028df6b2163edb8bb3f4809367d3dfcad7652cb4a836d36d57f7a9539d50e19cc0741e1e098a3d6448fab97b70fc260f77f2fcedc1b47157a71f282a9eb4c9ff00bcd14326d1368727f79933bc46fdecbf4aa668432eed4aac666d3237d0a7c277ba21b2aacfeb94d210886922f7a18e8275eb373725eeef066d6e4a2e8b7d50dcd73dc48b1a47d2f95ba2183c520e878ac39acda3359ad5822d31e25482a488a7e55c4e66f274c1f34a92c21ebbc820c968220e7668dbe68795c88636a5aff66b9b0704262f369984ba86ba8245bea64714590c1f25f09014eb780562e6c314574ce0882a242d73fd6e1b8f379f2715f4e7cff7d510e32580982ba81e8a23f847275d4f28d0784c5fcfed6ef18eafba1871155fa4bd226335b3390c16a107c3cb516fd8e8d39de1372840500f4c06571401095f0dcafca2469ee8d75401a854c130389d8773d662cb4e4598b8d2243310d98b85f352e1fbb373e7f88c40ced871dde0101103c6923a57bd15a0b392096fd62c9cb9d2178c6bd48df261847c60f5d4a97c2344106eb787f070000ffffe1c6ffa501").as_ref();

    let opaque = OpaqueExtrinsic::from_bytes(raw).unwrap();
    let indices = extract_dispatch_indices(&opaque);

    println!("Indices: {:?}", indices);
    assert_eq!(indices, Some((ORIGINAL_PALLET_INDEX, ORIGINAL_CALL_INDEX)));
    assert!(is_da_extrinsic(&opaque));
}
