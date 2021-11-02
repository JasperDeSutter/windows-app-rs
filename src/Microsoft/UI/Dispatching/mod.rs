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
pub struct DispatcherQueue(::windows::runtime::IInspectable);
impl DispatcherQueue {
    pub fn CreateTimer(&self) -> ::windows::runtime::Result<DispatcherQueueTimer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<DispatcherQueueTimer>(result__)
        }
    }
    pub fn TryEnqueue<'a, Param0: ::windows::runtime::IntoParam<'a, DispatcherQueueHandler>>(
        &self,
        callback: Param0,
    ) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn TryEnqueueWithPriority<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, DispatcherQueueHandler>,
    >(
        &self,
        priority: DispatcherQueuePriority,
        callback: Param1,
    ) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                priority,
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn ShutdownStarting<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::TypedEventHandler<
                DispatcherQueue,
                DispatcherQueueShutdownStartingEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::super::Windows::Foundation::EventRegistrationToken>
    {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Windows::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveShutdownStarting<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn ShutdownCompleted<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::TypedEventHandler<
                DispatcherQueue,
                ::windows::runtime::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::super::Windows::Foundation::EventRegistrationToken>
    {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Windows::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveShutdownCompleted<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn HasThreadAccess(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IDispatcherQueue2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn GetForCurrentThread() -> ::windows::runtime::Result<DispatcherQueue> {
        Self::IDispatcherQueueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<DispatcherQueue>(result__)
        })
    }
    pub fn IDispatcherQueueStatics<
        R,
        F: FnOnce(&IDispatcherQueueStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            DispatcherQueue,
            IDispatcherQueueStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DispatcherQueue {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Dispatching.DispatcherQueue;{f6ebf8fa-be1c-5bf6-a467-73da28738ae8})",
    );
}
unsafe impl ::windows::runtime::Interface for DispatcherQueue {
    type Vtable = IDispatcherQueue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4142659834,
        48668,
        23542,
        [164, 103, 115, 218, 40, 115, 138, 232],
    );
}
impl ::windows::runtime::RuntimeName for DispatcherQueue {
    const NAME: &'static str = "Microsoft.UI.Dispatching.DispatcherQueue";
}
impl ::std::convert::From<DispatcherQueue> for ::windows::runtime::IUnknown {
    fn from(value: DispatcherQueue) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DispatcherQueue> for ::windows::runtime::IUnknown {
    fn from(value: &DispatcherQueue) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DispatcherQueue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &DispatcherQueue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<DispatcherQueue> for ::windows::runtime::IInspectable {
    fn from(value: DispatcherQueue) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DispatcherQueue> for ::windows::runtime::IInspectable {
    fn from(value: &DispatcherQueue) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DispatcherQueue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a DispatcherQueue
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for DispatcherQueue {}
unsafe impl ::std::marker::Sync for DispatcherQueue {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct DispatcherQueueController(::windows::runtime::IInspectable);
impl DispatcherQueueController {
    pub fn DispatcherQueue(&self) -> ::windows::runtime::Result<DispatcherQueue> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<DispatcherQueue>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn ShutdownQueueAsync(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Windows::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn CreateOnDedicatedThread() -> ::windows::runtime::Result<DispatcherQueueController> {
        Self::IDispatcherQueueControllerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<DispatcherQueueController>(result__)
        })
    }
    pub fn CreateOnCurrentThread() -> ::windows::runtime::Result<DispatcherQueueController> {
        Self::IDispatcherQueueControllerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<DispatcherQueueController>(result__)
        })
    }
    pub fn IDispatcherQueueControllerStatics<
        R,
        F: FnOnce(&IDispatcherQueueControllerStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            DispatcherQueueController,
            IDispatcherQueueControllerStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DispatcherQueueController {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Dispatching.DispatcherQueueController;{bce8178d-2183-584c-9e5b-f9366f6ae484})" ) ;
}
unsafe impl ::windows::runtime::Interface for DispatcherQueueController {
    type Vtable = IDispatcherQueueController_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3169326989,
        8579,
        22604,
        [158, 91, 249, 54, 111, 106, 228, 132],
    );
}
impl ::windows::runtime::RuntimeName for DispatcherQueueController {
    const NAME: &'static str = "Microsoft.UI.Dispatching.DispatcherQueueController";
}
impl ::std::convert::From<DispatcherQueueController> for ::windows::runtime::IUnknown {
    fn from(value: DispatcherQueueController) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DispatcherQueueController> for ::windows::runtime::IUnknown {
    fn from(value: &DispatcherQueueController) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for DispatcherQueueController
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &DispatcherQueueController
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<DispatcherQueueController> for ::windows::runtime::IInspectable {
    fn from(value: DispatcherQueueController) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DispatcherQueueController> for ::windows::runtime::IInspectable {
    fn from(value: &DispatcherQueueController) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for DispatcherQueueController
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a DispatcherQueueController
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for DispatcherQueueController {}
unsafe impl ::std::marker::Sync for DispatcherQueueController {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct DispatcherQueueHandler(::windows::runtime::IUnknown);
impl DispatcherQueueHandler {
    pub fn new<F: FnMut() -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = DispatcherQueueHandler_box::<F> {
            vtable: &DispatcherQueueHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).3)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DispatcherQueueHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"delegate({2e0872a9-4e29-5f14-b688-fb96d5f9d5f8})",
    );
}
unsafe impl ::windows::runtime::Interface for DispatcherQueueHandler {
    type Vtable = DispatcherQueueHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        772305577,
        20009,
        24340,
        [182, 136, 251, 150, 213, 249, 213, 248],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct DispatcherQueueHandler_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct DispatcherQueueHandler_box<F: FnMut() -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const DispatcherQueueHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut() -> ::windows::runtime::Result<()> + 'static> DispatcherQueueHandler_box<F> {
    const VTABLE: DispatcherQueueHandler_abi = DispatcherQueueHandler_abi(
        Self::QueryInterface,
        Self::AddRef,
        Self::Release,
        Self::Invoke,
    );
    unsafe extern "system" fn QueryInterface(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<DispatcherQueueHandler as ::windows::runtime::Interface>::IID
            || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID
            || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)().into()
    }
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DispatcherQueuePriority(pub i32);
impl DispatcherQueuePriority {
    pub const Low: DispatcherQueuePriority = DispatcherQueuePriority(-10i32);
    pub const Normal: DispatcherQueuePriority = DispatcherQueuePriority(0i32);
    pub const High: DispatcherQueuePriority = DispatcherQueuePriority(10i32);
}
impl ::std::convert::From<i32> for DispatcherQueuePriority {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DispatcherQueuePriority {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DispatcherQueuePriority {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Dispatching.DispatcherQueuePriority;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct DispatcherQueueShutdownStartingEventArgs(::windows::runtime::IInspectable);
impl DispatcherQueueShutdownStartingEventArgs {
    #[cfg(feature = "undation")]
    pub fn GetDeferral(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Windows::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DispatcherQueueShutdownStartingEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Dispatching.DispatcherQueueShutdownStartingEventArgs;{32519be5-072b-5660-a70e-8835c9b8157d})" ) ;
}
unsafe impl ::windows::runtime::Interface for DispatcherQueueShutdownStartingEventArgs {
    type Vtable = IDispatcherQueueShutdownStartingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        844209125,
        1835,
        22112,
        [167, 14, 136, 53, 201, 184, 21, 125],
    );
}
impl ::windows::runtime::RuntimeName for DispatcherQueueShutdownStartingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Dispatching.DispatcherQueueShutdownStartingEventArgs";
}
impl ::std::convert::From<DispatcherQueueShutdownStartingEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: DispatcherQueueShutdownStartingEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DispatcherQueueShutdownStartingEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &DispatcherQueueShutdownStartingEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for DispatcherQueueShutdownStartingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &DispatcherQueueShutdownStartingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<DispatcherQueueShutdownStartingEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: DispatcherQueueShutdownStartingEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DispatcherQueueShutdownStartingEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &DispatcherQueueShutdownStartingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for DispatcherQueueShutdownStartingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a DispatcherQueueShutdownStartingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for DispatcherQueueShutdownStartingEventArgs {}
unsafe impl ::std::marker::Sync for DispatcherQueueShutdownStartingEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct DispatcherQueueTimer(::windows::runtime::IInspectable);
impl DispatcherQueueTimer {
    #[cfg(feature = "undation")]
    pub fn Interval(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Windows::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Windows::Foundation::TimeSpan =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn SetInterval<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::Windows::Foundation::TimeSpan>,
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
    pub fn IsRunning(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn IsRepeating(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn SetIsRepeating(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn Tick<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::TypedEventHandler<
                DispatcherQueueTimer,
                ::windows::runtime::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::super::Windows::Foundation::EventRegistrationToken>
    {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Windows::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveTick<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DispatcherQueueTimer {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Dispatching.DispatcherQueueTimer;{ad4d63fd-88fe-541f-ac11-bf2dc1ed2ce5})",
    );
}
unsafe impl ::windows::runtime::Interface for DispatcherQueueTimer {
    type Vtable = IDispatcherQueueTimer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2907530237,
        35070,
        21535,
        [172, 17, 191, 45, 193, 237, 44, 229],
    );
}
impl ::windows::runtime::RuntimeName for DispatcherQueueTimer {
    const NAME: &'static str = "Microsoft.UI.Dispatching.DispatcherQueueTimer";
}
impl ::std::convert::From<DispatcherQueueTimer> for ::windows::runtime::IUnknown {
    fn from(value: DispatcherQueueTimer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DispatcherQueueTimer> for ::windows::runtime::IUnknown {
    fn from(value: &DispatcherQueueTimer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DispatcherQueueTimer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &DispatcherQueueTimer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<DispatcherQueueTimer> for ::windows::runtime::IInspectable {
    fn from(value: DispatcherQueueTimer) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DispatcherQueueTimer> for ::windows::runtime::IInspectable {
    fn from(value: &DispatcherQueueTimer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for DispatcherQueueTimer
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a DispatcherQueueTimer
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for DispatcherQueueTimer {}
unsafe impl ::std::marker::Sync for DispatcherQueueTimer {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IDispatcherQueue(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDispatcherQueue {
    type Vtable = IDispatcherQueue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4142659834,
        48668,
        23542,
        [164, 103, 115, 218, 40, 115, 138, 232],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueue_abi(
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
        callback: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        priority: DispatcherQueuePriority,
        callback: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::Windows::Foundation::EventRegistrationToken,
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
pub struct IDispatcherQueue2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDispatcherQueue2 {
    type Vtable = IDispatcherQueue2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        217352017,
        61868,
        22968,
        [186, 82, 108, 231, 161, 68, 77, 111],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueue2_abi(
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
pub struct IDispatcherQueueController(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDispatcherQueueController {
    type Vtable = IDispatcherQueueController_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3169326989,
        8579,
        22604,
        [158, 91, 249, 54, 111, 106, 228, 132],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueController_abi(
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
pub struct IDispatcherQueueControllerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDispatcherQueueControllerStatics {
    type Vtable = IDispatcherQueueControllerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4052574533,
        29227,
        22845,
        [188, 242, 166, 30, 113, 63, 0, 55],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueControllerStatics_abi(
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
pub struct IDispatcherQueueShutdownStartingEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDispatcherQueueShutdownStartingEventArgs {
    type Vtable = IDispatcherQueueShutdownStartingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        844209125,
        1835,
        22112,
        [167, 14, 136, 53, 201, 184, 21, 125],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueShutdownStartingEventArgs_abi(
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
pub struct IDispatcherQueueStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDispatcherQueueStatics {
    type Vtable = IDispatcherQueueStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3442705130,
        42069,
        20772,
        [182, 58, 202, 64, 211, 76, 162, 60],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueStatics_abi(
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
#[doc(hidden)]
pub struct IDispatcherQueueTimer(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDispatcherQueueTimer {
    type Vtable = IDispatcherQueueTimer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2907530237,
        35070,
        21535,
        [172, 17, 191, 45, 193, 237, 44, 229],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueTimer_abi(
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
        result__: *mut super::super::super::Windows::Foundation::TimeSpan,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::super::super::Windows::Foundation::TimeSpan,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
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
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
);
