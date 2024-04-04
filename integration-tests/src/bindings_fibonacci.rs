pub use fibonacci::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod fibonacci {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("fibonacci"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("fibonacci"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("n"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static FIBONACCI_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15`\x0FW`\0\x80\xFD[Pa\x01\xE2\x80a\0\x1F`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80ca\x04\x7F\xF4\x14a\x000W[`\0\x80\xFD[a\0J`\x04\x806\x03\x81\x01\x90a\0E\x91\x90a\0\xF2V[a\0`V[`@Qa\0W\x91\x90a\x01.V[`@Q\x80\x91\x03\x90\xF3[`\0\x80\x82\x03a\0rW`\0\x90Pa\0\xB2V[`\0`\x01\x90P`\x01\x91P`\0`\x02\x90P[\x83\x81\x10\x15a\0\xAFW`\0\x83\x83a\0\x99\x91\x90a\x01xV[\x90P\x83\x92P\x80\x93PP\x80\x80`\x01\x01\x91PPa\0\x83V[PP[\x91\x90PV[`\0\x80\xFD[`\0\x81\x90P\x91\x90PV[a\0\xCF\x81a\0\xBCV[\x81\x14a\0\xDAW`\0\x80\xFD[PV[`\0\x815\x90Pa\0\xEC\x81a\0\xC6V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x01\x08Wa\x01\x07a\0\xB7V[[`\0a\x01\x16\x84\x82\x85\x01a\0\xDDV[\x91PP\x92\x91PPV[a\x01(\x81a\0\xBCV[\x82RPPV[`\0` \x82\x01\x90Pa\x01C`\0\x83\x01\x84a\x01\x1FV[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a\x01\x83\x82a\0\xBCV[\x91Pa\x01\x8E\x83a\0\xBCV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x01\xA6Wa\x01\xA5a\x01IV[[\x92\x91PPV\xFE\xA2dipfsX\"\x12 9\xC7.\x1A\x04-\xD9\xD2\xDF\xA3\x1EC\xA7\xAD+T\x96%d\x08\x9B\x885\xC8\x9C\x04\xC5o)\xBD\xFE\xC5dsolcC\0\x08\x19\x003";
    /// The bytecode of the contract.
    pub static FIBONACCI_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    pub struct fibonacci<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for fibonacci<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for fibonacci<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for fibonacci<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for fibonacci<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(fibonacci)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> fibonacci<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    FIBONACCI_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                FIBONACCI_ABI.clone(),
                FIBONACCI_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `fibonacci` (0x61047ff4) function
        pub fn fibonacci(
            &self,
            n: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([97, 4, 127, 244], n)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for fibonacci<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `fibonacci` function with signature `fibonacci(uint256)` and selector `0x61047ff4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "fibonacci", abi = "fibonacci(uint256)")]
    pub struct FibonacciCall {
        pub n: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `fibonacci` function with signature `fibonacci(uint256)` and selector `0x61047ff4`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct FibonacciReturn {
        pub b: ::ethers::core::types::U256,
    }
}
