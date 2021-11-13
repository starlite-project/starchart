pub mod action {
    #![allow(missing_copy_implementations)]
    //! The action structs for CRUD operations.
    mod r#impl {
        use serde::{Deserialize, Serialize};
        use super::{ActionKind, OperationTarget};
        /// Marker type for a Create operation.
        pub struct CreateOperation;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for CreateOperation {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    CreateOperation => ::core::fmt::Formatter::write_str(f, "CreateOperation"),
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for CreateOperation {
            #[inline]
            fn default() -> CreateOperation {
                CreateOperation {}
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for CreateOperation {
            #[inline]
            fn clone(&self) -> CreateOperation {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::marker::Copy for CreateOperation {}
        impl ::core::marker::StructuralPartialEq for CreateOperation {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for CreateOperation {
            #[inline]
            fn eq(&self, other: &CreateOperation) -> bool {
                match *other {
                    CreateOperation => match *self {
                        CreateOperation => true,
                    },
                }
            }
        }
        impl ::core::marker::StructuralEq for CreateOperation {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for CreateOperation {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {}
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialOrd for CreateOperation {
            #[inline]
            fn partial_cmp(
                &self,
                other: &CreateOperation,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                match *other {
                    CreateOperation => match *self {
                        CreateOperation => {
                            ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                        }
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Ord for CreateOperation {
            #[inline]
            fn cmp(&self, other: &CreateOperation) -> ::core::cmp::Ordering {
                match *other {
                    CreateOperation => match *self {
                        CreateOperation => ::core::cmp::Ordering::Equal,
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::hash::Hash for CreateOperation {
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                match *self {
                    CreateOperation => {}
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for CreateOperation {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    _serde::Serializer::serialize_unit_struct(__serializer, "CreateOperation")
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for CreateOperation {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    struct __Visitor;
                    impl<'de> _serde::de::Visitor<'de> for __Visitor {
                        type Value = CreateOperation;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "unit struct CreateOperation",
                            )
                        }
                        #[inline]
                        fn visit_unit<__E>(self) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private::Ok(CreateOperation)
                        }
                    }
                    _serde::Deserializer::deserialize_unit_struct(
                        __deserializer,
                        "CreateOperation",
                        __Visitor,
                    )
                }
            }
        };
        /// Marker type for a Read operation.
        pub struct ReadOperation;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for ReadOperation {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    ReadOperation => ::core::fmt::Formatter::write_str(f, "ReadOperation"),
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for ReadOperation {
            #[inline]
            fn default() -> ReadOperation {
                ReadOperation {}
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for ReadOperation {
            #[inline]
            fn clone(&self) -> ReadOperation {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::marker::Copy for ReadOperation {}
        impl ::core::marker::StructuralPartialEq for ReadOperation {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for ReadOperation {
            #[inline]
            fn eq(&self, other: &ReadOperation) -> bool {
                match *other {
                    ReadOperation => match *self {
                        ReadOperation => true,
                    },
                }
            }
        }
        impl ::core::marker::StructuralEq for ReadOperation {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for ReadOperation {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {}
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialOrd for ReadOperation {
            #[inline]
            fn partial_cmp(
                &self,
                other: &ReadOperation,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                match *other {
                    ReadOperation => match *self {
                        ReadOperation => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Ord for ReadOperation {
            #[inline]
            fn cmp(&self, other: &ReadOperation) -> ::core::cmp::Ordering {
                match *other {
                    ReadOperation => match *self {
                        ReadOperation => ::core::cmp::Ordering::Equal,
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::hash::Hash for ReadOperation {
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                match *self {
                    ReadOperation => {}
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for ReadOperation {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    _serde::Serializer::serialize_unit_struct(__serializer, "ReadOperation")
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for ReadOperation {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    struct __Visitor;
                    impl<'de> _serde::de::Visitor<'de> for __Visitor {
                        type Value = ReadOperation;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "unit struct ReadOperation",
                            )
                        }
                        #[inline]
                        fn visit_unit<__E>(self) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private::Ok(ReadOperation)
                        }
                    }
                    _serde::Deserializer::deserialize_unit_struct(
                        __deserializer,
                        "ReadOperation",
                        __Visitor,
                    )
                }
            }
        };
        /// Marker type for an Update operation.
        pub struct UpdateOperation;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for UpdateOperation {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    UpdateOperation => ::core::fmt::Formatter::write_str(f, "UpdateOperation"),
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for UpdateOperation {
            #[inline]
            fn default() -> UpdateOperation {
                UpdateOperation {}
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for UpdateOperation {
            #[inline]
            fn clone(&self) -> UpdateOperation {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::marker::Copy for UpdateOperation {}
        impl ::core::marker::StructuralPartialEq for UpdateOperation {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for UpdateOperation {
            #[inline]
            fn eq(&self, other: &UpdateOperation) -> bool {
                match *other {
                    UpdateOperation => match *self {
                        UpdateOperation => true,
                    },
                }
            }
        }
        impl ::core::marker::StructuralEq for UpdateOperation {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for UpdateOperation {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {}
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialOrd for UpdateOperation {
            #[inline]
            fn partial_cmp(
                &self,
                other: &UpdateOperation,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                match *other {
                    UpdateOperation => match *self {
                        UpdateOperation => {
                            ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                        }
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Ord for UpdateOperation {
            #[inline]
            fn cmp(&self, other: &UpdateOperation) -> ::core::cmp::Ordering {
                match *other {
                    UpdateOperation => match *self {
                        UpdateOperation => ::core::cmp::Ordering::Equal,
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::hash::Hash for UpdateOperation {
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                match *self {
                    UpdateOperation => {}
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for UpdateOperation {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    _serde::Serializer::serialize_unit_struct(__serializer, "UpdateOperation")
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for UpdateOperation {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    struct __Visitor;
                    impl<'de> _serde::de::Visitor<'de> for __Visitor {
                        type Value = UpdateOperation;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "unit struct UpdateOperation",
                            )
                        }
                        #[inline]
                        fn visit_unit<__E>(self) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private::Ok(UpdateOperation)
                        }
                    }
                    _serde::Deserializer::deserialize_unit_struct(
                        __deserializer,
                        "UpdateOperation",
                        __Visitor,
                    )
                }
            }
        };
        /// Marker type for a Delete operation.
        pub struct DeleteOperation;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for DeleteOperation {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    DeleteOperation => ::core::fmt::Formatter::write_str(f, "DeleteOperation"),
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for DeleteOperation {
            #[inline]
            fn default() -> DeleteOperation {
                DeleteOperation {}
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for DeleteOperation {
            #[inline]
            fn clone(&self) -> DeleteOperation {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::marker::Copy for DeleteOperation {}
        impl ::core::marker::StructuralPartialEq for DeleteOperation {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for DeleteOperation {
            #[inline]
            fn eq(&self, other: &DeleteOperation) -> bool {
                match *other {
                    DeleteOperation => match *self {
                        DeleteOperation => true,
                    },
                }
            }
        }
        impl ::core::marker::StructuralEq for DeleteOperation {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for DeleteOperation {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {}
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialOrd for DeleteOperation {
            #[inline]
            fn partial_cmp(
                &self,
                other: &DeleteOperation,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                match *other {
                    DeleteOperation => match *self {
                        DeleteOperation => {
                            ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                        }
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Ord for DeleteOperation {
            #[inline]
            fn cmp(&self, other: &DeleteOperation) -> ::core::cmp::Ordering {
                match *other {
                    DeleteOperation => match *self {
                        DeleteOperation => ::core::cmp::Ordering::Equal,
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::hash::Hash for DeleteOperation {
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                match *self {
                    DeleteOperation => {}
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for DeleteOperation {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    _serde::Serializer::serialize_unit_struct(__serializer, "DeleteOperation")
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for DeleteOperation {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    struct __Visitor;
                    impl<'de> _serde::de::Visitor<'de> for __Visitor {
                        type Value = DeleteOperation;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "unit struct DeleteOperation",
                            )
                        }
                        #[inline]
                        fn visit_unit<__E>(self) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private::Ok(DeleteOperation)
                        }
                    }
                    _serde::Deserializer::deserialize_unit_struct(
                        __deserializer,
                        "DeleteOperation",
                        __Visitor,
                    )
                }
            }
        };
        /// A sealed marker trait for helping an [`Action`] represent what type of operation will occur.
        ///
        /// [`Action`]: crate::action::Action
        pub trait CrudOperation: private::Sealed {
            #[doc(hidden)]
            fn kind() -> ActionKind;
        }
        impl CrudOperation for CreateOperation {
            fn kind() -> ActionKind {
                ActionKind::Create
            }
        }
        impl CrudOperation for ReadOperation {
            fn kind() -> ActionKind {
                ActionKind::Read
            }
        }
        impl CrudOperation for UpdateOperation {
            fn kind() -> ActionKind {
                ActionKind::Update
            }
        }
        impl CrudOperation for DeleteOperation {
            fn kind() -> ActionKind {
                ActionKind::Delete
            }
        }
        /// Marker type for a table operation.
        pub struct TableTarget;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for TableTarget {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    TableTarget => ::core::fmt::Formatter::write_str(f, "TableTarget"),
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for TableTarget {
            #[inline]
            fn default() -> TableTarget {
                TableTarget {}
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for TableTarget {
            #[inline]
            fn clone(&self) -> TableTarget {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::marker::Copy for TableTarget {}
        impl ::core::marker::StructuralPartialEq for TableTarget {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for TableTarget {
            #[inline]
            fn eq(&self, other: &TableTarget) -> bool {
                match *other {
                    TableTarget => match *self {
                        TableTarget => true,
                    },
                }
            }
        }
        impl ::core::marker::StructuralEq for TableTarget {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for TableTarget {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {}
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialOrd for TableTarget {
            #[inline]
            fn partial_cmp(
                &self,
                other: &TableTarget,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                match *other {
                    TableTarget => match *self {
                        TableTarget => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Ord for TableTarget {
            #[inline]
            fn cmp(&self, other: &TableTarget) -> ::core::cmp::Ordering {
                match *other {
                    TableTarget => match *self {
                        TableTarget => ::core::cmp::Ordering::Equal,
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::hash::Hash for TableTarget {
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                match *self {
                    TableTarget => {}
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for TableTarget {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    _serde::Serializer::serialize_unit_struct(__serializer, "TableTarget")
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for TableTarget {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    struct __Visitor;
                    impl<'de> _serde::de::Visitor<'de> for __Visitor {
                        type Value = TableTarget;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "unit struct TableTarget",
                            )
                        }
                        #[inline]
                        fn visit_unit<__E>(self) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private::Ok(TableTarget)
                        }
                    }
                    _serde::Deserializer::deserialize_unit_struct(
                        __deserializer,
                        "TableTarget",
                        __Visitor,
                    )
                }
            }
        };
        /// Marker type for an entry operation.
        pub struct EntryTarget;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for EntryTarget {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    EntryTarget => ::core::fmt::Formatter::write_str(f, "EntryTarget"),
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for EntryTarget {
            #[inline]
            fn default() -> EntryTarget {
                EntryTarget {}
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for EntryTarget {
            #[inline]
            fn clone(&self) -> EntryTarget {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::marker::Copy for EntryTarget {}
        impl ::core::marker::StructuralPartialEq for EntryTarget {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for EntryTarget {
            #[inline]
            fn eq(&self, other: &EntryTarget) -> bool {
                match *other {
                    EntryTarget => match *self {
                        EntryTarget => true,
                    },
                }
            }
        }
        impl ::core::marker::StructuralEq for EntryTarget {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for EntryTarget {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {}
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialOrd for EntryTarget {
            #[inline]
            fn partial_cmp(
                &self,
                other: &EntryTarget,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                match *other {
                    EntryTarget => match *self {
                        EntryTarget => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Ord for EntryTarget {
            #[inline]
            fn cmp(&self, other: &EntryTarget) -> ::core::cmp::Ordering {
                match *other {
                    EntryTarget => match *self {
                        EntryTarget => ::core::cmp::Ordering::Equal,
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::hash::Hash for EntryTarget {
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                match *self {
                    EntryTarget => {}
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for EntryTarget {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    _serde::Serializer::serialize_unit_struct(__serializer, "EntryTarget")
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for EntryTarget {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    struct __Visitor;
                    impl<'de> _serde::de::Visitor<'de> for __Visitor {
                        type Value = EntryTarget;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "unit struct EntryTarget",
                            )
                        }
                        #[inline]
                        fn visit_unit<__E>(self) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private::Ok(EntryTarget)
                        }
                    }
                    _serde::Deserializer::deserialize_unit_struct(
                        __deserializer,
                        "EntryTarget",
                        __Visitor,
                    )
                }
            }
        };
        /// A sealed marker trait for helping an [`Action`] represent what type of target the
        /// operation will cover.
        ///
        /// [`Action`]: crate::action::Action
        pub trait OpTarget: private::Sealed {
            #[doc(hidden)]
            fn target() -> OperationTarget;
        }
        impl OpTarget for TableTarget {
            fn target() -> OperationTarget {
                OperationTarget::Table
            }
        }
        impl OpTarget for EntryTarget {
            fn target() -> OperationTarget {
                OperationTarget::Entry
            }
        }
        mod private {
            use super::{
                CreateOperation, DeleteOperation, EntryTarget, ReadOperation, TableTarget,
                UpdateOperation,
            };
            pub trait Sealed {}
            impl Sealed for CreateOperation {}
            impl Sealed for ReadOperation {}
            impl Sealed for UpdateOperation {}
            impl Sealed for DeleteOperation {}
            impl Sealed for TableTarget {}
            impl Sealed for EntryTarget {}
        }
    }
    mod kind {
        use std::fmt::{Display, Formatter, Result as FmtResult};
        use serde::{Deserialize, Serialize};
        /// The type of [`CRUD`] action to perform
        ///
        /// [`CRUD`]: https://en.wikipedia.org/wiki/Create,_read,_update_and_delete
        #[must_use = "getting the information on what action will be performed has no side effects"]
        pub enum ActionKind {
            /// Signifies that the operation will be a Create.
            ///
            /// This locks the database and allows no other reads or writes until it is complete.
            Create,
            /// Signifies that the operation will be a Read.
            ///
            /// This allows multiple different readers, but doesn't allow writing until all Reads are complete.
            Read,
            /// Signifies that the operation will be an Update.
            ///
            /// This locks the database and allows no other reads or writes until it is complete.
            Update,
            /// Signifies that the operation will be a Delete.
            ///
            /// This locks the database and allows no other reads or writes until it is complete.
            Delete,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for ActionKind {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match (&*self,) {
                    (&ActionKind::Create,) => ::core::fmt::Formatter::write_str(f, "Create"),
                    (&ActionKind::Read,) => ::core::fmt::Formatter::write_str(f, "Read"),
                    (&ActionKind::Update,) => ::core::fmt::Formatter::write_str(f, "Update"),
                    (&ActionKind::Delete,) => ::core::fmt::Formatter::write_str(f, "Delete"),
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for ActionKind {
            #[inline]
            fn clone(&self) -> ActionKind {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::marker::Copy for ActionKind {}
        impl ::core::marker::StructuralPartialEq for ActionKind {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for ActionKind {
            #[inline]
            fn eq(&self, other: &ActionKind) -> bool {
                {
                    let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                    let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*other) {
                            _ => true,
                        }
                    } else {
                        false
                    }
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for ActionKind {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    match *self {
                        ActionKind::Create => _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ActionKind",
                            0u32,
                            "Create",
                        ),
                        ActionKind::Read => _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ActionKind",
                            1u32,
                            "Read",
                        ),
                        ActionKind::Update => _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ActionKind",
                            2u32,
                            "Update",
                        ),
                        ActionKind::Delete => _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ActionKind",
                            3u32,
                            "Delete",
                        ),
                    }
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for ActionKind {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "variant identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"variant index 0 <= i < 4",
                                )),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "Create" => _serde::__private::Ok(__Field::__field0),
                                "Read" => _serde::__private::Ok(__Field::__field1),
                                "Update" => _serde::__private::Ok(__Field::__field2),
                                "Delete" => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Err(_serde::de::Error::unknown_variant(
                                    __value, VARIANTS,
                                )),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"Create" => _serde::__private::Ok(__Field::__field0),
                                b"Read" => _serde::__private::Ok(__Field::__field1),
                                b"Update" => _serde::__private::Ok(__Field::__field2),
                                b"Delete" => _serde::__private::Ok(__Field::__field3),
                                _ => {
                                    let __value = &_serde::__private::from_utf8_lossy(__value);
                                    _serde::__private::Err(_serde::de::Error::unknown_variant(
                                        __value, VARIANTS,
                                    ))
                                }
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<ActionKind>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = ActionKind;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "enum ActionKind")
                        }
                        fn visit_enum<__A>(
                            self,
                            __data: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::EnumAccess<'de>,
                        {
                            match match _serde::de::EnumAccess::variant(__data) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                (__Field::__field0, __variant) => {
                                    match _serde::de::VariantAccess::unit_variant(__variant) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                    _serde::__private::Ok(ActionKind::Create)
                                }
                                (__Field::__field1, __variant) => {
                                    match _serde::de::VariantAccess::unit_variant(__variant) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                    _serde::__private::Ok(ActionKind::Read)
                                }
                                (__Field::__field2, __variant) => {
                                    match _serde::de::VariantAccess::unit_variant(__variant) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                    _serde::__private::Ok(ActionKind::Update)
                                }
                                (__Field::__field3, __variant) => {
                                    match _serde::de::VariantAccess::unit_variant(__variant) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                    _serde::__private::Ok(ActionKind::Delete)
                                }
                            }
                        }
                    }
                    const VARIANTS: &'static [&'static str] =
                        &["Create", "Read", "Update", "Delete"];
                    _serde::Deserializer::deserialize_enum(
                        __deserializer,
                        "ActionKind",
                        VARIANTS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<ActionKind>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        impl Display for ActionKind {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                match self {
                    Self::Create => f.write_str("Create"),
                    Self::Read => f.write_str("Read"),
                    Self::Update => f.write_str("Update"),
                    Self::Delete => f.write_str("Delete"),
                }
            }
        }
        impl Default for ActionKind {
            fn default() -> Self {
                Self::Read
            }
        }
    }
    pub mod result {
        //! Represents the many different results from actions.
        #![allow(clippy::must_use_candidate, clippy::missing_const_for_fn)]
        use std::{convert::TryFrom, ops::Deref};
        use thiserror::Error;
        use super::{ActionKind, OperationTarget};
        use crate::Entry;
        /// Trait for all the variants of [`ActionResult`] to easily convert
        /// between a table and entity [`Result`].
        ///
        /// This trait is sealed and cannot be implemented outside of this crate.
        pub trait MultiResult: private::Sealed
        where
            Self: Sized,
        {
            /// The result type for a table operation.
            type TableResult;
            /// The result type for an entity operation.
            type EntityResult;
            /// Similar to [`Result::ok`], returning the [`Self::TableResult`] if there was one and [`None`] if not.
            fn table(self) -> Option<Self::TableResult>;
            /// Similar to [`Result::ok`], returning the [`Self::EntityResult`] if there was one and [`None`] if not.
            fn entity(self) -> Option<Self::EntityResult>;
            /// Similar to [`Result::unwrap`], returning the [`Self::TableResult`], panicking otherwise.
            #[track_caller]
            fn unwrap_table(self) -> Self::TableResult {
                self.table()
                    .expect("called `MultiResult::unwrap_table()` on a `Entity` value")
            }
            /// Similar to [`Result::unwrap`], returning the [`Self::EntityResult`], panicking otherwise.
            #[track_caller]
            fn unwrap_entity(self) -> Self::EntityResult {
                self.entity()
                    .expect("called `MultiResult::unwrap_entity()` on a `Table` value")
            }
            /// Similar to [`Result::unwrap_unchecked`], returning the [`Self::TableResult`] without checking if
            /// it's valid first.
            ///
            /// # Safety
            ///
            /// Calling this method on a [`Self::EntityResult`] is *[undefined behavior]*.
            ///
            /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
            unsafe fn unwrap_table_unchecked(self) -> Self::TableResult {
                self.table().unwrap_unchecked()
            }
            /// Similar to [`Result::unwrap_unchecked`], returning the [`Self::EntityResult`] without checking if
            /// it's valid first.
            ///
            /// # Safety
            ///
            /// Calling this method on a [`Self::TableResult`] is *[undefined behavior]*.
            ///
            /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
            unsafe fn unwrap_entity_unchecked(self) -> Self::EntityResult {
                self.entity().unwrap_unchecked()
            }
        }
        /// The base [`Result`] type for [`Action`]s.
        ///
        /// [`Action`]: crate::action::Action
        #[must_use = "this `ActionResult` may be an Error of some kind, which should be handled"]
        pub enum ActionResult<T: Entry> {
            /// The result from an [`Action::create`].
            ///
            /// [`Action::create`]: crate::action::Action::create
            Create(CreateResult),
            /// The result from an [`Action::read`].
            ///
            /// [`Action::read`]: crate::action::Action::read
            Read(ReadResult<T>),
            /// The result from an [`Action::update`].
            ///
            /// [`Action::update`]: crate::action::Action::update
            Update(UpdateResult),
            /// The result from an [`Action::delete`].
            ///
            /// [`Action::delete`]: crate::action::Action::delete
            Delete(DeleteResult),
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl<T: ::core::fmt::Debug + Entry> ::core::fmt::Debug for ActionResult<T> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match (&*self,) {
                    (&ActionResult::Create(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "Create");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                    (&ActionResult::Read(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "Read");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                    (&ActionResult::Update(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "Update");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                    (&ActionResult::Delete(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "Delete");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                }
            }
        }
        impl<T: Entry> ActionResult<T> {
            /// Converts from [`ActionResult`] to [`Option`].
            ///
            /// This consumes `self`, returning a [`CreateResult`] if the [`ActionResult`] is a [`CreateResult`],
            /// and [`None`] otherwise.
            pub fn create(self) -> Option<CreateResult> {
                match self {
                    Self::Create(result) => Some(result),
                    _ => None,
                }
            }
            /// Converts from [`ActionResult`] to [`Option`].
            ///
            /// This consumes `self`, returning a [`ReadResult`] if the [`ActionResult`] is a [`ReadResult`],
            /// and [`None`] otherwise.
            pub fn read(self) -> Option<ReadResult<T>> {
                match self {
                    Self::Read(result) => Some(result),
                    _ => None,
                }
            }
            /// Converts from [`ActionResult`] to [`Option`].
            ///
            /// This consumes `self`, returning an [`UpdateResult`] if the [`ActionResult`] is an [`UpdateResult`],
            /// and [`None`] otherwise.
            pub fn update(self) -> Option<UpdateResult> {
                match self {
                    Self::Update(result) => Some(result),
                    _ => None,
                }
            }
            /// Converts from [`ActionResult`] to [`Option`].
            ///
            /// This consumes `self`, returning an [`DeleteResult`] if the [`ActionResult`] is an [`DeleteResult`],
            /// and [`None`] otherwise.
            pub fn delete(self) -> Option<DeleteResult> {
                match self {
                    Self::Delete(result) => Some(result),
                    _ => None,
                }
            }
            /// Returns the [`ActionKind`] that this [`ActionResult`] represents.
            pub fn kind(&self) -> ActionKind {
                match self {
                    Self::Create(_) => ActionKind::Create,
                    Self::Read(_) => ActionKind::Read,
                    Self::Update(_) => ActionKind::Update,
                    Self::Delete(_) => ActionKind::Delete,
                }
            }
            /// Returns the [`CreateResult`] from this [`ActionResult`].
            ///
            /// # Panics
            ///
            /// Panics if the [`ActionResult`] is not a [`CreateResult`].
            #[track_caller]
            pub fn unwrap_create(self) -> CreateResult {
                let kind = self.kind();
                self.create().unwrap_or_else(|| {
                    ::std::rt::panic_fmt(::core::fmt::Arguments::new_v1(
                        &["called `ActionResult::unwrap_create` on a `", "` value"],
                        &match (&kind,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ))
                })
            }
            /// Returns the [`ReadResult`] from this [`ActionResult`].
            ///
            /// # Panics
            ///
            /// Panics if the [`ActionResult`] is not a [`ReadResult`].
            #[track_caller]
            pub fn unwrap_read(self) -> ReadResult<T> {
                let kind = self.kind();
                self.read().unwrap_or_else(|| {
                    ::std::rt::panic_fmt(::core::fmt::Arguments::new_v1(
                        &["called `ActionResult::unwrap_read` on a `", "` value"],
                        &match (&kind,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ))
                })
            }
            /// Returns the [`UpdateResult`] from this [`ActionResult`].
            ///
            /// # Panics
            ///
            /// Panics if the [`ActionResult`] is not a [`UpdateResult`].
            #[track_caller]
            pub fn unwrap_update(self) -> UpdateResult {
                let kind = self.kind();
                self.update().unwrap_or_else(|| {
                    ::std::rt::panic_fmt(::core::fmt::Arguments::new_v1(
                        &["called `ActionResult::unwrap_update` on a `", "` value"],
                        &match (&kind,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ))
                })
            }
            /// Returns the [`DeleteResult`] from this [`ActionResult`].
            ///
            /// # Panics
            ///
            /// Panics if the [`ActionResult`] is not a [`DeleteResult`].
            #[track_caller]
            pub fn unwrap_delete(self) -> DeleteResult {
                let kind = self.kind();
                self.delete().unwrap_or_else(|| {
                    ::std::rt::panic_fmt(::core::fmt::Arguments::new_v1(
                        &["called `ActionResult::unwrap_delete` on a `", "` value"],
                        &match (&kind,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ))
                })
            }
        }
        /// A result from an [`Action::create`].
        ///
        /// [`Action::create`]: crate::action::Action::create
        #[must_use = "this `CreateResult` may be an Error of some kind, which should be handled"]
        pub enum CreateResult {
            /// A table creation result.
            Table(Result<(), CreateError>),
            /// An entity creation result.
            Entity(Result<(), CreateError>),
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for CreateResult {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match (&*self,) {
                    (&CreateResult::Table(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "Table");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                    (&CreateResult::Entity(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "Entity");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                }
            }
        }
        impl MultiResult for CreateResult {
            type EntityResult = Result<(), CreateError>;
            type TableResult = Result<(), CreateError>;
            fn table(self) -> Option<Self::TableResult> {
                if let Self::Table(r) = self {
                    Some(r)
                } else {
                    None
                }
            }
            fn entity(self) -> Option<Self::EntityResult> {
                if let Self::Entity(r) = self {
                    Some(r)
                } else {
                    None
                }
            }
        }
        impl From<CreateResult> for Result<(), CreateError> {
            fn from(res: CreateResult) -> Self {
                match res {
                    CreateResult::Entity(r) | CreateResult::Table(r) => r,
                }
            }
        }
        impl Deref for CreateResult {
            type Target = Result<(), CreateError>;
            fn deref(&self) -> &Self::Target {
                match self {
                    CreateResult::Table(r) | CreateResult::Entity(r) => r,
                }
            }
        }
        /// An error occurred during an [`Action::create`].
        ///
        /// [`Action::create`]: crate::action::Action::create
        #[error("an error happened during {target} creation")]
        pub struct CreateError {
            source: Box<dyn std::error::Error>,
            target: OperationTarget,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for CreateError {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    CreateError {
                        source: ref __self_0_0,
                        target: ref __self_0_1,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "CreateError");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "source",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "target",
                            &&(*__self_0_1),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[allow(unused_qualifications)]
        impl std::error::Error for CreateError {
            fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
                use thiserror::private::AsDynError;
                std::option::Option::Some(self.source.as_dyn_error())
            }
        }
        #[allow(unused_qualifications)]
        impl std::fmt::Display for CreateError {
            #[allow(clippy::used_underscore_binding)]
            fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                #[allow(unused_imports)]
                use thiserror::private::{DisplayAsDisplay, PathAsDisplay};
                #[allow(unused_variables, deprecated)]
                let Self { source, target } = self;
                __formatter.write_fmt(::core::fmt::Arguments::new_v1(
                    &["an error happened during ", " creation"],
                    &match (&target.as_display(),) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ))
            }
        }
        impl CreateError {
            /// The target the create operation was for.
            pub const fn target(&self) -> OperationTarget {
                self.target
            }
        }
        /// A result from an [`Action::read`].
        ///
        /// [`Action::read`]: crate::action::Action::read
        #[must_use = "this `ReadResult` may be an Error of some kind, which should be handled"]
        pub enum ReadResult<T: Entry> {
            /// A table read result.
            Table(Result<Vec<T>, ReadError>),
            /// An entry read result.
            ///
            /// # Note
            ///
            /// The return result will be a [`Vec`] with just one element, so to get the value indexing by 0 will
            /// never fail.
            ///
            /// However, if one wishes to get the inner value without indexing, the [`MultiResult`] impl
            /// provides easy to use methods to get said values.
            Entity(Result<Vec<T>, ReadError>),
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl<T: ::core::fmt::Debug + Entry> ::core::fmt::Debug for ReadResult<T> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match (&*self,) {
                    (&ReadResult::Table(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "Table");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                    (&ReadResult::Entity(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "Entity");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                }
            }
        }
        impl<T: Entry> MultiResult for ReadResult<T> {
            type EntityResult = Result<T, ReadError>;
            type TableResult = Result<Vec<T>, ReadError>;
            fn table(self) -> Option<Self::TableResult> {
                if let Self::Table(r) = self {
                    Some(r)
                } else {
                    None
                }
            }
            fn entity(self) -> Option<Self::EntityResult> {
                if let Self::Entity(r) = self {
                    Some(r.map(|val| val[0].clone()))
                } else {
                    None
                }
            }
        }
        impl<T: Entry> Deref for ReadResult<T> {
            type Target = Result<Vec<T>, ReadError>;
            fn deref(&self) -> &Self::Target {
                match self {
                    Self::Table(r) | Self::Entity(r) => r,
                }
            }
        }
        /// Represents a conversion error when using the [`TryFrom`] impls for [`ReadResult`].
        pub enum InvalidTargetError {
            /// Attempted to convert a [`ReadResult::Table`] into a [`Result<T, ReadError>`].
            #[error("attempted conversion of entity result into table")]
            ExpectedTable,
            /// Attempted to convert a [`ReadResult::Entity`] into a [`Result<Vec<T>, ReadError>`].
            #[error("attempted conversion of table result into entity")]
            ExpectedEntity,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for InvalidTargetError {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match (&*self,) {
                    (&InvalidTargetError::ExpectedTable,) => {
                        ::core::fmt::Formatter::write_str(f, "ExpectedTable")
                    }
                    (&InvalidTargetError::ExpectedEntity,) => {
                        ::core::fmt::Formatter::write_str(f, "ExpectedEntity")
                    }
                }
            }
        }
        #[allow(unused_qualifications)]
        impl std::error::Error for InvalidTargetError {}
        #[allow(unused_qualifications)]
        impl std::fmt::Display for InvalidTargetError {
            fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
                match self {
                    InvalidTargetError::ExpectedTable {} => {
                        __formatter.write_fmt(::core::fmt::Arguments::new_v1(
                            &["attempted conversion of entity result into table"],
                            &match () {
                                () => [],
                            },
                        ))
                    }
                    InvalidTargetError::ExpectedEntity {} => {
                        __formatter.write_fmt(::core::fmt::Arguments::new_v1(
                            &["attempted conversion of table result into entity"],
                            &match () {
                                () => [],
                            },
                        ))
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for InvalidTargetError {
            #[inline]
            fn clone(&self) -> InvalidTargetError {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::marker::Copy for InvalidTargetError {}
        impl<T: Entry> TryFrom<ReadResult<T>> for Result<Vec<T>, ReadError> {
            type Error = InvalidTargetError;
            fn try_from(value: ReadResult<T>) -> Result<Self, Self::Error> {
                if let ReadResult::Table(r) = value {
                    Ok(r)
                } else {
                    Err(InvalidTargetError::ExpectedTable)
                }
            }
        }
        impl<T: Entry> TryFrom<ReadResult<T>> for Result<T, ReadError> {
            type Error = InvalidTargetError;
            fn try_from(value: ReadResult<T>) -> Result<Self, Self::Error> {
                if let ReadResult::Entity(r) = value {
                    Ok(r.map(|v| v[0].clone()))
                } else {
                    Err(InvalidTargetError::ExpectedEntity)
                }
            }
        }
        /// An error occurred during an [`Action::read`].
        ///
        /// [`Action::read`]: crate::action::Action::read
        #[error("an error happened during {target} read")]
        pub struct ReadError {
            source: Box<dyn std::error::Error>,
            target: OperationTarget,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for ReadError {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    ReadError {
                        source: ref __self_0_0,
                        target: ref __self_0_1,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "ReadError");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "source",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "target",
                            &&(*__self_0_1),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[allow(unused_qualifications)]
        impl std::error::Error for ReadError {
            fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
                use thiserror::private::AsDynError;
                std::option::Option::Some(self.source.as_dyn_error())
            }
        }
        #[allow(unused_qualifications)]
        impl std::fmt::Display for ReadError {
            #[allow(clippy::used_underscore_binding)]
            fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                #[allow(unused_imports)]
                use thiserror::private::{DisplayAsDisplay, PathAsDisplay};
                #[allow(unused_variables, deprecated)]
                let Self { source, target } = self;
                __formatter.write_fmt(::core::fmt::Arguments::new_v1(
                    &["an error happened during ", " read"],
                    &match (&target.as_display(),) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ))
            }
        }
        impl ReadError {
            /// The target the read operation was for.
            pub const fn target(&self) -> OperationTarget {
                self.target
            }
        }
        /// A result from an [`Action::update`].
        ///
        /// [`Action::update`]: crate::action::Action::update
        #[must_use = "this `UpdateResult` may be an Error of some kind, which should be handled"]
        pub enum UpdateResult {
            /// A table update result.
            Table(Result<(), UpdateError>),
            /// An entity update result.
            Entity(Result<(), UpdateError>),
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for UpdateResult {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match (&*self,) {
                    (&UpdateResult::Table(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "Table");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                    (&UpdateResult::Entity(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "Entity");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                }
            }
        }
        impl MultiResult for UpdateResult {
            type EntityResult = Result<(), UpdateError>;
            type TableResult = Result<(), UpdateError>;
            fn table(self) -> Option<Self::TableResult> {
                if let Self::Table(r) = self {
                    Some(r)
                } else {
                    None
                }
            }
            fn entity(self) -> Option<Self::EntityResult> {
                if let Self::Entity(r) = self {
                    Some(r)
                } else {
                    None
                }
            }
        }
        impl From<UpdateResult> for Result<(), UpdateError> {
            fn from(val: UpdateResult) -> Self {
                match val {
                    UpdateResult::Entity(r) | UpdateResult::Table(r) => r,
                }
            }
        }
        impl Deref for UpdateResult {
            type Target = Result<(), UpdateError>;
            fn deref(&self) -> &Self::Target {
                match self {
                    UpdateResult::Table(r) | UpdateResult::Entity(r) => r,
                }
            }
        }
        /// An error occurred during an [`Action::update`].
        ///
        /// [`Action::update`]: crate::action::Action::update
        #[error("an error happened during {target} update")]
        pub struct UpdateError {
            source: Box<dyn std::error::Error>,
            target: OperationTarget,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for UpdateError {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    UpdateError {
                        source: ref __self_0_0,
                        target: ref __self_0_1,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "UpdateError");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "source",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "target",
                            &&(*__self_0_1),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[allow(unused_qualifications)]
        impl std::error::Error for UpdateError {
            fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
                use thiserror::private::AsDynError;
                std::option::Option::Some(self.source.as_dyn_error())
            }
        }
        #[allow(unused_qualifications)]
        impl std::fmt::Display for UpdateError {
            #[allow(clippy::used_underscore_binding)]
            fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                #[allow(unused_imports)]
                use thiserror::private::{DisplayAsDisplay, PathAsDisplay};
                #[allow(unused_variables, deprecated)]
                let Self { source, target } = self;
                __formatter.write_fmt(::core::fmt::Arguments::new_v1(
                    &["an error happened during ", " update"],
                    &match (&target.as_display(),) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ))
            }
        }
        impl UpdateError {
            /// The target the update operation was for.
            pub const fn target(&self) -> OperationTarget {
                self.target
            }
        }
        /// A result from an [`Action::delete`].
        ///
        /// [`Action::delete`]: crate::action::Action::delete
        #[must_use = "this `DeleteResult` may be an Error of some kind, which should be handled"]
        pub enum DeleteResult {
            /// A table delete result.
            Table(Result<bool, DeleteError>),
            /// An entity delete result.
            Entity(Result<bool, DeleteError>),
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for DeleteResult {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match (&*self,) {
                    (&DeleteResult::Table(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "Table");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                    (&DeleteResult::Entity(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "Entity");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                }
            }
        }
        impl MultiResult for DeleteResult {
            type EntityResult = Result<bool, DeleteError>;
            type TableResult = Result<bool, DeleteError>;
            fn table(self) -> Option<Self::TableResult> {
                if let Self::Table(r) = self {
                    Some(r)
                } else {
                    None
                }
            }
            fn entity(self) -> Option<Self::EntityResult> {
                if let Self::Entity(r) = self {
                    Some(r)
                } else {
                    None
                }
            }
        }
        impl From<DeleteResult> for Result<bool, DeleteError> {
            fn from(value: DeleteResult) -> Self {
                match value {
                    DeleteResult::Entity(r) | DeleteResult::Table(r) => r,
                }
            }
        }
        impl Deref for DeleteResult {
            type Target = Result<bool, DeleteError>;
            fn deref(&self) -> &Self::Target {
                match self {
                    DeleteResult::Table(r) | DeleteResult::Entity(r) => r,
                }
            }
        }
        /// An error occurred during an [`Action::delete`].
        ///
        /// [`Action::delete`]: crate::action::Action::delete
        #[error("an error happened during {target} deletion")]
        pub struct DeleteError {
            source: Box<dyn std::error::Error>,
            target: OperationTarget,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for DeleteError {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    DeleteError {
                        source: ref __self_0_0,
                        target: ref __self_0_1,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "DeleteError");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "source",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "target",
                            &&(*__self_0_1),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[allow(unused_qualifications)]
        impl std::error::Error for DeleteError {
            fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
                use thiserror::private::AsDynError;
                std::option::Option::Some(self.source.as_dyn_error())
            }
        }
        #[allow(unused_qualifications)]
        impl std::fmt::Display for DeleteError {
            #[allow(clippy::used_underscore_binding)]
            fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                #[allow(unused_imports)]
                use thiserror::private::{DisplayAsDisplay, PathAsDisplay};
                #[allow(unused_variables, deprecated)]
                let Self { source, target } = self;
                __formatter.write_fmt(::core::fmt::Arguments::new_v1(
                    &["an error happened during ", " deletion"],
                    &match (&target.as_display(),) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ))
            }
        }
        impl DeleteError {
            /// The target the delete operation was for.
            pub const fn target(&self) -> OperationTarget {
                self.target
            }
        }
        mod private {
            use super::{CreateResult, DeleteResult, ReadResult, UpdateResult};
            use crate::Entry;
            pub trait Sealed {}
            impl Sealed for CreateResult {}
            impl<T: Entry> Sealed for ReadResult<T> {}
            impl Sealed for UpdateResult {}
            impl Sealed for DeleteResult {}
        }
    }
    mod target {
        use std::fmt::{Display, Error as FmtError, Formatter, Result as FmtResult};
        use serde::{Deserialize, Serialize};
        /// The target of the [`CRUD`] operation.
        ///
        /// [`CRUD`]: https://en.wikipedia.org/wiki/Create,_read,_update_and_delete
        pub enum OperationTarget {
            /// The operation will be performed on a table.
            Table,
            /// The operation will be performed on a single entry.
            Entry,
            /// An unknown operation will occur, this raises an error if it's set when [`Action::validate`] is called.
            ///
            /// [`Action::validate`]: crate::action::Action::validate
            Unknown,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for OperationTarget {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match (&*self,) {
                    (&OperationTarget::Table,) => ::core::fmt::Formatter::write_str(f, "Table"),
                    (&OperationTarget::Entry,) => ::core::fmt::Formatter::write_str(f, "Entry"),
                    (&OperationTarget::Unknown,) => ::core::fmt::Formatter::write_str(f, "Unknown"),
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for OperationTarget {
            #[inline]
            fn clone(&self) -> OperationTarget {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::marker::Copy for OperationTarget {}
        impl ::core::marker::StructuralPartialEq for OperationTarget {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for OperationTarget {
            #[inline]
            fn eq(&self, other: &OperationTarget) -> bool {
                {
                    let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                    let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*other) {
                            _ => true,
                        }
                    } else {
                        false
                    }
                }
            }
        }
        impl ::core::marker::StructuralEq for OperationTarget {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for OperationTarget {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {}
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialOrd for OperationTarget {
            #[inline]
            fn partial_cmp(
                &self,
                other: &OperationTarget,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                {
                    let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                    let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*other) {
                            _ => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
                        }
                    } else {
                        ::core::cmp::PartialOrd::partial_cmp(&__self_vi, &__arg_1_vi)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Ord for OperationTarget {
            #[inline]
            fn cmp(&self, other: &OperationTarget) -> ::core::cmp::Ordering {
                {
                    let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                    let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*other) {
                            _ => ::core::cmp::Ordering::Equal,
                        }
                    } else {
                        ::core::cmp::Ord::cmp(&__self_vi, &__arg_1_vi)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::hash::Hash for OperationTarget {
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                match (&*self,) {
                    _ => ::core::hash::Hash::hash(
                        &::core::intrinsics::discriminant_value(self),
                        state,
                    ),
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for OperationTarget {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    match *self {
                        OperationTarget::Table => _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "OperationTarget",
                            0u32,
                            "Table",
                        ),
                        OperationTarget::Entry => _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "OperationTarget",
                            1u32,
                            "Entry",
                        ),
                        OperationTarget::Unknown => _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "OperationTarget",
                            2u32,
                            "Unknown",
                        ),
                    }
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for OperationTarget {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "variant identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"variant index 0 <= i < 3",
                                )),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "Table" => _serde::__private::Ok(__Field::__field0),
                                "Entry" => _serde::__private::Ok(__Field::__field1),
                                "Unknown" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Err(_serde::de::Error::unknown_variant(
                                    __value, VARIANTS,
                                )),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"Table" => _serde::__private::Ok(__Field::__field0),
                                b"Entry" => _serde::__private::Ok(__Field::__field1),
                                b"Unknown" => _serde::__private::Ok(__Field::__field2),
                                _ => {
                                    let __value = &_serde::__private::from_utf8_lossy(__value);
                                    _serde::__private::Err(_serde::de::Error::unknown_variant(
                                        __value, VARIANTS,
                                    ))
                                }
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<OperationTarget>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = OperationTarget;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "enum OperationTarget",
                            )
                        }
                        fn visit_enum<__A>(
                            self,
                            __data: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::EnumAccess<'de>,
                        {
                            match match _serde::de::EnumAccess::variant(__data) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                (__Field::__field0, __variant) => {
                                    match _serde::de::VariantAccess::unit_variant(__variant) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                    _serde::__private::Ok(OperationTarget::Table)
                                }
                                (__Field::__field1, __variant) => {
                                    match _serde::de::VariantAccess::unit_variant(__variant) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                    _serde::__private::Ok(OperationTarget::Entry)
                                }
                                (__Field::__field2, __variant) => {
                                    match _serde::de::VariantAccess::unit_variant(__variant) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                    _serde::__private::Ok(OperationTarget::Unknown)
                                }
                            }
                        }
                    }
                    const VARIANTS: &'static [&'static str] = &["Table", "Entry", "Unknown"];
                    _serde::Deserializer::deserialize_enum(
                        __deserializer,
                        "OperationTarget",
                        VARIANTS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<OperationTarget>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        impl Default for OperationTarget {
            fn default() -> Self {
                Self::Unknown
            }
        }
        impl Display for OperationTarget {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                match self {
                    Self::Table => f.write_str("Table"),
                    Self::Entry => f.write_str("Entry"),
                    Self::Unknown => Err(FmtError),
                }
            }
        }
    }
    use std::{
        cell::Cell,
        fmt::{Debug, Formatter, Result as FmtResult},
        marker::PhantomData,
    };
    use serde::{Serialize, Deserialize};
    use thiserror::Error;
    #[doc(inline)]
    pub use self::{
        kind::ActionKind,
        r#impl::{
            CreateOperation, CrudOperation, DeleteOperation, EntryTarget, OpTarget, ReadOperation,
            TableTarget, UpdateOperation,
        },
        result::ActionResult,
        target::OperationTarget,
    };
    use crate::{Entry, IndexEntry, Key};
    /// A type alias for an [`Action`] with [`CreateOperation`] and [`EntryTarget`] as the parameters.
    pub type CreateEntryAction<S> = Action<S, CreateOperation, EntryTarget>;
    /// A type alias for an [`Action`] with [`ReadOperation`] and [`EntryTarget`] as the parameters.
    pub type ReadEntryAction<S> = Action<S, ReadOperation, EntryTarget>;
    /// A type alias for an [`Action`] with [`UpdateOperation`] and [`EntryTarget`] as the parameters.
    pub type UpdateEntryAction<S> = Action<S, UpdateOperation, EntryTarget>;
    /// A type alias for an [`Action`] with [`DeleteOperation`] and [`EntryTarget`] as the parameters.
    pub type DeleteEntryAction<S> = Action<S, DeleteOperation, EntryTarget>;
    /// A type alias for an [`Action`] with [`CreateOperation`] and [`TableTarget`] as the parameters.
    pub type CreateTableAction<S> = Action<S, CreateOperation, TableTarget>;
    /// A type alias for an [`Action`] with [`ReadOperation`] and [`TableTarget`] as the parameters.
    pub type ReadTableAction<S> = Action<S, ReadOperation, TableTarget>;
    /// A type alias for an [`Action`] with [`UpdateOperation`] and [`TableTarget`] as the parameters.
    pub type UpdateTableAction<S> = Action<S, UpdateOperation, TableTarget>;
    /// A type alias for an [`Action`] with [`DeleteOperation`] and [`TableTarget`] as the parameters.
    pub type DeleteTableAction<S> = Action<S, DeleteOperation, TableTarget>;
    /// An error occurred during validation of an [`Action`].
    #[non_exhaustive]
    pub enum ActionError {
        /// The [`OperationTarget`] was not set.
        #[error("an invalid operation was set")]
        InvalidOperation,
        /// No data was passed when data was expected.
        #[error("no data was given when data was expected")]
        NoData,
        /// No key was passed when a key was expected.
        #[error("no key was given when a key was expected.")]
        NoKey,
        /// Attempted to [`ActionKind::Update`] an [`OperationTarget::Table`].
        #[error("updating an entire table is unsupported")]
        UpdatingTable,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for ActionError {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&ActionError::InvalidOperation,) => {
                    ::core::fmt::Formatter::write_str(f, "InvalidOperation")
                }
                (&ActionError::NoData,) => ::core::fmt::Formatter::write_str(f, "NoData"),
                (&ActionError::NoKey,) => ::core::fmt::Formatter::write_str(f, "NoKey"),
                (&ActionError::UpdatingTable,) => {
                    ::core::fmt::Formatter::write_str(f, "UpdatingTable")
                }
            }
        }
    }
    #[allow(unused_qualifications)]
    impl std::error::Error for ActionError {}
    #[allow(unused_qualifications)]
    impl std::fmt::Display for ActionError {
        fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
            match self {
                ActionError::InvalidOperation {} => {
                    __formatter.write_fmt(::core::fmt::Arguments::new_v1(
                        &["an invalid operation was set"],
                        &match () {
                            () => [],
                        },
                    ))
                }
                ActionError::NoData {} => __formatter.write_fmt(::core::fmt::Arguments::new_v1(
                    &["no data was given when data was expected"],
                    &match () {
                        () => [],
                    },
                )),
                ActionError::NoKey {} => __formatter.write_fmt(::core::fmt::Arguments::new_v1(
                    &["no key was given when a key was expected."],
                    &match () {
                        () => [],
                    },
                )),
                ActionError::UpdatingTable {} => {
                    __formatter.write_fmt(::core::fmt::Arguments::new_v1(
                        &["updating an entire table is unsupported"],
                        &match () {
                            () => [],
                        },
                    ))
                }
            }
        }
    }
    /// An [`Action`] for easy [`CRUD`] operations within a [`Gateway`].
    ///
    /// [`CRUD`]: https://en.wikipedia.org/wiki/Create,_read,_update_and_delete
    /// [`Gateway`]: crate::Gateway
    #[must_use = "an action alone has no side effects"]
    pub struct Action<S, C: CrudOperation, T: OpTarget> {
        pub(crate) inner: InternalAction<S, C, T>,
        pub(crate) validated: Cell<bool>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<S, C: CrudOperation, T: OpTarget> _serde::Serialize for Action<S, C, T>
        where
            S: _serde::Serialize,
            C: _serde::Serialize,
            T: _serde::Serialize,
        {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Action",
                    false as usize + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "inner",
                    &self.inner,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "validated",
                    &self.validated,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de, S, C: CrudOperation, T: OpTarget> _serde::Deserialize<'de> for Action<S, C, T>
        where
            S: _serde::Deserialize<'de>,
            C: _serde::Deserialize<'de>,
            T: _serde::Deserialize<'de>,
        {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "inner" => _serde::__private::Ok(__Field::__field0),
                            "validated" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"inner" => _serde::__private::Ok(__Field::__field0),
                            b"validated" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de, S, C: CrudOperation, T: OpTarget>
                where
                    S: _serde::Deserialize<'de>,
                    C: _serde::Deserialize<'de>,
                    T: _serde::Deserialize<'de>,
                {
                    marker: _serde::__private::PhantomData<Action<S, C, T>>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de, S, C: CrudOperation, T: OpTarget> _serde::de::Visitor<'de> for __Visitor<'de, S, C, T>
                where
                    S: _serde::Deserialize<'de>,
                    C: _serde::Deserialize<'de>,
                    T: _serde::Deserialize<'de>,
                {
                    type Value = Action<S, C, T>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct Action")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            InternalAction<S, C, T>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct Action with 2 elements",
                                ));
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<Cell<bool>>(
                            &mut __seq,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct Action with 2 elements",
                                ));
                            }
                        };
                        _serde::__private::Ok(Action {
                            inner: __field0,
                            validated: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<InternalAction<S, C, T>> =
                            _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Cell<bool>> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "inner",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            InternalAction<S, C, T>,
                                        >(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "validated",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Cell<bool>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("inner") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("validated") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(Action {
                            inner: __field0,
                            validated: __field1,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["inner", "validated"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Action",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Action<S, C, T>>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl<S, C: CrudOperation, T: OpTarget> Action<S, C, T> {
        /// Creates a new [`Action`] with the specified operation.
        pub fn new() -> Self {
            Self {
                inner: InternalAction::new(),
                validated: Cell::new(false),
            }
        }
        /// Returns the [`ActionKind`] we will be performing with said action.
        pub fn kind(&self) -> ActionKind {
            self.inner.kind()
        }
        /// Returns the [`OperationTarget`] we will be performing with said action.
        #[must_use]
        pub fn target(&self) -> OperationTarget {
            self.inner.target()
        }
        /// Whether the [`Action`] has been validated.
        #[must_use]
        pub fn is_validated(&self) -> bool {
            self.validated.get()
        }
    }
    impl<S: Entry, T: OpTarget> Action<S, CreateOperation, T> {
        /// Begins a [`CreateOperation`] action.
        pub fn create() -> Self {
            Self::new()
        }
    }
    impl<S: Entry, T: OpTarget> Action<S, ReadOperation, T> {
        /// Begins a [`ReadOperation`] action.
        pub fn read() -> Self {
            Self::new()
        }
    }
    impl<S: Entry, T: OpTarget> Action<S, UpdateOperation, T> {
        /// Begins an [`UpdateOperation`] action.
        pub fn update() -> Self {
            Self::new()
        }
    }
    impl<S: Entry, T: OpTarget> Action<S, DeleteOperation, T> {
        /// Begins a [`DeleteOperation`] action.
        pub fn delete() -> Self {
            Self::new()
        }
    }
    impl<S: Entry, C: CrudOperation> Action<S, C, TableTarget> {
        /// Creates a new [`TableTarget`] based operation.
        pub fn table() -> Self {
            Self::new()
        }
    }
    impl<S: Entry, C: CrudOperation> Action<S, C, EntryTarget> {
        /// Creates a new [`EntryTarget`] based operation.
        pub fn entry() -> Self {
            Self::new()
        }
    }
    impl<S: Entry, C: CrudOperation, T: OpTarget> Action<S, C, T> {
        /// Changes the [`CrudOperation`] of this [`Action`].
        pub fn into_operation<O: CrudOperation>(self) -> Action<S, O, T> {
            Action {
                inner: self.inner.into_operation(),
                validated: self.validated,
            }
        }
        /// Changes the [`OpTarget`] of this [`Action`].
        pub fn into_target<T2: OpTarget>(self) -> Action<S, C, T2> {
            Action {
                inner: self.inner.into_target(),
                validated: self.validated,
            }
        }
        /// Sets the [`CrudOperation`] of this [`Action`] to [`CreateOperation`].
        pub fn into_create(self) -> Action<S, CreateOperation, T> {
            self.into_operation()
        }
        /// Sets the [`CrudOperation`] of this [`Action`] to [`ReadOperation`].
        pub fn into_read(self) -> Action<S, ReadOperation, T> {
            self.into_operation()
        }
        /// Sets the [`CrudOperation`] of this [`Action`] to [`UpdateOperation`].
        pub fn into_update(self) -> Action<S, UpdateOperation, T> {
            self.into_operation()
        }
        /// Sets the [`CrudOperation`] of this [`Action`] to [`DeleteOperation`].
        pub fn into_delete(self) -> Action<S, DeleteOperation, T> {
            self.into_operation()
        }
        /// Sets the [`OpTarget`] of this [`Action`] to [`TableTarget`].
        pub fn into_table(self) -> Action<S, C, TableTarget> {
            self.into_target()
        }
        /// Sets the [`OpTarget`] of this [`Action`] to [`EntryTarget`].
        pub fn into_entry(self) -> Action<S, C, EntryTarget> {
            self.into_target()
        }
        /// Sets the target [`Entry`] of this [`Action`].
        pub fn with_entry<S2>(self) -> Action<S2, C, T> {
            Action {
                inner: self.inner.with_entry(),
                validated: self.validated,
            }
        }
        /// Validates the [`Action`].
        ///
        /// This is a no-op if the [`Action`] has already been validated.
        ///
        /// # Errors
        ///
        /// Returns an [`ActionError::InvalidOperation`] if the [`Action`] has not set an [`OperationTarget`].
        pub fn validate(&self) -> Result<(), ActionError> {
            if self.is_validated() {
                return Ok(());
            }
            if self.target() == OperationTarget::Unknown {
                return Err(ActionError::InvalidOperation);
            }
            if self.needs_data() && self.inner.data.is_none() {
                return Err(ActionError::NoData);
            }
            if self.needs_key() && self.inner.key.is_none() {
                return Err(ActionError::NoKey);
            }
            if self.is_updating_table() {
                return Err(ActionError::UpdatingTable);
            }
            self.validated.set(true);
            Ok(())
        }
        /// Sets the key for the action.
        ///
        /// Users should prefer to call [`Self::set_entry`] over this, as setting the
        /// entry will automatically call this.
        ///
        /// This is unused on [`OperationTarget::Table`] actions.
        pub fn set_key<K: Key>(&mut self, key: &K) -> &mut Self {
            self.inner.set_key(key.to_key());
            self.validated.set(false);
            self
        }
        /// Sets the data for the action.
        ///
        /// This is unused on [`OperationTarget::Table`] actions.
        pub fn set_data(&mut self, entity: &S) -> &mut Self {
            self.inner.set_entry(Box::new(entity.clone()));
            self
        }
        fn is_updating_table(&self) -> bool {
            self.kind() == ActionKind::Update && self.target() == OperationTarget::Table
        }
        fn needs_data(&self) -> bool {
            if self.kind() == ActionKind::Read {
                return false;
            }
            if self.kind() == ActionKind::Delete {
                return false;
            }
            if self.target() == OperationTarget::Table {
                return false;
            }
            true
        }
        fn needs_key(&self) -> bool {
            if self.target() == OperationTarget::Table {
                return false;
            }
            true
        }
    }
    impl<S: IndexEntry, C: CrudOperation, T: OpTarget> Action<S, C, T> {
        /// Sets the [`Entry`] and [`Key`] that this [`Action`] will act over.
        pub fn set_entry(&mut self, entity: &S) -> &mut Self {
            self.set_key(&entity.key());
            self.set_data(entity);
            self
        }
    }
    impl<S: Entry + Debug, C: CrudOperation, T: OpTarget> Debug for Action<S, C, T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.debug_struct("Action")
                .field("kind", &self.kind())
                .field("table_name", &self.inner.table_name)
                .field("data", &self.inner.data)
                .field("key", &self.inner.key)
                .field("target", &self.target())
                .finish()
        }
    }
    impl<S: Entry + Clone, C: CrudOperation, T: OpTarget> Clone for Action<S, C, T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
                validated: self.validated.clone(),
            }
        }
    }
    impl<S: Entry, C: CrudOperation, T: OpTarget> Default for Action<S, C, T> {
        fn default() -> Self {
            Self {
                inner: InternalAction::default(),
                validated: Cell::default(),
            }
        }
    }
    pub(crate) struct InternalAction<S, C: CrudOperation, T: OpTarget> {
        kind: PhantomData<C>,
        table_name: Option<String>,
        data: Option<Box<S>>,
        key: Option<String>,
        target: PhantomData<T>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<S, C: CrudOperation, T: OpTarget> _serde::Serialize for InternalAction<S, C, T>
        where
            S: _serde::Serialize,
        {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "InternalAction",
                    false as usize + 1 + 1 + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "kind",
                    &self.kind,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "table_name",
                    &self.table_name,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "data",
                    &self.data,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "key",
                    &self.key,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "target",
                    &self.target,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de, S, C: CrudOperation, T: OpTarget> _serde::Deserialize<'de> for InternalAction<S, C, T>
        where
            S: _serde::Deserialize<'de>,
        {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            4u64 => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "kind" => _serde::__private::Ok(__Field::__field0),
                            "table_name" => _serde::__private::Ok(__Field::__field1),
                            "data" => _serde::__private::Ok(__Field::__field2),
                            "key" => _serde::__private::Ok(__Field::__field3),
                            "target" => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"kind" => _serde::__private::Ok(__Field::__field0),
                            b"table_name" => _serde::__private::Ok(__Field::__field1),
                            b"data" => _serde::__private::Ok(__Field::__field2),
                            b"key" => _serde::__private::Ok(__Field::__field3),
                            b"target" => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de, S, C: CrudOperation, T: OpTarget>
                where
                    S: _serde::Deserialize<'de>,
                {
                    marker: _serde::__private::PhantomData<InternalAction<S, C, T>>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de, S, C: CrudOperation, T: OpTarget> _serde::de::Visitor<'de> for __Visitor<'de, S, C, T>
                where
                    S: _serde::Deserialize<'de>,
                {
                    type Value = InternalAction<S, C, T>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct InternalAction",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            PhantomData<C>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct InternalAction with 5 elements",
                                ));
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct InternalAction with 5 elements",
                                ));
                            }
                        };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<
                            Option<Box<S>>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct InternalAction with 5 elements",
                                ));
                            }
                        };
                        let __field3 = match match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    3usize,
                                    &"struct InternalAction with 5 elements",
                                ));
                            }
                        };
                        let __field4 = match match _serde::de::SeqAccess::next_element::<
                            PhantomData<T>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    4usize,
                                    &"struct InternalAction with 5 elements",
                                ));
                            }
                        };
                        _serde::__private::Ok(InternalAction {
                            kind: __field0,
                            table_name: __field1,
                            data: __field2,
                            key: __field3,
                            target: __field4,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<PhantomData<C>> =
                            _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Option<String>> =
                            _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Option<Box<S>>> =
                            _serde::__private::None;
                        let mut __field3: _serde::__private::Option<Option<String>> =
                            _serde::__private::None;
                        let mut __field4: _serde::__private::Option<PhantomData<T>> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "kind",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<PhantomData<C>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "table_name",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "data",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<Box<S>>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "key",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "target",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<PhantomData<T>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("kind") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("table_name") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("data") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("key") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("target") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(InternalAction {
                            kind: __field0,
                            table_name: __field1,
                            data: __field2,
                            key: __field3,
                            target: __field4,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["kind", "table_name", "data", "key", "target"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "InternalAction",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<InternalAction<S, C, T>>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl<S, C: CrudOperation, T: OpTarget> InternalAction<S, C, T> {
        pub(crate) fn new() -> Self {
            Self {
                kind: PhantomData,
                table_name: None,
                data: None,
                key: None,
                target: PhantomData,
            }
        }
        #[allow(clippy::unused_self)]
        pub(crate) fn kind(&self) -> ActionKind {
            C::kind()
        }
        #[allow(clippy::unused_self)]
        pub(crate) fn target(&self) -> OperationTarget {
            T::target()
        }
        pub(crate) fn into_target<New: OpTarget>(self) -> InternalAction<S, C, New> {
            InternalAction {
                kind: PhantomData,
                table_name: self.table_name,
                data: self.data,
                key: self.key,
                target: PhantomData,
            }
        }
        pub(crate) fn into_operation<New: CrudOperation>(self) -> InternalAction<S, New, T> {
            InternalAction {
                kind: PhantomData,
                table_name: self.table_name,
                data: self.data,
                key: self.key,
                target: PhantomData,
            }
        }
        pub(crate) fn with_entry<S2>(self) -> InternalAction<S2, C, T> {
            InternalAction {
                kind: self.kind,
                table_name: self.table_name,
                data: None,
                key: None,
                target: self.target,
            }
        }
    }
    impl<S: Entry, C: CrudOperation, O: OpTarget> InternalAction<S, C, O> {
        pub(crate) fn set_table_name(&mut self, table_name: String) -> &mut Self {
            self.table_name = Some(table_name);
            self
        }
        pub(crate) fn set_key(&mut self, key: String) -> &mut Self {
            self.key = Some(key);
            self
        }
        pub(crate) fn set_entry(&mut self, entity: Box<S>) -> &mut Self {
            self.data = Some(entity);
            self
        }
        pub(crate) fn set_data(&mut self, data: S) -> &mut Self {
            self.data = Some(Box::new(data));
            self
        }
    }
    impl<S: Entry + Clone, C: CrudOperation, T: OpTarget> Clone for InternalAction<S, C, T> {
        fn clone(&self) -> Self {
            Self {
                kind: PhantomData,
                table_name: self.table_name.clone(),
                data: self.data.clone(),
                key: self.key.clone(),
                target: PhantomData,
            }
        }
    }
    impl<S: Entry, C: CrudOperation, T: OpTarget> Default for InternalAction<S, C, T> {
        fn default() -> Self {
            Self {
                kind: PhantomData,
                table_name: Option::default(),
                data: Option::default(),
                key: Option::default(),
                target: PhantomData,
            }
        }
    }
}
