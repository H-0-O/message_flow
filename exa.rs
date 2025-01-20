#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use futures::StreamExt;
use message_flow::{registers, Result};
use message_flow_drive::{msg_flow, MsgDef};
use serde::{Deserialize, Serialize};
struct User {
    first_name: String,
}
#[automatically_derived]
impl message_flow::Message for User {}
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
                    _serde::__private::Formatter::write_str(__formatter, "struct User")
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
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "main"]
#[doc(hidden)]
pub const main: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("main"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/code_gen.rs",
        start_line: 14usize,
        start_col: 10usize,
        end_line: 14usize,
        end_col: 14usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(main())),
};
#[allow(dead_code)]
fn main() {
    let body = async {
        let re = message_flow::connection::connect(
                "".into(),
                {
                    use message_flow::Register;
                    &[<User>::register as message_flow::RegisterFn]
                },
            )
            .await;
        let rr = {
            use message_flow::Register;
            &[<User>::register as message_flow::RegisterFn]
        };
        {
            ::std::io::_print(format_args!("HELLO\n"));
        };
        let usr = User { first_name: "HO".into() };
        {
            ::std::io::_print(format_args!("{0:?}\n", usr.gr().await.unwrap()));
        };
    };
    let mut body = body;
    #[allow(unused_mut)]
    let mut body = unsafe { ::tokio::macros::support::Pin::new_unchecked(&mut body) };
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
const _: () = {
    #[allow(unused_imports)]
    use message_flow::Register;
    #[automatically_derived]
    impl message_flow::Register for User {
        #[allow(
            elided_named_lifetimes,
            clippy::async_yields_async,
            clippy::diverging_sub_expression,
            clippy::let_unit_value,
            clippy::needless_arbitrary_self_type,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn register<'async_trait>(
            client: std::sync::Arc<message_flow::Client>,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = message_flow::Result<()>,
                > + ::core::marker::Send + 'async_trait,
            >,
        > {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    message_flow::Result<()>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let client = client;
                let __ret: message_flow::Result<()> = {
                    let mut subscribe = client.subscribe("user").await?;
                    tokio::spawn({
                        let client = client.clone();
                        async move {
                            while let Some(request) = subscribe.next().await {
                                let __result = User::router(
                                        &request.subject.to_string(),
                                        request.payload.as_ref(),
                                    )
                                    .await;
                                if let Err(err) = __result {
                                    return Err(err);
                                }
                                if let Some(reply) = request.reply {
                                    let _ = client
                                        .publish(reply, __result.unwrap().to_json().into())
                                        .await?;
                                }
                            }
                            Ok::<(), async_nats::Error>(())
                        }
                    });
                    Ok(())
                };
                #[allow(unreachable_code)] __ret
            })
        }
    }
    #[allow(unused_imports)]
    use message_flow::Handler;
    #[automatically_derived]
    impl message_flow::Handler for User {
        #[allow(
            elided_named_lifetimes,
            clippy::async_yields_async,
            clippy::diverging_sub_expression,
            clippy::let_unit_value,
            clippy::needless_arbitrary_self_type,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn router<'life0, 'life1, 'async_trait>(
            subject: &'life0 String,
            payload: &'life1 [u8],
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = message_flow::Result<
                        ::std::boxed::Box<dyn message_flow::Message>,
                    >,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            'life1: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    message_flow::Result<::std::boxed::Box<dyn message_flow::Message>>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let __ret: message_flow::Result<
                    ::std::boxed::Box<dyn message_flow::Message>,
                > = {
                    let resolver = serde_json::from_slice::<Self>(payload).unwrap();
                    {
                        ::std::io::_print(
                            format_args!("IN HANDLE and message {0:?} \n", subject),
                        );
                    };
                    let func: ::std::boxed::Box<dyn message_flow::Message> = match subject
                        .as_str()
                    {
                        "service_A.greeting" => {
                            ::std::boxed::Box::new(resolver.greeting().await?)
                        }
                        _ => return Err(async_nats::Error::from("Pattern Not found")),
                    };
                    Ok(func)
                };
                #[allow(unreachable_code)] __ret
            })
        }
    }
    impl User {
        async fn greeting(&self) -> Result<String> {
            Ok("OK HEEELO FROM GREETING".into())
        }
        async fn gr(&self) -> Result<String> {
            Ok("OK HEEELO FROM gr".into())
        }
    }
};
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&main])
}
