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
pub struct AppWindow(::windows::runtime::IInspectable);
impl AppWindow {
    pub fn Id(&self) -> ::windows::runtime::Result<super::WindowId> {
        let this = self;
        unsafe {
            let mut result__: super::WindowId = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::WindowId>(result__)
        }
    }
    pub fn IsShownInSwitchers(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn SetIsShownInSwitchers(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsVisible(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn OwnerWindowId(&self) -> ::windows::runtime::Result<super::WindowId> {
        let this = self;
        unsafe {
            let mut result__: super::WindowId = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::WindowId>(result__)
        }
    }
    #[cfg(feature = "aphics")]
    pub fn Position(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Windows::Graphics::PointInt32> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Windows::Graphics::PointInt32 =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Graphics::PointInt32>(result__)
        }
    }
    pub fn Presenter(&self) -> ::windows::runtime::Result<AppWindowPresenter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AppWindowPresenter>(result__)
        }
    }
    #[cfg(feature = "aphics")]
    pub fn Size(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Windows::Graphics::SizeInt32> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Windows::Graphics::SizeInt32 =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Graphics::SizeInt32>(result__)
        }
    }
    pub fn Title(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetTitle<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn TitleBar(&self) -> ::windows::runtime::Result<AppWindowTitleBar> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AppWindowTitleBar>(result__)
        }
    }
    pub fn Destroy(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Hide(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "aphics")]
    pub fn Move<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::Windows::Graphics::PointInt32>,
    >(
        &self,
        position: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                position.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "aphics")]
    pub fn MoveAndResize<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::Windows::Graphics::RectInt32>,
    >(
        &self,
        rect: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                rect.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "aphics")]
    pub fn MoveAndResizeRelativeToDisplayArea<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::Windows::Graphics::RectInt32>,
        Param1: ::windows::runtime::IntoParam<'a, DisplayArea>,
    >(
        &self,
        rect: Param0,
        displayarea: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).21)(
                ::std::mem::transmute_copy(this),
                rect.into_param().abi(),
                displayarea.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "aphics")]
    pub fn Resize<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::Windows::Graphics::SizeInt32>,
    >(
        &self,
        size: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                size.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn SetIcon<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        iconpath: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                iconpath.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn SetIconWithIconId<'a, Param0: ::windows::runtime::IntoParam<'a, super::IconId>>(
        &self,
        iconid: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                iconid.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn SetPresenter<'a, Param0: ::windows::runtime::IntoParam<'a, AppWindowPresenter>>(
        &self,
        appwindowpresenter: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                appwindowpresenter.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn SetPresenterByKind(
        &self,
        appwindowpresenterkind: AppWindowPresenterKind,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                appwindowpresenterkind,
            )
            .ok()
        }
    }
    pub fn Show(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn ShowWithActivation(&self, activatewindow: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).28)(
                ::std::mem::transmute_copy(this),
                activatewindow,
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn Changed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::TypedEventHandler<
                AppWindow,
                AppWindowChangedEventArgs,
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
            (::windows::runtime::Interface::vtable(this).29)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveChanged<
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
            (::windows::runtime::Interface::vtable(this).30)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn Closing<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::TypedEventHandler<
                AppWindow,
                AppWindowClosingEventArgs,
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
            (::windows::runtime::Interface::vtable(this).31)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveClosing<
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
            (::windows::runtime::Interface::vtable(this).32)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn Destroying<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::TypedEventHandler<
                AppWindow,
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
            (::windows::runtime::Interface::vtable(this).33)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveDestroying<
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
            (::windows::runtime::Interface::vtable(this).34)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Create() -> ::windows::runtime::Result<AppWindow> {
        Self::IAppWindowStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AppWindow>(result__)
        })
    }
    pub fn CreateWithPresenter<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, AppWindowPresenter>,
    >(
        appwindowpresenter: Param0,
    ) -> ::windows::runtime::Result<AppWindow> {
        Self::IAppWindowStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                appwindowpresenter.into_param().abi(),
                &mut result__,
            )
            .from_abi::<AppWindow>(result__)
        })
    }
    pub fn CreateWithPresenterAndOwner<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, AppWindowPresenter>,
        Param1: ::windows::runtime::IntoParam<'a, super::WindowId>,
    >(
        appwindowpresenter: Param0,
        ownerwindowid: Param1,
    ) -> ::windows::runtime::Result<AppWindow> {
        Self::IAppWindowStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                appwindowpresenter.into_param().abi(),
                ownerwindowid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<AppWindow>(result__)
        })
    }
    pub fn GetFromWindowId<'a, Param0: ::windows::runtime::IntoParam<'a, super::WindowId>>(
        windowid: Param0,
    ) -> ::windows::runtime::Result<AppWindow> {
        Self::IAppWindowStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                windowid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<AppWindow>(result__)
        })
    }
    pub fn IAppWindowStatics<R, F: FnOnce(&IAppWindowStatics) -> ::windows::runtime::Result<R>>(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AppWindow, IAppWindowStatics> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppWindow {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Windowing.AppWindow;{cfa788b3-643b-5c5e-ad4e-321d48a82acd})",
    );
}
unsafe impl ::windows::runtime::Interface for AppWindow {
    type Vtable = IAppWindow_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3483863219,
        25659,
        23646,
        [173, 78, 50, 29, 72, 168, 42, 205],
    );
}
impl ::windows::runtime::RuntimeName for AppWindow {
    const NAME: &'static str = "Microsoft.UI.Windowing.AppWindow";
}
impl ::std::convert::From<AppWindow> for ::windows::runtime::IUnknown {
    fn from(value: AppWindow) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AppWindow> for ::windows::runtime::IUnknown {
    fn from(value: &AppWindow) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppWindow {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AppWindow {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<AppWindow> for ::windows::runtime::IInspectable {
    fn from(value: AppWindow) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AppWindow> for ::windows::runtime::IInspectable {
    fn from(value: &AppWindow) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppWindow {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppWindow {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AppWindow {}
unsafe impl ::std::marker::Sync for AppWindow {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AppWindowChangedEventArgs(::windows::runtime::IInspectable);
impl AppWindowChangedEventArgs {
    pub fn DidPositionChange(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn DidPresenterChange(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn DidSizeChange(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn DidVisibilityChange(&self) -> ::windows::runtime::Result<bool> {
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
}
unsafe impl ::windows::runtime::RuntimeType for AppWindowChangedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Windowing.AppWindowChangedEventArgs;{2182bc5d-fdac-5c3e-bf37-7d8d684e9d1d})" ) ;
}
unsafe impl ::windows::runtime::Interface for AppWindowChangedEventArgs {
    type Vtable = IAppWindowChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        562216029,
        64940,
        23614,
        [191, 55, 125, 141, 104, 78, 157, 29],
    );
}
impl ::windows::runtime::RuntimeName for AppWindowChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Windowing.AppWindowChangedEventArgs";
}
impl ::std::convert::From<AppWindowChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: AppWindowChangedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AppWindowChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &AppWindowChangedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for AppWindowChangedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &AppWindowChangedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<AppWindowChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: AppWindowChangedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AppWindowChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &AppWindowChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for AppWindowChangedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AppWindowChangedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AppWindowChangedEventArgs {}
unsafe impl ::std::marker::Sync for AppWindowChangedEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AppWindowClosingEventArgs(::windows::runtime::IInspectable);
impl AppWindowClosingEventArgs {
    pub fn Cancel(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn SetCancel(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppWindowClosingEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Windowing.AppWindowClosingEventArgs;{0e09d90b-2261-590b-9ad1-8504991d8754})" ) ;
}
unsafe impl ::windows::runtime::Interface for AppWindowClosingEventArgs {
    type Vtable = IAppWindowClosingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        235526411,
        8801,
        22795,
        [154, 209, 133, 4, 153, 29, 135, 84],
    );
}
impl ::windows::runtime::RuntimeName for AppWindowClosingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Windowing.AppWindowClosingEventArgs";
}
impl ::std::convert::From<AppWindowClosingEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: AppWindowClosingEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AppWindowClosingEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &AppWindowClosingEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for AppWindowClosingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &AppWindowClosingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<AppWindowClosingEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: AppWindowClosingEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AppWindowClosingEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &AppWindowClosingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for AppWindowClosingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AppWindowClosingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AppWindowClosingEventArgs {}
unsafe impl ::std::marker::Sync for AppWindowClosingEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AppWindowPresenter(::windows::runtime::IInspectable);
impl AppWindowPresenter {
    pub fn Kind(&self) -> ::windows::runtime::Result<AppWindowPresenterKind> {
        let this = self;
        unsafe {
            let mut result__: AppWindowPresenterKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AppWindowPresenterKind>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppWindowPresenter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Windowing.AppWindowPresenter;{bc3042c2-c6c6-5632-8989-ff0ec6d3b40d})",
    );
}
unsafe impl ::windows::runtime::Interface for AppWindowPresenter {
    type Vtable = IAppWindowPresenter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3157279426,
        50886,
        22066,
        [137, 137, 255, 14, 198, 211, 180, 13],
    );
}
impl ::windows::runtime::RuntimeName for AppWindowPresenter {
    const NAME: &'static str = "Microsoft.UI.Windowing.AppWindowPresenter";
}
impl ::std::convert::From<AppWindowPresenter> for ::windows::runtime::IUnknown {
    fn from(value: AppWindowPresenter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AppWindowPresenter> for ::windows::runtime::IUnknown {
    fn from(value: &AppWindowPresenter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppWindowPresenter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AppWindowPresenter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<AppWindowPresenter> for ::windows::runtime::IInspectable {
    fn from(value: AppWindowPresenter) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AppWindowPresenter> for ::windows::runtime::IInspectable {
    fn from(value: &AppWindowPresenter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for AppWindowPresenter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AppWindowPresenter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AppWindowPresenter {}
unsafe impl ::std::marker::Sync for AppWindowPresenter {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct AppWindowPresenterKind(pub i32);
impl AppWindowPresenterKind {
    pub const Default: AppWindowPresenterKind = AppWindowPresenterKind(0i32);
    pub const CompactOverlay: AppWindowPresenterKind = AppWindowPresenterKind(1i32);
    pub const FullScreen: AppWindowPresenterKind = AppWindowPresenterKind(2i32);
    pub const Overlapped: AppWindowPresenterKind = AppWindowPresenterKind(3i32);
}
impl ::std::convert::From<i32> for AppWindowPresenterKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AppWindowPresenterKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AppWindowPresenterKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Windowing.AppWindowPresenterKind;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AppWindowTitleBar(::windows::runtime::IInspectable);
impl AppWindowTitleBar {
    #[cfg(all(feature = "undation", feature = ""))]
    pub fn BackgroundColor(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::Windows::Foundation::IReference<
            super::super::super::Windows::UI::Color,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::IReference<
                super::super::super::Windows::UI::Color,
            >>(result__)
        }
    }
    #[cfg(all(feature = "undation", feature = ""))]
    pub fn SetBackgroundColor<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::IReference<
                super::super::super::Windows::UI::Color,
            >,
        >,
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
    #[cfg(all(feature = "undation", feature = ""))]
    pub fn ButtonBackgroundColor(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::Windows::Foundation::IReference<
            super::super::super::Windows::UI::Color,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::IReference<
                super::super::super::Windows::UI::Color,
            >>(result__)
        }
    }
    #[cfg(all(feature = "undation", feature = ""))]
    pub fn SetButtonBackgroundColor<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::IReference<
                super::super::super::Windows::UI::Color,
            >,
        >,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "undation", feature = ""))]
    pub fn ButtonForegroundColor(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::Windows::Foundation::IReference<
            super::super::super::Windows::UI::Color,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::IReference<
                super::super::super::Windows::UI::Color,
            >>(result__)
        }
    }
    #[cfg(all(feature = "undation", feature = ""))]
    pub fn SetButtonForegroundColor<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::IReference<
                super::super::super::Windows::UI::Color,
            >,
        >,
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
    #[cfg(all(feature = "undation", feature = ""))]
    pub fn ButtonHoverBackgroundColor(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::Windows::Foundation::IReference<
            super::super::super::Windows::UI::Color,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::IReference<
                super::super::super::Windows::UI::Color,
            >>(result__)
        }
    }
    #[cfg(all(feature = "undation", feature = ""))]
    pub fn SetButtonHoverBackgroundColor<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::IReference<
                super::super::super::Windows::UI::Color,
            >,
        >,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "undation", feature = ""))]
    pub fn ButtonHoverForegroundColor(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::Windows::Foundation::IReference<
            super::super::super::Windows::UI::Color,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::IReference<
                super::super::super::Windows::UI::Color,
            >>(result__)
        }
    }
    #[cfg(all(feature = "undation", feature = ""))]
    pub fn SetButtonHoverForegroundColor<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::IReference<
                super::super::super::Windows::UI::Color,
            >,
        >,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "undation", feature = ""))]
    pub fn ButtonInactiveBackgroundColor(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::Windows::Foundation::IReference<
            super::super::super::Windows::UI::Color,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::IReference<
                super::super::super::Windows::UI::Color,
            >>(result__)
        }
    }
    #[cfg(all(feature = "undation", feature = ""))]
    pub fn SetButtonInactiveBackgroundColor<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::IReference<
                super::super::super::Windows::UI::Color,
            >,
        >,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "undation", feature = ""))]
    pub fn ButtonInactiveForegroundColor(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::Windows::Foundation::IReference<
            super::super::super::Windows::UI::Color,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::IReference<
                super::super::super::Windows::UI::Color,
            >>(result__)
        }
    }
    #[cfg(all(feature = "undation", feature = ""))]
    pub fn SetButtonInactiveForegroundColor<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::IReference<
                super::super::super::Windows::UI::Color,
            >,
        >,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "undation", feature = ""))]
    pub fn ButtonPressedBackgroundColor(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::Windows::Foundation::IReference<
            super::super::super::Windows::UI::Color,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::IReference<
                super::super::super::Windows::UI::Color,
            >>(result__)
        }
    }
    #[cfg(all(feature = "undation", feature = ""))]
    pub fn SetButtonPressedBackgroundColor<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::IReference<
                super::super::super::Windows::UI::Color,
            >,
        >,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).21)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "undation", feature = ""))]
    pub fn ButtonPressedForegroundColor(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::Windows::Foundation::IReference<
            super::super::super::Windows::UI::Color,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::IReference<
                super::super::super::Windows::UI::Color,
            >>(result__)
        }
    }
    #[cfg(all(feature = "undation", feature = ""))]
    pub fn SetButtonPressedForegroundColor<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::IReference<
                super::super::super::Windows::UI::Color,
            >,
        >,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ExtendsContentIntoTitleBar(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExtendsContentIntoTitleBar(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(all(feature = "undation", feature = ""))]
    pub fn ForegroundColor(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::Windows::Foundation::IReference<
            super::super::super::Windows::UI::Color,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::IReference<
                super::super::super::Windows::UI::Color,
            >>(result__)
        }
    }
    #[cfg(all(feature = "undation", feature = ""))]
    pub fn SetForegroundColor<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::IReference<
                super::super::super::Windows::UI::Color,
            >,
        >,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).27)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Height(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn IconShowOptions(&self) -> ::windows::runtime::Result<IconShowOptions> {
        let this = self;
        unsafe {
            let mut result__: IconShowOptions = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IconShowOptions>(result__)
        }
    }
    pub fn SetIconShowOptions(&self, value: IconShowOptions) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).30)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(all(feature = "undation", feature = ""))]
    pub fn InactiveBackgroundColor(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::Windows::Foundation::IReference<
            super::super::super::Windows::UI::Color,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::IReference<
                super::super::super::Windows::UI::Color,
            >>(result__)
        }
    }
    #[cfg(all(feature = "undation", feature = ""))]
    pub fn SetInactiveBackgroundColor<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::IReference<
                super::super::super::Windows::UI::Color,
            >,
        >,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).32)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "undation", feature = ""))]
    pub fn InactiveForegroundColor(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::Windows::Foundation::IReference<
            super::super::super::Windows::UI::Color,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::IReference<
                super::super::super::Windows::UI::Color,
            >>(result__)
        }
    }
    #[cfg(all(feature = "undation", feature = ""))]
    pub fn SetInactiveForegroundColor<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::IReference<
                super::super::super::Windows::UI::Color,
            >,
        >,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).34)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn LeftInset(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn RightInset(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn ResetToDefault(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).37)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "aphics")]
    pub fn SetDragRectangles(
        &self,
        value : & [ < super::super::super::Windows::Graphics:: RectInt32 as :: windows :: runtime :: Abi > :: DefaultType ],
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).38)(
                ::std::mem::transmute_copy(this),
                value.len() as u32,
                ::std::mem::transmute(value.as_ptr()),
            )
            .ok()
        }
    }
    pub fn IsCustomizationSupported() -> ::windows::runtime::Result<bool> {
        Self::IAppWindowTitleBarStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn IAppWindowTitleBarStatics<
        R,
        F: FnOnce(&IAppWindowTitleBarStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            AppWindowTitleBar,
            IAppWindowTitleBarStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppWindowTitleBar {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Windowing.AppWindowTitleBar;{5574efa2-c91c-5700-a363-539c71a7aaf4})",
    );
}
unsafe impl ::windows::runtime::Interface for AppWindowTitleBar {
    type Vtable = IAppWindowTitleBar_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1433726882,
        51484,
        22272,
        [163, 99, 83, 156, 113, 167, 170, 244],
    );
}
impl ::windows::runtime::RuntimeName for AppWindowTitleBar {
    const NAME: &'static str = "Microsoft.UI.Windowing.AppWindowTitleBar";
}
impl ::std::convert::From<AppWindowTitleBar> for ::windows::runtime::IUnknown {
    fn from(value: AppWindowTitleBar) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AppWindowTitleBar> for ::windows::runtime::IUnknown {
    fn from(value: &AppWindowTitleBar) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppWindowTitleBar {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AppWindowTitleBar {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<AppWindowTitleBar> for ::windows::runtime::IInspectable {
    fn from(value: AppWindowTitleBar) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AppWindowTitleBar> for ::windows::runtime::IInspectable {
    fn from(value: &AppWindowTitleBar) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppWindowTitleBar {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AppWindowTitleBar
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AppWindowTitleBar {}
unsafe impl ::std::marker::Sync for AppWindowTitleBar {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CompactOverlayPresenter(::windows::runtime::IInspectable);
impl CompactOverlayPresenter {
    pub fn InitialSize(&self) -> ::windows::runtime::Result<CompactOverlaySize> {
        let this = self;
        unsafe {
            let mut result__: CompactOverlaySize = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CompactOverlaySize>(result__)
        }
    }
    pub fn SetInitialSize(&self, value: CompactOverlaySize) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn Create() -> ::windows::runtime::Result<CompactOverlayPresenter> {
        Self::ICompactOverlayPresenterStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CompactOverlayPresenter>(result__)
        })
    }
    pub fn Kind(&self) -> ::windows::runtime::Result<AppWindowPresenterKind> {
        let this = &::windows::runtime::Interface::cast::<IAppWindowPresenter>(self)?;
        unsafe {
            let mut result__: AppWindowPresenterKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AppWindowPresenterKind>(result__)
        }
    }
    pub fn ICompactOverlayPresenterStatics<
        R,
        F: FnOnce(&ICompactOverlayPresenterStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            CompactOverlayPresenter,
            ICompactOverlayPresenterStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CompactOverlayPresenter {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Windowing.CompactOverlayPresenter;{efeb0812-6fc7-5b7d-bd92-cc8f9a6454c9})" ) ;
}
unsafe impl ::windows::runtime::Interface for CompactOverlayPresenter {
    type Vtable = ICompactOverlayPresenter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4025157650,
        28615,
        23421,
        [189, 146, 204, 143, 154, 100, 84, 201],
    );
}
impl ::windows::runtime::RuntimeName for CompactOverlayPresenter {
    const NAME: &'static str = "Microsoft.UI.Windowing.CompactOverlayPresenter";
}
impl ::std::convert::From<CompactOverlayPresenter> for ::windows::runtime::IUnknown {
    fn from(value: CompactOverlayPresenter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CompactOverlayPresenter> for ::windows::runtime::IUnknown {
    fn from(value: &CompactOverlayPresenter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CompactOverlayPresenter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CompactOverlayPresenter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CompactOverlayPresenter> for ::windows::runtime::IInspectable {
    fn from(value: CompactOverlayPresenter) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CompactOverlayPresenter> for ::windows::runtime::IInspectable {
    fn from(value: &CompactOverlayPresenter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CompactOverlayPresenter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CompactOverlayPresenter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<CompactOverlayPresenter> for AppWindowPresenter {
    fn from(value: CompactOverlayPresenter) -> Self {
        ::std::convert::Into::<AppWindowPresenter>::into(&value)
    }
}
impl ::std::convert::From<&CompactOverlayPresenter> for AppWindowPresenter {
    fn from(value: &CompactOverlayPresenter) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, AppWindowPresenter> for CompactOverlayPresenter {
    fn into_param(self) -> ::windows::runtime::Param<'a, AppWindowPresenter> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<AppWindowPresenter>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, AppWindowPresenter> for &CompactOverlayPresenter {
    fn into_param(self) -> ::windows::runtime::Param<'a, AppWindowPresenter> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<AppWindowPresenter>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for CompactOverlayPresenter {}
unsafe impl ::std::marker::Sync for CompactOverlayPresenter {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CompactOverlaySize(pub i32);
impl CompactOverlaySize {
    pub const Small: CompactOverlaySize = CompactOverlaySize(0i32);
    pub const Medium: CompactOverlaySize = CompactOverlaySize(1i32);
    pub const Large: CompactOverlaySize = CompactOverlaySize(2i32);
}
impl ::std::convert::From<i32> for CompactOverlaySize {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CompactOverlaySize {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CompactOverlaySize {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Windowing.CompactOverlaySize;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct DisplayArea(::windows::runtime::IInspectable);
impl DisplayArea {
    pub fn DisplayId(&self) -> ::windows::runtime::Result<super::DisplayId> {
        let this = self;
        unsafe {
            let mut result__: super::DisplayId = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DisplayId>(result__)
        }
    }
    pub fn IsPrimary(&self) -> ::windows::runtime::Result<bool> {
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
    #[cfg(feature = "aphics")]
    pub fn OuterBounds(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Windows::Graphics::RectInt32> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Windows::Graphics::RectInt32 =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Graphics::RectInt32>(result__)
        }
    }
    #[cfg(feature = "aphics")]
    pub fn WorkArea(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Windows::Graphics::RectInt32> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Windows::Graphics::RectInt32 =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Graphics::RectInt32>(result__)
        }
    }
    pub fn Primary() -> ::windows::runtime::Result<DisplayArea> {
        Self::IDisplayAreaStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<DisplayArea>(result__)
        })
    }
    pub fn CreateWatcher() -> ::windows::runtime::Result<DisplayAreaWatcher> {
        Self::IDisplayAreaStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<DisplayAreaWatcher>(result__)
        })
    }
    #[cfg(feature = "undation_Collections")]
    pub fn FindAll() -> ::windows::runtime::Result<
        super::super::super::Windows::Foundation::Collections::IVectorView<DisplayArea>,
    > {
        Self::IDisplayAreaStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .8 ) ( :: std :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < super::super::super::Windows::Foundation::Collections:: IVectorView :: < DisplayArea > > ( result__ )
        })
    }
    pub fn GetFromWindowId<'a, Param0: ::windows::runtime::IntoParam<'a, super::WindowId>>(
        windowid: Param0,
        displayareafallback: DisplayAreaFallback,
    ) -> ::windows::runtime::Result<DisplayArea> {
        Self::IDisplayAreaStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                windowid.into_param().abi(),
                displayareafallback,
                &mut result__,
            )
            .from_abi::<DisplayArea>(result__)
        })
    }
    #[cfg(feature = "aphics")]
    pub fn GetFromPoint<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::Windows::Graphics::PointInt32>,
    >(
        point: Param0,
        displayareafallback: DisplayAreaFallback,
    ) -> ::windows::runtime::Result<DisplayArea> {
        Self::IDisplayAreaStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                point.into_param().abi(),
                displayareafallback,
                &mut result__,
            )
            .from_abi::<DisplayArea>(result__)
        })
    }
    #[cfg(feature = "aphics")]
    pub fn GetFromRect<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::Windows::Graphics::RectInt32>,
    >(
        rect: Param0,
        displayareafallback: DisplayAreaFallback,
    ) -> ::windows::runtime::Result<DisplayArea> {
        Self::IDisplayAreaStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                rect.into_param().abi(),
                displayareafallback,
                &mut result__,
            )
            .from_abi::<DisplayArea>(result__)
        })
    }
    pub fn IDisplayAreaStatics<
        R,
        F: FnOnce(&IDisplayAreaStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<DisplayArea, IDisplayAreaStatics> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DisplayArea {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Windowing.DisplayArea;{5c7e0537-b621-5579-bcae-a84aa8746167})",
    );
}
unsafe impl ::windows::runtime::Interface for DisplayArea {
    type Vtable = IDisplayArea_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1551762743,
        46625,
        21881,
        [188, 174, 168, 74, 168, 116, 97, 103],
    );
}
impl ::windows::runtime::RuntimeName for DisplayArea {
    const NAME: &'static str = "Microsoft.UI.Windowing.DisplayArea";
}
impl ::std::convert::From<DisplayArea> for ::windows::runtime::IUnknown {
    fn from(value: DisplayArea) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DisplayArea> for ::windows::runtime::IUnknown {
    fn from(value: &DisplayArea) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DisplayArea {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &DisplayArea {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<DisplayArea> for ::windows::runtime::IInspectable {
    fn from(value: DisplayArea) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DisplayArea> for ::windows::runtime::IInspectable {
    fn from(value: &DisplayArea) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DisplayArea {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DisplayArea {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for DisplayArea {}
unsafe impl ::std::marker::Sync for DisplayArea {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DisplayAreaFallback(pub i32);
impl DisplayAreaFallback {
    pub const None: DisplayAreaFallback = DisplayAreaFallback(0i32);
    pub const Primary: DisplayAreaFallback = DisplayAreaFallback(1i32);
    pub const Nearest: DisplayAreaFallback = DisplayAreaFallback(2i32);
}
impl ::std::convert::From<i32> for DisplayAreaFallback {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DisplayAreaFallback {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DisplayAreaFallback {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Windowing.DisplayAreaFallback;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct DisplayAreaWatcher(::windows::runtime::IInspectable);
impl DisplayAreaWatcher {
    pub fn Status(&self) -> ::windows::runtime::Result<DisplayAreaWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__: DisplayAreaWatcherStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<DisplayAreaWatcherStatus>(result__)
        }
    }
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn Added<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::TypedEventHandler<
                DisplayAreaWatcher,
                DisplayArea,
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
    pub fn RemoveAdded<
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
    pub fn EnumerationCompleted<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::TypedEventHandler<
                DisplayAreaWatcher,
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
    pub fn RemoveEnumerationCompleted<
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
    #[cfg(feature = "undation")]
    pub fn Removed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::TypedEventHandler<
                DisplayAreaWatcher,
                DisplayArea,
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
    pub fn RemoveRemoved<
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
    #[cfg(feature = "undation")]
    pub fn Stopped<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::TypedEventHandler<
                DisplayAreaWatcher,
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
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveStopped<
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
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn Updated<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::TypedEventHandler<
                DisplayAreaWatcher,
                DisplayArea,
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
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveUpdated<
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
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DisplayAreaWatcher {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Windowing.DisplayAreaWatcher;{83f6562f-d3a0-548b-8e4f-a99be3d95c9c})",
    );
}
unsafe impl ::windows::runtime::Interface for DisplayAreaWatcher {
    type Vtable = IDisplayAreaWatcher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2213959215,
        54176,
        21643,
        [142, 79, 169, 155, 227, 217, 92, 156],
    );
}
impl ::windows::runtime::RuntimeName for DisplayAreaWatcher {
    const NAME: &'static str = "Microsoft.UI.Windowing.DisplayAreaWatcher";
}
impl ::std::convert::From<DisplayAreaWatcher> for ::windows::runtime::IUnknown {
    fn from(value: DisplayAreaWatcher) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DisplayAreaWatcher> for ::windows::runtime::IUnknown {
    fn from(value: &DisplayAreaWatcher) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DisplayAreaWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &DisplayAreaWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<DisplayAreaWatcher> for ::windows::runtime::IInspectable {
    fn from(value: DisplayAreaWatcher) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DisplayAreaWatcher> for ::windows::runtime::IInspectable {
    fn from(value: &DisplayAreaWatcher) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for DisplayAreaWatcher
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a DisplayAreaWatcher
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for DisplayAreaWatcher {}
unsafe impl ::std::marker::Sync for DisplayAreaWatcher {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DisplayAreaWatcherStatus(pub i32);
impl DisplayAreaWatcherStatus {
    pub const Created: DisplayAreaWatcherStatus = DisplayAreaWatcherStatus(0i32);
    pub const Started: DisplayAreaWatcherStatus = DisplayAreaWatcherStatus(1i32);
    pub const EnumerationCompleted: DisplayAreaWatcherStatus = DisplayAreaWatcherStatus(2i32);
    pub const Stopping: DisplayAreaWatcherStatus = DisplayAreaWatcherStatus(3i32);
    pub const Stopped: DisplayAreaWatcherStatus = DisplayAreaWatcherStatus(4i32);
    pub const Aborted: DisplayAreaWatcherStatus = DisplayAreaWatcherStatus(5i32);
}
impl ::std::convert::From<i32> for DisplayAreaWatcherStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DisplayAreaWatcherStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DisplayAreaWatcherStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Windowing.DisplayAreaWatcherStatus;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct FullScreenPresenter(::windows::runtime::IInspectable);
impl FullScreenPresenter {
    pub fn Create() -> ::windows::runtime::Result<FullScreenPresenter> {
        Self::IFullScreenPresenterStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<FullScreenPresenter>(result__)
        })
    }
    pub fn Kind(&self) -> ::windows::runtime::Result<AppWindowPresenterKind> {
        let this = &::windows::runtime::Interface::cast::<IAppWindowPresenter>(self)?;
        unsafe {
            let mut result__: AppWindowPresenterKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AppWindowPresenterKind>(result__)
        }
    }
    pub fn IFullScreenPresenterStatics<
        R,
        F: FnOnce(&IFullScreenPresenterStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            FullScreenPresenter,
            IFullScreenPresenterStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for FullScreenPresenter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Windowing.FullScreenPresenter;{fa9141fd-b8dd-5da1-8b2b-7cdadb76f593})",
    );
}
unsafe impl ::windows::runtime::Interface for FullScreenPresenter {
    type Vtable = IFullScreenPresenter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4203823613,
        47325,
        23969,
        [139, 43, 124, 218, 219, 118, 245, 147],
    );
}
impl ::windows::runtime::RuntimeName for FullScreenPresenter {
    const NAME: &'static str = "Microsoft.UI.Windowing.FullScreenPresenter";
}
impl ::std::convert::From<FullScreenPresenter> for ::windows::runtime::IUnknown {
    fn from(value: FullScreenPresenter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&FullScreenPresenter> for ::windows::runtime::IUnknown {
    fn from(value: &FullScreenPresenter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for FullScreenPresenter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &FullScreenPresenter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<FullScreenPresenter> for ::windows::runtime::IInspectable {
    fn from(value: FullScreenPresenter) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FullScreenPresenter> for ::windows::runtime::IInspectable {
    fn from(value: &FullScreenPresenter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for FullScreenPresenter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a FullScreenPresenter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<FullScreenPresenter> for AppWindowPresenter {
    fn from(value: FullScreenPresenter) -> Self {
        ::std::convert::Into::<AppWindowPresenter>::into(&value)
    }
}
impl ::std::convert::From<&FullScreenPresenter> for AppWindowPresenter {
    fn from(value: &FullScreenPresenter) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, AppWindowPresenter> for FullScreenPresenter {
    fn into_param(self) -> ::windows::runtime::Param<'a, AppWindowPresenter> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<AppWindowPresenter>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, AppWindowPresenter> for &FullScreenPresenter {
    fn into_param(self) -> ::windows::runtime::Param<'a, AppWindowPresenter> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<AppWindowPresenter>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for FullScreenPresenter {}
unsafe impl ::std::marker::Sync for FullScreenPresenter {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IAppWindow(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppWindow {
    type Vtable = IAppWindow_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3483863219,
        25659,
        23646,
        [173, 78, 50, 29, 72, 168, 42, 205],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindow_abi(
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
        result__: *mut super::WindowId,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::WindowId,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "aphics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::Windows::Graphics::PointInt32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "aphics"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "aphics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::Windows::Graphics::SizeInt32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "aphics"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "aphics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        position: super::super::super::Windows::Graphics::PointInt32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "aphics"))] usize,
    #[cfg(feature = "aphics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        rect: super::super::super::Windows::Graphics::RectInt32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "aphics"))] usize,
    #[cfg(feature = "aphics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        rect: super::super::super::Windows::Graphics::RectInt32,
        displayarea: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "aphics"))] usize,
    #[cfg(feature = "aphics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        size: super::super::super::Windows::Graphics::SizeInt32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "aphics"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iconpath: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iconid: super::IconId,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        appwindowpresenter: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        appwindowpresenterkind: AppWindowPresenterKind,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        activatewindow: bool,
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
pub struct IAppWindowChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppWindowChangedEventArgs {
    type Vtable = IAppWindowChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        562216029,
        64940,
        23614,
        [191, 55, 125, 141, 104, 78, 157, 29],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowChangedEventArgs_abi(
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
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IAppWindowClosingEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppWindowClosingEventArgs {
    type Vtable = IAppWindowClosingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        235526411,
        8801,
        22795,
        [154, 209, 133, 4, 153, 29, 135, 84],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowClosingEventArgs_abi(
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
        value: bool,
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
pub struct IAppWindowPresenter(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppWindowPresenter {
    type Vtable = IAppWindowPresenter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3157279426,
        50886,
        22066,
        [137, 137, 255, 14, 198, 211, 180, 13],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowPresenter_abi(
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
        result__: *mut AppWindowPresenterKind,
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
pub struct IAppWindowPresenterFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppWindowPresenterFactory {
    type Vtable = IAppWindowPresenterFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1644703292,
        4968,
        21048,
        [144, 209, 233, 50, 220, 113, 138, 130],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowPresenterFactory_abi(
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
pub struct IAppWindowStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppWindowStatics {
    type Vtable = IAppWindowStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1009867812,
        54592,
        23922,
        [181, 24, 178, 38, 184, 54, 39, 203],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowStatics_abi(
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
        appwindowpresenter: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        appwindowpresenter: ::windows::runtime::RawPtr,
        ownerwindowid: super::WindowId,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        windowid: super::WindowId,
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
pub struct IAppWindowTitleBar(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppWindowTitleBar {
    type Vtable = IAppWindowTitleBar_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1433726882,
        51484,
        22272,
        [163, 99, 83, 156, 113, 167, 170, 244],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowTitleBar_abi(
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
    #[cfg(all(feature = "undation", feature = ""))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "undation", feature = "")))] usize,
    #[cfg(all(feature = "undation", feature = ""))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "undation", feature = "")))] usize,
    #[cfg(all(feature = "undation", feature = ""))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "undation", feature = "")))] usize,
    #[cfg(all(feature = "undation", feature = ""))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "undation", feature = "")))] usize,
    #[cfg(all(feature = "undation", feature = ""))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "undation", feature = "")))] usize,
    #[cfg(all(feature = "undation", feature = ""))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "undation", feature = "")))] usize,
    #[cfg(all(feature = "undation", feature = ""))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "undation", feature = "")))] usize,
    #[cfg(all(feature = "undation", feature = ""))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "undation", feature = "")))] usize,
    #[cfg(all(feature = "undation", feature = ""))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "undation", feature = "")))] usize,
    #[cfg(all(feature = "undation", feature = ""))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "undation", feature = "")))] usize,
    #[cfg(all(feature = "undation", feature = ""))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "undation", feature = "")))] usize,
    #[cfg(all(feature = "undation", feature = ""))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "undation", feature = "")))] usize,
    #[cfg(all(feature = "undation", feature = ""))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "undation", feature = "")))] usize,
    #[cfg(all(feature = "undation", feature = ""))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "undation", feature = "")))] usize,
    #[cfg(all(feature = "undation", feature = ""))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "undation", feature = "")))] usize,
    #[cfg(all(feature = "undation", feature = ""))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "undation", feature = "")))] usize,
    #[cfg(all(feature = "undation", feature = ""))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "undation", feature = "")))] usize,
    #[cfg(all(feature = "undation", feature = ""))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "undation", feature = "")))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "undation", feature = ""))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "undation", feature = "")))] usize,
    #[cfg(all(feature = "undation", feature = ""))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "undation", feature = "")))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut IconShowOptions,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: IconShowOptions,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "undation", feature = ""))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "undation", feature = "")))] usize,
    #[cfg(all(feature = "undation", feature = ""))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "undation", feature = "")))] usize,
    #[cfg(all(feature = "undation", feature = ""))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "undation", feature = "")))] usize,
    #[cfg(all(feature = "undation", feature = ""))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "undation", feature = "")))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "aphics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value_array_size: u32,
        value: *const super::super::super::Windows::Graphics::RectInt32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "aphics"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IAppWindowTitleBarStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppWindowTitleBarStatics {
    type Vtable = IAppWindowTitleBarStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2652742958,
        35605,
        21718,
        [168, 134, 247, 185, 249, 217, 48, 178],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowTitleBarStatics_abi(
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
pub struct ICompactOverlayPresenter(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICompactOverlayPresenter {
    type Vtable = ICompactOverlayPresenter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4025157650,
        28615,
        23421,
        [189, 146, 204, 143, 154, 100, 84, 201],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompactOverlayPresenter_abi(
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
        result__: *mut CompactOverlaySize,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: CompactOverlaySize,
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
pub struct ICompactOverlayPresenterStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICompactOverlayPresenterStatics {
    type Vtable = ICompactOverlayPresenterStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3938005382,
        20330,
        21241,
        [140, 3, 218, 87, 161, 82, 47, 110],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompactOverlayPresenterStatics_abi(
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
pub struct IDisplayArea(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDisplayArea {
    type Vtable = IDisplayArea_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1551762743,
        46625,
        21881,
        [188, 174, 168, 74, 168, 116, 97, 103],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayArea_abi(
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
        result__: *mut super::DisplayId,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "aphics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::Windows::Graphics::RectInt32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "aphics"))] usize,
    #[cfg(feature = "aphics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::Windows::Graphics::RectInt32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "aphics"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IDisplayAreaStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDisplayAreaStatics {
    type Vtable = IDisplayAreaStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        44779814,
        8478,
        23881,
        [142, 75, 42, 241, 147, 218, 237, 9],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayAreaStatics_abi(
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
    #[cfg(feature = "undation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation_Collections"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        windowid: super::WindowId,
        displayareafallback: DisplayAreaFallback,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "aphics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        point: super::super::super::Windows::Graphics::PointInt32,
        displayareafallback: DisplayAreaFallback,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "aphics"))] usize,
    #[cfg(feature = "aphics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        rect: super::super::super::Windows::Graphics::RectInt32,
        displayareafallback: DisplayAreaFallback,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "aphics"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IDisplayAreaWatcher(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDisplayAreaWatcher {
    type Vtable = IDisplayAreaWatcher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2213959215,
        54176,
        21643,
        [142, 79, 169, 155, 227, 217, 92, 156],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayAreaWatcher_abi(
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
        result__: *mut DisplayAreaWatcherStatus,
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
pub struct IFullScreenPresenter(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFullScreenPresenter {
    type Vtable = IFullScreenPresenter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4203823613,
        47325,
        23969,
        [139, 43, 124, 218, 219, 118, 245, 147],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFullScreenPresenter_abi(
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
pub struct IFullScreenPresenterStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFullScreenPresenterStatics {
    type Vtable = IFullScreenPresenterStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        784388801,
        57478,
        21947,
        [163, 178, 68, 148, 46, 35, 28, 103],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFullScreenPresenterStatics_abi(
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
pub struct IOverlappedPresenter(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IOverlappedPresenter {
    type Vtable = IOverlappedPresenter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        560544112,
        20300,
        20850,
        [158, 157, 104, 42, 45, 23, 72, 132],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IOverlappedPresenter_abi(
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
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut OverlappedPresenterState,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hasborder: bool,
        hastitlebar: bool,
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
pub struct IOverlappedPresenterStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IOverlappedPresenterStatics {
    type Vtable = IOverlappedPresenterStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2574394852,
        31488,
        23278,
        [164, 190, 212, 6, 141, 25, 153, 226],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IOverlappedPresenterStatics_abi(
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
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IconShowOptions(pub i32);
impl IconShowOptions {
    pub const ShowIconAndSystemMenu: IconShowOptions = IconShowOptions(0i32);
    pub const HideIconAndSystemMenu: IconShowOptions = IconShowOptions(1i32);
}
impl ::std::convert::From<i32> for IconShowOptions {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IconShowOptions {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for IconShowOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Windowing.IconShowOptions;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct OverlappedPresenter(::windows::runtime::IInspectable);
impl OverlappedPresenter {
    pub fn HasBorder(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn HasTitleBar(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn IsAlwaysOnTop(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn SetIsAlwaysOnTop(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsMaximizable(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn SetIsMaximizable(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsMinimizable(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn SetIsMinimizable(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsModal(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn SetIsModal(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsResizable(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsResizable(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn State(&self) -> ::windows::runtime::Result<OverlappedPresenterState> {
        let this = self;
        unsafe {
            let mut result__: OverlappedPresenterState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<OverlappedPresenterState>(result__)
        }
    }
    pub fn Maximize(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Minimize(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Restore(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn SetBorderAndTitleBar(
        &self,
        hasborder: bool,
        hastitlebar: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                hasborder,
                hastitlebar,
            )
            .ok()
        }
    }
    pub fn Create() -> ::windows::runtime::Result<OverlappedPresenter> {
        Self::IOverlappedPresenterStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<OverlappedPresenter>(result__)
        })
    }
    pub fn CreateForContextMenu() -> ::windows::runtime::Result<OverlappedPresenter> {
        Self::IOverlappedPresenterStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<OverlappedPresenter>(result__)
        })
    }
    pub fn CreateForDialog() -> ::windows::runtime::Result<OverlappedPresenter> {
        Self::IOverlappedPresenterStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<OverlappedPresenter>(result__)
        })
    }
    pub fn CreateForToolWindow() -> ::windows::runtime::Result<OverlappedPresenter> {
        Self::IOverlappedPresenterStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<OverlappedPresenter>(result__)
        })
    }
    pub fn Kind(&self) -> ::windows::runtime::Result<AppWindowPresenterKind> {
        let this = &::windows::runtime::Interface::cast::<IAppWindowPresenter>(self)?;
        unsafe {
            let mut result__: AppWindowPresenterKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AppWindowPresenterKind>(result__)
        }
    }
    pub fn IOverlappedPresenterStatics<
        R,
        F: FnOnce(&IOverlappedPresenterStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            OverlappedPresenter,
            IOverlappedPresenterStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for OverlappedPresenter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Windowing.OverlappedPresenter;{21693970-4f4c-5172-9e9d-682a2d174884})",
    );
}
unsafe impl ::windows::runtime::Interface for OverlappedPresenter {
    type Vtable = IOverlappedPresenter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        560544112,
        20300,
        20850,
        [158, 157, 104, 42, 45, 23, 72, 132],
    );
}
impl ::windows::runtime::RuntimeName for OverlappedPresenter {
    const NAME: &'static str = "Microsoft.UI.Windowing.OverlappedPresenter";
}
impl ::std::convert::From<OverlappedPresenter> for ::windows::runtime::IUnknown {
    fn from(value: OverlappedPresenter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&OverlappedPresenter> for ::windows::runtime::IUnknown {
    fn from(value: &OverlappedPresenter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for OverlappedPresenter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &OverlappedPresenter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<OverlappedPresenter> for ::windows::runtime::IInspectable {
    fn from(value: OverlappedPresenter) -> Self {
        value.0
    }
}
impl ::std::convert::From<&OverlappedPresenter> for ::windows::runtime::IInspectable {
    fn from(value: &OverlappedPresenter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for OverlappedPresenter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a OverlappedPresenter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<OverlappedPresenter> for AppWindowPresenter {
    fn from(value: OverlappedPresenter) -> Self {
        ::std::convert::Into::<AppWindowPresenter>::into(&value)
    }
}
impl ::std::convert::From<&OverlappedPresenter> for AppWindowPresenter {
    fn from(value: &OverlappedPresenter) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, AppWindowPresenter> for OverlappedPresenter {
    fn into_param(self) -> ::windows::runtime::Param<'a, AppWindowPresenter> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<AppWindowPresenter>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, AppWindowPresenter> for &OverlappedPresenter {
    fn into_param(self) -> ::windows::runtime::Param<'a, AppWindowPresenter> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<AppWindowPresenter>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for OverlappedPresenter {}
unsafe impl ::std::marker::Sync for OverlappedPresenter {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct OverlappedPresenterState(pub i32);
impl OverlappedPresenterState {
    pub const Maximized: OverlappedPresenterState = OverlappedPresenterState(0i32);
    pub const Minimized: OverlappedPresenterState = OverlappedPresenterState(1i32);
    pub const Restored: OverlappedPresenterState = OverlappedPresenterState(2i32);
}
impl ::std::convert::From<i32> for OverlappedPresenterState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OverlappedPresenterState {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for OverlappedPresenterState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Windowing.OverlappedPresenterState;i4)",
    );
}
