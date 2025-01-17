#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::sync::Arc;
use async_nats::ConnectOptions;
use futures::{future::BoxFuture, StreamExt};
#[cfg(test)]
mod tests {
    use core::panic;
    use std::{collections::HashMap, future::Future};
    use async_nats::Client;
    use message_flow_drive::{event_pattern, msg_flow, msg_pattern, MsgDef};
    use serde::{
        de::{DeserializeOwned, IntoDeserializer},
        Deserialize, Serialize, Serializer,
    };
    use super::*;
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "tests::main"]
    #[doc(hidden)]
    pub const main: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::main"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "src/lib.rs",
            start_line: 23usize,
            start_col: 14usize,
            end_line: 23usize,
            end_col: 18usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(main())),
    };
    fn main() {
        let body = async {};
        let mut body = body;
        #[allow(unused_mut)]
        let mut body = unsafe {
            ::tokio::macros::support::Pin::new_unchecked(&mut body)
        };
        let body: ::core::pin::Pin<&mut dyn ::core::future::Future<Output = ()>> = body;
        #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
        {
            return tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Failed building the Runtime")
                .block_on(body);
        }
    }
    pub async fn connect() -> Result<(), Box<dyn std::error::Error>> {
        let client = ConnectOptions::new().connect("localhost:4222").await?;
        run_h(client).await
    }
    async fn run_h(client: Client) -> Result<(), Box<dyn std::error::Error>> {
        let mut requests = client.subscribe("service_A.*").await?;
        let handle = tokio::spawn({
            let client = client.clone();
            async move {
                while let Some(request) = requests.next().await {
                    let key: &String = &request.subject.chars().skip(10).collect();
                    if let Some(reply) = request.reply {}
                }
                Ok::<(), async_nats::Error>(())
            }
        });
        let _ = handle.await;
        Ok(())
    }
    pub trait Message {
        fn to_json(&self) -> String {
            "fdf".into()
        }
    }
    trait Handler {
        const STRUCT_NAME: &str;
        async fn handle(&self, subject: String);
    }
    struct User {
        first_name: String,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for User {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "User",
                    false as usize + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "first_name",
                    &self.first_name,
                )?;
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
        impl<'de> _serde::Deserialize<'de> for User {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
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
                            "first_name" => _serde::__private::Ok(__Field::__field0),
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
                            b"first_name" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
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
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<User>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = User;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct User",
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
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct User with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(User { first_name: __field0 })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "first_name",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("first_name")?
                            }
                        };
                        _serde::__private::Ok(User { first_name: __field0 })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["first_name"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "User",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<User>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Message for User {}
    impl Handler for User {
        const STRUCT_NAME: &str = "User";
        async fn handle(&self, subject: String) {
            let func = match subject.as_str() {
                "greeting" => self.greeting(),
                _ => {
                    ::core::panicking::panic_fmt(format_args!("HEELOOO"));
                }
            };
            let result = func.to_json();
        }
    }
    impl User {
        fn greeting(&self) -> impl Message {
            User { first_name: "HKH".into() }
        }
        fn pattern_a(&self) -> impl Message {
            let func = || User { first_name: "Hello".into() };
            func()
        }
        async fn pattern_b(&self) -> impl Message {
            let func = || User { first_name: "Hello".into() };
            func()
        }
        async fn pattern_c(&self) -> impl Message {
            let func = || User {
                first_name: "Hossein".into(),
            };
            func()
        }
    }
    fn test_a() -> Box<dyn Message> {
        Box::new(User { first_name: "He".into() })
    }
    const _: () = {
        const PATTERNS: [(&str, fn() -> Box<dyn Message>); 1] = [("greeting", test_a)];
    };
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&main])
}
