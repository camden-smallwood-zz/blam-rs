pub trait TagDefinition {
    fn get_size() -> usize;
}

macro_rules! tag_definition_impl {
    ($type:ident) => {
        impl TagDefinition for $type {
            fn get_size() -> usize {
                std::mem::size_of::<$type>()
            }
        }
    }
}

macro_rules! tag_field_impl {
    ($field_type:ty) => {
        TagField::Undefined
    }
}

#[macro_export]
macro_rules! tag_definition {
    (
        $enum_vis:vis enum $enum_name:ident {
            $($option_name:ident $(= $option_value:expr)*),*
        }
    ) => {
        #[repr(C)]
        #[allow(clippy::identity_op)]
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
        $enum_vis enum $enum_name {
            $($option_name $(= $option_value)*,)*
        }
        tag_definition_impl!($enum_name);
        impl TagEnumDefinition for $enum_name {
            fn get_options() -> Vec<TagEnumOption> {
                vec![
                    $(
                        TagEnumOption {
                            name: stringify!($enum_name),
                            value: $enum_name::$option_name as isize
                        },
                    )*
                ]
            }
        }
    };

    (
        #[repr(flags)]
        $enum_vis:vis enum $enum_name:ident {
            $($option_name:ident = 1 << $bit_index_expr:expr),*
        }
    ) => {
        #[repr(C)]
        #[allow(clippy::identity_op)]
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
        $enum_vis enum $enum_name {
            $($option_name = 1 << $bit_index_expr,)*
        }
        tag_definition_impl!($enum_name);
        impl TagFlagsDefinition for $enum_name {
            fn get_bits() -> Vec<TagFlagsBit> {
                vec![
                    $(
                        TagFlagsBit {
                            name: stringify!($option_name),
                            value: 1 << $bit_index_expr
                        },
                    )*
                ]
            }
        }
        impl std::fmt::Display for $enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", match *self { $($enum_name::$option_name => stringify!($option_name),)* })
            }
        }
    };

    (
        $struct_vis:vis struct $struct_name:ident {
            $($field_vis:vis $field_name:ident: $field_type:ty),*
        }
    ) => {
        #[repr(C, packed)]
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
        $struct_vis struct $struct_name {
            $($field_vis $field_name: $field_type,)*
        }

        tag_definition_impl!($struct_name);

        impl TagStructDefinition for $struct_name {
            fn get_fields() -> Vec<TagFieldInfo> {
                vec![
                    $(
                        TagFieldInfo {
                            name: stringify!($field_name),
                            field: tag_field_impl!($field_type)
                        },
                    )*
                ]
            }
        }
    };

    (
        $struct_vis:vis struct $struct_name:ident : $base_type:ident {
            $($field_vis:vis $field_name:ident: $field_type:ty),*
        }
    ) => {
        #[repr(C, packed)]
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
        $struct_vis struct $struct_name {
            pub base: $base_type,
            $($field_vis $field_name: $field_type,)*
        }
        
        tag_definition_impl!($struct_name);

        impl TagStructDefinition for $struct_name {
            fn get_fields() -> Vec<TagFieldInfo> {
                vec![
                    $(
                        TagFieldInfo {
                            name: stringify!($field_name),
                            field: tag_field_impl!($field_type)
                        },
                    )*
                ]
            }
        }
    };

    (
        #[group_name = $group_name_expr:expr, group_tag = $group_tag_expr:expr]
        $struct_vis:vis struct $struct_name:ident {
            $($field_vis:vis $field_name:ident: $field_type:ty),*
        }
    ) => {
        #[repr(C, packed)]
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
        $struct_vis struct $struct_name {
            $($field_vis $field_name: $field_type,)*
        }
        
        tag_definition_impl!($struct_name);
        
        impl TagStructDefinition for $struct_name {
            fn get_fields() -> Vec<TagFieldInfo> {
                vec![
                    $(
                        TagFieldInfo {
                            name: stringify!($field_name),
                            field: tag_field_impl!($field_type)
                        },
                    )*
                ]
            }
        }

        impl TagGroupDefinition for $struct_name {
            fn get_group_name() -> &'static str { $group_name_expr }
            fn get_group_tag() -> Tag { Tag::from($group_tag_expr) }
        }
    };
    
    (
        #[group_name = $group_name_expr:expr, group_tag = $group_tag_expr:expr]
        $struct_vis:vis struct $struct_name:ident : $base_type:ident {
            $($field_vis:vis $field_name:ident: $field_type:ty),*
        }
    ) => {
        #[repr(C, packed)]
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
        $struct_vis struct $struct_name {
            pub base: $base_type,
            $($field_vis $field_name: $field_type,)*
        }
        
        tag_definition_impl!($struct_name);
        
        impl TagStructDefinition for $struct_name {
            fn get_fields() -> Vec<TagFieldInfo> {
                vec![
                    $(
                        TagFieldInfo {
                            name: stringify!($field_name),
                            field: tag_field_impl!($field_type)
                        },
                    )*
                ]
            }
        }

        impl TagGroupDefinition for $struct_name {
            fn get_group_name() -> &'static str { $group_name_expr }
            fn get_group_tag() -> Tag { Tag::from($group_tag_expr) }
        }
    };
    
}