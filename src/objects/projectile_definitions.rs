use crate::tags::*;

tag_definition! {
    #[flags, repr(i32)]
    pub enum ProjectileDefinitionFlags {
        OrientedAlongVelocity = 1,
        AiMustUseBallisticAiming = 2,
        DetonationMaxTimeIfAttached = 4,
        HasSuperCombiningExplosion = 8,
        DamageScalesBasedOnDistance = 16,
        TravelsInstantaneously = 32,
        SteeringAdjustsOrientation = 64,
        DoNotNoiseUpSteering = 128,
        CanTrackBehindItself = 256,
        RobotronSteering = 512,
        FasterWhenOwnedByPlayer = 1024
    }
}

tag_definition! {
    #[repr(i16)]
    pub enum ProjectileDetonationTimerStart {
        Immediately,
        AfterFirstBounce,
        WhenAtRest,
        AfterFirstBounceOffAnySurface
    }
}

tag_definition! {
    #[repr(i16)]
    pub enum ProjectileNoiseLevel {
        Silent,
        Medium,
        Loud,
        Shout,
        Quiet
    }
}

