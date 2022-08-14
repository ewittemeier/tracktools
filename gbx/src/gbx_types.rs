use std::collections::HashMap;
use std::ops::Deref;

/// Human-readable representation of Gbx object types, as listed on the mania tech wiki (@13.08.2022),
/// which seems reasonably up to date (Still unsure about the validity of all this though).

struct GbxFileDict {
    map: HashMap<i32, GbxFileType>,
}

impl GbxFileDict {
    fn new() -> GbxFileDict {
        let mut map = HashMap::new();

        map.insert(0x03043000, GbxFileType::Challenge);
        map.insert(0x0305B000, GbxFileType::ChallengeParameters);
        map.insert(0x03059000, GbxFileType::BlockSkin);
        map.insert(0x0313B000, GbxFileType::WaypointSpecialProperty);
        map.insert(0x03101000, GbxFileType::AnchoredObject);
        map.insert(0x03093000, GbxFileType::ReplayRecord);
        map.insert(0x03092000, GbxFileType::CtnGhost);
        map.insert(0x0303F005, GbxFileType::Ghost);
        map.insert(0x03033000, GbxFileType::Collection);
        map.insert(0x0301A000, GbxFileType::Collector);
        map.insert(0x0301B000, GbxFileType::CollectorList);
        map.insert(0x0301C000, GbxFileType::ObjectInfo);
        map.insert(0x03038000, GbxFileType::Decoration);
        map.insert(0x03031000, GbxFileType::Skin);
        map.insert(0x0308C000, GbxFileType::PlayerProfile);
        map.insert(0x07010000, GbxFileType::EffectSimi);
        map.insert(0x0307A000, GbxFileType::MediaClipGroup);
        map.insert(0x03079000, GbxFileType::MediaClip);
        map.insert(0x03078000, GbxFileType::MediaTrack);
        map.insert(0x030A1000, GbxFileType::MediaBlockCameraPath);
        map.insert(0x030A2000, GbxFileType::MediaBlockCameraCustom);
        map.insert(0x030A4000, GbxFileType::MediaBlockCameraEffectShake);
        map.insert(0x030A5000, GbxFileType::MediaBlockImage);
        map.insert(0x030A6000, GbxFileType::MediaBlockMusicEffect);
        map.insert(0x030A7000, GbxFileType::MediaBlockSound);
        map.insert(0x030A8000, GbxFileType::MediaBlockText);
        map.insert(0x030A9000, GbxFileType::MediaBlockTrails);
        map.insert(0x030AB000, GbxFileType::MediaBlockTransitionFade);
        map.insert(0x03080000, GbxFileType::MediaBlockFxColors);
        map.insert(0x03081000, GbxFileType::MediaBlockFxBlurDepth);
        map.insert(0x03082000, GbxFileType::MediaBlockBlurMotion);
        map.insert(0x03083000, GbxFileType::MediaBlockFxBloom);
        map.insert(0x03084000, GbxFileType::CameraFree);
        map.insert(0x03085000, GbxFileType::MediaBlockTime);
        map.insert(0x03024000, GbxFileType::MediaBlock3dStereo);
        map.insert(0x03029000, GbxFileType::MediaBlockTriangles);
        map.insert(0x030E5000, GbxFileType::MediaBlockGhost);

        GbxFileDict { map }
    }
}

impl Deref for GbxFileDict {
    type Target = HashMap<i32, GbxFileType>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}


enum GbxFileType {
    Challenge,
    ChallengeParameters,
    BlockSkin,
    WaypointSpecialProperty,
    AnchoredObject,
    ReplayRecord,
    CtnGhost,
    Ghost,
    Collection,
    Collector,
    CollectorList,
    ObjectInfo,
    Decoration,
    Skin,
    PlayerProfile,
    EffectSimi,
    MediaClipGroup,
    MediaClip,
    MediaTrack,
    MediaBlockCameraPath,
    MediaBlockCameraCustom,
    MediaBlockCameraEffectShake,
    MediaBlockImage,
    MediaBlockMusicEffect,
    MediaBlockSound,
    MediaBlockText,
    MediaBlockTrails,
    MediaBlockTransitionFade,
    MediaBlockFxColors,
    MediaBlockFxBlurDepth,
    MediaBlockBlurMotion,
    MediaBlockFxBloom,
    CameraFree,
    MediaBlockTime,
    MediaBlock3dStereo,
    MediaBlockTriangles,
    MediaBlockGhost,
}