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
        impl TagEnumDefinition for $enum_name {
            type BaseType = $base_type;
        }
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
            $($item_name:ident),*
        }
    ) => {
        #[repr($base_type)]
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
        $enum_vis enum $enum_name {
            $($item_name,)*
        }
        tag_enum_impl!($enum_name, $base_type);
        impl std::fmt::Display for $enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", match *self { $($enum_name::$item_name => stringify!($item_name),)* })
            }
        }
    };

    (
        #[flags, repr($base_type:ident)]
        $enum_vis:vis enum $enum_name:ident {
            $($item_name:ident = $item_value_expr:expr),*
        }
    ) => {
        #[repr($base_type)]
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
        $enum_vis enum $enum_name {
            $($item_name = $item_value_expr as $base_type,)*
        }
        tag_enum_impl!($enum_name, $base_type);
        impl std::fmt::Display for $enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", match *self { $($enum_name::$item_name => stringify!($item_name),)* })
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
                            name: stringify!($field_name).to_string(),
                            field: tag_field_impl!($field_type)
                        },
                    )*]
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
                            name: stringify!($field_name).to_string(),
                            field: tag_field_impl!($field_type)
                        },
                    )*]
            }
        }

        impl TagGroupDefinition for $struct_name {
            fn get_group_name() -> String { $group_name_expr.to_string() }
            fn get_group_tag() -> Tag { Tag::from($group_tag_expr) }
        }
    };
}