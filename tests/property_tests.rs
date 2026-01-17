use photon_core::{encode_data, decode_data};
use proptest::prelude::*;

proptest! {
    // Fuzz test for the codec: Arbitrary byte vectors should round-trip correctly (without noise)
    #[test]
    fn test_codec_roundtrip_noiseless(data in proptest::collection::vec(any::<u8>(), 0..1000)) {
        let voxels = encode_data(&data);
        let decoded = decode_data(&voxels, false);
        
        // The current codec is byte-aligned (1 byte -> 1 voxel), so lengths should match exactly.
        prop_assert_eq!(data, decoded, "Round-trip failed");
    }
}
