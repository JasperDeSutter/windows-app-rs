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
pub struct DesktopAcrylicController(::windows::runtime::IInspectable);
impl DesktopAcrylicController {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            DesktopAcrylicController,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "")]
    pub fn FallbackColor(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Color>(result__)
        }
    }
    #[cfg(feature = "")]
    pub fn SetFallbackColor<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Windows::UI::Color>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn LuminosityOpacity(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn SetLuminosityOpacity(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "")]
    pub fn TintColor(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Color>(result__)
        }
    }
    #[cfg(feature = "")]
    pub fn SetTintColor<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Windows::UI::Color>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn TintOpacity(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn SetTintOpacity(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "_Composition")]
    pub fn SetTargetWithWindowId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::WindowId>,
        Param1: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::UI::Composition::CompositionTarget,
        >,
    >(
        &self,
        windowid: Param0,
        desktopwindowtarget: Param1,
    ) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ISystemBackdropController>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                windowid.into_param().abi(),
                desktopwindowtarget.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "_Composition", feature = "_Core"))]
    pub fn SetTargetWithCoreWindow<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::UI::Core::CoreWindow,
        >,
        Param1: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::UI::Composition::CompositionTarget,
        >,
    >(
        &self,
        corewindow: Param0,
        compositiontarget: Param1,
    ) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ISystemBackdropController>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                corewindow.into_param().abi(),
                compositiontarget.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::super::super::Windows::Foundation::IClosable,
        >(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn IsSupported() -> ::windows::runtime::Result<bool> {
        Self::IDesktopAcrylicControllerStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn IDesktopAcrylicControllerStatics<
        R,
        F: FnOnce(&IDesktopAcrylicControllerStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            DesktopAcrylicController,
            IDesktopAcrylicControllerStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DesktopAcrylicController {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.SystemBackdrops.DesktopAcrylicController;{7c20a6af-8eb3-5f08-bdfc-6d35e35dfe45})" ) ;
}
unsafe impl ::windows::runtime::Interface for DesktopAcrylicController {
    type Vtable = IDesktopAcrylicController_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2082514607,
        36531,
        24328,
        [189, 252, 109, 53, 227, 93, 254, 69],
    );
}
impl ::windows::runtime::RuntimeName for DesktopAcrylicController {
    const NAME: &'static str = "Microsoft.UI.Composition.SystemBackdrops.DesktopAcrylicController";
}
impl ::std::convert::From<DesktopAcrylicController> for ::windows::runtime::IUnknown {
    fn from(value: DesktopAcrylicController) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DesktopAcrylicController> for ::windows::runtime::IUnknown {
    fn from(value: &DesktopAcrylicController) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for DesktopAcrylicController
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &DesktopAcrylicController
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<DesktopAcrylicController> for ::windows::runtime::IInspectable {
    fn from(value: DesktopAcrylicController) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DesktopAcrylicController> for ::windows::runtime::IInspectable {
    fn from(value: &DesktopAcrylicController) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for DesktopAcrylicController
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a DesktopAcrylicController
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<DesktopAcrylicController> for ISystemBackdropController {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DesktopAcrylicController) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&DesktopAcrylicController> for ISystemBackdropController {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DesktopAcrylicController) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISystemBackdropController> for DesktopAcrylicController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISystemBackdropController> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISystemBackdropController>
    for &DesktopAcrylicController
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ISystemBackdropController> {
        ::std::convert::TryInto::<ISystemBackdropController>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "undation")]
impl ::std::convert::TryFrom<DesktopAcrylicController>
    for super::super::super::super::Windows::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: DesktopAcrylicController) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "undation")]
impl ::std::convert::TryFrom<&DesktopAcrylicController>
    for super::super::super::super::Windows::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DesktopAcrylicController) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "undation")]
impl<'a>
    ::windows::runtime::IntoParam<'a, super::super::super::super::Windows::Foundation::IClosable>
    for DesktopAcrylicController
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::super::super::Windows::Foundation::IClosable>
    {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "undation")]
impl<'a>
    ::windows::runtime::IntoParam<'a, super::super::super::super::Windows::Foundation::IClosable>
    for &DesktopAcrylicController
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::super::super::Windows::Foundation::IClosable>
    {
        :: std :: convert :: TryInto :: < super::super::super::super::Windows::Foundation:: IClosable > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
unsafe impl ::std::marker::Send for DesktopAcrylicController {}
unsafe impl ::std::marker::Sync for DesktopAcrylicController {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IDesktopAcrylicController(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDesktopAcrylicController {
    type Vtable = IDesktopAcrylicController_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2082514607,
        36531,
        24328,
        [189, 252, 109, 53, 227, 93, 254, 69],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopAcrylicController_abi(
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
    #[cfg(feature = "")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = ""))] usize,
    #[cfg(feature = "")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::super::super::super::Windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = ""))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = ""))] usize,
    #[cfg(feature = "")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::super::super::super::Windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = ""))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IDesktopAcrylicControllerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDesktopAcrylicControllerStatics {
    type Vtable = IDesktopAcrylicControllerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2850617232,
        31215,
        21526,
        [155, 103, 107, 207, 232, 103, 200, 183],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopAcrylicControllerStatics_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IMicaController(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMicaController {
    type Vtable = IMicaController_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        770283177,
        2602,
        22665,
        [168, 156, 31, 132, 6, 10, 140, 171],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicaController_abi(
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
    #[cfg(feature = "")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = ""))] usize,
    #[cfg(feature = "")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::super::super::super::Windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = ""))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = ""))] usize,
    #[cfg(feature = "")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::super::super::super::Windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = ""))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IMicaControllerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMicaControllerStatics {
    type Vtable = IMicaControllerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2105923636,
        54548,
        21072,
        [183, 196, 11, 120, 80, 209, 239, 220],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicaControllerStatics_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ISystemBackdropController(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISystemBackdropController {
    type Vtable = ISystemBackdropController_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1446172524,
        2932,
        23378,
        [170, 51, 128, 38, 32, 104, 174, 178],
    );
}
impl ISystemBackdropController {
    #[cfg(feature = "_Composition")]
    pub fn SetTargetWithWindowId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::WindowId>,
        Param1: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::UI::Composition::CompositionTarget,
        >,
    >(
        &self,
        windowid: Param0,
        desktopwindowtarget: Param1,
    ) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                windowid.into_param().abi(),
                desktopwindowtarget.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "_Composition", feature = "_Core"))]
    pub fn SetTargetWithCoreWindow<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::UI::Core::CoreWindow,
        >,
        Param1: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::UI::Composition::CompositionTarget,
        >,
    >(
        &self,
        corewindow: Param0,
        compositiontarget: Param1,
    ) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                corewindow.into_param().abi(),
                compositiontarget.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::super::super::Windows::Foundation::IClosable,
        >(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ISystemBackdropController {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{5632d76c-0b74-5b52-aa33-80262068aeb2}");
}
impl ::std::convert::From<ISystemBackdropController> for ::windows::runtime::IUnknown {
    fn from(value: ISystemBackdropController) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISystemBackdropController> for ::windows::runtime::IUnknown {
    fn from(value: &ISystemBackdropController) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ISystemBackdropController
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ISystemBackdropController
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ISystemBackdropController> for ::windows::runtime::IInspectable {
    fn from(value: ISystemBackdropController) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISystemBackdropController> for ::windows::runtime::IInspectable {
    fn from(value: &ISystemBackdropController) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ISystemBackdropController
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ISystemBackdropController
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "undation")]
impl ::std::convert::TryFrom<ISystemBackdropController>
    for super::super::super::super::Windows::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: ISystemBackdropController) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "undation")]
impl ::std::convert::TryFrom<&ISystemBackdropController>
    for super::super::super::super::Windows::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ISystemBackdropController) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "undation")]
impl<'a>
    ::windows::runtime::IntoParam<'a, super::super::super::super::Windows::Foundation::IClosable>
    for ISystemBackdropController
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::super::super::Windows::Foundation::IClosable>
    {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "undation")]
impl<'a>
    ::windows::runtime::IntoParam<'a, super::super::super::super::Windows::Foundation::IClosable>
    for &ISystemBackdropController
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::super::super::Windows::Foundation::IClosable>
    {
        :: std :: convert :: TryInto :: < super::super::super::super::Windows::Foundation:: IClosable > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemBackdropController_abi(
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
    #[cfg(feature = "_Composition")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        windowid: super::super::WindowId,
        desktopwindowtarget: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "_Composition"))] usize,
    #[cfg(all(feature = "_Composition", feature = "_Core"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        corewindow: ::windows::runtime::RawPtr,
        compositiontarget: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "_Composition", feature = "_Core")))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct MicaController(::windows::runtime::IInspectable);
impl MicaController {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            MicaController,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "")]
    pub fn FallbackColor(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Color>(result__)
        }
    }
    #[cfg(feature = "")]
    pub fn SetFallbackColor<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Windows::UI::Color>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn LuminosityOpacity(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn SetLuminosityOpacity(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "")]
    pub fn TintColor(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Color>(result__)
        }
    }
    #[cfg(feature = "")]
    pub fn SetTintColor<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Windows::UI::Color>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn TintOpacity(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn SetTintOpacity(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "_Composition")]
    pub fn SetTargetWithWindowId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::WindowId>,
        Param1: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::UI::Composition::CompositionTarget,
        >,
    >(
        &self,
        windowid: Param0,
        desktopwindowtarget: Param1,
    ) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ISystemBackdropController>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                windowid.into_param().abi(),
                desktopwindowtarget.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "_Composition", feature = "_Core"))]
    pub fn SetTargetWithCoreWindow<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::UI::Core::CoreWindow,
        >,
        Param1: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::UI::Composition::CompositionTarget,
        >,
    >(
        &self,
        corewindow: Param0,
        compositiontarget: Param1,
    ) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ISystemBackdropController>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                corewindow.into_param().abi(),
                compositiontarget.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::super::super::Windows::Foundation::IClosable,
        >(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn IsSupported() -> ::windows::runtime::Result<bool> {
        Self::IMicaControllerStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn IMicaControllerStatics<
        R,
        F: FnOnce(&IMicaControllerStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            MicaController,
            IMicaControllerStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MicaController {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.SystemBackdrops.MicaController;{2de996a9-0a2a-5889-a89c-1f84060a8cab})" ) ;
}
unsafe impl ::windows::runtime::Interface for MicaController {
    type Vtable = IMicaController_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        770283177,
        2602,
        22665,
        [168, 156, 31, 132, 6, 10, 140, 171],
    );
}
impl ::windows::runtime::RuntimeName for MicaController {
    const NAME: &'static str = "Microsoft.UI.Composition.SystemBackdrops.MicaController";
}
impl ::std::convert::From<MicaController> for ::windows::runtime::IUnknown {
    fn from(value: MicaController) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MicaController> for ::windows::runtime::IUnknown {
    fn from(value: &MicaController) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MicaController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MicaController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<MicaController> for ::windows::runtime::IInspectable {
    fn from(value: MicaController) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MicaController> for ::windows::runtime::IInspectable {
    fn from(value: &MicaController) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MicaController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a MicaController
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<MicaController> for ISystemBackdropController {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MicaController) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&MicaController> for ISystemBackdropController {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MicaController) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISystemBackdropController> for MicaController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISystemBackdropController> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISystemBackdropController> for &MicaController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISystemBackdropController> {
        ::std::convert::TryInto::<ISystemBackdropController>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "undation")]
impl ::std::convert::TryFrom<MicaController>
    for super::super::super::super::Windows::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: MicaController) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "undation")]
impl ::std::convert::TryFrom<&MicaController>
    for super::super::super::super::Windows::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MicaController) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "undation")]
impl<'a>
    ::windows::runtime::IntoParam<'a, super::super::super::super::Windows::Foundation::IClosable>
    for MicaController
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::super::super::Windows::Foundation::IClosable>
    {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "undation")]
impl<'a>
    ::windows::runtime::IntoParam<'a, super::super::super::super::Windows::Foundation::IClosable>
    for &MicaController
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::super::super::Windows::Foundation::IClosable>
    {
        :: std :: convert :: TryInto :: < super::super::super::super::Windows::Foundation:: IClosable > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
unsafe impl ::std::marker::Send for MicaController {}
unsafe impl ::std::marker::Sync for MicaController {}
