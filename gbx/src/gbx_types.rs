/// Human-readable representation of Gbx object types, as listed on the mania tech wiki (@13.08.2022),
/// which seems reasonably up to date (Still unsure about the validity of all this though).
enum GbxObjectType {
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

impl TryFrom<i32> for GbxObjectType {
    type Error = GbxTypeParsingError;

/// Parsing from raw u32 as usually given in Gbx files in hexadecimal notation to human-readable
/// representation.
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0x03043000 => Ok(GbxObjectType::Challenge),
            0x0305B000 => Ok(GbxObjectType::ChallengeParameters),
            0x03059000 => Ok(GbxObjectType::BlockSkin),
            0x0313B000 => Ok(GbxObjectType::WaypointSpecialProperty),
            0x03101000 => Ok(GbxObjectType::AnchoredObject),
            0x03093000 => Ok(GbxObjectType::ReplayRecord),
            0x03092000 => Ok(GbxObjectType::CtnGhost),
            0x0303F005 => Ok(GbxObjectType::Ghost),
            0x03033000 => Ok(GbxObjectType::Collection),
            0x0301A000 => Ok(GbxObjectType::Collector),
            0x0301B000 => Ok(GbxObjectType::CollectorList),
            0x0301C000 => Ok(GbxObjectType::ObjectInfo),
            0x03038000 => Ok(GbxObjectType::Decoration),
            0x03031000 => Ok(GbxObjectType::Skin),
            0x0308C000 => Ok(GbxObjectType::PlayerProfile),
            0x07010000 => Ok(GbxObjectType::EffectSimi),
            0x0307A000 => Ok(GbxObjectType::MediaClipGroup),
            0x03079000 => Ok(GbxObjectType::MediaClip),
            0x03078000 => Ok(GbxObjectType::MediaTrack),
            0x030A1000 => Ok(GbxObjectType::MediaBlockCameraPath),
            0x030A2000 => Ok(GbxObjectType::MediaBlockCameraCustom),
            0x030A4000 => Ok(GbxObjectType::MediaBlockCameraEffectShake),
            0x030A5000 => Ok(GbxObjectType::MediaBlockImage),
            0x030A6000 => Ok(GbxObjectType::MediaBlockMusicEffect),
            0x030A7000 => Ok(GbxObjectType::MediaBlockSound),
            0x030A8000 => Ok(GbxObjectType::MediaBlockText),
            0x030A9000 => Ok(GbxObjectType::MediaBlockTrails),
            0x030AB000 => Ok(GbxObjectType::MediaBlockTransitionFade),
            0x03080000 => Ok(GbxObjectType::MediaBlockFxColors),
            0x03081000 => Ok(GbxObjectType::MediaBlockFxBlurDepth),
            0x03082000 => Ok(GbxObjectType::MediaBlockBlurMotion),
            0x03083000 => Ok(GbxObjectType::MediaBlockFxBloom),
            0x03084000 => Ok(GbxObjectType::CameraFree),
            0x03085000 => Ok(GbxObjectType::MediaBlockTime),
            0x03024000 => Ok(GbxObjectType::MediaBlock3dStereo),
            0x03029000 => Ok(GbxObjectType::MediaBlockTriangles),
            0x030E5000 => Ok(GbxObjectType::MediaBlockGhost),
            _ => Err(GbxTypeParsingError { value })
        }
    }
}

struct GbxTypeParsingError {
    value: i32,
}