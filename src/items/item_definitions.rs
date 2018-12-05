use crate::{math::*, objects::*, tags::*, text::*};

tag_definition! {
    #[flags, repr(i32)]
    pub enum ItemDefinitionFlags {
        AlwaysMaintainsZUp = 1 << 0,
        DestroyedByExplosions = 1 << 1,
        UnaffectedByGravity = 1 << 2
    }
}

tag_definition! {
    pub struct ItemPredictedBitmap {
        pub bitmap: TagReference
    }
}

tag_definition! {
    #[group_name = "item", group_tag = "item"]
    pub struct ItemDefinition : ObjectDefinition {
        pub item_flags: TagEnum<i32, ItemDefinitionFlags>,
        pub old_message_index: i16,
        pub sort_order: i16,
        pub old_multiplayer_on_ground_scale: f32,
        pub old_campaign_on_ground_scale: f32,
        pub pickup_message: StringId,
        pub swap_message: StringId,
        pub pickup_or_dual_wield_message: StringId,
        pub swap_or_dual_wield_message: StringId,
        pub picked_up_message: StringId,
        pub switch_to_message: StringId,
        pub switch_to_from_ai_message: StringId,
        pub all_weapons_empty_message: StringId,
        pub collision_sound: TagReference,
        pub predicted_bitmaps: TagBlock<ItemPredictedBitmap>,
        pub detonation_damage_effect: TagReference,
        pub detonation_delay_bounds: Bounds<f32>,
        pub detonating_effect: TagReference,
        pub detonation_effect: TagReference,
        pub campaign_ground_scale: f32,
        pub multiplayer_ground_scale: f32,
        pub small_hold_scale: f32,
        pub small_holster_scale: f32,
        pub medium_hold_scale: f32,
        pub medium_holster_scale: f32,
        pub large_hold_scale: f32,
        pub large_holster_scale: f32,
        pub huge_hold_scale: f32,
        pub huge_holster_scale: f32,
        pub grounded_friction_length: f32,
        pub grounded_friction_unknown: f32
    }
}