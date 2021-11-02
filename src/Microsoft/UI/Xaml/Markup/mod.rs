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
pub struct IComponentConnector(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IComponentConnector {
    type Vtable = IComponentConnector_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2906658834,
        45201,
        20944,
        [185, 21, 45, 104, 44, 210, 175, 16],
    );
}
impl IComponentConnector {
    pub fn Connect<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        connectionid: i32,
        target: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                connectionid,
                target.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn GetBindingConnector<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        connectionid: i32,
        target: Param1,
    ) -> ::windows::runtime::Result<IComponentConnector> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                connectionid,
                target.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IComponentConnector>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IComponentConnector {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{ad401812-b091-51d0-b915-2d682cd2af10}");
}
impl ::std::convert::From<IComponentConnector> for ::windows::runtime::IUnknown {
    fn from(value: IComponentConnector) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IComponentConnector> for ::windows::runtime::IUnknown {
    fn from(value: &IComponentConnector) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IComponentConnector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IComponentConnector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IComponentConnector> for ::windows::runtime::IInspectable {
    fn from(value: IComponentConnector) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IComponentConnector> for ::windows::runtime::IInspectable {
    fn from(value: &IComponentConnector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for IComponentConnector
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IComponentConnector
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComponentConnector_abi(
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
        connectionid: i32,
        target: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        connectionid: i32,
        target: ::windows::runtime::RawPtr,
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
pub struct IDataTemplateComponent(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataTemplateComponent {
    type Vtable = IDataTemplateComponent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        390323703,
        14522,
        22729,
        [162, 166, 176, 174, 40, 113, 59, 238],
    );
}
impl IDataTemplateComponent {
    pub fn Recycle(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn ProcessBindings<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        item: Param0,
        itemindex: i32,
        phase: i32,
        nextphase: &mut i32,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                item.into_param().abi(),
                itemindex,
                phase,
                nextphase,
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IDataTemplateComponent {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{1743ddf7-38ba-58c9-a2a6-b0ae28713bee}");
}
impl ::std::convert::From<IDataTemplateComponent> for ::windows::runtime::IUnknown {
    fn from(value: IDataTemplateComponent) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDataTemplateComponent> for ::windows::runtime::IUnknown {
    fn from(value: &IDataTemplateComponent) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IDataTemplateComponent
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IDataTemplateComponent
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDataTemplateComponent> for ::windows::runtime::IInspectable {
    fn from(value: IDataTemplateComponent) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDataTemplateComponent> for ::windows::runtime::IInspectable {
    fn from(value: &IDataTemplateComponent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for IDataTemplateComponent
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IDataTemplateComponent
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataTemplateComponent_abi(
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
        item: ::windows::runtime::RawPtr,
        itemindex: i32,
        phase: i32,
        nextphase: *mut i32,
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
pub struct IMarkupExtension(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMarkupExtension {
    type Vtable = IMarkupExtension_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3277141790,
        2333,
        20790,
        [175, 74, 186, 245, 224, 6, 22, 189],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IMarkupExtension_abi(
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
#[doc(hidden)]
pub struct IMarkupExtensionFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMarkupExtensionFactory {
    type Vtable = IMarkupExtensionFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        543496954,
        24378,
        24332,
        [173, 177, 182, 85, 31, 83, 166, 160],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IMarkupExtensionFactory_abi(
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
        baseinterface: ::windows::runtime::RawPtr,
        innerinterface: *mut ::windows::runtime::RawPtr,
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
pub struct IMarkupExtensionOverrides(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMarkupExtensionOverrides {
    type Vtable = IMarkupExtensionOverrides_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2703926645,
        23857,
        23400,
        [163, 15, 132, 149, 65, 42, 53, 29],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IMarkupExtensionOverrides_abi(
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
        serviceprovider: ::windows::runtime::RawPtr,
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
pub struct IProvideValueTarget(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProvideValueTarget {
    type Vtable = IProvideValueTarget_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1057095528,
        16125,
        22813,
        [165, 6, 222, 19, 252, 170, 189, 131],
    );
}
impl IProvideValueTarget {
    pub fn TargetObject(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
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
    pub fn TargetProperty(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IProvideValueTarget {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{3f01ff68-3efd-591d-a506-de13fcaabd83}");
}
impl ::std::convert::From<IProvideValueTarget> for ::windows::runtime::IUnknown {
    fn from(value: IProvideValueTarget) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IProvideValueTarget> for ::windows::runtime::IUnknown {
    fn from(value: &IProvideValueTarget) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IProvideValueTarget {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IProvideValueTarget {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IProvideValueTarget> for ::windows::runtime::IInspectable {
    fn from(value: IProvideValueTarget) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IProvideValueTarget> for ::windows::runtime::IInspectable {
    fn from(value: &IProvideValueTarget) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for IProvideValueTarget
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IProvideValueTarget
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvideValueTarget_abi(
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
#[doc(hidden)]
pub struct IProvideValueTargetProperty(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProvideValueTargetProperty {
    type Vtable = IProvideValueTargetProperty_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3463936799,
        46126,
        22993,
        [135, 13, 18, 253, 240, 98, 145, 51],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvideValueTargetProperty_abi(
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
    #[cfg(feature = "_Xaml_Interop")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<
            super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
        >,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "_Xaml_Interop"))] usize,
    #[cfg(feature = "_Xaml_Interop")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<
            super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
        >,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "_Xaml_Interop"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IRootObjectProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRootObjectProvider {
    type Vtable = IRootObjectProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        332805529,
        13615,
        24248,
        [129, 193, 188, 98, 251, 18, 214, 218],
    );
}
impl IRootObjectProvider {
    pub fn RootObject(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
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
unsafe impl ::windows::runtime::RuntimeType for IRootObjectProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{13d63599-352f-5eb8-81c1-bc62fb12d6da}");
}
impl ::std::convert::From<IRootObjectProvider> for ::windows::runtime::IUnknown {
    fn from(value: IRootObjectProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IRootObjectProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IRootObjectProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRootObjectProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IRootObjectProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IRootObjectProvider> for ::windows::runtime::IInspectable {
    fn from(value: IRootObjectProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IRootObjectProvider> for ::windows::runtime::IInspectable {
    fn from(value: &IRootObjectProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for IRootObjectProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IRootObjectProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRootObjectProvider_abi(
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
pub struct IUriContext(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUriContext {
    type Vtable = IUriContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4219864566,
        36613,
        21230,
        [160, 28, 58, 158, 17, 138, 110, 162],
    );
}
impl IUriContext {
    #[cfg(feature = "undation")]
    pub fn BaseUri(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Uri>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IUriContext {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{fb8605f6-8f05-52ee-a01c-3a9e118a6ea2}");
}
impl ::std::convert::From<IUriContext> for ::windows::runtime::IUnknown {
    fn from(value: IUriContext) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUriContext> for ::windows::runtime::IUnknown {
    fn from(value: &IUriContext) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUriContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUriContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IUriContext> for ::windows::runtime::IInspectable {
    fn from(value: IUriContext) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IUriContext> for ::windows::runtime::IInspectable {
    fn from(value: &IUriContext) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IUriContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IUriContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUriContext_abi(
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
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
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
#[doc(hidden)]
pub struct IXamlBinaryWriter(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlBinaryWriter {
    type Vtable = IXamlBinaryWriter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2410962491,
        59017,
        21951,
        [170, 17, 216, 59, 28, 28, 221, 161],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlBinaryWriter_abi(
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
#[doc(hidden)]
pub struct IXamlBinaryWriterStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlBinaryWriterStatics {
    type Vtable = IXamlBinaryWriterStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2001274876,
        51270,
        20863,
        [171, 204, 195, 247, 232, 195, 255, 201],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlBinaryWriterStatics_abi(
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
    #[cfg(all(feature = "undation_Collections", feature = "orage_Streams"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        inputstreams: ::windows::runtime::RawPtr,
        outputstreams: ::windows::runtime::RawPtr,
        xamlmetadataprovider: ::windows::runtime::RawPtr,
        result__: *mut XamlBinaryWriterErrorInformation,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "undation_Collections", feature = "orage_Streams")))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IXamlBindScopeDiagnostics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlBindScopeDiagnostics {
    type Vtable = IXamlBindScopeDiagnostics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1051217486,
        65022,
        21928,
        [165, 97, 237, 245, 105, 120, 70, 215],
    );
}
impl IXamlBindScopeDiagnostics {
    pub fn Disable(&self, linenumber: i32, columnnumber: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                linenumber,
                columnnumber,
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IXamlBindScopeDiagnostics {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{3ea84e4e-fdfe-55a8-a561-edf5697846d7}");
}
impl ::std::convert::From<IXamlBindScopeDiagnostics> for ::windows::runtime::IUnknown {
    fn from(value: IXamlBindScopeDiagnostics) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IXamlBindScopeDiagnostics> for ::windows::runtime::IUnknown {
    fn from(value: &IXamlBindScopeDiagnostics) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IXamlBindScopeDiagnostics
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IXamlBindScopeDiagnostics
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IXamlBindScopeDiagnostics> for ::windows::runtime::IInspectable {
    fn from(value: IXamlBindScopeDiagnostics) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IXamlBindScopeDiagnostics> for ::windows::runtime::IInspectable {
    fn from(value: &IXamlBindScopeDiagnostics) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for IXamlBindScopeDiagnostics
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IXamlBindScopeDiagnostics
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlBindScopeDiagnostics_abi(
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
        linenumber: i32,
        columnnumber: i32,
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
pub struct IXamlBindingHelper(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlBindingHelper {
    type Vtable = IXamlBindingHelper_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1618648050,
        23149,
        23689,
        [167, 86, 187, 68, 242, 79, 40, 248],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlBindingHelper_abi(
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
#[doc(hidden)]
pub struct IXamlBindingHelperStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlBindingHelperStatics {
    type Vtable = IXamlBindingHelperStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2479348435,
        63938,
        21362,
        [132, 220, 158, 156, 70, 97, 208, 131],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlBindingHelperStatics_abi(
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
        element: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        target: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        target: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "_Xaml_Interop")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        r#type: ::std::mem::ManuallyDrop<
            super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
        >,
        value: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "_Xaml_Interop"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dependencyobject: ::windows::runtime::RawPtr,
        propertytoset: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dependencyobject: ::windows::runtime::RawPtr,
        propertytoset: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dependencyobject: ::windows::runtime::RawPtr,
        propertytoset: ::windows::runtime::RawPtr,
        value: u16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dependencyobject: ::windows::runtime::RawPtr,
        propertytoset: ::windows::runtime::RawPtr,
        value: super::super::super::super::Windows::Foundation::DateTime,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dependencyobject: ::windows::runtime::RawPtr,
        propertytoset: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dependencyobject: ::windows::runtime::RawPtr,
        propertytoset: ::windows::runtime::RawPtr,
        value: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dependencyobject: ::windows::runtime::RawPtr,
        propertytoset: ::windows::runtime::RawPtr,
        value: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dependencyobject: ::windows::runtime::RawPtr,
        propertytoset: ::windows::runtime::RawPtr,
        value: i64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dependencyobject: ::windows::runtime::RawPtr,
        propertytoset: ::windows::runtime::RawPtr,
        value: u64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dependencyobject: ::windows::runtime::RawPtr,
        propertytoset: ::windows::runtime::RawPtr,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dependencyobject: ::windows::runtime::RawPtr,
        propertytoset: ::windows::runtime::RawPtr,
        value: super::super::super::super::Windows::Foundation::Point,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dependencyobject: ::windows::runtime::RawPtr,
        propertytoset: ::windows::runtime::RawPtr,
        value: super::super::super::super::Windows::Foundation::Rect,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dependencyobject: ::windows::runtime::RawPtr,
        propertytoset: ::windows::runtime::RawPtr,
        value: super::super::super::super::Windows::Foundation::Size,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dependencyobject: ::windows::runtime::RawPtr,
        propertytoset: ::windows::runtime::RawPtr,
        value: super::super::super::super::Windows::Foundation::TimeSpan,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dependencyobject: ::windows::runtime::RawPtr,
        propertytoset: ::windows::runtime::RawPtr,
        value: u8,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dependencyobject: ::windows::runtime::RawPtr,
        propertytoset: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dependencyobject: ::windows::runtime::RawPtr,
        propertytoset: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
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
pub struct IXamlMarkupHelper(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlMarkupHelper {
    type Vtable = IXamlMarkupHelper_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3446108944,
        15110,
        23059,
        [179, 26, 64, 24, 73, 87, 8, 88],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlMarkupHelper_abi(
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
#[doc(hidden)]
pub struct IXamlMarkupHelperStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlMarkupHelperStatics {
    type Vtable = IXamlMarkupHelperStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3651204835,
        50892,
        23734,
        [137, 153, 133, 120, 135, 1, 243, 57],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlMarkupHelperStatics_abi(
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
        element: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IXamlMember(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlMember {
    type Vtable = IXamlMember_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3208259859,
        23651,
        20716,
        [134, 96, 97, 128, 155, 231, 185, 185],
    );
}
impl IXamlMember {
    pub fn IsAttachable(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn IsDependencyProperty(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn IsReadOnly(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn TargetType(&self) -> ::windows::runtime::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IXamlType>(result__)
        }
    }
    pub fn Type(&self) -> ::windows::runtime::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IXamlType>(result__)
        }
    }
    pub fn GetValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        instance: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                instance.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    pub fn SetValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        instance: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                instance.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IXamlMember {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{bf3a2913-5c63-50ec-8660-61809be7b9b9}");
}
impl ::std::convert::From<IXamlMember> for ::windows::runtime::IUnknown {
    fn from(value: IXamlMember) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IXamlMember> for ::windows::runtime::IUnknown {
    fn from(value: &IXamlMember) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXamlMember {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IXamlMember {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IXamlMember> for ::windows::runtime::IInspectable {
    fn from(value: IXamlMember) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IXamlMember> for ::windows::runtime::IInspectable {
    fn from(value: &IXamlMember) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IXamlMember {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IXamlMember {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlMember_abi(
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
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        instance: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        instance: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IXamlMetadataProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlMetadataProvider {
    type Vtable = IXamlMetadataProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2841793008,
        8724,
        23891,
        [135, 70, 206, 153, 162, 89, 60, 215],
    );
}
impl IXamlMetadataProvider {
    #[cfg(feature = "_Xaml_Interop")]
    pub fn GetXamlType<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
        >,
    >(
        &self,
        r#type: Param0,
    ) -> ::windows::runtime::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                r#type.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IXamlType>(result__)
        }
    }
    pub fn GetXamlTypeByFullName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        fullname: Param0,
    ) -> ::windows::runtime::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                fullname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IXamlType>(result__)
        }
    }
    pub fn GetXmlnsDefinitions(
        &self,
    ) -> ::windows::runtime::Result<::windows::runtime::Array<XmlnsDefinition>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<XmlnsDefinition> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                ::windows::runtime::Array::<XmlnsDefinition>::set_abi_len(&mut result__),
                &mut result__ as *mut _ as _,
            )
            .and_then(|| result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IXamlMetadataProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{a96251f0-2214-5d53-8746-ce99a2593cd7}");
}
impl ::std::convert::From<IXamlMetadataProvider> for ::windows::runtime::IUnknown {
    fn from(value: IXamlMetadataProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IXamlMetadataProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IXamlMetadataProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXamlMetadataProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IXamlMetadataProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IXamlMetadataProvider> for ::windows::runtime::IInspectable {
    fn from(value: IXamlMetadataProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IXamlMetadataProvider> for ::windows::runtime::IInspectable {
    fn from(value: &IXamlMetadataProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for IXamlMetadataProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IXamlMetadataProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlMetadataProvider_abi(
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
    #[cfg(feature = "_Xaml_Interop")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        r#type: ::std::mem::ManuallyDrop<
            super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
        >,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "_Xaml_Interop"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        fullname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result_size__: *mut u32,
        result__: *mut *mut ::std::mem::ManuallyDrop<XmlnsDefinition>,
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
pub struct IXamlReader(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlReader {
    type Vtable = IXamlReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1422808264,
        14534,
        20697,
        [172, 152, 75, 3, 237, 219, 222, 159],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlReader_abi(
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
#[doc(hidden)]
pub struct IXamlReaderStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlReaderStatics {
    type Vtable = IXamlReaderStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2191838622,
        17246,
        23275,
        [140, 79, 48, 12, 236, 228, 92, 174],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlReaderStatics_abi(
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
        xaml: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        xaml: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
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
pub struct IXamlType(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlType {
    type Vtable = IXamlType_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3527547359,
        32457,
        22513,
        [162, 123, 106, 242, 81, 217, 197, 188],
    );
}
impl IXamlType {
    pub fn BaseType(&self) -> ::windows::runtime::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IXamlType>(result__)
        }
    }
    pub fn ContentProperty(&self) -> ::windows::runtime::Result<IXamlMember> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IXamlMember>(result__)
        }
    }
    pub fn FullName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn IsArray(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn IsCollection(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn IsConstructible(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn IsDictionary(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn IsMarkupExtension(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn IsBindable(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn ItemType(&self) -> ::windows::runtime::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IXamlType>(result__)
        }
    }
    pub fn KeyType(&self) -> ::windows::runtime::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IXamlType>(result__)
        }
    }
    pub fn BoxedType(&self) -> ::windows::runtime::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IXamlType>(result__)
        }
    }
    #[cfg(feature = "_Xaml_Interop")]
    pub fn UnderlyingType(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Xaml::Interop::TypeName>
    {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<
                super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
            > = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Xaml::Interop::TypeName>(result__)
        }
    }
    pub fn ActivateInstance(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    pub fn CreateFromString<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    pub fn GetMember<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::runtime::Result<IXamlMember> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
                ::std::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IXamlMember>(result__)
        }
    }
    pub fn AddToVector<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        instance: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                instance.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn AddToMap<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        instance: Param0,
        key: Param1,
        value: Param2,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                instance.into_param().abi(),
                key.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn RunInitializer(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IXamlType {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{d24219df-7ec9-57f1-a27b-6af251d9c5bc}");
}
impl ::std::convert::From<IXamlType> for ::windows::runtime::IUnknown {
    fn from(value: IXamlType) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IXamlType> for ::windows::runtime::IUnknown {
    fn from(value: &IXamlType) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXamlType {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IXamlType {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IXamlType> for ::windows::runtime::IInspectable {
    fn from(value: IXamlType) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IXamlType> for ::windows::runtime::IInspectable {
    fn from(value: &IXamlType) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IXamlType {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IXamlType {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlType_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
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
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "_Xaml_Interop")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<
            super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
        >,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "_Xaml_Interop"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        instance: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        instance: ::windows::runtime::RawPtr,
        key: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
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
pub struct IXamlTypeResolver(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlTypeResolver {
    type Vtable = IXamlTypeResolver_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1067537941,
        51919,
        21631,
        [177, 237, 137, 218, 232, 198, 116, 82],
    );
}
impl IXamlTypeResolver {
    #[cfg(feature = "_Xaml_Interop")]
    pub fn Resolve<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        qualifiedtypename: Param0,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Xaml::Interop::TypeName>
    {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<
                super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
            > = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                qualifiedtypename.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Xaml::Interop::TypeName>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IXamlTypeResolver {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{3fa15615-cacf-547f-b1ed-89dae8c67452}");
}
impl ::std::convert::From<IXamlTypeResolver> for ::windows::runtime::IUnknown {
    fn from(value: IXamlTypeResolver) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IXamlTypeResolver> for ::windows::runtime::IUnknown {
    fn from(value: &IXamlTypeResolver) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXamlTypeResolver {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IXamlTypeResolver {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IXamlTypeResolver> for ::windows::runtime::IInspectable {
    fn from(value: IXamlTypeResolver) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IXamlTypeResolver> for ::windows::runtime::IInspectable {
    fn from(value: &IXamlTypeResolver) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IXamlTypeResolver {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IXamlTypeResolver
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlTypeResolver_abi(
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
    #[cfg(feature = "_Xaml_Interop")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        qualifiedtypename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::std::mem::ManuallyDrop<
            super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
        >,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "_Xaml_Interop"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct MarkupExtension(::windows::runtime::IInspectable);
impl MarkupExtension {
    pub fn ProvideValue(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IMarkupExtensionOverrides>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    pub fn ProvideValueWithIXamlServiceProvider<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::IXamlServiceProvider>,
    >(
        &self,
        serviceprovider: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IMarkupExtensionOverrides>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                serviceprovider.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    pub fn new() -> ::windows::runtime::Result<MarkupExtension> {
        Self::IMarkupExtensionFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                ::std::ptr::null_mut(),
                &mut ::std::option::Option::<::windows::runtime::IInspectable>::None as *mut _ as _,
                &mut result__,
            )
            .from_abi::<MarkupExtension>(result__)
        })
    }
    pub fn IMarkupExtensionFactory<
        R,
        F: FnOnce(&IMarkupExtensionFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            MarkupExtension,
            IMarkupExtensionFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MarkupExtension {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Markup.MarkupExtension;{c355371e-091d-5136-af4a-baf5e00616bd})",
    );
}
unsafe impl ::windows::runtime::Interface for MarkupExtension {
    type Vtable = IMarkupExtension_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3277141790,
        2333,
        20790,
        [175, 74, 186, 245, 224, 6, 22, 189],
    );
}
impl ::windows::runtime::RuntimeName for MarkupExtension {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.MarkupExtension";
}
impl ::std::convert::From<MarkupExtension> for ::windows::runtime::IUnknown {
    fn from(value: MarkupExtension) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MarkupExtension> for ::windows::runtime::IUnknown {
    fn from(value: &MarkupExtension) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MarkupExtension {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MarkupExtension {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<MarkupExtension> for ::windows::runtime::IInspectable {
    fn from(value: MarkupExtension) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MarkupExtension> for ::windows::runtime::IInspectable {
    fn from(value: &MarkupExtension) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MarkupExtension {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a MarkupExtension
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MarkupExtension {}
unsafe impl ::std::marker::Sync for MarkupExtension {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ProvideValueTargetProperty(::windows::runtime::IInspectable);
impl ProvideValueTargetProperty {
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
            ProvideValueTargetProperty,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    #[cfg(feature = "_Xaml_Interop")]
    pub fn Type(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Xaml::Interop::TypeName>
    {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<
                super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
            > = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Xaml::Interop::TypeName>(result__)
        }
    }
    #[cfg(feature = "_Xaml_Interop")]
    pub fn DeclaringType(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Xaml::Interop::TypeName>
    {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<
                super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
            > = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Xaml::Interop::TypeName>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ProvideValueTargetProperty {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Markup.ProvideValueTargetProperty;{ce777b1f-b42e-59d1-870d-12fdf0629133})" ) ;
}
unsafe impl ::windows::runtime::Interface for ProvideValueTargetProperty {
    type Vtable = IProvideValueTargetProperty_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3463936799,
        46126,
        22993,
        [135, 13, 18, 253, 240, 98, 145, 51],
    );
}
impl ::windows::runtime::RuntimeName for ProvideValueTargetProperty {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.ProvideValueTargetProperty";
}
impl ::std::convert::From<ProvideValueTargetProperty> for ::windows::runtime::IUnknown {
    fn from(value: ProvideValueTargetProperty) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ProvideValueTargetProperty> for ::windows::runtime::IUnknown {
    fn from(value: &ProvideValueTargetProperty) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ProvideValueTargetProperty
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ProvideValueTargetProperty
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ProvideValueTargetProperty> for ::windows::runtime::IInspectable {
    fn from(value: ProvideValueTargetProperty) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ProvideValueTargetProperty> for ::windows::runtime::IInspectable {
    fn from(value: &ProvideValueTargetProperty) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ProvideValueTargetProperty
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ProvideValueTargetProperty
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ProvideValueTargetProperty {}
unsafe impl ::std::marker::Sync for ProvideValueTargetProperty {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct XamlBinaryWriter(::windows::runtime::IInspectable);
impl XamlBinaryWriter {
    #[cfg(all(feature = "undation_Collections", feature = "orage_Streams"))]
    pub fn Write<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::Collections::IVector<
                super::super::super::super::Windows::Storage::Streams::IRandomAccessStream,
            >,
        >,
        Param1: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::Collections::IVector<
                super::super::super::super::Windows::Storage::Streams::IRandomAccessStream,
            >,
        >,
        Param2: ::windows::runtime::IntoParam<'a, IXamlMetadataProvider>,
    >(
        inputstreams: Param0,
        outputstreams: Param1,
        xamlmetadataprovider: Param2,
    ) -> ::windows::runtime::Result<XamlBinaryWriterErrorInformation> {
        Self::IXamlBinaryWriterStatics(|this| unsafe {
            let mut result__: XamlBinaryWriterErrorInformation = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                inputstreams.into_param().abi(),
                outputstreams.into_param().abi(),
                xamlmetadataprovider.into_param().abi(),
                &mut result__,
            )
            .from_abi::<XamlBinaryWriterErrorInformation>(result__)
        })
    }
    pub fn IXamlBinaryWriterStatics<
        R,
        F: FnOnce(&IXamlBinaryWriterStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            XamlBinaryWriter,
            IXamlBinaryWriterStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for XamlBinaryWriter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Markup.XamlBinaryWriter;{8fb45e3b-e689-55bf-aa11-d83b1c1cdda1})",
    );
}
unsafe impl ::windows::runtime::Interface for XamlBinaryWriter {
    type Vtable = IXamlBinaryWriter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2410962491,
        59017,
        21951,
        [170, 17, 216, 59, 28, 28, 221, 161],
    );
}
impl ::windows::runtime::RuntimeName for XamlBinaryWriter {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.XamlBinaryWriter";
}
impl ::std::convert::From<XamlBinaryWriter> for ::windows::runtime::IUnknown {
    fn from(value: XamlBinaryWriter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&XamlBinaryWriter> for ::windows::runtime::IUnknown {
    fn from(value: &XamlBinaryWriter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for XamlBinaryWriter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &XamlBinaryWriter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<XamlBinaryWriter> for ::windows::runtime::IInspectable {
    fn from(value: XamlBinaryWriter) -> Self {
        value.0
    }
}
impl ::std::convert::From<&XamlBinaryWriter> for ::windows::runtime::IInspectable {
    fn from(value: &XamlBinaryWriter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for XamlBinaryWriter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a XamlBinaryWriter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for XamlBinaryWriter {}
unsafe impl ::std::marker::Sync for XamlBinaryWriter {}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct XamlBinaryWriterErrorInformation {
    pub InputStreamIndex: u32,
    pub LineNumber: u32,
    pub LinePosition: u32,
}
impl XamlBinaryWriterErrorInformation {}
impl ::std::default::Default for XamlBinaryWriterErrorInformation {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for XamlBinaryWriterErrorInformation {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("XamlBinaryWriterErrorInformation")
            .field("InputStreamIndex", &self.InputStreamIndex)
            .field("LineNumber", &self.LineNumber)
            .field("LinePosition", &self.LinePosition)
            .finish()
    }
}
impl ::std::cmp::PartialEq for XamlBinaryWriterErrorInformation {
    fn eq(&self, other: &Self) -> bool {
        self.InputStreamIndex == other.InputStreamIndex
            && self.LineNumber == other.LineNumber
            && self.LinePosition == other.LinePosition
    }
}
impl ::std::cmp::Eq for XamlBinaryWriterErrorInformation {}
unsafe impl ::windows::runtime::Abi for XamlBinaryWriterErrorInformation {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for XamlBinaryWriterErrorInformation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"struct(Microsoft.UI.Xaml.Markup.XamlBinaryWriterErrorInformation;u4;u4;u4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct XamlBindingHelper(::windows::runtime::IInspectable);
impl XamlBindingHelper {
    pub fn DataTemplateComponentProperty() -> ::windows::runtime::Result<super::DependencyProperty>
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetDataTemplateComponent<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<IDataTemplateComponent> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IDataTemplateComponent>(result__)
        })
    }
    pub fn SetDataTemplateComponent<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
        Param1: ::windows::runtime::IntoParam<'a, IDataTemplateComponent>,
    >(
        element: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn SuspendRendering<'a, Param0: ::windows::runtime::IntoParam<'a, super::UIElement>>(
        target: Param0,
    ) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                target.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn ResumeRendering<'a, Param0: ::windows::runtime::IntoParam<'a, super::UIElement>>(
        target: Param0,
    ) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                target.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(feature = "_Xaml_Interop")]
    pub fn ConvertValue<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
        >,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        r#type: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                r#type.into_param().abi(),
                value.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    pub fn SetPropertyFromString<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        dependencyobject: Param0,
        propertytoset: Param1,
        value: Param2,
    ) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn SetPropertyFromBoolean<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        dependencyobject: Param0,
        propertytoset: Param1,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn SetPropertyFromChar16<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        dependencyobject: Param0,
        propertytoset: Param1,
        value: u16,
    ) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "undation")]
    pub fn SetPropertyFromDateTime<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param2: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::DateTime,
        >,
    >(
        dependencyobject: Param0,
        propertytoset: Param1,
        value: Param2,
    ) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn SetPropertyFromDouble<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        dependencyobject: Param0,
        propertytoset: Param1,
        value: f64,
    ) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn SetPropertyFromInt32<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        dependencyobject: Param0,
        propertytoset: Param1,
        value: i32,
    ) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn SetPropertyFromUInt32<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        dependencyobject: Param0,
        propertytoset: Param1,
        value: u32,
    ) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn SetPropertyFromInt64<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        dependencyobject: Param0,
        propertytoset: Param1,
        value: i64,
    ) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn SetPropertyFromUInt64<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        dependencyobject: Param0,
        propertytoset: Param1,
        value: u64,
    ) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn SetPropertyFromSingle<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        dependencyobject: Param0,
        propertytoset: Param1,
        value: f32,
    ) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).21)(
                ::std::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "undation")]
    pub fn SetPropertyFromPoint<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::super::super::Windows::Foundation::Point>,
    >(
        dependencyobject: Param0,
        propertytoset: Param1,
        value: Param2,
    ) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(feature = "undation")]
    pub fn SetPropertyFromRect<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::super::super::Windows::Foundation::Rect>,
    >(
        dependencyobject: Param0,
        propertytoset: Param1,
        value: Param2,
    ) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(feature = "undation")]
    pub fn SetPropertyFromSize<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::super::super::Windows::Foundation::Size>,
    >(
        dependencyobject: Param0,
        propertytoset: Param1,
        value: Param2,
    ) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(feature = "undation")]
    pub fn SetPropertyFromTimeSpan<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param2: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TimeSpan,
        >,
    >(
        dependencyobject: Param0,
        propertytoset: Param1,
        value: Param2,
    ) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn SetPropertyFromByte<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        dependencyobject: Param0,
        propertytoset: Param1,
        value: u8,
    ) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "undation")]
    pub fn SetPropertyFromUri<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::super::super::Windows::Foundation::Uri>,
    >(
        dependencyobject: Param0,
        propertytoset: Param1,
        value: Param2,
    ) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).27)(
                ::std::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn SetPropertyFromObject<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        dependencyobject: Param0,
        propertytoset: Param1,
        value: Param2,
    ) -> ::windows::runtime::Result<()> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).28)(
                ::std::mem::transmute_copy(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn IXamlBindingHelperStatics<
        R,
        F: FnOnce(&IXamlBindingHelperStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            XamlBindingHelper,
            IXamlBindingHelperStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for XamlBindingHelper {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Markup.XamlBindingHelper;{607a9bf2-5a6d-5c89-a756-bb44f24f28f8})",
    );
}
unsafe impl ::windows::runtime::Interface for XamlBindingHelper {
    type Vtable = IXamlBindingHelper_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1618648050,
        23149,
        23689,
        [167, 86, 187, 68, 242, 79, 40, 248],
    );
}
impl ::windows::runtime::RuntimeName for XamlBindingHelper {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.XamlBindingHelper";
}
impl ::std::convert::From<XamlBindingHelper> for ::windows::runtime::IUnknown {
    fn from(value: XamlBindingHelper) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&XamlBindingHelper> for ::windows::runtime::IUnknown {
    fn from(value: &XamlBindingHelper) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for XamlBindingHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &XamlBindingHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<XamlBindingHelper> for ::windows::runtime::IInspectable {
    fn from(value: XamlBindingHelper) -> Self {
        value.0
    }
}
impl ::std::convert::From<&XamlBindingHelper> for ::windows::runtime::IInspectable {
    fn from(value: &XamlBindingHelper) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for XamlBindingHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a XamlBindingHelper
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for XamlBindingHelper {}
unsafe impl ::std::marker::Sync for XamlBindingHelper {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct XamlMarkupHelper(::windows::runtime::IInspectable);
impl XamlMarkupHelper {
    pub fn UnloadObject<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::runtime::Result<()> {
        Self::IXamlMarkupHelperStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn IXamlMarkupHelperStatics<
        R,
        F: FnOnce(&IXamlMarkupHelperStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            XamlMarkupHelper,
            IXamlMarkupHelperStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for XamlMarkupHelper {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Markup.XamlMarkupHelper;{cd677310-3b06-5a13-b31a-401849570858})",
    );
}
unsafe impl ::windows::runtime::Interface for XamlMarkupHelper {
    type Vtable = IXamlMarkupHelper_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3446108944,
        15110,
        23059,
        [179, 26, 64, 24, 73, 87, 8, 88],
    );
}
impl ::windows::runtime::RuntimeName for XamlMarkupHelper {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.XamlMarkupHelper";
}
impl ::std::convert::From<XamlMarkupHelper> for ::windows::runtime::IUnknown {
    fn from(value: XamlMarkupHelper) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&XamlMarkupHelper> for ::windows::runtime::IUnknown {
    fn from(value: &XamlMarkupHelper) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for XamlMarkupHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &XamlMarkupHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<XamlMarkupHelper> for ::windows::runtime::IInspectable {
    fn from(value: XamlMarkupHelper) -> Self {
        value.0
    }
}
impl ::std::convert::From<&XamlMarkupHelper> for ::windows::runtime::IInspectable {
    fn from(value: &XamlMarkupHelper) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for XamlMarkupHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a XamlMarkupHelper
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for XamlMarkupHelper {}
unsafe impl ::std::marker::Sync for XamlMarkupHelper {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct XamlReader(::windows::runtime::IInspectable);
impl XamlReader {
    pub fn Load<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        xaml: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IXamlReaderStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                xaml.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    pub fn LoadWithInitialTemplateValidation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        xaml: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IXamlReaderStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                xaml.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    pub fn IXamlReaderStatics<
        R,
        F: FnOnce(&IXamlReaderStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<XamlReader, IXamlReaderStatics> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for XamlReader {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Markup.XamlReader;{54ce54c8-38c6-50d9-ac98-4b03eddbde9f})",
    );
}
unsafe impl ::windows::runtime::Interface for XamlReader {
    type Vtable = IXamlReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1422808264,
        14534,
        20697,
        [172, 152, 75, 3, 237, 219, 222, 159],
    );
}
impl ::windows::runtime::RuntimeName for XamlReader {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.XamlReader";
}
impl ::std::convert::From<XamlReader> for ::windows::runtime::IUnknown {
    fn from(value: XamlReader) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&XamlReader> for ::windows::runtime::IUnknown {
    fn from(value: &XamlReader) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for XamlReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &XamlReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<XamlReader> for ::windows::runtime::IInspectable {
    fn from(value: XamlReader) -> Self {
        value.0
    }
}
impl ::std::convert::From<&XamlReader> for ::windows::runtime::IInspectable {
    fn from(value: &XamlReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for XamlReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a XamlReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for XamlReader {}
unsafe impl ::std::marker::Sync for XamlReader {}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
pub struct XmlnsDefinition {
    pub XmlNamespace: ::windows::runtime::HSTRING,
    pub Namespace: ::windows::runtime::HSTRING,
}
impl XmlnsDefinition {}
impl ::std::default::Default for XmlnsDefinition {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for XmlnsDefinition {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("XmlnsDefinition")
            .field("XmlNamespace", &self.XmlNamespace)
            .field("Namespace", &self.Namespace)
            .finish()
    }
}
impl ::std::cmp::PartialEq for XmlnsDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.XmlNamespace == other.XmlNamespace && self.Namespace == other.Namespace
    }
}
impl ::std::cmp::Eq for XmlnsDefinition {}
unsafe impl ::windows::runtime::Abi for XmlnsDefinition {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for XmlnsDefinition {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"struct(Microsoft.UI.Xaml.Markup.XmlnsDefinition;string;string)",
    );
}
