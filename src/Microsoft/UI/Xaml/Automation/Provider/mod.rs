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
pub struct IAnnotationProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAnnotationProvider {
    type Vtable = IAnnotationProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1416278414,
        39021,
        24043,
        [143, 42, 45, 147, 3, 164, 48, 6],
    );
}
impl IAnnotationProvider {
    pub fn AnnotationTypeId(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn AnnotationTypeName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn Author(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn DateTime(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn Target(&self) -> ::windows::runtime::Result<IRawElementProviderSimple> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IRawElementProviderSimple>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IAnnotationProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{546ab18e-986d-5deb-8f2a-2d9303a43006}");
}
impl ::std::convert::From<IAnnotationProvider> for ::windows::runtime::IUnknown {
    fn from(value: IAnnotationProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IAnnotationProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IAnnotationProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAnnotationProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IAnnotationProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IAnnotationProvider> for ::windows::runtime::IInspectable {
    fn from(value: IAnnotationProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAnnotationProvider> for ::windows::runtime::IInspectable {
    fn from(value: &IAnnotationProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for IAnnotationProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IAnnotationProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnnotationProvider_abi(
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
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ICustomNavigationProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICustomNavigationProvider {
    type Vtable = ICustomNavigationProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3402961698,
        64169,
        23083,
        [144, 240, 183, 98, 196, 97, 120, 179],
    );
}
impl ICustomNavigationProvider {
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub fn NavigateCustom(
        &self,
        direction: super::Peers::AutomationNavigationDirection,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                direction,
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ICustomNavigationProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{cad51322-faa9-5a2b-90f0-b762c46178b3}");
}
impl ::std::convert::From<ICustomNavigationProvider> for ::windows::runtime::IUnknown {
    fn from(value: ICustomNavigationProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ICustomNavigationProvider> for ::windows::runtime::IUnknown {
    fn from(value: &ICustomNavigationProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ICustomNavigationProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ICustomNavigationProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ICustomNavigationProvider> for ::windows::runtime::IInspectable {
    fn from(value: ICustomNavigationProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ICustomNavigationProvider> for ::windows::runtime::IInspectable {
    fn from(value: &ICustomNavigationProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ICustomNavigationProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ICustomNavigationProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomNavigationProvider_abi(
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
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        direction: super::Peers::AutomationNavigationDirection,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDockProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDockProvider {
    type Vtable = IDockProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2558703985,
        28906,
        23661,
        [168, 24, 122, 122, 182, 140, 111, 59],
    );
}
impl IDockProvider {
    pub fn DockPosition(&self) -> ::windows::runtime::Result<super::DockPosition> {
        let this = self;
        unsafe {
            let mut result__: super::DockPosition = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DockPosition>(result__)
        }
    }
    pub fn SetDockPosition(
        &self,
        dockposition: super::DockPosition,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                dockposition,
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IDockProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{9882b971-70ea-5c6d-a818-7a7ab68c6f3b}");
}
impl ::std::convert::From<IDockProvider> for ::windows::runtime::IUnknown {
    fn from(value: IDockProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDockProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IDockProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDockProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDockProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDockProvider> for ::windows::runtime::IInspectable {
    fn from(value: IDockProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDockProvider> for ::windows::runtime::IInspectable {
    fn from(value: &IDockProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IDockProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IDockProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDockProvider_abi(
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
        result__: *mut super::DockPosition,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dockposition: super::DockPosition,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDragProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDragProvider {
    type Vtable = IDragProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3322656323,
        41814,
        20786,
        [162, 88, 255, 186, 108, 116, 128, 242],
    );
}
impl IDragProvider {
    pub fn IsGrabbed(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn DropEffect(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn DropEffects(
        &self,
    ) -> ::windows::runtime::Result<::windows::runtime::Array<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                ::windows::runtime::Array::<::windows::runtime::HSTRING>::set_abi_len(
                    &mut result__,
                ),
                &mut result__ as *mut _ as _,
            )
            .and_then(|| result__)
        }
    }
    pub fn GetGrabbedItems(
        &self,
    ) -> ::windows::runtime::Result<::windows::runtime::Array<IRawElementProviderSimple>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<IRawElementProviderSimple> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                ::windows::runtime::Array::<IRawElementProviderSimple>::set_abi_len(&mut result__),
                &mut result__ as *mut _ as _,
            )
            .and_then(|| result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IDragProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{c60bb643-a356-5132-a258-ffba6c7480f2}");
}
impl ::std::convert::From<IDragProvider> for ::windows::runtime::IUnknown {
    fn from(value: IDragProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDragProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IDragProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDragProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDragProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDragProvider> for ::windows::runtime::IInspectable {
    fn from(value: IDragProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDragProvider> for ::windows::runtime::IInspectable {
    fn from(value: &IDragProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IDragProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IDragProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragProvider_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result_size__: *mut u32,
        result__: *mut *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result_size__: *mut u32,
        result__: *mut *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDropTargetProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDropTargetProvider {
    type Vtable = IDropTargetProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2603261757,
        48049,
        20749,
        [153, 232, 14, 10, 225, 74, 110, 59],
    );
}
impl IDropTargetProvider {
    pub fn DropEffect(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn DropEffects(
        &self,
    ) -> ::windows::runtime::Result<::windows::runtime::Array<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                ::windows::runtime::Array::<::windows::runtime::HSTRING>::set_abi_len(
                    &mut result__,
                ),
                &mut result__ as *mut _ as _,
            )
            .and_then(|| result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IDropTargetProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{9b2a9f3d-bbb1-510d-99e8-0e0ae14a6e3b}");
}
impl ::std::convert::From<IDropTargetProvider> for ::windows::runtime::IUnknown {
    fn from(value: IDropTargetProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDropTargetProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IDropTargetProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDropTargetProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDropTargetProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDropTargetProvider> for ::windows::runtime::IInspectable {
    fn from(value: IDropTargetProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDropTargetProvider> for ::windows::runtime::IInspectable {
    fn from(value: &IDropTargetProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for IDropTargetProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IDropTargetProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDropTargetProvider_abi(
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
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result_size__: *mut u32,
        result__: *mut *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IExpandCollapseProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IExpandCollapseProvider {
    type Vtable = IExpandCollapseProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1827615900,
        45441,
        23819,
        [178, 151, 195, 176, 22, 97, 32, 195],
    );
}
impl IExpandCollapseProvider {
    pub fn ExpandCollapseState(&self) -> ::windows::runtime::Result<super::ExpandCollapseState> {
        let this = self;
        unsafe {
            let mut result__: super::ExpandCollapseState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ExpandCollapseState>(result__)
        }
    }
    pub fn Collapse(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Expand(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IExpandCollapseProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{6cef349c-b181-5d0b-b297-c3b0166120c3}");
}
impl ::std::convert::From<IExpandCollapseProvider> for ::windows::runtime::IUnknown {
    fn from(value: IExpandCollapseProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IExpandCollapseProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IExpandCollapseProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IExpandCollapseProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IExpandCollapseProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IExpandCollapseProvider> for ::windows::runtime::IInspectable {
    fn from(value: IExpandCollapseProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IExpandCollapseProvider> for ::windows::runtime::IInspectable {
    fn from(value: &IExpandCollapseProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for IExpandCollapseProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IExpandCollapseProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpandCollapseProvider_abi(
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
        result__: *mut super::ExpandCollapseState,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IGridItemProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGridItemProvider {
    type Vtable = IGridItemProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3528817166,
        26889,
        20848,
        [166, 128, 96, 114, 141, 243, 57, 180],
    );
}
impl IGridItemProvider {
    pub fn Column(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn ColumnSpan(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn ContainingGrid(&self) -> ::windows::runtime::Result<IRawElementProviderSimple> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IRawElementProviderSimple>(result__)
        }
    }
    pub fn Row(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn RowSpan(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IGridItemProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{d2557a0e-6909-5170-a680-60728df339b4}");
}
impl ::std::convert::From<IGridItemProvider> for ::windows::runtime::IUnknown {
    fn from(value: IGridItemProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IGridItemProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IGridItemProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGridItemProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IGridItemProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IGridItemProvider> for ::windows::runtime::IInspectable {
    fn from(value: IGridItemProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IGridItemProvider> for ::windows::runtime::IInspectable {
    fn from(value: &IGridItemProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IGridItemProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IGridItemProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridItemProvider_abi(
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
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IGridProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGridProvider {
    type Vtable = IGridProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1352215902,
        53797,
        22249,
        [162, 90, 120, 195, 114, 232, 25, 85],
    );
}
impl IGridProvider {
    pub fn ColumnCount(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn RowCount(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn GetItem(
        &self,
        row: i32,
        column: i32,
    ) -> ::windows::runtime::Result<IRawElementProviderSimple> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                row,
                column,
                &mut result__,
            )
            .from_abi::<IRawElementProviderSimple>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IGridProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{50992d5e-d225-56e9-a25a-78c372e81955}");
}
impl ::std::convert::From<IGridProvider> for ::windows::runtime::IUnknown {
    fn from(value: IGridProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IGridProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IGridProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGridProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IGridProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IGridProvider> for ::windows::runtime::IInspectable {
    fn from(value: IGridProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IGridProvider> for ::windows::runtime::IInspectable {
    fn from(value: &IGridProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IGridProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IGridProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridProvider_abi(
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
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        row: i32,
        column: i32,
        result__: *mut ::windows::runtime::RawPtr,
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
pub struct IIRawElementProviderSimple(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIRawElementProviderSimple {
    type Vtable = IIRawElementProviderSimple_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4178297401,
        44514,
        21961,
        [168, 56, 163, 176, 87, 151, 99, 197],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IIRawElementProviderSimple_abi(
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
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IInvokeProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInvokeProvider {
    type Vtable = IInvokeProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        38277381,
        13176,
        21581,
        [180, 225, 161, 179, 104, 175, 188, 2],
    );
}
impl IInvokeProvider {
    pub fn Invoke(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IInvokeProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{02481105-3378-544d-b4e1-a1b368afbc02}");
}
impl ::std::convert::From<IInvokeProvider> for ::windows::runtime::IUnknown {
    fn from(value: IInvokeProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IInvokeProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IInvokeProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInvokeProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IInvokeProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IInvokeProvider> for ::windows::runtime::IInspectable {
    fn from(value: IInvokeProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInvokeProvider> for ::windows::runtime::IInspectable {
    fn from(value: &IInvokeProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IInvokeProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IInvokeProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInvokeProvider_abi(
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IItemContainerProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IItemContainerProvider {
    type Vtable = IItemContainerProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2905174883,
        26958,
        22661,
        [153, 125, 162, 214, 223, 244, 21, 167],
    );
}
impl IItemContainerProvider {
    pub fn FindItemByProperty<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IRawElementProviderSimple>,
        Param1: ::windows::runtime::IntoParam<'a, super::AutomationProperty>,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        startafter: Param0,
        automationproperty: Param1,
        value: Param2,
    ) -> ::windows::runtime::Result<IRawElementProviderSimple> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                startafter.into_param().abi(),
                automationproperty.into_param().abi(),
                value.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IRawElementProviderSimple>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IItemContainerProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{ad297363-694e-5885-997d-a2d6dff415a7}");
}
impl ::std::convert::From<IItemContainerProvider> for ::windows::runtime::IUnknown {
    fn from(value: IItemContainerProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IItemContainerProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IItemContainerProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IItemContainerProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IItemContainerProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IItemContainerProvider> for ::windows::runtime::IInspectable {
    fn from(value: IItemContainerProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IItemContainerProvider> for ::windows::runtime::IInspectable {
    fn from(value: &IItemContainerProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for IItemContainerProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IItemContainerProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IItemContainerProvider_abi(
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
        startafter: ::windows::runtime::RawPtr,
        automationproperty: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMultipleViewProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMultipleViewProvider {
    type Vtable = IMultipleViewProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1623086212,
        15759,
        20989,
        [190, 171, 66, 52, 34, 238, 30, 3],
    );
}
impl IMultipleViewProvider {
    pub fn CurrentView(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn GetSupportedViews(&self) -> ::windows::runtime::Result<::windows::runtime::Array<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<i32> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                ::windows::runtime::Array::<i32>::set_abi_len(&mut result__),
                &mut result__ as *mut _ as _,
            )
            .and_then(|| result__)
        }
    }
    pub fn GetViewName(
        &self,
        viewid: i32,
    ) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                viewid,
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetCurrentView(&self, viewid: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                viewid,
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IMultipleViewProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{60be5484-3d8f-51fd-beab-423422ee1e03}");
}
impl ::std::convert::From<IMultipleViewProvider> for ::windows::runtime::IUnknown {
    fn from(value: IMultipleViewProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMultipleViewProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IMultipleViewProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMultipleViewProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IMultipleViewProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IMultipleViewProvider> for ::windows::runtime::IInspectable {
    fn from(value: IMultipleViewProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IMultipleViewProvider> for ::windows::runtime::IInspectable {
    fn from(value: &IMultipleViewProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for IMultipleViewProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IMultipleViewProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultipleViewProvider_abi(
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
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result_size__: *mut u32,
        result__: *mut *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        viewid: i32,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        viewid: i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IObjectModelProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IObjectModelProvider {
    type Vtable = IObjectModelProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2459254480,
        19416,
        22052,
        [142, 61, 120, 212, 95, 222, 156, 242],
    );
}
impl IObjectModelProvider {
    pub fn GetUnderlyingObjectModel(
        &self,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IObjectModelProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{92953ed0-4bd8-5624-8e3d-78d45fde9cf2}");
}
impl ::std::convert::From<IObjectModelProvider> for ::windows::runtime::IUnknown {
    fn from(value: IObjectModelProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IObjectModelProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IObjectModelProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IObjectModelProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IObjectModelProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IObjectModelProvider> for ::windows::runtime::IInspectable {
    fn from(value: IObjectModelProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IObjectModelProvider> for ::windows::runtime::IInspectable {
    fn from(value: &IObjectModelProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for IObjectModelProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IObjectModelProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectModelProvider_abi(
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
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IRangeValueProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRangeValueProvider {
    type Vtable = IRangeValueProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1922753556,
        7823,
        20512,
        [130, 187, 187, 87, 77, 20, 95, 216],
    );
}
impl IRangeValueProvider {
    pub fn IsReadOnly(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn LargeChange(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn Maximum(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn Minimum(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SmallChange(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn Value(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetValue(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IRangeValueProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{729ae414-1e8f-5020-82bb-bb574d145fd8}");
}
impl ::std::convert::From<IRangeValueProvider> for ::windows::runtime::IUnknown {
    fn from(value: IRangeValueProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IRangeValueProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IRangeValueProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRangeValueProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IRangeValueProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IRangeValueProvider> for ::windows::runtime::IInspectable {
    fn from(value: IRangeValueProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IRangeValueProvider> for ::windows::runtime::IInspectable {
    fn from(value: &IRangeValueProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for IRangeValueProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IRangeValueProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeValueProvider_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IRawElementProviderSimple(::windows::runtime::IInspectable);
impl IRawElementProviderSimple {
    pub fn GetValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    pub fn SetValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ClearValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ReadLocalValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    #[cfg(feature = "_Core")]
    pub fn Dispatcher(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::super::super::Windows::UI::Core::CoreDispatcher,
    > {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::super::Windows::UI::Core::CoreDispatcher>(
                result__,
            )
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IRawElementProviderSimple {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.Provider.IRawElementProviderSimple;{f90bc239-ade2-55c9-a838-a3b0579763c5})" ) ;
}
unsafe impl ::windows::runtime::Interface for IRawElementProviderSimple {
    type Vtable = IIRawElementProviderSimple_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4178297401,
        44514,
        21961,
        [168, 56, 163, 176, 87, 151, 99, 197],
    );
}
impl ::windows::runtime::RuntimeName for IRawElementProviderSimple {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.Provider.IRawElementProviderSimple";
}
impl ::std::convert::From<IRawElementProviderSimple> for ::windows::runtime::IUnknown {
    fn from(value: IRawElementProviderSimple) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IRawElementProviderSimple> for ::windows::runtime::IUnknown {
    fn from(value: &IRawElementProviderSimple) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IRawElementProviderSimple
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IRawElementProviderSimple
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IRawElementProviderSimple> for ::windows::runtime::IInspectable {
    fn from(value: IRawElementProviderSimple) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IRawElementProviderSimple> for ::windows::runtime::IInspectable {
    fn from(value: &IRawElementProviderSimple) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for IRawElementProviderSimple
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IRawElementProviderSimple
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IRawElementProviderSimple> for super::super::DependencyObject {
    fn from(value: IRawElementProviderSimple) -> Self {
        ::std::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::std::convert::From<&IRawElementProviderSimple> for super::super::DependencyObject {
    fn from(value: &IRawElementProviderSimple) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject>
    for IRawElementProviderSimple
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::super::DependencyObject>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject>
    for &IRawElementProviderSimple
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::super::DependencyObject>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
unsafe impl ::std::marker::Send for IRawElementProviderSimple {}
unsafe impl ::std::marker::Sync for IRawElementProviderSimple {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IScrollItemProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IScrollItemProvider {
    type Vtable = IScrollItemProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2322577643,
        58865,
        22763,
        [142, 114, 139, 149, 242, 54, 252, 71],
    );
}
impl IScrollItemProvider {
    pub fn ScrollIntoView(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IScrollItemProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{8a6fb8eb-e5f1-58eb-8e72-8b95f236fc47}");
}
impl ::std::convert::From<IScrollItemProvider> for ::windows::runtime::IUnknown {
    fn from(value: IScrollItemProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IScrollItemProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IScrollItemProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IScrollItemProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IScrollItemProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IScrollItemProvider> for ::windows::runtime::IInspectable {
    fn from(value: IScrollItemProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IScrollItemProvider> for ::windows::runtime::IInspectable {
    fn from(value: &IScrollItemProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for IScrollItemProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IScrollItemProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollItemProvider_abi(
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IScrollProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IScrollProvider {
    type Vtable = IScrollProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2116967155,
        65360,
        21349,
        [188, 254, 239, 66, 75, 47, 213, 144],
    );
}
impl IScrollProvider {
    pub fn HorizontallyScrollable(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn HorizontalScrollPercent(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn HorizontalViewSize(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn VerticallyScrollable(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn VerticalScrollPercent(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn VerticalViewSize(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn Scroll(
        &self,
        horizontalamount: super::ScrollAmount,
        verticalamount: super::ScrollAmount,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                horizontalamount,
                verticalamount,
            )
            .ok()
        }
    }
    pub fn SetScrollPercent(
        &self,
        horizontalpercent: f64,
        verticalpercent: f64,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                horizontalpercent,
                verticalpercent,
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IScrollProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{7e2e5af3-ff50-5365-bcfe-ef424b2fd590}");
}
impl ::std::convert::From<IScrollProvider> for ::windows::runtime::IUnknown {
    fn from(value: IScrollProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IScrollProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IScrollProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IScrollProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IScrollProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IScrollProvider> for ::windows::runtime::IInspectable {
    fn from(value: IScrollProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IScrollProvider> for ::windows::runtime::IInspectable {
    fn from(value: &IScrollProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IScrollProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IScrollProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollProvider_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        horizontalamount: super::ScrollAmount,
        verticalamount: super::ScrollAmount,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        horizontalpercent: f64,
        verticalpercent: f64,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ISelectionItemProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISelectionItemProvider {
    type Vtable = ISelectionItemProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3386891649,
        54444,
        23857,
        [190, 127, 36, 250, 177, 96, 96, 228],
    );
}
impl ISelectionItemProvider {
    pub fn IsSelected(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SelectionContainer(&self) -> ::windows::runtime::Result<IRawElementProviderSimple> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IRawElementProviderSimple>(result__)
        }
    }
    pub fn AddToSelection(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn RemoveFromSelection(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Select(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ISelectionItemProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{c9dfdd81-d4ac-5d31-be7f-24fab16060e4}");
}
impl ::std::convert::From<ISelectionItemProvider> for ::windows::runtime::IUnknown {
    fn from(value: ISelectionItemProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISelectionItemProvider> for ::windows::runtime::IUnknown {
    fn from(value: &ISelectionItemProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ISelectionItemProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ISelectionItemProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ISelectionItemProvider> for ::windows::runtime::IInspectable {
    fn from(value: ISelectionItemProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISelectionItemProvider> for ::windows::runtime::IInspectable {
    fn from(value: &ISelectionItemProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ISelectionItemProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ISelectionItemProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectionItemProvider_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ISelectionProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISelectionProvider {
    type Vtable = ISelectionProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2161470798,
        82,
        21535,
        [148, 17, 157, 23, 120, 179, 191, 202],
    );
}
impl ISelectionProvider {
    pub fn CanSelectMultiple(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn IsSelectionRequired(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn GetSelection(
        &self,
    ) -> ::windows::runtime::Result<::windows::runtime::Array<IRawElementProviderSimple>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<IRawElementProviderSimple> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                ::windows::runtime::Array::<IRawElementProviderSimple>::set_abi_len(&mut result__),
                &mut result__ as *mut _ as _,
            )
            .and_then(|| result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ISelectionProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{80d56d4e-0052-541f-9411-9d1778b3bfca}");
}
impl ::std::convert::From<ISelectionProvider> for ::windows::runtime::IUnknown {
    fn from(value: ISelectionProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISelectionProvider> for ::windows::runtime::IUnknown {
    fn from(value: &ISelectionProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISelectionProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ISelectionProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ISelectionProvider> for ::windows::runtime::IInspectable {
    fn from(value: ISelectionProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISelectionProvider> for ::windows::runtime::IInspectable {
    fn from(value: &ISelectionProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ISelectionProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ISelectionProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectionProvider_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result_size__: *mut u32,
        result__: *mut *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ISpreadsheetItemProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpreadsheetItemProvider {
    type Vtable = ISpreadsheetItemProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1371655817,
        45599,
        22828,
        [135, 104, 10, 204, 222, 253, 87, 56],
    );
}
impl ISpreadsheetItemProvider {
    pub fn Formula(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn GetAnnotationObjects(
        &self,
    ) -> ::windows::runtime::Result<::windows::runtime::Array<IRawElementProviderSimple>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<IRawElementProviderSimple> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                ::windows::runtime::Array::<IRawElementProviderSimple>::set_abi_len(&mut result__),
                &mut result__ as *mut _ as _,
            )
            .and_then(|| result__)
        }
    }
    pub fn GetAnnotationTypes(
        &self,
    ) -> ::windows::runtime::Result<::windows::runtime::Array<super::AnnotationType>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<super::AnnotationType> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                ::windows::runtime::Array::<super::AnnotationType>::set_abi_len(&mut result__),
                &mut result__ as *mut _ as _,
            )
            .and_then(|| result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ISpreadsheetItemProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{51c1ce89-b21f-592c-8768-0accdefd5738}");
}
impl ::std::convert::From<ISpreadsheetItemProvider> for ::windows::runtime::IUnknown {
    fn from(value: ISpreadsheetItemProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISpreadsheetItemProvider> for ::windows::runtime::IUnknown {
    fn from(value: &ISpreadsheetItemProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ISpreadsheetItemProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ISpreadsheetItemProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ISpreadsheetItemProvider> for ::windows::runtime::IInspectable {
    fn from(value: ISpreadsheetItemProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISpreadsheetItemProvider> for ::windows::runtime::IInspectable {
    fn from(value: &ISpreadsheetItemProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ISpreadsheetItemProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ISpreadsheetItemProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpreadsheetItemProvider_abi(
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
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result_size__: *mut u32,
        result__: *mut *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result_size__: *mut u32,
        result__: *mut *mut super::AnnotationType,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ISpreadsheetProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpreadsheetProvider {
    type Vtable = ISpreadsheetProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        536091564,
        55779,
        24136,
        [181, 248, 158, 171, 15, 178, 217, 216],
    );
}
impl ISpreadsheetProvider {
    pub fn GetItemByName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        name: Param0,
    ) -> ::windows::runtime::Result<IRawElementProviderSimple> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IRawElementProviderSimple>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ISpreadsheetProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{1ff41bac-d9e3-5e48-b5f8-9eab0fb2d9d8}");
}
impl ::std::convert::From<ISpreadsheetProvider> for ::windows::runtime::IUnknown {
    fn from(value: ISpreadsheetProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISpreadsheetProvider> for ::windows::runtime::IUnknown {
    fn from(value: &ISpreadsheetProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISpreadsheetProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ISpreadsheetProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ISpreadsheetProvider> for ::windows::runtime::IInspectable {
    fn from(value: ISpreadsheetProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISpreadsheetProvider> for ::windows::runtime::IInspectable {
    fn from(value: &ISpreadsheetProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ISpreadsheetProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ISpreadsheetProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpreadsheetProvider_abi(
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
        name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IStylesProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IStylesProvider {
    type Vtable = IStylesProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3632879673,
        72,
        21726,
        [156, 31, 21, 45, 230, 102, 94, 128],
    );
}
impl IStylesProvider {
    pub fn ExtendedProperties(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "")]
    pub fn FillColor(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::super::Windows::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::super::Windows::UI::Color =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::super::Windows::UI::Color>(result__)
        }
    }
    #[cfg(feature = "")]
    pub fn FillPatternColor(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::super::Windows::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::super::Windows::UI::Color =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::super::Windows::UI::Color>(result__)
        }
    }
    pub fn FillPatternStyle(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn Shape(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn StyleId(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn StyleName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IStylesProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{d8895839-0048-54de-9c1f-152de6665e80}");
}
impl ::std::convert::From<IStylesProvider> for ::windows::runtime::IUnknown {
    fn from(value: IStylesProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IStylesProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IStylesProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IStylesProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IStylesProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IStylesProvider> for ::windows::runtime::IInspectable {
    fn from(value: IStylesProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IStylesProvider> for ::windows::runtime::IInspectable {
    fn from(value: &IStylesProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IStylesProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IStylesProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStylesProvider_abi(
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
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::super::Windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = ""))] usize,
    #[cfg(feature = "")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::super::Windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = ""))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ISynchronizedInputProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISynchronizedInputProvider {
    type Vtable = ISynchronizedInputProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3311490579,
        37741,
        21129,
        [161, 144, 232, 32, 87, 224, 255, 90],
    );
}
impl ISynchronizedInputProvider {
    pub fn Cancel(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn StartListening(
        &self,
        inputtype: super::SynchronizedInputType,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                inputtype,
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ISynchronizedInputProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{c5615613-936d-5289-a190-e82057e0ff5a}");
}
impl ::std::convert::From<ISynchronizedInputProvider> for ::windows::runtime::IUnknown {
    fn from(value: ISynchronizedInputProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISynchronizedInputProvider> for ::windows::runtime::IUnknown {
    fn from(value: &ISynchronizedInputProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ISynchronizedInputProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ISynchronizedInputProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ISynchronizedInputProvider> for ::windows::runtime::IInspectable {
    fn from(value: ISynchronizedInputProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISynchronizedInputProvider> for ::windows::runtime::IInspectable {
    fn from(value: &ISynchronizedInputProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ISynchronizedInputProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ISynchronizedInputProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronizedInputProvider_abi(
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        inputtype: super::SynchronizedInputType,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ITableItemProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITableItemProvider {
    type Vtable = ITableItemProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1827074104,
        21716,
        21843,
        [164, 173, 3, 203, 207, 53, 129, 151],
    );
}
impl ITableItemProvider {
    pub fn GetColumnHeaderItems(
        &self,
    ) -> ::windows::runtime::Result<::windows::runtime::Array<IRawElementProviderSimple>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<IRawElementProviderSimple> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                ::windows::runtime::Array::<IRawElementProviderSimple>::set_abi_len(&mut result__),
                &mut result__ as *mut _ as _,
            )
            .and_then(|| result__)
        }
    }
    pub fn GetRowHeaderItems(
        &self,
    ) -> ::windows::runtime::Result<::windows::runtime::Array<IRawElementProviderSimple>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<IRawElementProviderSimple> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                ::windows::runtime::Array::<IRawElementProviderSimple>::set_abi_len(&mut result__),
                &mut result__ as *mut _ as _,
            )
            .and_then(|| result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ITableItemProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{6ce6f038-54d4-5553-a4ad-03cbcf358197}");
}
impl ::std::convert::From<ITableItemProvider> for ::windows::runtime::IUnknown {
    fn from(value: ITableItemProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITableItemProvider> for ::windows::runtime::IUnknown {
    fn from(value: &ITableItemProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITableItemProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITableItemProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ITableItemProvider> for ::windows::runtime::IInspectable {
    fn from(value: ITableItemProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ITableItemProvider> for ::windows::runtime::IInspectable {
    fn from(value: &ITableItemProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ITableItemProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ITableItemProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITableItemProvider_abi(
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
        result_size__: *mut u32,
        result__: *mut *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result_size__: *mut u32,
        result__: *mut *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ITableProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITableProvider {
    type Vtable = ITableProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2595907364,
        45613,
        23992,
        [138, 187, 129, 249, 17, 241, 138, 242],
    );
}
impl ITableProvider {
    pub fn RowOrColumnMajor(&self) -> ::windows::runtime::Result<super::RowOrColumnMajor> {
        let this = self;
        unsafe {
            let mut result__: super::RowOrColumnMajor = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::RowOrColumnMajor>(result__)
        }
    }
    pub fn GetColumnHeaders(
        &self,
    ) -> ::windows::runtime::Result<::windows::runtime::Array<IRawElementProviderSimple>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<IRawElementProviderSimple> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                ::windows::runtime::Array::<IRawElementProviderSimple>::set_abi_len(&mut result__),
                &mut result__ as *mut _ as _,
            )
            .and_then(|| result__)
        }
    }
    pub fn GetRowHeaders(
        &self,
    ) -> ::windows::runtime::Result<::windows::runtime::Array<IRawElementProviderSimple>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<IRawElementProviderSimple> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                ::windows::runtime::Array::<IRawElementProviderSimple>::set_abi_len(&mut result__),
                &mut result__ as *mut _ as _,
            )
            .and_then(|| result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ITableProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{9aba6724-b22d-5db8-8abb-81f911f18af2}");
}
impl ::std::convert::From<ITableProvider> for ::windows::runtime::IUnknown {
    fn from(value: ITableProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITableProvider> for ::windows::runtime::IUnknown {
    fn from(value: &ITableProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITableProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITableProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ITableProvider> for ::windows::runtime::IInspectable {
    fn from(value: ITableProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ITableProvider> for ::windows::runtime::IInspectable {
    fn from(value: &ITableProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ITableProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ITableProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITableProvider_abi(
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
        result__: *mut super::RowOrColumnMajor,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result_size__: *mut u32,
        result__: *mut *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result_size__: *mut u32,
        result__: *mut *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ITextChildProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextChildProvider {
    type Vtable = ITextChildProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2087904607,
        63325,
        21794,
        [174, 181, 193, 248, 44, 50, 147, 59],
    );
}
impl ITextChildProvider {
    pub fn TextContainer(&self) -> ::windows::runtime::Result<IRawElementProviderSimple> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IRawElementProviderSimple>(result__)
        }
    }
    pub fn TextRange(&self) -> ::windows::runtime::Result<ITextRangeProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ITextRangeProvider>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ITextChildProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{7c72e55f-f75d-5522-aeb5-c1f82c32933b}");
}
impl ::std::convert::From<ITextChildProvider> for ::windows::runtime::IUnknown {
    fn from(value: ITextChildProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITextChildProvider> for ::windows::runtime::IUnknown {
    fn from(value: &ITextChildProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextChildProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITextChildProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ITextChildProvider> for ::windows::runtime::IInspectable {
    fn from(value: ITextChildProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ITextChildProvider> for ::windows::runtime::IInspectable {
    fn from(value: &ITextChildProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ITextChildProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ITextChildProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextChildProvider_abi(
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
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ITextEditProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextEditProvider {
    type Vtable = ITextEditProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2131344360,
        48807,
        24019,
        [186, 107, 40, 219, 180, 2, 250, 212],
    );
}
impl ITextEditProvider {
    pub fn GetActiveComposition(&self) -> ::windows::runtime::Result<ITextRangeProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ITextRangeProvider>(result__)
        }
    }
    pub fn GetConversionTarget(&self) -> ::windows::runtime::Result<ITextRangeProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ITextRangeProvider>(result__)
        }
    }
    pub fn DocumentRange(&self) -> ::windows::runtime::Result<ITextRangeProvider> {
        let this = &::windows::runtime::Interface::cast::<ITextProvider>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ITextRangeProvider>(result__)
        }
    }
    pub fn SupportedTextSelection(
        &self,
    ) -> ::windows::runtime::Result<super::SupportedTextSelection> {
        let this = &::windows::runtime::Interface::cast::<ITextProvider>(self)?;
        unsafe {
            let mut result__: super::SupportedTextSelection = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::SupportedTextSelection>(result__)
        }
    }
    pub fn GetSelection(
        &self,
    ) -> ::windows::runtime::Result<::windows::runtime::Array<ITextRangeProvider>> {
        let this = &::windows::runtime::Interface::cast::<ITextProvider>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::Array<ITextRangeProvider> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                ::windows::runtime::Array::<ITextRangeProvider>::set_abi_len(&mut result__),
                &mut result__ as *mut _ as _,
            )
            .and_then(|| result__)
        }
    }
    pub fn GetVisibleRanges(
        &self,
    ) -> ::windows::runtime::Result<::windows::runtime::Array<ITextRangeProvider>> {
        let this = &::windows::runtime::Interface::cast::<ITextProvider>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::Array<ITextRangeProvider> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                ::windows::runtime::Array::<ITextRangeProvider>::set_abi_len(&mut result__),
                &mut result__ as *mut _ as _,
            )
            .and_then(|| result__)
        }
    }
    pub fn RangeFromChild<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IRawElementProviderSimple>,
    >(
        &self,
        childelement: Param0,
    ) -> ::windows::runtime::Result<ITextRangeProvider> {
        let this = &::windows::runtime::Interface::cast::<ITextProvider>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                childelement.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ITextRangeProvider>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn RangeFromPoint<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::super::Windows::Foundation::Point,
        >,
    >(
        &self,
        screenlocation: Param0,
    ) -> ::windows::runtime::Result<ITextRangeProvider> {
        let this = &::windows::runtime::Interface::cast::<ITextProvider>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                screenlocation.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ITextRangeProvider>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ITextEditProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{7f09bbe8-bea7-5dd3-ba6b-28dbb402fad4}");
}
impl ::std::convert::From<ITextEditProvider> for ::windows::runtime::IUnknown {
    fn from(value: ITextEditProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITextEditProvider> for ::windows::runtime::IUnknown {
    fn from(value: &ITextEditProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextEditProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITextEditProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ITextEditProvider> for ::windows::runtime::IInspectable {
    fn from(value: ITextEditProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ITextEditProvider> for ::windows::runtime::IInspectable {
    fn from(value: &ITextEditProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ITextEditProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ITextEditProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<ITextEditProvider> for ITextProvider {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ITextEditProvider) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ITextEditProvider> for ITextProvider {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ITextEditProvider) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextProvider> for ITextEditProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextProvider> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextProvider> for &ITextEditProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextProvider> {
        ::std::convert::TryInto::<ITextProvider>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextEditProvider_abi(
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
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ITextProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextProvider {
    type Vtable = ITextProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        937942246,
        65146,
        22183,
        [164, 122, 148, 98, 135, 44, 103, 239],
    );
}
impl ITextProvider {
    pub fn DocumentRange(&self) -> ::windows::runtime::Result<ITextRangeProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ITextRangeProvider>(result__)
        }
    }
    pub fn SupportedTextSelection(
        &self,
    ) -> ::windows::runtime::Result<super::SupportedTextSelection> {
        let this = self;
        unsafe {
            let mut result__: super::SupportedTextSelection = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::SupportedTextSelection>(result__)
        }
    }
    pub fn GetSelection(
        &self,
    ) -> ::windows::runtime::Result<::windows::runtime::Array<ITextRangeProvider>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<ITextRangeProvider> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                ::windows::runtime::Array::<ITextRangeProvider>::set_abi_len(&mut result__),
                &mut result__ as *mut _ as _,
            )
            .and_then(|| result__)
        }
    }
    pub fn GetVisibleRanges(
        &self,
    ) -> ::windows::runtime::Result<::windows::runtime::Array<ITextRangeProvider>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<ITextRangeProvider> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                ::windows::runtime::Array::<ITextRangeProvider>::set_abi_len(&mut result__),
                &mut result__ as *mut _ as _,
            )
            .and_then(|| result__)
        }
    }
    pub fn RangeFromChild<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IRawElementProviderSimple>,
    >(
        &self,
        childelement: Param0,
    ) -> ::windows::runtime::Result<ITextRangeProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                childelement.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ITextRangeProvider>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn RangeFromPoint<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::super::Windows::Foundation::Point,
        >,
    >(
        &self,
        screenlocation: Param0,
    ) -> ::windows::runtime::Result<ITextRangeProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                screenlocation.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ITextRangeProvider>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ITextProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{37e7dce6-fe7a-56a7-a47a-9462872c67ef}");
}
impl ::std::convert::From<ITextProvider> for ::windows::runtime::IUnknown {
    fn from(value: ITextProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITextProvider> for ::windows::runtime::IUnknown {
    fn from(value: &ITextProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITextProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ITextProvider> for ::windows::runtime::IInspectable {
    fn from(value: ITextProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ITextProvider> for ::windows::runtime::IInspectable {
    fn from(value: &ITextProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ITextProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ITextProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextProvider_abi(
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
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::SupportedTextSelection,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result_size__: *mut u32,
        result__: *mut *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result_size__: *mut u32,
        result__: *mut *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        childelement: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        screenlocation: super::super::super::super::super::Windows::Foundation::Point,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ITextProvider2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextProvider2 {
    type Vtable = ITextProvider2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1749348370,
        51174,
        22371,
        [186, 4, 91, 109, 185, 16, 205, 52],
    );
}
impl ITextProvider2 {
    pub fn RangeFromAnnotation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IRawElementProviderSimple>,
    >(
        &self,
        annotationelement: Param0,
    ) -> ::windows::runtime::Result<ITextRangeProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                annotationelement.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ITextRangeProvider>(result__)
        }
    }
    pub fn GetCaretRange(
        &self,
        isactive: &mut bool,
    ) -> ::windows::runtime::Result<ITextRangeProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                isactive,
                &mut result__,
            )
            .from_abi::<ITextRangeProvider>(result__)
        }
    }
    pub fn DocumentRange(&self) -> ::windows::runtime::Result<ITextRangeProvider> {
        let this = &::windows::runtime::Interface::cast::<ITextProvider>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ITextRangeProvider>(result__)
        }
    }
    pub fn SupportedTextSelection(
        &self,
    ) -> ::windows::runtime::Result<super::SupportedTextSelection> {
        let this = &::windows::runtime::Interface::cast::<ITextProvider>(self)?;
        unsafe {
            let mut result__: super::SupportedTextSelection = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::SupportedTextSelection>(result__)
        }
    }
    pub fn GetSelection(
        &self,
    ) -> ::windows::runtime::Result<::windows::runtime::Array<ITextRangeProvider>> {
        let this = &::windows::runtime::Interface::cast::<ITextProvider>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::Array<ITextRangeProvider> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                ::windows::runtime::Array::<ITextRangeProvider>::set_abi_len(&mut result__),
                &mut result__ as *mut _ as _,
            )
            .and_then(|| result__)
        }
    }
    pub fn GetVisibleRanges(
        &self,
    ) -> ::windows::runtime::Result<::windows::runtime::Array<ITextRangeProvider>> {
        let this = &::windows::runtime::Interface::cast::<ITextProvider>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::Array<ITextRangeProvider> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                ::windows::runtime::Array::<ITextRangeProvider>::set_abi_len(&mut result__),
                &mut result__ as *mut _ as _,
            )
            .and_then(|| result__)
        }
    }
    pub fn RangeFromChild<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IRawElementProviderSimple>,
    >(
        &self,
        childelement: Param0,
    ) -> ::windows::runtime::Result<ITextRangeProvider> {
        let this = &::windows::runtime::Interface::cast::<ITextProvider>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                childelement.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ITextRangeProvider>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn RangeFromPoint<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::super::Windows::Foundation::Point,
        >,
    >(
        &self,
        screenlocation: Param0,
    ) -> ::windows::runtime::Result<ITextRangeProvider> {
        let this = &::windows::runtime::Interface::cast::<ITextProvider>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                screenlocation.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ITextRangeProvider>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ITextProvider2 {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{6844f012-c7e6-5763-ba04-5b6db910cd34}");
}
impl ::std::convert::From<ITextProvider2> for ::windows::runtime::IUnknown {
    fn from(value: ITextProvider2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITextProvider2> for ::windows::runtime::IUnknown {
    fn from(value: &ITextProvider2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextProvider2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITextProvider2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ITextProvider2> for ::windows::runtime::IInspectable {
    fn from(value: ITextProvider2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ITextProvider2> for ::windows::runtime::IInspectable {
    fn from(value: &ITextProvider2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ITextProvider2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ITextProvider2
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<ITextProvider2> for ITextProvider {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ITextProvider2) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ITextProvider2> for ITextProvider {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ITextProvider2) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextProvider> for ITextProvider2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextProvider> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextProvider> for &ITextProvider2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextProvider> {
        ::std::convert::TryInto::<ITextProvider>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextProvider2_abi(
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
        annotationelement: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        isactive: *mut bool,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ITextRangeProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextRangeProvider {
    type Vtable = ITextRangeProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2216756065,
        27874,
        20612,
        [191, 59, 40, 175, 166, 233, 133, 31],
    );
}
impl ITextRangeProvider {
    pub fn Clone(&self) -> ::windows::runtime::Result<ITextRangeProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ITextRangeProvider>(result__)
        }
    }
    pub fn Compare<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRangeProvider>>(
        &self,
        textrangeprovider: Param0,
    ) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                textrangeprovider.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Automation_Text")]
    pub fn CompareEndpoints<'a, Param1: ::windows::runtime::IntoParam<'a, ITextRangeProvider>>(
        &self,
        endpoint: super::Text::TextPatternRangeEndpoint,
        textrangeprovider: Param1,
        targetendpoint: super::Text::TextPatternRangeEndpoint,
    ) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                endpoint,
                textrangeprovider.into_param().abi(),
                targetendpoint,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Automation_Text")]
    pub fn ExpandToEnclosingUnit(
        &self,
        unit: super::Text::TextUnit,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), unit)
                .ok()
        }
    }
    pub fn FindAttribute<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        attributeid: i32,
        value: Param1,
        backward: bool,
    ) -> ::windows::runtime::Result<ITextRangeProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                attributeid,
                value.into_param().abi(),
                backward,
                &mut result__,
            )
            .from_abi::<ITextRangeProvider>(result__)
        }
    }
    pub fn FindText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        text: Param0,
        backward: bool,
        ignorecase: bool,
    ) -> ::windows::runtime::Result<ITextRangeProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                text.into_param().abi(),
                backward,
                ignorecase,
                &mut result__,
            )
            .from_abi::<ITextRangeProvider>(result__)
        }
    }
    pub fn GetAttributeValue(
        &self,
        attributeid: i32,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                attributeid,
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    pub fn GetBoundingRectangles(
        &self,
        returnvalue: &mut ::windows::runtime::Array<f64>,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                returnvalue.set_abi_len(),
                returnvalue as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetEnclosingElement(&self) -> ::windows::runtime::Result<IRawElementProviderSimple> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IRawElementProviderSimple>(result__)
        }
    }
    pub fn GetText(
        &self,
        maxlength: i32,
    ) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                maxlength,
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Automation_Text")]
    pub fn Move(&self, unit: super::Text::TextUnit, count: i32) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                unit,
                count,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Automation_Text")]
    pub fn MoveEndpointByUnit(
        &self,
        endpoint: super::Text::TextPatternRangeEndpoint,
        unit: super::Text::TextUnit,
        count: i32,
    ) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                endpoint,
                unit,
                count,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Automation_Text")]
    pub fn MoveEndpointByRange<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ITextRangeProvider>,
    >(
        &self,
        endpoint: super::Text::TextPatternRangeEndpoint,
        textrangeprovider: Param1,
        targetendpoint: super::Text::TextPatternRangeEndpoint,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                endpoint,
                textrangeprovider.into_param().abi(),
                targetendpoint,
            )
            .ok()
        }
    }
    pub fn Select(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn AddToSelection(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn RemoveFromSelection(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn ScrollIntoView(&self, aligntotop: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                aligntotop,
            )
            .ok()
        }
    }
    pub fn GetChildren(
        &self,
    ) -> ::windows::runtime::Result<::windows::runtime::Array<IRawElementProviderSimple>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<IRawElementProviderSimple> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                ::windows::runtime::Array::<IRawElementProviderSimple>::set_abi_len(&mut result__),
                &mut result__ as *mut _ as _,
            )
            .and_then(|| result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ITextRangeProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{84210361-6ce2-5084-bf3b-28afa6e9851f}");
}
impl ::std::convert::From<ITextRangeProvider> for ::windows::runtime::IUnknown {
    fn from(value: ITextRangeProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITextRangeProvider> for ::windows::runtime::IUnknown {
    fn from(value: &ITextRangeProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextRangeProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITextRangeProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ITextRangeProvider> for ::windows::runtime::IInspectable {
    fn from(value: ITextRangeProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ITextRangeProvider> for ::windows::runtime::IInspectable {
    fn from(value: &ITextRangeProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ITextRangeProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ITextRangeProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextRangeProvider_abi(
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
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        textrangeprovider: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Xaml_Automation_Text")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        endpoint: super::Text::TextPatternRangeEndpoint,
        textrangeprovider: ::windows::runtime::RawPtr,
        targetendpoint: super::Text::TextPatternRangeEndpoint,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Text"))] usize,
    #[cfg(feature = "UI_Xaml_Automation_Text")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        unit: super::Text::TextUnit,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Text"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        attributeid: i32,
        value: ::windows::runtime::RawPtr,
        backward: bool,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        text: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        backward: bool,
        ignorecase: bool,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        attributeid: i32,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        returnValue_array_size: *mut u32,
        returnvalue: *mut *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        maxlength: i32,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Xaml_Automation_Text")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        unit: super::Text::TextUnit,
        count: i32,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Text"))] usize,
    #[cfg(feature = "UI_Xaml_Automation_Text")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        endpoint: super::Text::TextPatternRangeEndpoint,
        unit: super::Text::TextUnit,
        count: i32,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Text"))] usize,
    #[cfg(feature = "UI_Xaml_Automation_Text")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        endpoint: super::Text::TextPatternRangeEndpoint,
        textrangeprovider: ::windows::runtime::RawPtr,
        targetendpoint: super::Text::TextPatternRangeEndpoint,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Text"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        aligntotop: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result_size__: *mut u32,
        result__: *mut *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ITextRangeProvider2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextRangeProvider2 {
    type Vtable = ITextRangeProvider2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        886351886,
        14011,
        21346,
        [165, 59, 73, 4, 40, 168, 179, 103],
    );
}
impl ITextRangeProvider2 {
    pub fn ShowContextMenu(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Clone(&self) -> ::windows::runtime::Result<ITextRangeProvider> {
        let this = &::windows::runtime::Interface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ITextRangeProvider>(result__)
        }
    }
    pub fn Compare<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRangeProvider>>(
        &self,
        textrangeprovider: Param0,
    ) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                textrangeprovider.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Automation_Text")]
    pub fn CompareEndpoints<'a, Param1: ::windows::runtime::IntoParam<'a, ITextRangeProvider>>(
        &self,
        endpoint: super::Text::TextPatternRangeEndpoint,
        textrangeprovider: Param1,
        targetendpoint: super::Text::TextPatternRangeEndpoint,
    ) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                endpoint,
                textrangeprovider.into_param().abi(),
                targetendpoint,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Automation_Text")]
    pub fn ExpandToEnclosingUnit(
        &self,
        unit: super::Text::TextUnit,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), unit)
                .ok()
        }
    }
    pub fn FindAttribute<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        attributeid: i32,
        value: Param1,
        backward: bool,
    ) -> ::windows::runtime::Result<ITextRangeProvider> {
        let this = &::windows::runtime::Interface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                attributeid,
                value.into_param().abi(),
                backward,
                &mut result__,
            )
            .from_abi::<ITextRangeProvider>(result__)
        }
    }
    pub fn FindText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        text: Param0,
        backward: bool,
        ignorecase: bool,
    ) -> ::windows::runtime::Result<ITextRangeProvider> {
        let this = &::windows::runtime::Interface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                text.into_param().abi(),
                backward,
                ignorecase,
                &mut result__,
            )
            .from_abi::<ITextRangeProvider>(result__)
        }
    }
    pub fn GetAttributeValue(
        &self,
        attributeid: i32,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                attributeid,
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    pub fn GetBoundingRectangles(
        &self,
        returnvalue: &mut ::windows::runtime::Array<f64>,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                returnvalue.set_abi_len(),
                returnvalue as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetEnclosingElement(&self) -> ::windows::runtime::Result<IRawElementProviderSimple> {
        let this = &::windows::runtime::Interface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IRawElementProviderSimple>(result__)
        }
    }
    pub fn GetText(
        &self,
        maxlength: i32,
    ) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                maxlength,
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Automation_Text")]
    pub fn Move(&self, unit: super::Text::TextUnit, count: i32) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                unit,
                count,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Automation_Text")]
    pub fn MoveEndpointByUnit(
        &self,
        endpoint: super::Text::TextPatternRangeEndpoint,
        unit: super::Text::TextUnit,
        count: i32,
    ) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                endpoint,
                unit,
                count,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Automation_Text")]
    pub fn MoveEndpointByRange<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ITextRangeProvider>,
    >(
        &self,
        endpoint: super::Text::TextPatternRangeEndpoint,
        textrangeprovider: Param1,
        targetendpoint: super::Text::TextPatternRangeEndpoint,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                endpoint,
                textrangeprovider.into_param().abi(),
                targetendpoint,
            )
            .ok()
        }
    }
    pub fn Select(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn AddToSelection(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn RemoveFromSelection(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn ScrollIntoView(&self, aligntotop: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                aligntotop,
            )
            .ok()
        }
    }
    pub fn GetChildren(
        &self,
    ) -> ::windows::runtime::Result<::windows::runtime::Array<IRawElementProviderSimple>> {
        let this = &::windows::runtime::Interface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::Array<IRawElementProviderSimple> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                ::windows::runtime::Array::<IRawElementProviderSimple>::set_abi_len(&mut result__),
                &mut result__ as *mut _ as _,
            )
            .and_then(|| result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ITextRangeProvider2 {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{34d4a80e-36bb-5362-a53b-490428a8b367}");
}
impl ::std::convert::From<ITextRangeProvider2> for ::windows::runtime::IUnknown {
    fn from(value: ITextRangeProvider2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITextRangeProvider2> for ::windows::runtime::IUnknown {
    fn from(value: &ITextRangeProvider2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextRangeProvider2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITextRangeProvider2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ITextRangeProvider2> for ::windows::runtime::IInspectable {
    fn from(value: ITextRangeProvider2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ITextRangeProvider2> for ::windows::runtime::IInspectable {
    fn from(value: &ITextRangeProvider2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ITextRangeProvider2
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ITextRangeProvider2
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<ITextRangeProvider2> for ITextRangeProvider {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ITextRangeProvider2) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ITextRangeProvider2> for ITextRangeProvider {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ITextRangeProvider2) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextRangeProvider> for ITextRangeProvider2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextRangeProvider> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextRangeProvider> for &ITextRangeProvider2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextRangeProvider> {
        ::std::convert::TryInto::<ITextRangeProvider>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextRangeProvider2_abi(
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IToggleProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IToggleProvider {
    type Vtable = IToggleProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        34635970,
        12457,
        21231,
        [188, 50, 43, 121, 132, 123, 107, 167],
    );
}
impl IToggleProvider {
    pub fn ToggleState(&self) -> ::windows::runtime::Result<super::ToggleState> {
        let this = self;
        unsafe {
            let mut result__: super::ToggleState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ToggleState>(result__)
        }
    }
    pub fn Toggle(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IToggleProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{021080c2-30a9-52ef-bc32-2b79847b6ba7}");
}
impl ::std::convert::From<IToggleProvider> for ::windows::runtime::IUnknown {
    fn from(value: IToggleProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IToggleProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IToggleProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IToggleProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IToggleProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IToggleProvider> for ::windows::runtime::IInspectable {
    fn from(value: IToggleProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IToggleProvider> for ::windows::runtime::IInspectable {
    fn from(value: &IToggleProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IToggleProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IToggleProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IToggleProvider_abi(
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
        result__: *mut super::ToggleState,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ITransformProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITransformProvider {
    type Vtable = ITransformProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1876388232,
        36690,
        24306,
        [168, 38, 156, 140, 73, 81, 201, 17],
    );
}
impl ITransformProvider {
    pub fn CanMove(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn CanResize(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn CanRotate(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn Move(&self, x: f64, y: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), x, y)
                .ok()
        }
    }
    pub fn Resize(&self, width: f64, height: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                width,
                height,
            )
            .ok()
        }
    }
    pub fn Rotate(&self, degrees: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                degrees,
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ITransformProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{6fd76988-8f52-5ef2-a826-9c8c4951c911}");
}
impl ::std::convert::From<ITransformProvider> for ::windows::runtime::IUnknown {
    fn from(value: ITransformProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITransformProvider> for ::windows::runtime::IUnknown {
    fn from(value: &ITransformProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITransformProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITransformProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ITransformProvider> for ::windows::runtime::IInspectable {
    fn from(value: ITransformProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ITransformProvider> for ::windows::runtime::IInspectable {
    fn from(value: &ITransformProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ITransformProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ITransformProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformProvider_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        x: f64,
        y: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        width: f64,
        height: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        degrees: f64,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ITransformProvider2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITransformProvider2 {
    type Vtable = ITransformProvider2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2106708013,
        33793,
        23800,
        [187, 196, 71, 57, 26, 82, 66, 21],
    );
}
impl ITransformProvider2 {
    pub fn CanZoom(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn ZoomLevel(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn MaxZoom(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn MinZoom(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn Zoom(&self, zoom: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), zoom)
                .ok()
        }
    }
    pub fn ZoomByUnit(&self, zoomunit: super::ZoomUnit) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                zoomunit,
            )
            .ok()
        }
    }
    pub fn CanMove(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITransformProvider>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn CanResize(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITransformProvider>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn CanRotate(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITransformProvider>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn Move(&self, x: f64, y: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITransformProvider>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), x, y)
                .ok()
        }
    }
    pub fn Resize(&self, width: f64, height: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITransformProvider>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                width,
                height,
            )
            .ok()
        }
    }
    pub fn Rotate(&self, degrees: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITransformProvider>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                degrees,
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ITransformProvider2 {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{7d91d02d-8401-5cf8-bbc4-47391a524215}");
}
impl ::std::convert::From<ITransformProvider2> for ::windows::runtime::IUnknown {
    fn from(value: ITransformProvider2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITransformProvider2> for ::windows::runtime::IUnknown {
    fn from(value: &ITransformProvider2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITransformProvider2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITransformProvider2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ITransformProvider2> for ::windows::runtime::IInspectable {
    fn from(value: ITransformProvider2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ITransformProvider2> for ::windows::runtime::IInspectable {
    fn from(value: &ITransformProvider2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ITransformProvider2
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ITransformProvider2
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<ITransformProvider2> for ITransformProvider {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ITransformProvider2) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ITransformProvider2> for ITransformProvider {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ITransformProvider2) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITransformProvider> for ITransformProvider2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITransformProvider> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITransformProvider> for &ITransformProvider2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITransformProvider> {
        ::std::convert::TryInto::<ITransformProvider>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformProvider2_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        zoom: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        zoomunit: super::ZoomUnit,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IValueProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IValueProvider {
    type Vtable = IValueProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2555318735,
        17937,
        22670,
        [181, 46, 185, 106, 18, 50, 44, 113],
    );
}
impl IValueProvider {
    pub fn IsReadOnly(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn Value(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IValueProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{984f11cf-4611-588e-b52e-b96a12322c71}");
}
impl ::std::convert::From<IValueProvider> for ::windows::runtime::IUnknown {
    fn from(value: IValueProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IValueProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IValueProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IValueProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IValueProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IValueProvider> for ::windows::runtime::IInspectable {
    fn from(value: IValueProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IValueProvider> for ::windows::runtime::IInspectable {
    fn from(value: &IValueProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IValueProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IValueProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IValueProvider_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IVirtualizedItemProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVirtualizedItemProvider {
    type Vtable = IVirtualizedItemProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        160400778,
        11875,
        22917,
        [171, 135, 248, 235, 219, 28, 87, 64],
    );
}
impl IVirtualizedItemProvider {
    pub fn Realize(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IVirtualizedItemProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{098f858a-2e63-5985-ab87-f8ebdb1c5740}");
}
impl ::std::convert::From<IVirtualizedItemProvider> for ::windows::runtime::IUnknown {
    fn from(value: IVirtualizedItemProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVirtualizedItemProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IVirtualizedItemProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IVirtualizedItemProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IVirtualizedItemProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IVirtualizedItemProvider> for ::windows::runtime::IInspectable {
    fn from(value: IVirtualizedItemProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IVirtualizedItemProvider> for ::windows::runtime::IInspectable {
    fn from(value: &IVirtualizedItemProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for IVirtualizedItemProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IVirtualizedItemProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVirtualizedItemProvider_abi(
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IWindowProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWindowProvider {
    type Vtable = IWindowProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2213666713,
        40415,
        22366,
        [166, 81, 46, 230, 87, 253, 22, 224],
    );
}
impl IWindowProvider {
    pub fn IsModal(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn IsTopmost(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn Maximizable(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn Minimizable(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn InteractionState(&self) -> ::windows::runtime::Result<super::WindowInteractionState> {
        let this = self;
        unsafe {
            let mut result__: super::WindowInteractionState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::WindowInteractionState>(result__)
        }
    }
    pub fn VisualState(&self) -> ::windows::runtime::Result<super::WindowVisualState> {
        let this = self;
        unsafe {
            let mut result__: super::WindowVisualState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::WindowVisualState>(result__)
        }
    }
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn SetVisualState(
        &self,
        state: super::WindowVisualState,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                state,
            )
            .ok()
        }
    }
    pub fn WaitForInputIdle(&self, milliseconds: i32) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                milliseconds,
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IWindowProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{83f1df99-9ddf-575e-a651-2ee657fd16e0}");
}
impl ::std::convert::From<IWindowProvider> for ::windows::runtime::IUnknown {
    fn from(value: IWindowProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWindowProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IWindowProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWindowProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWindowProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IWindowProvider> for ::windows::runtime::IInspectable {
    fn from(value: IWindowProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWindowProvider> for ::windows::runtime::IInspectable {
    fn from(value: &IWindowProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IWindowProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IWindowProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowProvider_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::WindowInteractionState,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::WindowVisualState,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        state: super::WindowVisualState,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        milliseconds: i32,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
);
