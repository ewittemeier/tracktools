use std::collections::HashMap;
use std::ops::Deref;

/// Human-readable representation of Gbx object types, as listed on the mania tech wiki (@13.08.2022),
/// which seems reasonably up to date (Still unsure about the validity of all this though).

struct GbxClassDict {
    map: HashMap<i32, GbxClassType>
}

impl GbxClassDict {
    fn new() -> GbxObjectDict {
        let mut map = HashMap::new();

        map.insert(0x03043000, GbxClassType::Challenge);
        map.insert(0x0305B000, GbxClassType::ChallengeParameters);
        map.insert(0x03059000, GbxClassType::BlockSkin);
        map.insert(0x0313B000, GbxClassType::WaypointSpecialProperty);
        map.insert(0x03101000, GbxClassType::AnchoredObject);
        map.insert(0x03093000, GbxClassType::ReplayRecord);
        map.insert(0x03092000, GbxClassType::CtnGhost);
        map.insert(0x0303F005, GbxClassType::Ghost);
        map.insert(0x03033000, GbxClassType::Collection);
        map.insert(0x0301A000, GbxClassType::Collector);
        map.insert(0x0301B000, GbxClassType::CollectorList);
        map.insert(0x0301C000, GbxClassType::ObjectInfo);
        map.insert(0x03038000, GbxClassType::Decoration);
        map.insert(0x03031000, GbxClassType::Skin);
        map.insert(0x0308C000, GbxClassType::PlayerProfile);
        map.insert(0x07010000, GbxClassType::EffectSimi);
        map.insert(0x0307A000, GbxClassType::MediaClipGroup);
        map.insert(0x03079000, GbxClassType::MediaClip);
        map.insert(0x03078000, GbxClassType::MediaTrack);
        map.insert(0x030A1000, GbxClassType::MediaBlockCameraPath);
        map.insert(0x030A2000, GbxClassType::MediaBlockCameraCustom);
        map.insert(0x030A4000, GbxClassType::MediaBlockCameraEffectShake);
        map.insert(0x030A5000, GbxClassType::MediaBlockImage);
        map.insert(0x030A6000, GbxClassType::MediaBlockMusicEffect);
        map.insert(0x030A7000, GbxClassType::MediaBlockSound);
        map.insert(0x030A8000, GbxClassType::MediaBlockText);
        map.insert(0x030A9000, GbxClassType::MediaBlockTrails);
        map.insert(0x030AB000, GbxClassType::MediaBlockTransitionFade);
        map.insert(0x03080000, GbxClassType::MediaBlockFxColors);
        map.insert(0x03081000, GbxClassType::MediaBlockFxBlurDepth);
        map.insert(0x03082000, GbxClassType::MediaBlockBlurMotion);
        map.insert(0x03083000, GbxClassType::MediaBlockFxBloom);
        map.insert(0x03084000, GbxClassType::CameraFree);
        map.insert(0x03085000, GbxClassType::MediaBlockTime);
        map.insert(0x03024000, GbxClassType::MediaBlock3dStereo);
        map.insert(0x03029000, GbxClassType::MediaBlockTriangles);
        map.insert(0x030E5000, GbxClassType::MediaBlockGhost);

        GbxObjectDict { map }
    }
}

impl Deref for GbxClassDict {
    type Target = HashMap<i32, GbxClassType>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}


enum GbxClassType {
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