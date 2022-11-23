use bevy::prelude::*;
use crate::prelude::*;

/// ![](../assets/objects_nm.png).
#[derive(Debug, Clone, Reflect)]
pub enum GraphicsType {
    // ◢
    Slope,
    Block,
    BallSmall,
    // ◴
    BallBig,
    Pop,
    Unknown1,
    UiLife,
    Deflate,
    // ◢
    BouncySlope,
    BouncyBlock,
    // Teleport ?
    Unknown2,
    // Teleport ?
    Unknown3,
    Thorn,
    // ◰
    Spike,
    // ◰
    Exit,
    Bubble,
    PickUp,
    UiRing,
    Inflate,
    // ↓
    Respawn,
    // ↽
    HoopBig,
    HoopSmall,
    HoopBigGrayed,
    HoopSmallGrayed,
}
