#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IPenDeviceInteropStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPenDeviceInteropStatics {
    type Vtable = IPenDeviceInteropStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3265634090,
        57463,
        23856,
        [161, 189, 207, 132, 221, 9, 238, 57],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenDeviceInteropStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "vices_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pointerpoint: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "vices_Input"))] usize,
);
pub struct PenDeviceInterop {}
impl PenDeviceInterop {
    #[cfg(feature = "vices_Input")]
    pub fn FromPointerPoint<'a, Param0: ::windows::runtime::IntoParam<'a, super::PointerPoint>>(
        pointerpoint: Param0,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::Devices::Input::PenDevice>
    {
        Self::IPenDeviceInteropStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                pointerpoint.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Devices::Input::PenDevice>(result__)
        })
    }
    pub fn IPenDeviceInteropStatics<
        R,
        F: FnOnce(&IPenDeviceInteropStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            PenDeviceInterop,
            IPenDeviceInteropStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for PenDeviceInterop {
    const NAME: &'static str = "Microsoft.UI.Input.Interop.PenDeviceInterop";
}
