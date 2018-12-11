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

macro_rules! tag_enum_impl {
    ($enum_name:ident, $base_type:ident) => {
        tag_definition_impl!($enum_name);
        impl From<$base_type> for $enum_name {
            fn from(value: $base_type) -> $enum_name {
                assert!(std::mem::size_of::<$base_type>() == std::mem::size_of::<$enum_name>());
                unsafe { std::mem::transmute(value) }
            }
        }
        impl<> From<TagEnum<$base_type, $enum_name>> for $enum_name {
            fn from(value: TagEnum<$base_type, $enum_name>) -> $enum_name {
                assert!(std::mem::size_of::<$base_type>() == std::mem::size_of::<$enum_name>());
                unsafe { std::mem::transmute(value.0) }
            }
        }
        impl<> TagEnum<$base_type, $enum_name> {
            pub fn unwrap(self) -> $enum_name {
                self.into()
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
        #[repr($base_type:ident)]
        $enum_vis:vis enum $enum_name:ident {
            $($option_name:ident),*
        }
    ) => {
        #[allow(clippy::identity_op)]
        #[repr($base_type)]
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
        $enum_vis enum $enum_name {
            $($option_name,)*
        }
        
        tag_enum_impl!($enum_name, $base_type);

        impl TagEnumDefinition for $enum_name {
            type BaseType = $base_type;
            fn get_options() -> Vec<TagEnumOption<$base_type>> {
                vec![
                    $(
                        TagEnumOption {
                            name: stringify!($option_name),
                            value: unsafe { std::mem::transmute::<$enum_name, $base_type>($enum_name::$option_name) }
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
        #[flags, repr($base_type:ident)]
        $enum_vis:vis enum $enum_name:ident {
            $($option_name:ident = $option_value_expr:expr),*
        }
    ) => {
        #[allow(clippy::identity_op)]
        #[repr($base_type)]
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
        $enum_vis enum $enum_name {
            $($option_name = $option_value_expr as $base_type,)*
        }
        
        tag_enum_impl!($enum_name, $base_type);

        impl TagEnumDefinition for $enum_name {
            type BaseType = $base_type;
            fn get_options() -> Vec<TagEnumOption<$base_type>> {
                vec![
                    $(
                        TagEnumOption {
                            name: stringify!($option_name),
                            value: unsafe { std::mem::transmute::<$enum_name, $base_type>($enum_name::$option_name) }
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

    ($struct_vis:vis struct $struct_name:ident {
        $($field_vis:vis $field_name:ident: $field_type:ty),*
    }) => {
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

    ($struct_vis:vis struct $struct_name:ident : $base_type:ident {
        $($field_vis:vis $field_name:ident: $field_type:ty),*
    }) => {
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