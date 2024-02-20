// Generated by `wit-bindgen` 0.16.0. DO NOT EDIT!
pub mod miden {
    pub mod base {

        #[allow(clippy::all)]
        pub mod types {
            #[used]
            #[doc(hidden)]
            #[cfg(target_arch = "wasm32")]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_section;
            /// Represents base field element in the field using Montgomery representation.
            /// Internal values represent x * R mod M where R = 2^64 mod M and x in [0, M).
            /// The backing type is `u64` but the internal values are always in the range [0, M).
            /// Field modulus M = 2^64 - 2^32 + 1
            pub type Felt = u64;
            /// A group of four field elements in the Miden base field.
            pub type Word = (Felt, Felt, Felt, Felt);
            /// Unique identifier of an account.
            ///
            /// Account ID consists of 1 field element (~64 bits). This field element uniquely identifies a
            /// single account and also specifies the type of the underlying account. Specifically:
            /// - The two most significant bits of the ID specify the type of the account:
            /// - 00 - regular account with updatable code.
            /// - 01 - regular account with immutable code.
            /// - 10 - fungible asset faucet with immutable code.
            /// - 11 - non-fungible asset faucet with immutable code.
            /// - The third most significant bit of the ID specifies whether the account data is stored on-chain:
            /// - 0 - full account data is stored on-chain.
            /// - 1 - only the account hash is stored on-chain which serves as a commitment to the account state.
            /// As such the three most significant bits fully describes the type of the account.
            pub type AccountId = Felt;
            /// Recipient of the note, i.e., hash(hash(hash(serial_num, [0; 4]), note_script_hash), input_hash)
            pub type Recipient = Word;
            pub type Tag = Felt;
            /// A fungible asset
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct FungibleAsset {
                /// Faucet ID of the faucet which issued the asset as well as the asset amount.
                pub asset: AccountId,
                /// Asset amount is guaranteed to be 2^63 - 1 or smaller.
                pub amount: u64,
            }
            impl ::core::fmt::Debug for FungibleAsset {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("FungibleAsset")
                        .field("asset", &self.asset)
                        .field("amount", &self.amount)
                        .finish()
                }
            }
            /// A commitment to a non-fungible asset.
            ///
            /// A non-fungible asset consists of 4 field elements which are computed by hashing asset data
            /// (which can be of arbitrary length) to produce: [d0, d1, d2, d3].  We then replace d1 with the
            /// faucet_id that issued the asset: [d0, faucet_id, d2, d3]. We then set the most significant bit
            /// of the most significant element to ZERO.
            pub type NonFungibleAsset = Word;
            /// A fungible or a non-fungible asset.
            ///
            /// All assets are encoded using a single word (4 elements) such that it is easy to determine the
            /// type of an asset both inside and outside Miden VM. Specifically:
            /// Element 1 will be:
            /// - ZERO for a fungible asset
            /// - non-ZERO for a non-fungible asset
            /// The most significant bit will be:
            /// - ONE for a fungible asset
            /// - ZERO for a non-fungible asset
            ///
            /// The above properties guarantee that there can never be a collision between a fungible and a
            /// non-fungible asset.
            ///
            /// The methodology for constructing fungible and non-fungible assets is described below.
            ///
            /// # Fungible assets
            /// The most significant element of a fungible asset is set to the ID of the faucet which issued
            /// the asset. This guarantees the properties described above (the first bit is ONE).
            ///
            /// The least significant element is set to the amount of the asset. This amount cannot be greater
            /// than 2^63 - 1 and thus requires 63-bits to store.
            ///
            /// Elements 1 and 2 are set to ZERO.
            ///
            /// It is impossible to find a collision between two fungible assets issued by different faucets as
            /// the faucet_id is included in the description of the asset and this is guaranteed to be different
            /// for each faucet as per the faucet creation logic.
            ///
            /// # Non-fungible assets
            /// The 4 elements of non-fungible assets are computed as follows:
            /// - First the asset data is hashed. This compresses an asset of an arbitrary length to 4 field
            /// elements: [d0, d1, d2, d3].
            /// - d1 is then replaced with the faucet_id which issues the asset: [d0, faucet_id, d2, d3].
            /// - Lastly, the most significant bit of d3 is set to ZERO.
            ///
            /// It is impossible to find a collision between two non-fungible assets issued by different faucets
            /// as the faucet_id is included in the description of the non-fungible asset and this is guaranteed
            /// to be different as per the faucet creation logic. Collision resistance for non-fungible assets
            /// issued by the same faucet is ~2^95.
            #[derive(Clone, Copy)]
            pub enum Asset {
                Fungible(FungibleAsset),
                NonFungible(NonFungibleAsset),
            }
            impl ::core::fmt::Debug for Asset {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self {
                        Asset::Fungible(e) => f.debug_tuple("Asset::Fungible").field(e).finish(),
                        Asset::NonFungible(e) => {
                            f.debug_tuple("Asset::NonFungible").field(e).finish()
                        }
                    }
                }
            }
            /// Inputs of the currently executed note, never exceeds 16 felts
            pub type NoteInputs = ::cargo_component_bindings::rt::vec::Vec<Felt>;
        }

        #[allow(clippy::all)]
        pub mod tx_kernel {
            #[used]
            #[doc(hidden)]
            #[cfg(target_arch = "wasm32")]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_section;
            pub type Asset = super::super::super::miden::base::types::Asset;
            pub type Tag = super::super::super::miden::base::types::Tag;
            pub type Recipient = super::super::super::miden::base::types::Recipient;
            pub type NoteInputs = super::super::super::miden::base::types::NoteInputs;
            pub type AccountId = super::super::super::miden::base::types::AccountId;
            #[allow(unused_unsafe, clippy::all)]
            /// Account-related functions
            /// Get the id of the currently executing account
            pub fn get_id() -> AccountId {
                #[allow(unused_imports)]
                use cargo_component_bindings::rt::{alloc, string::String, vec::Vec};
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:base/tx-kernel@1.0.0")]
                    extern "C" {
                        #[link_name = "get-id"]
                        fn wit_import() -> i64;
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() -> i64 {
                        unreachable!()
                    }
                    let ret = wit_import();
                    ret as u64
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Add the specified asset to the vault
            pub fn add_asset(asset: Asset) -> Asset {
                #[allow(unused_imports)]
                use cargo_component_bindings::rt::{alloc, string::String, vec::Vec};
                unsafe {
                    #[repr(align(8))]
                    struct RetArea([u8; 40]);
                    let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                    use super::super::super::miden::base::types::Asset as V2;
                    let (result3_0, result3_1, result3_2, result3_3, result3_4) = match asset {
                        V2::Fungible(e) => {
                            let super::super::super::miden::base::types::FungibleAsset {
                                asset: asset0,
                                amount: amount0,
                            } = e;

                            (
                                0i32,
                                ::cargo_component_bindings::rt::as_i64(asset0),
                                ::cargo_component_bindings::rt::as_i64(amount0),
                                0i64,
                                0i64,
                            )
                        }
                        V2::NonFungible(e) => {
                            let (t1_0, t1_1, t1_2, t1_3) = e;

                            (
                                1i32,
                                ::cargo_component_bindings::rt::as_i64(t1_0),
                                ::cargo_component_bindings::rt::as_i64(t1_1),
                                ::cargo_component_bindings::rt::as_i64(t1_2),
                                ::cargo_component_bindings::rt::as_i64(t1_3),
                            )
                        }
                    };
                    let ptr4 = ret_area.as_mut_ptr() as i32;
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:base/tx-kernel@1.0.0")]
                    extern "C" {
                        #[link_name = "add-asset"]
                        fn wit_import(_: i32, _: i64, _: i64, _: i64, _: i64, _: i32);
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32, _: i64, _: i64, _: i64, _: i64, _: i32) {
                        unreachable!()
                    }
                    wit_import(result3_0, result3_1, result3_2, result3_3, result3_4, ptr4);
                    let l5 = i32::from(*((ptr4 + 0) as *const u8));
                    use super::super::super::miden::base::types::Asset as V12;
                    let v12 = match l5 {
                        0 => {
                            let e12 = {
                                let l6 = *((ptr4 + 8) as *const i64);
                                let l7 = *((ptr4 + 16) as *const i64);

                                super::super::super::miden::base::types::FungibleAsset {
                                    asset: l6 as u64,
                                    amount: l7 as u64,
                                }
                            };
                            V12::Fungible(e12)
                        }
                        n => {
                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                            let e12 = {
                                let l8 = *((ptr4 + 8) as *const i64);
                                let l9 = *((ptr4 + 16) as *const i64);
                                let l10 = *((ptr4 + 24) as *const i64);
                                let l11 = *((ptr4 + 32) as *const i64);

                                (l8 as u64, l9 as u64, l10 as u64, l11 as u64)
                            };
                            V12::NonFungible(e12)
                        }
                    };
                    v12
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Remove the specified asset from the vault
            pub fn remove_asset(asset: Asset) -> Asset {
                #[allow(unused_imports)]
                use cargo_component_bindings::rt::{alloc, string::String, vec::Vec};
                unsafe {
                    #[repr(align(8))]
                    struct RetArea([u8; 40]);
                    let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                    use super::super::super::miden::base::types::Asset as V2;
                    let (result3_0, result3_1, result3_2, result3_3, result3_4) = match asset {
                        V2::Fungible(e) => {
                            let super::super::super::miden::base::types::FungibleAsset {
                                asset: asset0,
                                amount: amount0,
                            } = e;

                            (
                                0i32,
                                ::cargo_component_bindings::rt::as_i64(asset0),
                                ::cargo_component_bindings::rt::as_i64(amount0),
                                0i64,
                                0i64,
                            )
                        }
                        V2::NonFungible(e) => {
                            let (t1_0, t1_1, t1_2, t1_3) = e;

                            (
                                1i32,
                                ::cargo_component_bindings::rt::as_i64(t1_0),
                                ::cargo_component_bindings::rt::as_i64(t1_1),
                                ::cargo_component_bindings::rt::as_i64(t1_2),
                                ::cargo_component_bindings::rt::as_i64(t1_3),
                            )
                        }
                    };
                    let ptr4 = ret_area.as_mut_ptr() as i32;
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:base/tx-kernel@1.0.0")]
                    extern "C" {
                        #[link_name = "remove-asset"]
                        fn wit_import(_: i32, _: i64, _: i64, _: i64, _: i64, _: i32);
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32, _: i64, _: i64, _: i64, _: i64, _: i32) {
                        unreachable!()
                    }
                    wit_import(result3_0, result3_1, result3_2, result3_3, result3_4, ptr4);
                    let l5 = i32::from(*((ptr4 + 0) as *const u8));
                    use super::super::super::miden::base::types::Asset as V12;
                    let v12 = match l5 {
                        0 => {
                            let e12 = {
                                let l6 = *((ptr4 + 8) as *const i64);
                                let l7 = *((ptr4 + 16) as *const i64);

                                super::super::super::miden::base::types::FungibleAsset {
                                    asset: l6 as u64,
                                    amount: l7 as u64,
                                }
                            };
                            V12::Fungible(e12)
                        }
                        n => {
                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                            let e12 = {
                                let l8 = *((ptr4 + 8) as *const i64);
                                let l9 = *((ptr4 + 16) as *const i64);
                                let l10 = *((ptr4 + 24) as *const i64);
                                let l11 = *((ptr4 + 32) as *const i64);

                                (l8 as u64, l9 as u64, l10 as u64, l11 as u64)
                            };
                            V12::NonFungible(e12)
                        }
                    };
                    v12
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Note-related functions
            /// Creates a new note.
            /// asset is the asset to be included in the note.
            /// tag is the tag to be included in the note.
            /// recipient is the recipient of the note.
            pub fn create_note(asset: Asset, tag: Tag, recipient: Recipient) {
                #[allow(unused_imports)]
                use cargo_component_bindings::rt::{alloc, string::String, vec::Vec};
                unsafe {
                    use super::super::super::miden::base::types::Asset as V2;
                    let (result3_0, result3_1, result3_2, result3_3, result3_4) = match asset {
                        V2::Fungible(e) => {
                            let super::super::super::miden::base::types::FungibleAsset {
                                asset: asset0,
                                amount: amount0,
                            } = e;

                            (
                                0i32,
                                ::cargo_component_bindings::rt::as_i64(asset0),
                                ::cargo_component_bindings::rt::as_i64(amount0),
                                0i64,
                                0i64,
                            )
                        }
                        V2::NonFungible(e) => {
                            let (t1_0, t1_1, t1_2, t1_3) = e;

                            (
                                1i32,
                                ::cargo_component_bindings::rt::as_i64(t1_0),
                                ::cargo_component_bindings::rt::as_i64(t1_1),
                                ::cargo_component_bindings::rt::as_i64(t1_2),
                                ::cargo_component_bindings::rt::as_i64(t1_3),
                            )
                        }
                    };
                    let (t4_0, t4_1, t4_2, t4_3) = recipient;

                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:base/tx-kernel@1.0.0")]
                    extern "C" {
                        #[link_name = "create-note"]
                        fn wit_import(
                            _: i32,
                            _: i64,
                            _: i64,
                            _: i64,
                            _: i64,
                            _: i64,
                            _: i64,
                            _: i64,
                            _: i64,
                            _: i64,
                        );
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(
                        _: i32,
                        _: i64,
                        _: i64,
                        _: i64,
                        _: i64,
                        _: i64,
                        _: i64,
                        _: i64,
                        _: i64,
                        _: i64,
                    ) {
                        unreachable!()
                    }
                    wit_import(
                        result3_0,
                        result3_1,
                        result3_2,
                        result3_3,
                        result3_4,
                        ::cargo_component_bindings::rt::as_i64(tag),
                        ::cargo_component_bindings::rt::as_i64(t4_0),
                        ::cargo_component_bindings::rt::as_i64(t4_1),
                        ::cargo_component_bindings::rt::as_i64(t4_2),
                        ::cargo_component_bindings::rt::as_i64(t4_3),
                    );
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Get the inputs of the currently executed note
            pub fn get_inputs() -> NoteInputs {
                #[allow(unused_imports)]
                use cargo_component_bindings::rt::{alloc, string::String, vec::Vec};
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([u8; 8]);
                    let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                    let ptr0 = ret_area.as_mut_ptr() as i32;
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:base/tx-kernel@1.0.0")]
                    extern "C" {
                        #[link_name = "get-inputs"]
                        fn wit_import(_: i32);
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = *((ptr0 + 0) as *const i32);
                    let l2 = *((ptr0 + 4) as *const i32);
                    let len3 = l2 as usize;
                    Vec::from_raw_parts(l1 as *mut _, len3, len3)
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Get the assets of the currently executing note
            pub fn get_assets() -> ::cargo_component_bindings::rt::vec::Vec<Asset> {
                #[allow(unused_imports)]
                use cargo_component_bindings::rt::{alloc, string::String, vec::Vec};
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([u8; 8]);
                    let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                    let ptr0 = ret_area.as_mut_ptr() as i32;
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:base/tx-kernel@1.0.0")]
                    extern "C" {
                        #[link_name = "get-assets"]
                        fn wit_import(_: i32);
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = *((ptr0 + 0) as *const i32);
                    let l2 = *((ptr0 + 4) as *const i32);
                    let base11 = l1;
                    let len11 = l2;
                    let mut result11 = Vec::with_capacity(len11 as usize);
                    for i in 0..len11 {
                        let base = base11 + i * 40;
                        let e11 = {
                            let l3 = i32::from(*((base + 0) as *const u8));
                            use super::super::super::miden::base::types::Asset as V10;
                            let v10 = match l3 {
                                0 => {
                                    let e10 = {
                                        let l4 = *((base + 8) as *const i64);
                                        let l5 = *((base + 16) as *const i64);

                                        super::super::super::miden::base::types::FungibleAsset {
                                            asset: l4 as u64,
                                            amount: l5 as u64,
                                        }
                                    };
                                    V10::Fungible(e10)
                                }
                                n => {
                                    debug_assert_eq!(n, 1, "invalid enum discriminant");
                                    let e10 = {
                                        let l6 = *((base + 8) as *const i64);
                                        let l7 = *((base + 16) as *const i64);
                                        let l8 = *((base + 24) as *const i64);
                                        let l9 = *((base + 32) as *const i64);

                                        (l6 as u64, l7 as u64, l8 as u64, l9 as u64)
                                    };
                                    V10::NonFungible(e10)
                                }
                            };

                            v10
                        };
                        result11.push(e11);
                    }
                    ::cargo_component_bindings::rt::dealloc(base11, (len11 as usize) * 40, 8);
                    result11
                }
            }
        }
    }
    pub mod basic_wallet {

        #[allow(clippy::all)]
        pub mod basic_wallet {
            #[used]
            #[doc(hidden)]
            #[cfg(target_arch = "wasm32")]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_section;
            pub type Asset = super::super::super::miden::base::types::Asset;
            pub type Tag = super::super::super::miden::base::types::Tag;
            pub type Recipient = super::super::super::miden::base::types::Recipient;
            #[allow(unused_unsafe, clippy::all)]
            pub fn receive_asset(asset: Asset) {
                #[allow(unused_imports)]
                use cargo_component_bindings::rt::{alloc, string::String, vec::Vec};
                unsafe {
                    use super::super::super::miden::base::types::Asset as V2;
                    let (result3_0, result3_1, result3_2, result3_3, result3_4) = match asset {
                        V2::Fungible(e) => {
                            let super::super::super::miden::base::types::FungibleAsset {
                                asset: asset0,
                                amount: amount0,
                            } = e;

                            (
                                0i32,
                                ::cargo_component_bindings::rt::as_i64(asset0),
                                ::cargo_component_bindings::rt::as_i64(amount0),
                                0i64,
                                0i64,
                            )
                        }
                        V2::NonFungible(e) => {
                            let (t1_0, t1_1, t1_2, t1_3) = e;

                            (
                                1i32,
                                ::cargo_component_bindings::rt::as_i64(t1_0),
                                ::cargo_component_bindings::rt::as_i64(t1_1),
                                ::cargo_component_bindings::rt::as_i64(t1_2),
                                ::cargo_component_bindings::rt::as_i64(t1_3),
                            )
                        }
                    };

                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:basic-wallet/basic-wallet@1.0.0")]
                    extern "C" {
                        #[link_name = "receive-asset"]
                        fn wit_import(_: i32, _: i64, _: i64, _: i64, _: i64);
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32, _: i64, _: i64, _: i64, _: i64) {
                        unreachable!()
                    }
                    wit_import(result3_0, result3_1, result3_2, result3_3, result3_4);
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn send_asset(asset: Asset, tag: Tag, recipient: Recipient) {
                #[allow(unused_imports)]
                use cargo_component_bindings::rt::{alloc, string::String, vec::Vec};
                unsafe {
                    use super::super::super::miden::base::types::Asset as V2;
                    let (result3_0, result3_1, result3_2, result3_3, result3_4) = match asset {
                        V2::Fungible(e) => {
                            let super::super::super::miden::base::types::FungibleAsset {
                                asset: asset0,
                                amount: amount0,
                            } = e;

                            (
                                0i32,
                                ::cargo_component_bindings::rt::as_i64(asset0),
                                ::cargo_component_bindings::rt::as_i64(amount0),
                                0i64,
                                0i64,
                            )
                        }
                        V2::NonFungible(e) => {
                            let (t1_0, t1_1, t1_2, t1_3) = e;

                            (
                                1i32,
                                ::cargo_component_bindings::rt::as_i64(t1_0),
                                ::cargo_component_bindings::rt::as_i64(t1_1),
                                ::cargo_component_bindings::rt::as_i64(t1_2),
                                ::cargo_component_bindings::rt::as_i64(t1_3),
                            )
                        }
                    };
                    let (t4_0, t4_1, t4_2, t4_3) = recipient;

                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:basic-wallet/basic-wallet@1.0.0")]
                    extern "C" {
                        #[link_name = "send-asset"]
                        fn wit_import(
                            _: i32,
                            _: i64,
                            _: i64,
                            _: i64,
                            _: i64,
                            _: i64,
                            _: i64,
                            _: i64,
                            _: i64,
                            _: i64,
                        );
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(
                        _: i32,
                        _: i64,
                        _: i64,
                        _: i64,
                        _: i64,
                        _: i64,
                        _: i64,
                        _: i64,
                        _: i64,
                        _: i64,
                    ) {
                        unreachable!()
                    }
                    wit_import(
                        result3_0,
                        result3_1,
                        result3_2,
                        result3_3,
                        result3_4,
                        ::cargo_component_bindings::rt::as_i64(tag),
                        ::cargo_component_bindings::rt::as_i64(t4_0),
                        ::cargo_component_bindings::rt::as_i64(t4_1),
                        ::cargo_component_bindings::rt::as_i64(t4_2),
                        ::cargo_component_bindings::rt::as_i64(t4_3),
                    );
                }
            }
        }
    }
}
pub mod exports {
    pub mod miden {
        pub mod base {

            #[allow(clippy::all)]
            pub mod note {
                #[used]
                #[doc(hidden)]
                #[cfg(target_arch = "wasm32")]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_section;
                const _: () = {
                    #[doc(hidden)]
                    #[export_name = "miden:base/note@1.0.0#note-script"]
                    #[allow(non_snake_case)]
                    unsafe extern "C" fn __export_note_script() {
                        #[allow(unused_imports)]
                        use cargo_component_bindings::rt::{alloc, string::String, vec::Vec};

                        // Before executing any other code, use this function to run all static
                        // constructors, if they have not yet been run. This is a hack required
                        // to work around wasi-libc ctors calling import functions to initialize
                        // the environment.
                        //
                        // This functionality will be removed once rust 1.69.0 is stable, at which
                        // point wasi-libc will no longer have this behavior.
                        //
                        // See
                        // https://github.com/bytecodealliance/preview2-prototyping/issues/99
                        // for more details.
                        #[cfg(target_arch = "wasm32")]
                        ::cargo_component_bindings::rt::run_ctors_once();

                        <_GuestImpl as Guest>::note_script();
                    }
                };
                use super::super::super::super::super::Component as _GuestImpl;
                pub trait Guest {
                    fn note_script();
                }
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:notes-world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 950] = [
    3, 0, 11, 110, 111, 116, 101, 115, 45, 119, 111, 114, 108, 100, 0, 97, 115, 109, 13, 0, 1, 0,
    7, 176, 6, 1, 65, 2, 1, 65, 13, 1, 66, 14, 1, 119, 4, 0, 4, 102, 101, 108, 116, 3, 0, 0, 1,
    111, 4, 1, 1, 1, 1, 4, 0, 4, 119, 111, 114, 100, 3, 0, 2, 4, 0, 10, 97, 99, 99, 111, 117, 110,
    116, 45, 105, 100, 3, 0, 1, 4, 0, 9, 114, 101, 99, 105, 112, 105, 101, 110, 116, 3, 0, 3, 4, 0,
    3, 116, 97, 103, 3, 0, 1, 1, 114, 2, 5, 97, 115, 115, 101, 116, 4, 6, 97, 109, 111, 117, 110,
    116, 119, 4, 0, 14, 102, 117, 110, 103, 105, 98, 108, 101, 45, 97, 115, 115, 101, 116, 3, 0, 7,
    4, 0, 18, 110, 111, 110, 45, 102, 117, 110, 103, 105, 98, 108, 101, 45, 97, 115, 115, 101, 116,
    3, 0, 3, 1, 113, 2, 8, 102, 117, 110, 103, 105, 98, 108, 101, 1, 8, 0, 12, 110, 111, 110, 45,
    102, 117, 110, 103, 105, 98, 108, 101, 1, 9, 0, 4, 0, 5, 97, 115, 115, 101, 116, 3, 0, 10, 1,
    112, 1, 4, 0, 11, 110, 111, 116, 101, 45, 105, 110, 112, 117, 116, 115, 3, 0, 12, 3, 1, 22,
    109, 105, 100, 101, 110, 58, 98, 97, 115, 101, 47, 116, 121, 112, 101, 115, 64, 49, 46, 48, 46,
    48, 5, 0, 2, 3, 0, 0, 5, 97, 115, 115, 101, 116, 2, 3, 0, 0, 3, 116, 97, 103, 2, 3, 0, 0, 9,
    114, 101, 99, 105, 112, 105, 101, 110, 116, 2, 3, 0, 0, 11, 110, 111, 116, 101, 45, 105, 110,
    112, 117, 116, 115, 2, 3, 0, 0, 10, 97, 99, 99, 111, 117, 110, 116, 45, 105, 100, 1, 66, 22, 2,
    3, 2, 1, 1, 4, 0, 5, 97, 115, 115, 101, 116, 3, 0, 0, 2, 3, 2, 1, 2, 4, 0, 3, 116, 97, 103, 3,
    0, 2, 2, 3, 2, 1, 3, 4, 0, 9, 114, 101, 99, 105, 112, 105, 101, 110, 116, 3, 0, 4, 2, 3, 2, 1,
    4, 4, 0, 11, 110, 111, 116, 101, 45, 105, 110, 112, 117, 116, 115, 3, 0, 6, 2, 3, 2, 1, 5, 4,
    0, 10, 97, 99, 99, 111, 117, 110, 116, 45, 105, 100, 3, 0, 8, 1, 64, 0, 0, 9, 4, 0, 6, 103,
    101, 116, 45, 105, 100, 1, 10, 1, 64, 1, 5, 97, 115, 115, 101, 116, 1, 0, 1, 4, 0, 9, 97, 100,
    100, 45, 97, 115, 115, 101, 116, 1, 11, 4, 0, 12, 114, 101, 109, 111, 118, 101, 45, 97, 115,
    115, 101, 116, 1, 11, 1, 64, 3, 5, 97, 115, 115, 101, 116, 1, 3, 116, 97, 103, 3, 9, 114, 101,
    99, 105, 112, 105, 101, 110, 116, 5, 1, 0, 4, 0, 11, 99, 114, 101, 97, 116, 101, 45, 110, 111,
    116, 101, 1, 12, 1, 64, 0, 0, 7, 4, 0, 10, 103, 101, 116, 45, 105, 110, 112, 117, 116, 115, 1,
    13, 1, 112, 1, 1, 64, 0, 0, 14, 4, 0, 10, 103, 101, 116, 45, 97, 115, 115, 101, 116, 115, 1,
    15, 3, 1, 26, 109, 105, 100, 101, 110, 58, 98, 97, 115, 101, 47, 116, 120, 45, 107, 101, 114,
    110, 101, 108, 64, 49, 46, 48, 46, 48, 5, 6, 1, 66, 10, 2, 3, 2, 1, 1, 4, 0, 5, 97, 115, 115,
    101, 116, 3, 0, 0, 2, 3, 2, 1, 2, 4, 0, 3, 116, 97, 103, 3, 0, 2, 2, 3, 2, 1, 3, 4, 0, 9, 114,
    101, 99, 105, 112, 105, 101, 110, 116, 3, 0, 4, 1, 64, 1, 5, 97, 115, 115, 101, 116, 1, 1, 0,
    4, 0, 13, 114, 101, 99, 101, 105, 118, 101, 45, 97, 115, 115, 101, 116, 1, 6, 1, 64, 3, 5, 97,
    115, 115, 101, 116, 1, 3, 116, 97, 103, 3, 9, 114, 101, 99, 105, 112, 105, 101, 110, 116, 5, 1,
    0, 4, 0, 10, 115, 101, 110, 100, 45, 97, 115, 115, 101, 116, 1, 7, 3, 1, 37, 109, 105, 100,
    101, 110, 58, 98, 97, 115, 105, 99, 45, 119, 97, 108, 108, 101, 116, 47, 98, 97, 115, 105, 99,
    45, 119, 97, 108, 108, 101, 116, 64, 49, 46, 48, 46, 48, 5, 7, 1, 66, 2, 1, 64, 0, 1, 0, 4, 0,
    11, 110, 111, 116, 101, 45, 115, 99, 114, 105, 112, 116, 1, 0, 4, 1, 21, 109, 105, 100, 101,
    110, 58, 98, 97, 115, 101, 47, 110, 111, 116, 101, 64, 49, 46, 48, 46, 48, 5, 8, 4, 1, 28, 109,
    105, 100, 101, 110, 58, 112, 50, 105, 100, 47, 110, 111, 116, 101, 115, 45, 119, 111, 114, 108,
    100, 64, 49, 46, 48, 46, 48, 4, 0, 11, 17, 1, 0, 11, 110, 111, 116, 101, 115, 45, 119, 111,
    114, 108, 100, 3, 0, 0, 0, 16, 12, 112, 97, 99, 107, 97, 103, 101, 45, 100, 111, 99, 115, 0,
    123, 125, 0, 70, 9, 112, 114, 111, 100, 117, 99, 101, 114, 115, 1, 12, 112, 114, 111, 99, 101,
    115, 115, 101, 100, 45, 98, 121, 2, 13, 119, 105, 116, 45, 99, 111, 109, 112, 111, 110, 101,
    110, 116, 6, 48, 46, 49, 56, 46, 50, 16, 119, 105, 116, 45, 98, 105, 110, 100, 103, 101, 110,
    45, 114, 117, 115, 116, 6, 48, 46, 49, 54, 46, 48,
];

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_section() {}
