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
pub struct CoreWebView2(::windows::runtime::IInspectable);
impl CoreWebView2 {
    pub fn Settings(&self) -> ::windows::runtime::Result<CoreWebView2Settings> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2Settings>(result__)
        }
    }
    pub fn Source(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn BrowserProcessId(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn CanGoBack(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn CanGoForward(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn DocumentTitle(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn ContainsFullScreenElement(&self) -> ::windows::runtime::Result<bool> {
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
    #[cfg(feature = "undation")]
    pub fn NavigationStarting<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2NavigationStartingEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = self;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::EventRegistrationToken>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveNavigationStarting<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
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
    pub fn ContentLoading<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2ContentLoadingEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = self;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::EventRegistrationToken>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveContentLoading<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
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
    pub fn SourceChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2SourceChangedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = self;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::EventRegistrationToken>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveSourceChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
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
    #[cfg(feature = "undation")]
    pub fn HistoryChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                CoreWebView2,
                ::windows::runtime::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = self;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::EventRegistrationToken>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveHistoryChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn NavigationCompleted<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2NavigationCompletedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = self;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).21)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::EventRegistrationToken>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveNavigationCompleted<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn FrameNavigationStarting<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2NavigationStartingEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = self;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::EventRegistrationToken>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveFrameNavigationStarting<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn FrameNavigationCompleted<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2NavigationCompletedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = self;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::EventRegistrationToken>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveFrameNavigationCompleted<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn ScriptDialogOpening<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2ScriptDialogOpeningEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = self;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).27)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::EventRegistrationToken>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveScriptDialogOpening<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).28)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn PermissionRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2PermissionRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = self;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).29)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::EventRegistrationToken>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemovePermissionRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
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
    pub fn ProcessFailed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2ProcessFailedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = self;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).31)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::EventRegistrationToken>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveProcessFailed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
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
    pub fn WebMessageReceived<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2WebMessageReceivedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = self;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).33)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::EventRegistrationToken>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveWebMessageReceived<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
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
    #[cfg(feature = "undation")]
    pub fn NewWindowRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2NewWindowRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = self;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).35)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::EventRegistrationToken>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveNewWindowRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).36)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn DocumentTitleChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                CoreWebView2,
                ::windows::runtime::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = self;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).37)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::EventRegistrationToken>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveDocumentTitleChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).38)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn ContainsFullScreenElementChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                CoreWebView2,
                ::windows::runtime::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = self;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).39)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::EventRegistrationToken>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveContainsFullScreenElementChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).40)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn WebResourceRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2WebResourceRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = self;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).41)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::EventRegistrationToken>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveWebResourceRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).42)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn WindowCloseRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                CoreWebView2,
                ::windows::runtime::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = self;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).43)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::EventRegistrationToken>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveWindowCloseRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).44)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Navigate<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        uri: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).45)(
                ::std::mem::transmute_copy(this),
                uri.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn NavigateToString<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        htmlcontent: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).46)(
                ::std::mem::transmute_copy(this),
                htmlcontent.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn AddScriptToExecuteOnDocumentCreatedAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        javascript: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::IAsyncOperation<
            ::windows::runtime::HSTRING,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).47)(
                ::std::mem::transmute_copy(this),
                javascript.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::IAsyncOperation<
                ::windows::runtime::HSTRING,
            >>(result__)
        }
    }
    pub fn RemoveScriptToExecuteOnDocumentCreated<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        id: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).48)(
                ::std::mem::transmute_copy(this),
                id.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn ExecuteScriptAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        javascript: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::IAsyncOperation<
            ::windows::runtime::HSTRING,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).49)(
                ::std::mem::transmute_copy(this),
                javascript.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::IAsyncOperation<
                ::windows::runtime::HSTRING,
            >>(result__)
        }
    }
    #[cfg(all(feature = "undation", feature = "orage_Streams"))]
    pub fn CapturePreviewAsync<
        'a,
        Param1: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Storage::Streams::IRandomAccessStream,
        >,
    >(
        &self,
        imageformat: CoreWebView2CapturePreviewImageFormat,
        imagestream: Param1,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::Foundation::IAsyncAction>
    {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).50)(
                ::std::mem::transmute_copy(this),
                imageformat,
                imagestream.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn Reload(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).51)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn PostWebMessageAsJson<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        webmessageasjson: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).52)(
                ::std::mem::transmute_copy(this),
                webmessageasjson.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn PostWebMessageAsString<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        webmessageasstring: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).53)(
                ::std::mem::transmute_copy(this),
                webmessageasstring.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn CallDevToolsProtocolMethodAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        methodname: Param0,
        parametersasjson: Param1,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::IAsyncOperation<
            ::windows::runtime::HSTRING,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).54)(
                ::std::mem::transmute_copy(this),
                methodname.into_param().abi(),
                parametersasjson.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::IAsyncOperation<
                ::windows::runtime::HSTRING,
            >>(result__)
        }
    }
    pub fn GoBack(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).55)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn GoForward(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).56)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn GetDevToolsProtocolEventReceiver<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        eventname: Param0,
    ) -> ::windows::runtime::Result<CoreWebView2DevToolsProtocolEventReceiver> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).57)(
                ::std::mem::transmute_copy(this),
                eventname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<CoreWebView2DevToolsProtocolEventReceiver>(result__)
        }
    }
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).58)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn AddHostObjectToScript<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        name: Param0,
        rawobject: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).59)(
                ::std::mem::transmute_copy(this),
                name.into_param().abi(),
                rawobject.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveHostObjectFromScript<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        name: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).60)(
                ::std::mem::transmute_copy(this),
                name.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn OpenDevToolsWindow(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).61)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn AddWebResourceRequestedFilter<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        uri: Param0,
        resourcecontext: CoreWebView2WebResourceContext,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).62)(
                ::std::mem::transmute_copy(this),
                uri.into_param().abi(),
                resourcecontext,
            )
            .ok()
        }
    }
    pub fn RemoveWebResourceRequestedFilter<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        uri: Param0,
        resourcecontext: CoreWebView2WebResourceContext,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).63)(
                ::std::mem::transmute_copy(this),
                uri.into_param().abi(),
                resourcecontext,
            )
            .ok()
        }
    }
    pub fn CookieManager(&self) -> ::windows::runtime::Result<CoreWebView2CookieManager> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2_2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2CookieManager>(result__)
        }
    }
    pub fn Environment(&self) -> ::windows::runtime::Result<CoreWebView2Environment> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2_2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2Environment>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn WebResourceResponseReceived<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2WebResourceResponseReceivedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2_2>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::EventRegistrationToken>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveWebResourceResponseReceived<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2_2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn DOMContentLoaded<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2DOMContentLoadedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2_2>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::EventRegistrationToken>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveDOMContentLoaded<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2_2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn NavigateWithWebResourceRequest<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, CoreWebView2WebResourceRequest>,
    >(
        &self,
        request: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2_2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                request.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IsSuspended(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2_3>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn TrySuspendAsync(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::IAsyncOperation<bool>,
    > {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2_3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::IAsyncOperation<bool>>(
                result__,
            )
        }
    }
    pub fn Resume(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2_3>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn SetVirtualHostNameToFolderMapping<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        hostname: Param0,
        folderpath: Param1,
        accesskind: CoreWebView2HostResourceAccessKind,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2_3>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                hostname.into_param().abi(),
                folderpath.into_param().abi(),
                accesskind,
            )
            .ok()
        }
    }
    pub fn ClearVirtualHostNameToFolderMapping<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        hostname: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2_3>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                hostname.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn FrameCreated<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2FrameCreatedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2_4>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::EventRegistrationToken>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveFrameCreated<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2_4>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn DownloadStarting<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2DownloadStartingEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2_4>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::EventRegistrationToken>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveDownloadStarting<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2_4>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn ClientCertificateRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2ClientCertificateRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2_5>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::EventRegistrationToken>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveClientCertificateRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2_5>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2 {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.Web.WebView2.Core.CoreWebView2;{3a3f559a-e5e9-5338-bb67-4eb0504a4f14})",
    );
}
unsafe impl ::windows::runtime::Interface for CoreWebView2 {
    type Vtable = ICoreWebView2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        977229210,
        58857,
        21304,
        [187, 103, 78, 176, 80, 74, 79, 20],
    );
}
impl ::windows::runtime::RuntimeName for CoreWebView2 {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2";
}
impl ::std::convert::From<CoreWebView2> for ::windows::runtime::IUnknown {
    fn from(value: CoreWebView2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreWebView2> for ::windows::runtime::IUnknown {
    fn from(value: &CoreWebView2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CoreWebView2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &CoreWebView2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CoreWebView2> for ::windows::runtime::IInspectable {
    fn from(value: CoreWebView2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreWebView2> for ::windows::runtime::IInspectable {
    fn from(value: &CoreWebView2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CoreWebView2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CoreWebView2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreWebView2 {}
unsafe impl ::std::marker::Sync for CoreWebView2 {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CoreWebView2AcceleratorKeyPressedEventArgs(::windows::runtime::IInspectable);
impl CoreWebView2AcceleratorKeyPressedEventArgs {
    pub fn KeyEventKind(&self) -> ::windows::runtime::Result<CoreWebView2KeyEventKind> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2KeyEventKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2KeyEventKind>(result__)
        }
    }
    pub fn VirtualKey(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn KeyEventLParam(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn PhysicalKeyStatus(&self) -> ::windows::runtime::Result<CoreWebView2PhysicalKeyStatus> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2PhysicalKeyStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2PhysicalKeyStatus>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn SetHandled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2AcceleratorKeyPressedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2AcceleratorKeyPressedEventArgs;{41a56100-92a5-59d1-9e71-9222e33ae38b})" ) ;
}
unsafe impl ::windows::runtime::Interface for CoreWebView2AcceleratorKeyPressedEventArgs {
    type Vtable = ICoreWebView2AcceleratorKeyPressedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1101357312,
        37541,
        22993,
        [158, 113, 146, 34, 227, 58, 227, 139],
    );
}
impl ::windows::runtime::RuntimeName for CoreWebView2AcceleratorKeyPressedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2AcceleratorKeyPressedEventArgs";
}
impl ::std::convert::From<CoreWebView2AcceleratorKeyPressedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: CoreWebView2AcceleratorKeyPressedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreWebView2AcceleratorKeyPressedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &CoreWebView2AcceleratorKeyPressedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CoreWebView2AcceleratorKeyPressedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CoreWebView2AcceleratorKeyPressedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CoreWebView2AcceleratorKeyPressedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: CoreWebView2AcceleratorKeyPressedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreWebView2AcceleratorKeyPressedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &CoreWebView2AcceleratorKeyPressedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CoreWebView2AcceleratorKeyPressedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CoreWebView2AcceleratorKeyPressedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreWebView2AcceleratorKeyPressedEventArgs {}
unsafe impl ::std::marker::Sync for CoreWebView2AcceleratorKeyPressedEventArgs {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CoreWebView2BoundsMode(pub i32);
impl CoreWebView2BoundsMode {
    pub const UseRawPixels: CoreWebView2BoundsMode = CoreWebView2BoundsMode(0i32);
    pub const UseRasterizationScale: CoreWebView2BoundsMode = CoreWebView2BoundsMode(1i32);
}
impl ::std::convert::From<i32> for CoreWebView2BoundsMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CoreWebView2BoundsMode {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2BoundsMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2BoundsMode;i4)",
    );
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
pub struct CoreWebView2CapturePreviewImageFormat(pub i32);
impl CoreWebView2CapturePreviewImageFormat {
    pub const Png: CoreWebView2CapturePreviewImageFormat =
        CoreWebView2CapturePreviewImageFormat(0i32);
    pub const Jpeg: CoreWebView2CapturePreviewImageFormat =
        CoreWebView2CapturePreviewImageFormat(1i32);
}
impl ::std::convert::From<i32> for CoreWebView2CapturePreviewImageFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CoreWebView2CapturePreviewImageFormat {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2CapturePreviewImageFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2CapturePreviewImageFormat;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CoreWebView2ClientCertificate(::windows::runtime::IInspectable);
impl CoreWebView2ClientCertificate {
    pub fn Subject(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn Issuer(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn ValidFrom(&self) -> ::windows::runtime::Result<f64> {
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
    pub fn ValidTo(&self) -> ::windows::runtime::Result<f64> {
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
    pub fn DerEncodedSerialNumber(
        &self,
    ) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "undation_Collections")]
    pub fn PemEncodedIssuerCertificateChain(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::Collections::IVectorView<
            ::windows::runtime::HSTRING,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Collections::IVectorView<
                ::windows::runtime::HSTRING,
            >>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::runtime::Result<CoreWebView2ClientCertificateKind> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2ClientCertificateKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2ClientCertificateKind>(result__)
        }
    }
    pub fn ToPemEncoding(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    #[cfg(feature = "curity_Cryptography_Certificates")]
    pub fn ToCertificate(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Security::Cryptography::Certificates::Certificate,
    > {
        let this =
            &::windows::runtime::Interface::cast::<CoreWebView2ClientCertificate_Manual>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .6 ) ( :: std :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < super::super::super::super::Windows::Security::Cryptography::Certificates:: Certificate > ( result__ )
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2ClientCertificate {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2ClientCertificate;{091b39f2-68df-52b4-8fb0-fd3561af41f2})" ) ;
}
unsafe impl ::windows::runtime::Interface for CoreWebView2ClientCertificate {
    type Vtable = ICoreWebView2ClientCertificate_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        152779250,
        26847,
        21172,
        [143, 176, 253, 53, 97, 175, 65, 242],
    );
}
impl ::windows::runtime::RuntimeName for CoreWebView2ClientCertificate {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2ClientCertificate";
}
impl ::std::convert::From<CoreWebView2ClientCertificate> for ::windows::runtime::IUnknown {
    fn from(value: CoreWebView2ClientCertificate) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreWebView2ClientCertificate> for ::windows::runtime::IUnknown {
    fn from(value: &CoreWebView2ClientCertificate) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CoreWebView2ClientCertificate
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CoreWebView2ClientCertificate
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CoreWebView2ClientCertificate> for ::windows::runtime::IInspectable {
    fn from(value: CoreWebView2ClientCertificate) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreWebView2ClientCertificate> for ::windows::runtime::IInspectable {
    fn from(value: &CoreWebView2ClientCertificate) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CoreWebView2ClientCertificate
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CoreWebView2ClientCertificate
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreWebView2ClientCertificate {}
unsafe impl ::std::marker::Sync for CoreWebView2ClientCertificate {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CoreWebView2ClientCertificateKind(pub i32);
impl CoreWebView2ClientCertificateKind {
    pub const SmartCard: CoreWebView2ClientCertificateKind =
        CoreWebView2ClientCertificateKind(0i32);
    pub const Pin: CoreWebView2ClientCertificateKind = CoreWebView2ClientCertificateKind(1i32);
    pub const Other: CoreWebView2ClientCertificateKind = CoreWebView2ClientCertificateKind(2i32);
}
impl ::std::convert::From<i32> for CoreWebView2ClientCertificateKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CoreWebView2ClientCertificateKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2ClientCertificateKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2ClientCertificateKind;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CoreWebView2ClientCertificateRequestedEventArgs(::windows::runtime::IInspectable);
impl CoreWebView2ClientCertificateRequestedEventArgs {
    pub fn Host(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn Port(&self) -> ::windows::runtime::Result<i32> {
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
    pub fn IsProxy(&self) -> ::windows::runtime::Result<bool> {
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
    #[cfg(feature = "undation_Collections")]
    pub fn AllowedCertificateAuthorities(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::Collections::IVectorView<
            ::windows::runtime::HSTRING,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Collections::IVectorView<
                ::windows::runtime::HSTRING,
            >>(result__)
        }
    }
    #[cfg(feature = "undation_Collections")]
    pub fn MutuallyTrustedCertificates(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::Collections::IVectorView<
            CoreWebView2ClientCertificate,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Collections::IVectorView<
                CoreWebView2ClientCertificate,
            >>(result__)
        }
    }
    pub fn SelectedCertificate(&self) -> ::windows::runtime::Result<CoreWebView2ClientCertificate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2ClientCertificate>(result__)
        }
    }
    pub fn SetSelectedCertificate<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, CoreWebView2ClientCertificate>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Cancel(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn SetCancel(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn Handled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn GetDeferral(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2ClientCertificateRequestedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2ClientCertificateRequestedEventArgs;{93287b55-31f9-55a0-b68b-d9841d7e1bf4})" ) ;
}
unsafe impl ::windows::runtime::Interface for CoreWebView2ClientCertificateRequestedEventArgs {
    type Vtable = ICoreWebView2ClientCertificateRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2468903765,
        12793,
        21920,
        [182, 139, 217, 132, 29, 126, 27, 244],
    );
}
impl ::windows::runtime::RuntimeName for CoreWebView2ClientCertificateRequestedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2ClientCertificateRequestedEventArgs";
}
impl ::std::convert::From<CoreWebView2ClientCertificateRequestedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: CoreWebView2ClientCertificateRequestedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreWebView2ClientCertificateRequestedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &CoreWebView2ClientCertificateRequestedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CoreWebView2ClientCertificateRequestedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CoreWebView2ClientCertificateRequestedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CoreWebView2ClientCertificateRequestedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: CoreWebView2ClientCertificateRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreWebView2ClientCertificateRequestedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &CoreWebView2ClientCertificateRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CoreWebView2ClientCertificateRequestedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CoreWebView2ClientCertificateRequestedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreWebView2ClientCertificateRequestedEventArgs {}
unsafe impl ::std::marker::Sync for CoreWebView2ClientCertificateRequestedEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct CoreWebView2ClientCertificate_Manual(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for CoreWebView2ClientCertificate_Manual {
    type Vtable = CoreWebView2ClientCertificate_Manual_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4210028482,
        8387,
        23942,
        [138, 116, 246, 216, 125, 111, 248, 250],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct CoreWebView2ClientCertificate_Manual_abi(
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
    #[cfg(feature = "curity_Cryptography_Certificates")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "curity_Cryptography_Certificates"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CoreWebView2CompositionController(::windows::runtime::IInspectable);
impl CoreWebView2CompositionController {
    pub fn RootVisualTarget(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
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
    pub fn SetRootVisualTarget<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
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
    #[cfg(feature = "undation")]
    pub fn CursorChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                CoreWebView2CompositionController,
                ::windows::runtime::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = self;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::EventRegistrationToken>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveCursorChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn SendMouseInput<
        'a,
        Param3: ::windows::runtime::IntoParam<'a, super::super::super::super::Windows::Foundation::Point>,
    >(
        &self,
        eventkind: CoreWebView2MouseEventKind,
        virtualkeys: CoreWebView2MouseEventVirtualKeys,
        mousedata: u32,
        point: Param3,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                eventkind,
                virtualkeys,
                mousedata,
                point.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn SendPointerInput<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, CoreWebView2PointerInfo>,
    >(
        &self,
        eventkind: CoreWebView2PointerEventKind,
        pointerinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                eventkind,
                pointerinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "_Core")]
    pub fn Cursor(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Core::CoreCursor> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Core::CoreCursor>(result__)
        }
    }
    pub fn IsVisible(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsVisible(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn Bounds(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::Foundation::Rect> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::Foundation::Rect =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn SetBounds<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Windows::Foundation::Rect>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ZoomFactor(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetZoomFactor(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn ParentWindow(
        &self,
    ) -> ::windows::runtime::Result<CoreWebView2ControllerWindowReference> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2ControllerWindowReference>(result__)
        }
    }
    pub fn SetParentWindow<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, CoreWebView2ControllerWindowReference>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn CoreWebView2(&self) -> ::windows::runtime::Result<CoreWebView2> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn ZoomFactorChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                CoreWebView2Controller,
                ::windows::runtime::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::EventRegistrationToken>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveZoomFactorChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn MoveFocusRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                CoreWebView2Controller,
                CoreWebView2MoveFocusRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::EventRegistrationToken>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveMoveFocusRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn GotFocus<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                CoreWebView2Controller,
                ::windows::runtime::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::EventRegistrationToken>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveGotFocus<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn LostFocus<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                CoreWebView2Controller,
                ::windows::runtime::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).21)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::EventRegistrationToken>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveLostFocus<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn AcceleratorKeyPressed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                CoreWebView2Controller,
                CoreWebView2AcceleratorKeyPressedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::EventRegistrationToken>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveAcceleratorKeyPressed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn SetBoundsAndZoomFactor<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Windows::Foundation::Rect>,
    >(
        &self,
        bounds: Param0,
        zoomfactor: f64,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                bounds.into_param().abi(),
                zoomfactor,
            )
            .ok()
        }
    }
    pub fn MoveFocus(&self, reason: CoreWebView2MoveFocusReason) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                reason,
            )
            .ok()
        }
    }
    pub fn NotifyParentWindowPositionChanged(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "")]
    pub fn DefaultBackgroundColor(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Color> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller2>(self)?;
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
    pub fn SetDefaultBackgroundColor<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Windows::UI::Color>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn RasterizationScale(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetRasterizationScale(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn ShouldDetectMonitorScaleChanges(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetShouldDetectMonitorScaleChanges(
        &self,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn BoundsMode(&self) -> ::windows::runtime::Result<CoreWebView2BoundsMode> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            let mut result__: CoreWebView2BoundsMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2BoundsMode>(result__)
        }
    }
    pub fn SetBoundsMode(&self, value: CoreWebView2BoundsMode) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn RasterizationScaleChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                CoreWebView2Controller,
                ::windows::runtime::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::EventRegistrationToken>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveRasterizationScaleChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2CompositionController {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2CompositionController;{4fb8b7b3-4a2e-5787-94b9-cc48c4d364d7})" ) ;
}
unsafe impl ::windows::runtime::Interface for CoreWebView2CompositionController {
    type Vtable = ICoreWebView2CompositionController_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1337505715,
        18990,
        22407,
        [148, 185, 204, 72, 196, 211, 100, 215],
    );
}
impl ::windows::runtime::RuntimeName for CoreWebView2CompositionController {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2CompositionController";
}
impl ::std::convert::From<CoreWebView2CompositionController> for ::windows::runtime::IUnknown {
    fn from(value: CoreWebView2CompositionController) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreWebView2CompositionController> for ::windows::runtime::IUnknown {
    fn from(value: &CoreWebView2CompositionController) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CoreWebView2CompositionController
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CoreWebView2CompositionController
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CoreWebView2CompositionController> for ::windows::runtime::IInspectable {
    fn from(value: CoreWebView2CompositionController) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreWebView2CompositionController> for ::windows::runtime::IInspectable {
    fn from(value: &CoreWebView2CompositionController) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CoreWebView2CompositionController
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CoreWebView2CompositionController
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<CoreWebView2CompositionController> for CoreWebView2Controller {
    fn from(value: CoreWebView2CompositionController) -> Self {
        ::std::convert::Into::<CoreWebView2Controller>::into(&value)
    }
}
impl ::std::convert::From<&CoreWebView2CompositionController> for CoreWebView2Controller {
    fn from(value: &CoreWebView2CompositionController) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, CoreWebView2Controller>
    for CoreWebView2CompositionController
{
    fn into_param(self) -> ::windows::runtime::Param<'a, CoreWebView2Controller> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<CoreWebView2Controller>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, CoreWebView2Controller>
    for &CoreWebView2CompositionController
{
    fn into_param(self) -> ::windows::runtime::Param<'a, CoreWebView2Controller> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<CoreWebView2Controller>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for CoreWebView2CompositionController {}
unsafe impl ::std::marker::Sync for CoreWebView2CompositionController {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CoreWebView2ContentLoadingEventArgs(::windows::runtime::IInspectable);
impl CoreWebView2ContentLoadingEventArgs {
    pub fn IsErrorPage(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn NavigationId(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u64>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2ContentLoadingEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2ContentLoadingEventArgs;{6cf95373-946c-5dae-9b3e-0fe23d5aa29f})" ) ;
}
unsafe impl ::windows::runtime::Interface for CoreWebView2ContentLoadingEventArgs {
    type Vtable = ICoreWebView2ContentLoadingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1828279155,
        37996,
        23982,
        [155, 62, 15, 226, 61, 90, 162, 159],
    );
}
impl ::windows::runtime::RuntimeName for CoreWebView2ContentLoadingEventArgs {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2ContentLoadingEventArgs";
}
impl ::std::convert::From<CoreWebView2ContentLoadingEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: CoreWebView2ContentLoadingEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreWebView2ContentLoadingEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &CoreWebView2ContentLoadingEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CoreWebView2ContentLoadingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CoreWebView2ContentLoadingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CoreWebView2ContentLoadingEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: CoreWebView2ContentLoadingEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreWebView2ContentLoadingEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &CoreWebView2ContentLoadingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CoreWebView2ContentLoadingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CoreWebView2ContentLoadingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreWebView2ContentLoadingEventArgs {}
unsafe impl ::std::marker::Sync for CoreWebView2ContentLoadingEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CoreWebView2Controller(::windows::runtime::IInspectable);
impl CoreWebView2Controller {
    pub fn IsVisible(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn SetIsVisible(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn Bounds(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Windows::Foundation::Rect =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn SetBounds<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Windows::Foundation::Rect>,
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
    pub fn ZoomFactor(&self) -> ::windows::runtime::Result<f64> {
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
    pub fn SetZoomFactor(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn ParentWindow(
        &self,
    ) -> ::windows::runtime::Result<CoreWebView2ControllerWindowReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2ControllerWindowReference>(result__)
        }
    }
    pub fn SetParentWindow<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, CoreWebView2ControllerWindowReference>,
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
    pub fn CoreWebView2(&self) -> ::windows::runtime::Result<CoreWebView2> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn ZoomFactorChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                CoreWebView2Controller,
                ::windows::runtime::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = self;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::EventRegistrationToken>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveZoomFactorChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
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
    pub fn MoveFocusRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                CoreWebView2Controller,
                CoreWebView2MoveFocusRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = self;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::EventRegistrationToken>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveMoveFocusRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
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
    #[cfg(feature = "undation")]
    pub fn GotFocus<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                CoreWebView2Controller,
                ::windows::runtime::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = self;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::EventRegistrationToken>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveGotFocus<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn LostFocus<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                CoreWebView2Controller,
                ::windows::runtime::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = self;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).21)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::EventRegistrationToken>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveLostFocus<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn AcceleratorKeyPressed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                CoreWebView2Controller,
                CoreWebView2AcceleratorKeyPressedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = self;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::EventRegistrationToken>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveAcceleratorKeyPressed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn SetBoundsAndZoomFactor<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Windows::Foundation::Rect>,
    >(
        &self,
        bounds: Param0,
        zoomfactor: f64,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                bounds.into_param().abi(),
                zoomfactor,
            )
            .ok()
        }
    }
    pub fn MoveFocus(&self, reason: CoreWebView2MoveFocusReason) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                reason,
            )
            .ok()
        }
    }
    pub fn NotifyParentWindowPositionChanged(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "")]
    pub fn DefaultBackgroundColor(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Color> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller2>(self)?;
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
    pub fn SetDefaultBackgroundColor<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Windows::UI::Color>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn RasterizationScale(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetRasterizationScale(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn ShouldDetectMonitorScaleChanges(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetShouldDetectMonitorScaleChanges(
        &self,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn BoundsMode(&self) -> ::windows::runtime::Result<CoreWebView2BoundsMode> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            let mut result__: CoreWebView2BoundsMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2BoundsMode>(result__)
        }
    }
    pub fn SetBoundsMode(&self, value: CoreWebView2BoundsMode) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn RasterizationScaleChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                CoreWebView2Controller,
                ::windows::runtime::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::EventRegistrationToken>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveRasterizationScaleChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2Controller {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2Controller;{a588121c-53bf-590e-80e5-29d729cbd743})" ) ;
}
unsafe impl ::windows::runtime::Interface for CoreWebView2Controller {
    type Vtable = ICoreWebView2Controller_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2777158172,
        21439,
        22798,
        [128, 229, 41, 215, 41, 203, 215, 67],
    );
}
impl ::windows::runtime::RuntimeName for CoreWebView2Controller {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2Controller";
}
impl ::std::convert::From<CoreWebView2Controller> for ::windows::runtime::IUnknown {
    fn from(value: CoreWebView2Controller) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreWebView2Controller> for ::windows::runtime::IUnknown {
    fn from(value: &CoreWebView2Controller) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CoreWebView2Controller
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CoreWebView2Controller
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CoreWebView2Controller> for ::windows::runtime::IInspectable {
    fn from(value: CoreWebView2Controller) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreWebView2Controller> for ::windows::runtime::IInspectable {
    fn from(value: &CoreWebView2Controller) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CoreWebView2Controller
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CoreWebView2Controller
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreWebView2Controller {}
unsafe impl ::std::marker::Sync for CoreWebView2Controller {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CoreWebView2ControllerWindowReference(::windows::runtime::IInspectable);
impl CoreWebView2ControllerWindowReference {
    pub fn WindowHandle(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "_Core")]
    pub fn CoreWindow(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Core::CoreWindow> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Core::CoreWindow>(result__)
        }
    }
    pub fn CreateFromWindowHandle(
        windowhandle: u64,
    ) -> ::windows::runtime::Result<CoreWebView2ControllerWindowReference> {
        Self::ICoreWebView2ControllerWindowReferenceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                windowhandle,
                &mut result__,
            )
            .from_abi::<CoreWebView2ControllerWindowReference>(result__)
        })
    }
    #[cfg(feature = "_Core")]
    pub fn CreateFromCoreWindow<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::UI::Core::CoreWindow,
        >,
    >(
        corewindow: Param0,
    ) -> ::windows::runtime::Result<CoreWebView2ControllerWindowReference> {
        Self::ICoreWebView2ControllerWindowReferenceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                corewindow.into_param().abi(),
                &mut result__,
            )
            .from_abi::<CoreWebView2ControllerWindowReference>(result__)
        })
    }
    pub fn ICoreWebView2ControllerWindowReferenceStatics<
        R,
        F: FnOnce(&ICoreWebView2ControllerWindowReferenceStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            CoreWebView2ControllerWindowReference,
            ICoreWebView2ControllerWindowReferenceStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2ControllerWindowReference {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2ControllerWindowReference;{0feddad4-48a3-5cc4-9f61-e7adfd1e9c76})" ) ;
}
unsafe impl ::windows::runtime::Interface for CoreWebView2ControllerWindowReference {
    type Vtable = ICoreWebView2ControllerWindowReference_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        267246292,
        18595,
        23748,
        [159, 97, 231, 173, 253, 30, 156, 118],
    );
}
impl ::windows::runtime::RuntimeName for CoreWebView2ControllerWindowReference {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2ControllerWindowReference";
}
impl ::std::convert::From<CoreWebView2ControllerWindowReference> for ::windows::runtime::IUnknown {
    fn from(value: CoreWebView2ControllerWindowReference) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreWebView2ControllerWindowReference> for ::windows::runtime::IUnknown {
    fn from(value: &CoreWebView2ControllerWindowReference) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CoreWebView2ControllerWindowReference
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CoreWebView2ControllerWindowReference
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CoreWebView2ControllerWindowReference>
    for ::windows::runtime::IInspectable
{
    fn from(value: CoreWebView2ControllerWindowReference) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreWebView2ControllerWindowReference>
    for ::windows::runtime::IInspectable
{
    fn from(value: &CoreWebView2ControllerWindowReference) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CoreWebView2ControllerWindowReference
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CoreWebView2ControllerWindowReference
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreWebView2ControllerWindowReference {}
unsafe impl ::std::marker::Sync for CoreWebView2ControllerWindowReference {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CoreWebView2Cookie(::windows::runtime::IInspectable);
impl CoreWebView2Cookie {
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
    pub fn Domain(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn Path(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn Expires(&self) -> ::windows::runtime::Result<f64> {
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
    pub fn SetExpires(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsHttpOnly(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn SetIsHttpOnly(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn SameSite(&self) -> ::windows::runtime::Result<CoreWebView2CookieSameSiteKind> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2CookieSameSiteKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2CookieSameSiteKind>(result__)
        }
    }
    pub fn SetSameSite(
        &self,
        value: CoreWebView2CookieSameSiteKind,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsSecure(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsSecure(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsSession(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2Cookie {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2Cookie;{52f670fe-8ca2-5aad-aedb-25f7903b7038})" ) ;
}
unsafe impl ::windows::runtime::Interface for CoreWebView2Cookie {
    type Vtable = ICoreWebView2Cookie_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1391882494,
        36002,
        23213,
        [174, 219, 37, 247, 144, 59, 112, 56],
    );
}
impl ::windows::runtime::RuntimeName for CoreWebView2Cookie {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2Cookie";
}
impl ::std::convert::From<CoreWebView2Cookie> for ::windows::runtime::IUnknown {
    fn from(value: CoreWebView2Cookie) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreWebView2Cookie> for ::windows::runtime::IUnknown {
    fn from(value: &CoreWebView2Cookie) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CoreWebView2Cookie {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &CoreWebView2Cookie {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CoreWebView2Cookie> for ::windows::runtime::IInspectable {
    fn from(value: CoreWebView2Cookie) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreWebView2Cookie> for ::windows::runtime::IInspectable {
    fn from(value: &CoreWebView2Cookie) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CoreWebView2Cookie
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CoreWebView2Cookie
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreWebView2Cookie {}
unsafe impl ::std::marker::Sync for CoreWebView2Cookie {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CoreWebView2CookieManager(::windows::runtime::IInspectable);
impl CoreWebView2CookieManager {
    pub fn CreateCookie<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        name: Param0,
        value: Param1,
        domain: Param2,
        path: Param3,
    ) -> ::windows::runtime::Result<CoreWebView2Cookie> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                name.into_param().abi(),
                value.into_param().abi(),
                domain.into_param().abi(),
                path.into_param().abi(),
                &mut result__,
            )
            .from_abi::<CoreWebView2Cookie>(result__)
        }
    }
    pub fn CopyCookie<'a, Param0: ::windows::runtime::IntoParam<'a, CoreWebView2Cookie>>(
        &self,
        cookieparam: Param0,
    ) -> ::windows::runtime::Result<CoreWebView2Cookie> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                cookieparam.into_param().abi(),
                &mut result__,
            )
            .from_abi::<CoreWebView2Cookie>(result__)
        }
    }
    pub fn AddOrUpdateCookie<'a, Param0: ::windows::runtime::IntoParam<'a, CoreWebView2Cookie>>(
        &self,
        cookie: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                cookie.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn DeleteCookie<'a, Param0: ::windows::runtime::IntoParam<'a, CoreWebView2Cookie>>(
        &self,
        cookie: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                cookie.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn DeleteCookies<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        name: Param0,
        uri: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                name.into_param().abi(),
                uri.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn DeleteCookiesWithDomainAndPath<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        name: Param0,
        domain: Param1,
        path: Param2,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                name.into_param().abi(),
                domain.into_param().abi(),
                path.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn DeleteAllCookies(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2CookieManager {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2CookieManager;{4098f516-adca-5563-aaa5-d7affd847aa3})" ) ;
}
unsafe impl ::windows::runtime::Interface for CoreWebView2CookieManager {
    type Vtable = ICoreWebView2CookieManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1083766038,
        44490,
        21859,
        [170, 165, 215, 175, 253, 132, 122, 163],
    );
}
impl ::windows::runtime::RuntimeName for CoreWebView2CookieManager {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2CookieManager";
}
impl ::std::convert::From<CoreWebView2CookieManager> for ::windows::runtime::IUnknown {
    fn from(value: CoreWebView2CookieManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreWebView2CookieManager> for ::windows::runtime::IUnknown {
    fn from(value: &CoreWebView2CookieManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CoreWebView2CookieManager
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CoreWebView2CookieManager
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CoreWebView2CookieManager> for ::windows::runtime::IInspectable {
    fn from(value: CoreWebView2CookieManager) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreWebView2CookieManager> for ::windows::runtime::IInspectable {
    fn from(value: &CoreWebView2CookieManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CoreWebView2CookieManager
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CoreWebView2CookieManager
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreWebView2CookieManager {}
unsafe impl ::std::marker::Sync for CoreWebView2CookieManager {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CoreWebView2CookieSameSiteKind(pub i32);
impl CoreWebView2CookieSameSiteKind {
    pub const None: CoreWebView2CookieSameSiteKind = CoreWebView2CookieSameSiteKind(0i32);
    pub const Lax: CoreWebView2CookieSameSiteKind = CoreWebView2CookieSameSiteKind(1i32);
    pub const Strict: CoreWebView2CookieSameSiteKind = CoreWebView2CookieSameSiteKind(2i32);
}
impl ::std::convert::From<i32> for CoreWebView2CookieSameSiteKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CoreWebView2CookieSameSiteKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2CookieSameSiteKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2CookieSameSiteKind;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CoreWebView2DOMContentLoadedEventArgs(::windows::runtime::IInspectable);
impl CoreWebView2DOMContentLoadedEventArgs {
    pub fn NavigationId(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u64>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2DOMContentLoadedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2DOMContentLoadedEventArgs;{c474d0a3-24ac-59fc-b78b-da7562a6a052})" ) ;
}
unsafe impl ::windows::runtime::Interface for CoreWebView2DOMContentLoadedEventArgs {
    type Vtable = ICoreWebView2DOMContentLoadedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3295989923,
        9388,
        23036,
        [183, 139, 218, 117, 98, 166, 160, 82],
    );
}
impl ::windows::runtime::RuntimeName for CoreWebView2DOMContentLoadedEventArgs {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2DOMContentLoadedEventArgs";
}
impl ::std::convert::From<CoreWebView2DOMContentLoadedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: CoreWebView2DOMContentLoadedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreWebView2DOMContentLoadedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &CoreWebView2DOMContentLoadedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CoreWebView2DOMContentLoadedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CoreWebView2DOMContentLoadedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CoreWebView2DOMContentLoadedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: CoreWebView2DOMContentLoadedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreWebView2DOMContentLoadedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &CoreWebView2DOMContentLoadedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CoreWebView2DOMContentLoadedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CoreWebView2DOMContentLoadedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreWebView2DOMContentLoadedEventArgs {}
unsafe impl ::std::marker::Sync for CoreWebView2DOMContentLoadedEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CoreWebView2DevToolsProtocolEventReceivedEventArgs(::windows::runtime::IInspectable);
impl CoreWebView2DevToolsProtocolEventReceivedEventArgs {
    pub fn ParameterObjectAsJson(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2DevToolsProtocolEventReceivedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2DevToolsProtocolEventReceivedEventArgs;{b6a4b41d-fd18-59fa-923a-c57555d960ce})" ) ;
}
unsafe impl ::windows::runtime::Interface for CoreWebView2DevToolsProtocolEventReceivedEventArgs {
    type Vtable = ICoreWebView2DevToolsProtocolEventReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3064247325,
        64792,
        23034,
        [146, 58, 197, 117, 85, 217, 96, 206],
    );
}
impl ::windows::runtime::RuntimeName for CoreWebView2DevToolsProtocolEventReceivedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2DevToolsProtocolEventReceivedEventArgs";
}
impl ::std::convert::From<CoreWebView2DevToolsProtocolEventReceivedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: CoreWebView2DevToolsProtocolEventReceivedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreWebView2DevToolsProtocolEventReceivedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &CoreWebView2DevToolsProtocolEventReceivedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CoreWebView2DevToolsProtocolEventReceivedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CoreWebView2DevToolsProtocolEventReceivedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CoreWebView2DevToolsProtocolEventReceivedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: CoreWebView2DevToolsProtocolEventReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreWebView2DevToolsProtocolEventReceivedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &CoreWebView2DevToolsProtocolEventReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CoreWebView2DevToolsProtocolEventReceivedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CoreWebView2DevToolsProtocolEventReceivedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreWebView2DevToolsProtocolEventReceivedEventArgs {}
unsafe impl ::std::marker::Sync for CoreWebView2DevToolsProtocolEventReceivedEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CoreWebView2DevToolsProtocolEventReceiver(::windows::runtime::IInspectable);
impl CoreWebView2DevToolsProtocolEventReceiver {
    #[cfg(feature = "undation")]
    pub fn DevToolsProtocolEventReceived<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2DevToolsProtocolEventReceivedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = self;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::EventRegistrationToken>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveDevToolsProtocolEventReceived<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2DevToolsProtocolEventReceiver {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2DevToolsProtocolEventReceiver;{b2a2be79-65fc-5537-8715-3d92bf31090b})" ) ;
}
unsafe impl ::windows::runtime::Interface for CoreWebView2DevToolsProtocolEventReceiver {
    type Vtable = ICoreWebView2DevToolsProtocolEventReceiver_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2997010041,
        26108,
        21815,
        [135, 21, 61, 146, 191, 49, 9, 11],
    );
}
impl ::windows::runtime::RuntimeName for CoreWebView2DevToolsProtocolEventReceiver {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2DevToolsProtocolEventReceiver";
}
impl ::std::convert::From<CoreWebView2DevToolsProtocolEventReceiver>
    for ::windows::runtime::IUnknown
{
    fn from(value: CoreWebView2DevToolsProtocolEventReceiver) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreWebView2DevToolsProtocolEventReceiver>
    for ::windows::runtime::IUnknown
{
    fn from(value: &CoreWebView2DevToolsProtocolEventReceiver) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CoreWebView2DevToolsProtocolEventReceiver
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CoreWebView2DevToolsProtocolEventReceiver
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CoreWebView2DevToolsProtocolEventReceiver>
    for ::windows::runtime::IInspectable
{
    fn from(value: CoreWebView2DevToolsProtocolEventReceiver) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreWebView2DevToolsProtocolEventReceiver>
    for ::windows::runtime::IInspectable
{
    fn from(value: &CoreWebView2DevToolsProtocolEventReceiver) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CoreWebView2DevToolsProtocolEventReceiver
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CoreWebView2DevToolsProtocolEventReceiver
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreWebView2DevToolsProtocolEventReceiver {}
unsafe impl ::std::marker::Sync for CoreWebView2DevToolsProtocolEventReceiver {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CoreWebView2DownloadInterruptReason(pub i32);
impl CoreWebView2DownloadInterruptReason {
    pub const None: CoreWebView2DownloadInterruptReason = CoreWebView2DownloadInterruptReason(0i32);
    pub const FileFailed: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(1i32);
    pub const FileAccessDenied: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(2i32);
    pub const FileNoSpace: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(3i32);
    pub const FileNameTooLong: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(4i32);
    pub const FileTooLarge: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(5i32);
    pub const FileMalicious: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(6i32);
    pub const FileTransientError: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(7i32);
    pub const FileBlockedByPolicy: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(8i32);
    pub const FileSecurityCheckFailed: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(9i32);
    pub const FileTooShort: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(10i32);
    pub const FileHashMismatch: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(11i32);
    pub const NetworkFailed: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(12i32);
    pub const NetworkTimeout: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(13i32);
    pub const NetworkDisconnected: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(14i32);
    pub const NetworkServerDown: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(15i32);
    pub const NetworkInvalidRequest: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(16i32);
    pub const ServerFailed: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(17i32);
    pub const ServerNoRange: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(18i32);
    pub const ServerBadContent: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(19i32);
    pub const ServerUnauthorized: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(20i32);
    pub const ServerCertificateProblem: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(21i32);
    pub const ServerForbidden: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(22i32);
    pub const ServerUnexpectedResponse: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(23i32);
    pub const ServerContentLengthMismatch: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(24i32);
    pub const ServerCrossOriginRedirect: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(25i32);
    pub const UserCanceled: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(26i32);
    pub const UserShutdown: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(27i32);
    pub const UserPaused: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(28i32);
    pub const DownloadProcessCrashed: CoreWebView2DownloadInterruptReason =
        CoreWebView2DownloadInterruptReason(29i32);
}
impl ::std::convert::From<i32> for CoreWebView2DownloadInterruptReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CoreWebView2DownloadInterruptReason {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2DownloadInterruptReason {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2DownloadInterruptReason;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CoreWebView2DownloadOperation(::windows::runtime::IInspectable);
impl CoreWebView2DownloadOperation {
    pub fn Uri(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn ContentDisposition(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn MimeType(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn TotalBytesToReceive(&self) -> ::windows::runtime::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    pub fn BytesReceived(&self) -> ::windows::runtime::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    pub fn EstimatedEndTime(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn ResultFilePath(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn State(&self) -> ::windows::runtime::Result<CoreWebView2DownloadState> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2DownloadState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2DownloadState>(result__)
        }
    }
    pub fn InterruptReason(
        &self,
    ) -> ::windows::runtime::Result<CoreWebView2DownloadInterruptReason> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2DownloadInterruptReason = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2DownloadInterruptReason>(result__)
        }
    }
    pub fn CanResume(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn BytesReceivedChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                CoreWebView2DownloadOperation,
                ::windows::runtime::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = self;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::EventRegistrationToken>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveBytesReceivedChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn EstimatedEndTimeChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                CoreWebView2DownloadOperation,
                ::windows::runtime::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = self;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::EventRegistrationToken>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveEstimatedEndTimeChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn StateChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                CoreWebView2DownloadOperation,
                ::windows::runtime::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = self;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::EventRegistrationToken>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveStateChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).21)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Cancel(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Pause(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Resume(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2DownloadOperation {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2DownloadOperation;{afe73e6b-e760-5a06-9bf6-1e743c13cd2d})" ) ;
}
unsafe impl ::windows::runtime::Interface for CoreWebView2DownloadOperation {
    type Vtable = ICoreWebView2DownloadOperation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2951167595,
        59232,
        23046,
        [155, 246, 30, 116, 60, 19, 205, 45],
    );
}
impl ::windows::runtime::RuntimeName for CoreWebView2DownloadOperation {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2DownloadOperation";
}
impl ::std::convert::From<CoreWebView2DownloadOperation> for ::windows::runtime::IUnknown {
    fn from(value: CoreWebView2DownloadOperation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreWebView2DownloadOperation> for ::windows::runtime::IUnknown {
    fn from(value: &CoreWebView2DownloadOperation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CoreWebView2DownloadOperation
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CoreWebView2DownloadOperation
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CoreWebView2DownloadOperation> for ::windows::runtime::IInspectable {
    fn from(value: CoreWebView2DownloadOperation) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreWebView2DownloadOperation> for ::windows::runtime::IInspectable {
    fn from(value: &CoreWebView2DownloadOperation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CoreWebView2DownloadOperation
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CoreWebView2DownloadOperation
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreWebView2DownloadOperation {}
unsafe impl ::std::marker::Sync for CoreWebView2DownloadOperation {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CoreWebView2DownloadStartingEventArgs(::windows::runtime::IInspectable);
impl CoreWebView2DownloadStartingEventArgs {
    pub fn DownloadOperation(&self) -> ::windows::runtime::Result<CoreWebView2DownloadOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2DownloadOperation>(result__)
        }
    }
    pub fn Cancel(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn SetCancel(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn ResultFilePath(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn SetResultFilePath<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Handled(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn SetHandled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn GetDeferral(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2DownloadStartingEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2DownloadStartingEventArgs;{45d982ba-9256-5b35-b023-26a438599110})" ) ;
}
unsafe impl ::windows::runtime::Interface for CoreWebView2DownloadStartingEventArgs {
    type Vtable = ICoreWebView2DownloadStartingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1171882682,
        37462,
        23349,
        [176, 35, 38, 164, 56, 89, 145, 16],
    );
}
impl ::windows::runtime::RuntimeName for CoreWebView2DownloadStartingEventArgs {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2DownloadStartingEventArgs";
}
impl ::std::convert::From<CoreWebView2DownloadStartingEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: CoreWebView2DownloadStartingEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreWebView2DownloadStartingEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &CoreWebView2DownloadStartingEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CoreWebView2DownloadStartingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CoreWebView2DownloadStartingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CoreWebView2DownloadStartingEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: CoreWebView2DownloadStartingEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreWebView2DownloadStartingEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &CoreWebView2DownloadStartingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CoreWebView2DownloadStartingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CoreWebView2DownloadStartingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreWebView2DownloadStartingEventArgs {}
unsafe impl ::std::marker::Sync for CoreWebView2DownloadStartingEventArgs {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CoreWebView2DownloadState(pub i32);
impl CoreWebView2DownloadState {
    pub const InProgress: CoreWebView2DownloadState = CoreWebView2DownloadState(0i32);
    pub const Interrupted: CoreWebView2DownloadState = CoreWebView2DownloadState(1i32);
    pub const Completed: CoreWebView2DownloadState = CoreWebView2DownloadState(2i32);
}
impl ::std::convert::From<i32> for CoreWebView2DownloadState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CoreWebView2DownloadState {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2DownloadState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2DownloadState;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CoreWebView2Environment(::windows::runtime::IInspectable);
impl CoreWebView2Environment {
    pub fn BrowserVersionString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    #[cfg(feature = "undation")]
    pub fn NewBrowserVersionAvailable<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                CoreWebView2Environment,
                ::windows::runtime::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = self;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::EventRegistrationToken>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveNewBrowserVersionAvailable<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn CreateCoreWebView2ControllerAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, CoreWebView2ControllerWindowReference>,
    >(
        &self,
        parentwindow: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::IAsyncOperation<CoreWebView2Controller>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .9 ) ( :: std :: mem :: transmute_copy ( this ) , parentwindow . into_param ( ) . abi ( ) , & mut result__ ) . from_abi :: < super::super::super::super::Windows::Foundation:: IAsyncOperation :: < CoreWebView2Controller > > ( result__ )
        }
    }
    #[cfg(feature = "orage_Streams")]
    pub fn CreateWebResourceResponse<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Storage::Streams::IRandomAccessStream,
        >,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        content: Param0,
        statuscode: i32,
        reasonphrase: Param2,
        headers: Param3,
    ) -> ::windows::runtime::Result<CoreWebView2WebResourceResponse> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                content.into_param().abi(),
                statuscode,
                reasonphrase.into_param().abi(),
                headers.into_param().abi(),
                &mut result__,
            )
            .from_abi::<CoreWebView2WebResourceResponse>(result__)
        }
    }
    #[cfg(feature = "orage_Streams")]
    pub fn CreateWebResourceRequest<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param2: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Storage::Streams::IRandomAccessStream,
        >,
        Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        uri: Param0,
        method: Param1,
        postdata: Param2,
        headers: Param3,
    ) -> ::windows::runtime::Result<CoreWebView2WebResourceRequest> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Environment2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                uri.into_param().abi(),
                method.into_param().abi(),
                postdata.into_param().abi(),
                headers.into_param().abi(),
                &mut result__,
            )
            .from_abi::<CoreWebView2WebResourceRequest>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn CreateCoreWebView2CompositionControllerAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, CoreWebView2ControllerWindowReference>,
    >(
        &self,
        parentwindow: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::IAsyncOperation<
            CoreWebView2CompositionController,
        >,
    > {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Environment3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                parentwindow.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::IAsyncOperation<
                CoreWebView2CompositionController,
            >>(result__)
        }
    }
    pub fn CreateCoreWebView2PointerInfo(
        &self,
    ) -> ::windows::runtime::Result<CoreWebView2PointerInfo> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Environment3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2PointerInfo>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn CreateAsync() -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::IAsyncOperation<CoreWebView2Environment>,
    > {
        Self::ICoreWebView2EnvironmentStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::IAsyncOperation<
                CoreWebView2Environment,
            >>(result__)
        })
    }
    #[cfg(feature = "undation")]
    pub fn CreateWithOptionsAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param2: ::windows::runtime::IntoParam<'a, CoreWebView2EnvironmentOptions>,
    >(
        browserexecutablefolder: Param0,
        userdatafolder: Param1,
        options: Param2,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::IAsyncOperation<CoreWebView2Environment>,
    > {
        Self::ICoreWebView2EnvironmentStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                browserexecutablefolder.into_param().abi(),
                userdatafolder.into_param().abi(),
                options.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::IAsyncOperation<
                CoreWebView2Environment,
            >>(result__)
        })
    }
    pub fn GetAvailableBrowserVersionString(
    ) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::ICoreWebView2EnvironmentStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn GetAvailableBrowserVersionString2<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        browserexecutablefolder: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::ICoreWebView2EnvironmentStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                browserexecutablefolder.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn CompareBrowserVersionString<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        browserversionstring1: Param0,
        browserversionstring2: Param1,
    ) -> ::windows::runtime::Result<i32> {
        Self::ICoreWebView2EnvironmentStatics(|this| unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                browserversionstring1.into_param().abi(),
                browserversionstring2.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        })
    }
    pub fn ICoreWebView2EnvironmentStatics<
        R,
        F: FnOnce(&ICoreWebView2EnvironmentStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            CoreWebView2Environment,
            ICoreWebView2EnvironmentStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2Environment {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2Environment;{d8cc7831-b783-556b-b9ce-899c1e95d585})" ) ;
}
unsafe impl ::windows::runtime::Interface for CoreWebView2Environment {
    type Vtable = ICoreWebView2Environment_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3637278769,
        46979,
        21867,
        [185, 206, 137, 156, 30, 149, 213, 133],
    );
}
impl ::windows::runtime::RuntimeName for CoreWebView2Environment {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2Environment";
}
impl ::std::convert::From<CoreWebView2Environment> for ::windows::runtime::IUnknown {
    fn from(value: CoreWebView2Environment) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreWebView2Environment> for ::windows::runtime::IUnknown {
    fn from(value: &CoreWebView2Environment) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CoreWebView2Environment
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CoreWebView2Environment
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CoreWebView2Environment> for ::windows::runtime::IInspectable {
    fn from(value: CoreWebView2Environment) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreWebView2Environment> for ::windows::runtime::IInspectable {
    fn from(value: &CoreWebView2Environment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CoreWebView2Environment
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CoreWebView2Environment
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreWebView2Environment {}
unsafe impl ::std::marker::Sync for CoreWebView2Environment {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CoreWebView2EnvironmentOptions(::windows::runtime::IInspectable);
impl CoreWebView2EnvironmentOptions {
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
            CoreWebView2EnvironmentOptions,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn AdditionalBrowserArguments(
        &self,
    ) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn SetAdditionalBrowserArguments<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
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
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn SetLanguage<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
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
    pub fn TargetCompatibleBrowserVersion(
        &self,
    ) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn SetTargetCompatibleBrowserVersion<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
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
    pub fn AllowSingleSignOnUsingOSPrimaryAccount(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn SetAllowSingleSignOnUsingOSPrimaryAccount(
        &self,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2EnvironmentOptions {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2EnvironmentOptions;{25d6dc39-0062-5735-8b09-a6f535f19e97})" ) ;
}
unsafe impl ::windows::runtime::Interface for CoreWebView2EnvironmentOptions {
    type Vtable = ICoreWebView2EnvironmentOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        634838073,
        98,
        22325,
        [139, 9, 166, 245, 53, 241, 158, 151],
    );
}
impl ::windows::runtime::RuntimeName for CoreWebView2EnvironmentOptions {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2EnvironmentOptions";
}
impl ::std::convert::From<CoreWebView2EnvironmentOptions> for ::windows::runtime::IUnknown {
    fn from(value: CoreWebView2EnvironmentOptions) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreWebView2EnvironmentOptions> for ::windows::runtime::IUnknown {
    fn from(value: &CoreWebView2EnvironmentOptions) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CoreWebView2EnvironmentOptions
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CoreWebView2EnvironmentOptions
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CoreWebView2EnvironmentOptions> for ::windows::runtime::IInspectable {
    fn from(value: CoreWebView2EnvironmentOptions) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreWebView2EnvironmentOptions> for ::windows::runtime::IInspectable {
    fn from(value: &CoreWebView2EnvironmentOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CoreWebView2EnvironmentOptions
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CoreWebView2EnvironmentOptions
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreWebView2EnvironmentOptions {}
unsafe impl ::std::marker::Sync for CoreWebView2EnvironmentOptions {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CoreWebView2Frame(::windows::runtime::IInspectable);
impl CoreWebView2Frame {
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
    #[cfg(feature = "undation")]
    pub fn NameChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                CoreWebView2Frame,
                ::windows::runtime::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = self;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::EventRegistrationToken>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveNameChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn Destroyed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                CoreWebView2Frame,
                ::windows::runtime::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = self;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::EventRegistrationToken>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveDestroyed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
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
    pub fn RemoveHostObjectFromScript<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        name: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                name.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IsDestroyed(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2Frame {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.Web.WebView2.Core.CoreWebView2Frame;{02ffcbf9-19e7-5bb8-8273-346420fb1503})",
    );
}
unsafe impl ::windows::runtime::Interface for CoreWebView2Frame {
    type Vtable = ICoreWebView2Frame_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        50318329,
        6631,
        23480,
        [130, 115, 52, 100, 32, 251, 21, 3],
    );
}
impl ::windows::runtime::RuntimeName for CoreWebView2Frame {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2Frame";
}
impl ::std::convert::From<CoreWebView2Frame> for ::windows::runtime::IUnknown {
    fn from(value: CoreWebView2Frame) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreWebView2Frame> for ::windows::runtime::IUnknown {
    fn from(value: &CoreWebView2Frame) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CoreWebView2Frame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &CoreWebView2Frame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CoreWebView2Frame> for ::windows::runtime::IInspectable {
    fn from(value: CoreWebView2Frame) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreWebView2Frame> for ::windows::runtime::IInspectable {
    fn from(value: &CoreWebView2Frame) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CoreWebView2Frame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CoreWebView2Frame
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreWebView2Frame {}
unsafe impl ::std::marker::Sync for CoreWebView2Frame {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CoreWebView2FrameCreatedEventArgs(::windows::runtime::IInspectable);
impl CoreWebView2FrameCreatedEventArgs {
    pub fn Frame(&self) -> ::windows::runtime::Result<CoreWebView2Frame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2Frame>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2FrameCreatedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2FrameCreatedEventArgs;{527b01b8-fc6d-5543-8dce-96cdfdb32c81})" ) ;
}
unsafe impl ::windows::runtime::Interface for CoreWebView2FrameCreatedEventArgs {
    type Vtable = ICoreWebView2FrameCreatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1383793080,
        64621,
        21827,
        [141, 206, 150, 205, 253, 179, 44, 129],
    );
}
impl ::windows::runtime::RuntimeName for CoreWebView2FrameCreatedEventArgs {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2FrameCreatedEventArgs";
}
impl ::std::convert::From<CoreWebView2FrameCreatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: CoreWebView2FrameCreatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreWebView2FrameCreatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &CoreWebView2FrameCreatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CoreWebView2FrameCreatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CoreWebView2FrameCreatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CoreWebView2FrameCreatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: CoreWebView2FrameCreatedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreWebView2FrameCreatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &CoreWebView2FrameCreatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CoreWebView2FrameCreatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CoreWebView2FrameCreatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreWebView2FrameCreatedEventArgs {}
unsafe impl ::std::marker::Sync for CoreWebView2FrameCreatedEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CoreWebView2FrameInfo(::windows::runtime::IInspectable);
impl CoreWebView2FrameInfo {
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
    pub fn Source(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2FrameInfo {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2FrameInfo;{f9b82e06-73f3-513b-bc2c-445ddedba976})" ) ;
}
unsafe impl ::windows::runtime::Interface for CoreWebView2FrameInfo {
    type Vtable = ICoreWebView2FrameInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4189597190,
        29683,
        20795,
        [188, 44, 68, 93, 222, 219, 169, 118],
    );
}
impl ::windows::runtime::RuntimeName for CoreWebView2FrameInfo {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2FrameInfo";
}
impl ::std::convert::From<CoreWebView2FrameInfo> for ::windows::runtime::IUnknown {
    fn from(value: CoreWebView2FrameInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreWebView2FrameInfo> for ::windows::runtime::IUnknown {
    fn from(value: &CoreWebView2FrameInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CoreWebView2FrameInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CoreWebView2FrameInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CoreWebView2FrameInfo> for ::windows::runtime::IInspectable {
    fn from(value: CoreWebView2FrameInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreWebView2FrameInfo> for ::windows::runtime::IInspectable {
    fn from(value: &CoreWebView2FrameInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CoreWebView2FrameInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CoreWebView2FrameInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreWebView2FrameInfo {}
unsafe impl ::std::marker::Sync for CoreWebView2FrameInfo {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CoreWebView2HostResourceAccessKind(pub i32);
impl CoreWebView2HostResourceAccessKind {
    pub const Deny: CoreWebView2HostResourceAccessKind = CoreWebView2HostResourceAccessKind(0i32);
    pub const Allow: CoreWebView2HostResourceAccessKind = CoreWebView2HostResourceAccessKind(1i32);
    pub const DenyCors: CoreWebView2HostResourceAccessKind =
        CoreWebView2HostResourceAccessKind(2i32);
}
impl ::std::convert::From<i32> for CoreWebView2HostResourceAccessKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CoreWebView2HostResourceAccessKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2HostResourceAccessKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2HostResourceAccessKind;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CoreWebView2HttpHeadersCollectionIterator(::windows::runtime::IInspectable);
impl CoreWebView2HttpHeadersCollectionIterator {
    #[cfg(feature = "undation_Collections")]
    pub fn Current(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::Collections::IKeyValuePair<
            ::windows::runtime::HSTRING,
            ::windows::runtime::HSTRING,
        >,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::super::super::Windows::Foundation::Collections::IIterator<
                super::super::super::super::Windows::Foundation::Collections::IKeyValuePair<
                    ::windows::runtime::HSTRING,
                    ::windows::runtime::HSTRING,
                >,
            >,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Collections::IKeyValuePair<
                ::windows::runtime::HSTRING,
                ::windows::runtime::HSTRING,
            >>(result__)
        }
    }
    #[cfg(feature = "undation_Collections")]
    pub fn HasCurrent(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::super::super::Windows::Foundation::Collections::IIterator<
                super::super::super::super::Windows::Foundation::Collections::IKeyValuePair<
                    ::windows::runtime::HSTRING,
                    ::windows::runtime::HSTRING,
                >,
            >,
        >(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "undation_Collections")]
    pub fn MoveNext(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::super::super::Windows::Foundation::Collections::IIterator<
                super::super::super::super::Windows::Foundation::Collections::IKeyValuePair<
                    ::windows::runtime::HSTRING,
                    ::windows::runtime::HSTRING,
                >,
            >,
        >(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "undation_Collections")]
    pub fn GetMany(
        &self,
        items: &mut [<super::super::super::super::Windows::Foundation::Collections::IKeyValuePair<
            ::windows::runtime::HSTRING,
            ::windows::runtime::HSTRING,
        > as ::windows::runtime::Abi>::DefaultType],
    ) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::super::super::Windows::Foundation::Collections::IIterator<
                super::super::super::super::Windows::Foundation::Collections::IKeyValuePair<
                    ::windows::runtime::HSTRING,
                    ::windows::runtime::HSTRING,
                >,
            >,
        >(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                items.len() as u32,
                ::std::mem::transmute_copy(&items),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2HttpHeadersCollectionIterator {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2HttpHeadersCollectionIterator;{adf264ee-d980-5f48-a60e-8705de046608})" ) ;
}
unsafe impl ::windows::runtime::Interface for CoreWebView2HttpHeadersCollectionIterator {
    type Vtable = ICoreWebView2HttpHeadersCollectionIterator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2918343918,
        55680,
        24392,
        [166, 14, 135, 5, 222, 4, 102, 8],
    );
}
impl ::windows::runtime::RuntimeName for CoreWebView2HttpHeadersCollectionIterator {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2HttpHeadersCollectionIterator";
}
impl ::std::convert::From<CoreWebView2HttpHeadersCollectionIterator>
    for ::windows::runtime::IUnknown
{
    fn from(value: CoreWebView2HttpHeadersCollectionIterator) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreWebView2HttpHeadersCollectionIterator>
    for ::windows::runtime::IUnknown
{
    fn from(value: &CoreWebView2HttpHeadersCollectionIterator) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CoreWebView2HttpHeadersCollectionIterator
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CoreWebView2HttpHeadersCollectionIterator
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CoreWebView2HttpHeadersCollectionIterator>
    for ::windows::runtime::IInspectable
{
    fn from(value: CoreWebView2HttpHeadersCollectionIterator) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreWebView2HttpHeadersCollectionIterator>
    for ::windows::runtime::IInspectable
{
    fn from(value: &CoreWebView2HttpHeadersCollectionIterator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CoreWebView2HttpHeadersCollectionIterator
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CoreWebView2HttpHeadersCollectionIterator
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "undation_Collections")]
impl ::std::convert::TryFrom<CoreWebView2HttpHeadersCollectionIterator>
    for super::super::super::super::Windows::Foundation::Collections::IIterator<
        super::super::super::super::Windows::Foundation::Collections::IKeyValuePair<
            ::windows::runtime::HSTRING,
            ::windows::runtime::HSTRING,
        >,
    >
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: CoreWebView2HttpHeadersCollectionIterator,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "undation_Collections")]
impl ::std::convert::TryFrom<&CoreWebView2HttpHeadersCollectionIterator>
    for super::super::super::super::Windows::Foundation::Collections::IIterator<
        super::super::super::super::Windows::Foundation::Collections::IKeyValuePair<
            ::windows::runtime::HSTRING,
            ::windows::runtime::HSTRING,
        >,
    >
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &CoreWebView2HttpHeadersCollectionIterator,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "undation_Collections")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::super::super::Windows::Foundation::Collections::IIterator<
            super::super::super::super::Windows::Foundation::Collections::IKeyValuePair<
                ::windows::runtime::HSTRING,
                ::windows::runtime::HSTRING,
            >,
        >,
    > for CoreWebView2HttpHeadersCollectionIterator
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::super::super::Windows::Foundation::Collections::IIterator<
            super::super::super::super::Windows::Foundation::Collections::IKeyValuePair<
                ::windows::runtime::HSTRING,
                ::windows::runtime::HSTRING,
            >,
        >,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "undation_Collections")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::super::super::Windows::Foundation::Collections::IIterator<
            super::super::super::super::Windows::Foundation::Collections::IKeyValuePair<
                ::windows::runtime::HSTRING,
                ::windows::runtime::HSTRING,
            >,
        >,
    > for &CoreWebView2HttpHeadersCollectionIterator
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::super::super::Windows::Foundation::Collections::IIterator<
            super::super::super::super::Windows::Foundation::Collections::IKeyValuePair<
                ::windows::runtime::HSTRING,
                ::windows::runtime::HSTRING,
            >,
        >,
    > {
        ::std::convert::TryInto::<
            super::super::super::super::Windows::Foundation::Collections::IIterator<
                super::super::super::super::Windows::Foundation::Collections::IKeyValuePair<
                    ::windows::runtime::HSTRING,
                    ::windows::runtime::HSTRING,
                >,
            >,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for CoreWebView2HttpHeadersCollectionIterator {}
unsafe impl ::std::marker::Sync for CoreWebView2HttpHeadersCollectionIterator {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CoreWebView2HttpRequestHeaders(::windows::runtime::IInspectable);
impl CoreWebView2HttpRequestHeaders {
    pub fn GetHeader<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn GetHeaders<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        name: Param0,
    ) -> ::windows::runtime::Result<CoreWebView2HttpHeadersCollectionIterator> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<CoreWebView2HttpHeadersCollectionIterator>(result__)
        }
    }
    pub fn Contains<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetHeader<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        name: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                name.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveHeader<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        name: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                name.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation_Collections")]
    pub fn First(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::Collections::IIterator<
            super::super::super::super::Windows::Foundation::Collections::IKeyValuePair<
                ::windows::runtime::HSTRING,
                ::windows::runtime::HSTRING,
            >,
        >,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::super::super::Windows::Foundation::Collections::IIterable<
                super::super::super::super::Windows::Foundation::Collections::IKeyValuePair<
                    ::windows::runtime::HSTRING,
                    ::windows::runtime::HSTRING,
                >,
            >,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Collections::IIterator<
                super::super::super::super::Windows::Foundation::Collections::IKeyValuePair<
                    ::windows::runtime::HSTRING,
                    ::windows::runtime::HSTRING,
                >,
            >>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2HttpRequestHeaders {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2HttpRequestHeaders;{dc2226c7-3515-55bb-bcb2-57b78f86b91d})" ) ;
}
unsafe impl ::windows::runtime::Interface for CoreWebView2HttpRequestHeaders {
    type Vtable = ICoreWebView2HttpRequestHeaders_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3693225671,
        13589,
        21947,
        [188, 178, 87, 183, 143, 134, 185, 29],
    );
}
impl ::windows::runtime::RuntimeName for CoreWebView2HttpRequestHeaders {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2HttpRequestHeaders";
}
impl ::std::convert::From<CoreWebView2HttpRequestHeaders> for ::windows::runtime::IUnknown {
    fn from(value: CoreWebView2HttpRequestHeaders) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreWebView2HttpRequestHeaders> for ::windows::runtime::IUnknown {
    fn from(value: &CoreWebView2HttpRequestHeaders) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CoreWebView2HttpRequestHeaders
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CoreWebView2HttpRequestHeaders
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CoreWebView2HttpRequestHeaders> for ::windows::runtime::IInspectable {
    fn from(value: CoreWebView2HttpRequestHeaders) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreWebView2HttpRequestHeaders> for ::windows::runtime::IInspectable {
    fn from(value: &CoreWebView2HttpRequestHeaders) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CoreWebView2HttpRequestHeaders
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CoreWebView2HttpRequestHeaders
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "undation_Collections")]
impl ::std::convert::TryFrom<CoreWebView2HttpRequestHeaders>
    for super::super::super::super::Windows::Foundation::Collections::IIterable<
        super::super::super::super::Windows::Foundation::Collections::IKeyValuePair<
            ::windows::runtime::HSTRING,
            ::windows::runtime::HSTRING,
        >,
    >
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: CoreWebView2HttpRequestHeaders) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "undation_Collections")]
impl ::std::convert::TryFrom<&CoreWebView2HttpRequestHeaders>
    for super::super::super::super::Windows::Foundation::Collections::IIterable<
        super::super::super::super::Windows::Foundation::Collections::IKeyValuePair<
            ::windows::runtime::HSTRING,
            ::windows::runtime::HSTRING,
        >,
    >
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &CoreWebView2HttpRequestHeaders) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "undation_Collections")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::super::super::Windows::Foundation::Collections::IIterable<
            super::super::super::super::Windows::Foundation::Collections::IKeyValuePair<
                ::windows::runtime::HSTRING,
                ::windows::runtime::HSTRING,
            >,
        >,
    > for CoreWebView2HttpRequestHeaders
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::super::super::Windows::Foundation::Collections::IIterable<
            super::super::super::super::Windows::Foundation::Collections::IKeyValuePair<
                ::windows::runtime::HSTRING,
                ::windows::runtime::HSTRING,
            >,
        >,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "undation_Collections")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::super::super::Windows::Foundation::Collections::IIterable<
            super::super::super::super::Windows::Foundation::Collections::IKeyValuePair<
                ::windows::runtime::HSTRING,
                ::windows::runtime::HSTRING,
            >,
        >,
    > for &CoreWebView2HttpRequestHeaders
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::super::super::Windows::Foundation::Collections::IIterable<
            super::super::super::super::Windows::Foundation::Collections::IKeyValuePair<
                ::windows::runtime::HSTRING,
                ::windows::runtime::HSTRING,
            >,
        >,
    > {
        ::std::convert::TryInto::<
            super::super::super::super::Windows::Foundation::Collections::IIterable<
                super::super::super::super::Windows::Foundation::Collections::IKeyValuePair<
                    ::windows::runtime::HSTRING,
                    ::windows::runtime::HSTRING,
                >,
            >,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for CoreWebView2HttpRequestHeaders {}
unsafe impl ::std::marker::Sync for CoreWebView2HttpRequestHeaders {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for CoreWebView2HttpRequestHeaders {
    type Item = super::super::super::super::Windows::Foundation::Collections::IKeyValuePair<
        ::windows::runtime::HSTRING,
        ::windows::runtime::HSTRING,
    >;
    type IntoIter =
        super::super::super::super::Windows::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for &CoreWebView2HttpRequestHeaders {
    type Item = super::super::super::super::Windows::Foundation::Collections::IKeyValuePair<
        ::windows::runtime::HSTRING,
        ::windows::runtime::HSTRING,
    >;
    type IntoIter =
        super::super::super::super::Windows::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CoreWebView2HttpResponseHeaders(::windows::runtime::IInspectable);
impl CoreWebView2HttpResponseHeaders {
    pub fn AppendHeader<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        name: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                name.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Contains<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn GetHeader<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn GetHeaders<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        name: Param0,
    ) -> ::windows::runtime::Result<CoreWebView2HttpHeadersCollectionIterator> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<CoreWebView2HttpHeadersCollectionIterator>(result__)
        }
    }
    #[cfg(feature = "undation_Collections")]
    pub fn First(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::Collections::IIterator<
            super::super::super::super::Windows::Foundation::Collections::IKeyValuePair<
                ::windows::runtime::HSTRING,
                ::windows::runtime::HSTRING,
            >,
        >,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::super::super::Windows::Foundation::Collections::IIterable<
                super::super::super::super::Windows::Foundation::Collections::IKeyValuePair<
                    ::windows::runtime::HSTRING,
                    ::windows::runtime::HSTRING,
                >,
            >,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Collections::IIterator<
                super::super::super::super::Windows::Foundation::Collections::IKeyValuePair<
                    ::windows::runtime::HSTRING,
                    ::windows::runtime::HSTRING,
                >,
            >>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2HttpResponseHeaders {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2HttpResponseHeaders;{f3d383e9-747f-5574-8662-9a6b920cecd4})" ) ;
}
unsafe impl ::windows::runtime::Interface for CoreWebView2HttpResponseHeaders {
    type Vtable = ICoreWebView2HttpResponseHeaders_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4090725353,
        29823,
        21876,
        [134, 98, 154, 107, 146, 12, 236, 212],
    );
}
impl ::windows::runtime::RuntimeName for CoreWebView2HttpResponseHeaders {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2HttpResponseHeaders";
}
impl ::std::convert::From<CoreWebView2HttpResponseHeaders> for ::windows::runtime::IUnknown {
    fn from(value: CoreWebView2HttpResponseHeaders) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreWebView2HttpResponseHeaders> for ::windows::runtime::IUnknown {
    fn from(value: &CoreWebView2HttpResponseHeaders) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CoreWebView2HttpResponseHeaders
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CoreWebView2HttpResponseHeaders
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CoreWebView2HttpResponseHeaders> for ::windows::runtime::IInspectable {
    fn from(value: CoreWebView2HttpResponseHeaders) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreWebView2HttpResponseHeaders> for ::windows::runtime::IInspectable {
    fn from(value: &CoreWebView2HttpResponseHeaders) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CoreWebView2HttpResponseHeaders
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CoreWebView2HttpResponseHeaders
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "undation_Collections")]
impl ::std::convert::TryFrom<CoreWebView2HttpResponseHeaders>
    for super::super::super::super::Windows::Foundation::Collections::IIterable<
        super::super::super::super::Windows::Foundation::Collections::IKeyValuePair<
            ::windows::runtime::HSTRING,
            ::windows::runtime::HSTRING,
        >,
    >
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: CoreWebView2HttpResponseHeaders) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "undation_Collections")]
impl ::std::convert::TryFrom<&CoreWebView2HttpResponseHeaders>
    for super::super::super::super::Windows::Foundation::Collections::IIterable<
        super::super::super::super::Windows::Foundation::Collections::IKeyValuePair<
            ::windows::runtime::HSTRING,
            ::windows::runtime::HSTRING,
        >,
    >
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &CoreWebView2HttpResponseHeaders) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "undation_Collections")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::super::super::Windows::Foundation::Collections::IIterable<
            super::super::super::super::Windows::Foundation::Collections::IKeyValuePair<
                ::windows::runtime::HSTRING,
                ::windows::runtime::HSTRING,
            >,
        >,
    > for CoreWebView2HttpResponseHeaders
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::super::super::Windows::Foundation::Collections::IIterable<
            super::super::super::super::Windows::Foundation::Collections::IKeyValuePair<
                ::windows::runtime::HSTRING,
                ::windows::runtime::HSTRING,
            >,
        >,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "undation_Collections")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::super::super::Windows::Foundation::Collections::IIterable<
            super::super::super::super::Windows::Foundation::Collections::IKeyValuePair<
                ::windows::runtime::HSTRING,
                ::windows::runtime::HSTRING,
            >,
        >,
    > for &CoreWebView2HttpResponseHeaders
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::super::super::Windows::Foundation::Collections::IIterable<
            super::super::super::super::Windows::Foundation::Collections::IKeyValuePair<
                ::windows::runtime::HSTRING,
                ::windows::runtime::HSTRING,
            >,
        >,
    > {
        ::std::convert::TryInto::<
            super::super::super::super::Windows::Foundation::Collections::IIterable<
                super::super::super::super::Windows::Foundation::Collections::IKeyValuePair<
                    ::windows::runtime::HSTRING,
                    ::windows::runtime::HSTRING,
                >,
            >,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for CoreWebView2HttpResponseHeaders {}
unsafe impl ::std::marker::Sync for CoreWebView2HttpResponseHeaders {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for CoreWebView2HttpResponseHeaders {
    type Item = super::super::super::super::Windows::Foundation::Collections::IKeyValuePair<
        ::windows::runtime::HSTRING,
        ::windows::runtime::HSTRING,
    >;
    type IntoIter =
        super::super::super::super::Windows::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for &CoreWebView2HttpResponseHeaders {
    type Item = super::super::super::super::Windows::Foundation::Collections::IKeyValuePair<
        ::windows::runtime::HSTRING,
        ::windows::runtime::HSTRING,
    >;
    type IntoIter =
        super::super::super::super::Windows::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
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
pub struct CoreWebView2KeyEventKind(pub i32);
impl CoreWebView2KeyEventKind {
    pub const KeyDown: CoreWebView2KeyEventKind = CoreWebView2KeyEventKind(0i32);
    pub const KeyUp: CoreWebView2KeyEventKind = CoreWebView2KeyEventKind(1i32);
    pub const SystemKeyDown: CoreWebView2KeyEventKind = CoreWebView2KeyEventKind(2i32);
    pub const SystemKeyUp: CoreWebView2KeyEventKind = CoreWebView2KeyEventKind(3i32);
}
impl ::std::convert::From<i32> for CoreWebView2KeyEventKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CoreWebView2KeyEventKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2KeyEventKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2KeyEventKind;i4)",
    );
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
pub struct CoreWebView2MouseEventKind(pub i32);
impl CoreWebView2MouseEventKind {
    pub const HorizontalWheel: CoreWebView2MouseEventKind = CoreWebView2MouseEventKind(526i32);
    pub const LeftButtonDoubleClick: CoreWebView2MouseEventKind =
        CoreWebView2MouseEventKind(515i32);
    pub const LeftButtonDown: CoreWebView2MouseEventKind = CoreWebView2MouseEventKind(513i32);
    pub const LeftButtonUp: CoreWebView2MouseEventKind = CoreWebView2MouseEventKind(514i32);
    pub const Leave: CoreWebView2MouseEventKind = CoreWebView2MouseEventKind(675i32);
    pub const MiddleButtonDoubleClick: CoreWebView2MouseEventKind =
        CoreWebView2MouseEventKind(521i32);
    pub const MiddleButtonDown: CoreWebView2MouseEventKind = CoreWebView2MouseEventKind(519i32);
    pub const MiddleButtonUp: CoreWebView2MouseEventKind = CoreWebView2MouseEventKind(520i32);
    pub const Move: CoreWebView2MouseEventKind = CoreWebView2MouseEventKind(512i32);
    pub const RightButtonDoubleClick: CoreWebView2MouseEventKind =
        CoreWebView2MouseEventKind(518i32);
    pub const RightButtonDown: CoreWebView2MouseEventKind = CoreWebView2MouseEventKind(516i32);
    pub const RightButtonUp: CoreWebView2MouseEventKind = CoreWebView2MouseEventKind(517i32);
    pub const Wheel: CoreWebView2MouseEventKind = CoreWebView2MouseEventKind(522i32);
    pub const XButtonDoubleClick: CoreWebView2MouseEventKind = CoreWebView2MouseEventKind(525i32);
    pub const XButtonDown: CoreWebView2MouseEventKind = CoreWebView2MouseEventKind(523i32);
    pub const XButtonUp: CoreWebView2MouseEventKind = CoreWebView2MouseEventKind(524i32);
}
impl ::std::convert::From<i32> for CoreWebView2MouseEventKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CoreWebView2MouseEventKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2MouseEventKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2MouseEventKind;i4)",
    );
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
pub struct CoreWebView2MouseEventVirtualKeys(pub u32);
impl CoreWebView2MouseEventVirtualKeys {
    pub const None: CoreWebView2MouseEventVirtualKeys = CoreWebView2MouseEventVirtualKeys(0u32);
    pub const LeftButton: CoreWebView2MouseEventVirtualKeys =
        CoreWebView2MouseEventVirtualKeys(1u32);
    pub const RightButton: CoreWebView2MouseEventVirtualKeys =
        CoreWebView2MouseEventVirtualKeys(2u32);
    pub const Shift: CoreWebView2MouseEventVirtualKeys = CoreWebView2MouseEventVirtualKeys(4u32);
    pub const Control: CoreWebView2MouseEventVirtualKeys = CoreWebView2MouseEventVirtualKeys(8u32);
    pub const MiddleButton: CoreWebView2MouseEventVirtualKeys =
        CoreWebView2MouseEventVirtualKeys(16u32);
    pub const XButton1: CoreWebView2MouseEventVirtualKeys =
        CoreWebView2MouseEventVirtualKeys(32u32);
    pub const XButton2: CoreWebView2MouseEventVirtualKeys =
        CoreWebView2MouseEventVirtualKeys(64u32);
}
impl ::std::convert::From<u32> for CoreWebView2MouseEventVirtualKeys {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CoreWebView2MouseEventVirtualKeys {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2MouseEventVirtualKeys {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2MouseEventVirtualKeys;u4)",
    );
}
impl ::std::ops::BitOr for CoreWebView2MouseEventVirtualKeys {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CoreWebView2MouseEventVirtualKeys {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CoreWebView2MouseEventVirtualKeys {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CoreWebView2MouseEventVirtualKeys {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CoreWebView2MouseEventVirtualKeys {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
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
pub struct CoreWebView2MoveFocusReason(pub i32);
impl CoreWebView2MoveFocusReason {
    pub const Programmatic: CoreWebView2MoveFocusReason = CoreWebView2MoveFocusReason(0i32);
    pub const Next: CoreWebView2MoveFocusReason = CoreWebView2MoveFocusReason(1i32);
    pub const Previous: CoreWebView2MoveFocusReason = CoreWebView2MoveFocusReason(2i32);
}
impl ::std::convert::From<i32> for CoreWebView2MoveFocusReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CoreWebView2MoveFocusReason {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2MoveFocusReason {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2MoveFocusReason;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CoreWebView2MoveFocusRequestedEventArgs(::windows::runtime::IInspectable);
impl CoreWebView2MoveFocusRequestedEventArgs {
    pub fn Reason(&self) -> ::windows::runtime::Result<CoreWebView2MoveFocusReason> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2MoveFocusReason = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2MoveFocusReason>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn SetHandled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2MoveFocusRequestedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2MoveFocusRequestedEventArgs;{2e29103b-ecdd-5c1d-b288-3f066d608919})" ) ;
}
unsafe impl ::windows::runtime::Interface for CoreWebView2MoveFocusRequestedEventArgs {
    type Vtable = ICoreWebView2MoveFocusRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        774443067,
        60637,
        23581,
        [178, 136, 63, 6, 109, 96, 137, 25],
    );
}
impl ::windows::runtime::RuntimeName for CoreWebView2MoveFocusRequestedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2MoveFocusRequestedEventArgs";
}
impl ::std::convert::From<CoreWebView2MoveFocusRequestedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: CoreWebView2MoveFocusRequestedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreWebView2MoveFocusRequestedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &CoreWebView2MoveFocusRequestedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CoreWebView2MoveFocusRequestedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CoreWebView2MoveFocusRequestedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CoreWebView2MoveFocusRequestedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: CoreWebView2MoveFocusRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreWebView2MoveFocusRequestedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &CoreWebView2MoveFocusRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CoreWebView2MoveFocusRequestedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CoreWebView2MoveFocusRequestedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreWebView2MoveFocusRequestedEventArgs {}
unsafe impl ::std::marker::Sync for CoreWebView2MoveFocusRequestedEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CoreWebView2NavigationCompletedEventArgs(::windows::runtime::IInspectable);
impl CoreWebView2NavigationCompletedEventArgs {
    pub fn IsSuccess(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn WebErrorStatus(&self) -> ::windows::runtime::Result<CoreWebView2WebErrorStatus> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2WebErrorStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2WebErrorStatus>(result__)
        }
    }
    pub fn NavigationId(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u64>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2NavigationCompletedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2NavigationCompletedEventArgs;{4865e238-036a-5664-95a3-447ec44cf498})" ) ;
}
unsafe impl ::windows::runtime::Interface for CoreWebView2NavigationCompletedEventArgs {
    type Vtable = ICoreWebView2NavigationCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1214636600,
        874,
        22116,
        [149, 163, 68, 126, 196, 76, 244, 152],
    );
}
impl ::windows::runtime::RuntimeName for CoreWebView2NavigationCompletedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2NavigationCompletedEventArgs";
}
impl ::std::convert::From<CoreWebView2NavigationCompletedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: CoreWebView2NavigationCompletedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreWebView2NavigationCompletedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &CoreWebView2NavigationCompletedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CoreWebView2NavigationCompletedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CoreWebView2NavigationCompletedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CoreWebView2NavigationCompletedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: CoreWebView2NavigationCompletedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreWebView2NavigationCompletedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &CoreWebView2NavigationCompletedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CoreWebView2NavigationCompletedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CoreWebView2NavigationCompletedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreWebView2NavigationCompletedEventArgs {}
unsafe impl ::std::marker::Sync for CoreWebView2NavigationCompletedEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CoreWebView2NavigationStartingEventArgs(::windows::runtime::IInspectable);
impl CoreWebView2NavigationStartingEventArgs {
    pub fn Uri(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn IsUserInitiated(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn IsRedirected(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn RequestHeaders(&self) -> ::windows::runtime::Result<CoreWebView2HttpRequestHeaders> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2HttpRequestHeaders>(result__)
        }
    }
    pub fn Cancel(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn SetCancel(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn NavigationId(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u64>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2NavigationStartingEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2NavigationStartingEventArgs;{548d23d3-fea3-5616-bd05-ae08066c86d3})" ) ;
}
unsafe impl ::windows::runtime::Interface for CoreWebView2NavigationStartingEventArgs {
    type Vtable = ICoreWebView2NavigationStartingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1418535891,
        65187,
        22038,
        [189, 5, 174, 8, 6, 108, 134, 211],
    );
}
impl ::windows::runtime::RuntimeName for CoreWebView2NavigationStartingEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2NavigationStartingEventArgs";
}
impl ::std::convert::From<CoreWebView2NavigationStartingEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: CoreWebView2NavigationStartingEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreWebView2NavigationStartingEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &CoreWebView2NavigationStartingEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CoreWebView2NavigationStartingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CoreWebView2NavigationStartingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CoreWebView2NavigationStartingEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: CoreWebView2NavigationStartingEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreWebView2NavigationStartingEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &CoreWebView2NavigationStartingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CoreWebView2NavigationStartingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CoreWebView2NavigationStartingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreWebView2NavigationStartingEventArgs {}
unsafe impl ::std::marker::Sync for CoreWebView2NavigationStartingEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CoreWebView2NewWindowRequestedEventArgs(::windows::runtime::IInspectable);
impl CoreWebView2NewWindowRequestedEventArgs {
    pub fn Uri(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn NewWindow(&self) -> ::windows::runtime::Result<CoreWebView2> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2>(result__)
        }
    }
    pub fn SetNewWindow<'a, Param0: ::windows::runtime::IntoParam<'a, CoreWebView2>>(
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
    pub fn Handled(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn SetHandled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsUserInitiated(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn WindowFeatures(&self) -> ::windows::runtime::Result<CoreWebView2WindowFeatures> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2WindowFeatures>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn GetDeferral(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2NewWindowRequestedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2NewWindowRequestedEventArgs;{e6e013ba-aec8-532e-9ac9-1590af7b25ec})" ) ;
}
unsafe impl ::windows::runtime::Interface for CoreWebView2NewWindowRequestedEventArgs {
    type Vtable = ICoreWebView2NewWindowRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3873444794,
        44744,
        21294,
        [154, 201, 21, 144, 175, 123, 37, 236],
    );
}
impl ::windows::runtime::RuntimeName for CoreWebView2NewWindowRequestedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2NewWindowRequestedEventArgs";
}
impl ::std::convert::From<CoreWebView2NewWindowRequestedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: CoreWebView2NewWindowRequestedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreWebView2NewWindowRequestedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &CoreWebView2NewWindowRequestedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CoreWebView2NewWindowRequestedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CoreWebView2NewWindowRequestedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CoreWebView2NewWindowRequestedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: CoreWebView2NewWindowRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreWebView2NewWindowRequestedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &CoreWebView2NewWindowRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CoreWebView2NewWindowRequestedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CoreWebView2NewWindowRequestedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreWebView2NewWindowRequestedEventArgs {}
unsafe impl ::std::marker::Sync for CoreWebView2NewWindowRequestedEventArgs {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CoreWebView2PermissionKind(pub i32);
impl CoreWebView2PermissionKind {
    pub const UnknownPermission: CoreWebView2PermissionKind = CoreWebView2PermissionKind(0i32);
    pub const Microphone: CoreWebView2PermissionKind = CoreWebView2PermissionKind(1i32);
    pub const Camera: CoreWebView2PermissionKind = CoreWebView2PermissionKind(2i32);
    pub const Geolocation: CoreWebView2PermissionKind = CoreWebView2PermissionKind(3i32);
    pub const Notifications: CoreWebView2PermissionKind = CoreWebView2PermissionKind(4i32);
    pub const OtherSensors: CoreWebView2PermissionKind = CoreWebView2PermissionKind(5i32);
    pub const ClipboardRead: CoreWebView2PermissionKind = CoreWebView2PermissionKind(6i32);
}
impl ::std::convert::From<i32> for CoreWebView2PermissionKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CoreWebView2PermissionKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2PermissionKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2PermissionKind;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CoreWebView2PermissionRequestedEventArgs(::windows::runtime::IInspectable);
impl CoreWebView2PermissionRequestedEventArgs {
    pub fn Uri(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn PermissionKind(&self) -> ::windows::runtime::Result<CoreWebView2PermissionKind> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2PermissionKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2PermissionKind>(result__)
        }
    }
    pub fn IsUserInitiated(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn State(&self) -> ::windows::runtime::Result<CoreWebView2PermissionState> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2PermissionState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2PermissionState>(result__)
        }
    }
    pub fn SetState(&self, value: CoreWebView2PermissionState) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn GetDeferral(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2PermissionRequestedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2PermissionRequestedEventArgs;{118bdd9b-cef1-5910-929e-c1a321328239})" ) ;
}
unsafe impl ::windows::runtime::Interface for CoreWebView2PermissionRequestedEventArgs {
    type Vtable = ICoreWebView2PermissionRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        294378907,
        52977,
        22800,
        [146, 158, 193, 163, 33, 50, 130, 57],
    );
}
impl ::windows::runtime::RuntimeName for CoreWebView2PermissionRequestedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2PermissionRequestedEventArgs";
}
impl ::std::convert::From<CoreWebView2PermissionRequestedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: CoreWebView2PermissionRequestedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreWebView2PermissionRequestedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &CoreWebView2PermissionRequestedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CoreWebView2PermissionRequestedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CoreWebView2PermissionRequestedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CoreWebView2PermissionRequestedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: CoreWebView2PermissionRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreWebView2PermissionRequestedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &CoreWebView2PermissionRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CoreWebView2PermissionRequestedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CoreWebView2PermissionRequestedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreWebView2PermissionRequestedEventArgs {}
unsafe impl ::std::marker::Sync for CoreWebView2PermissionRequestedEventArgs {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CoreWebView2PermissionState(pub i32);
impl CoreWebView2PermissionState {
    pub const Default: CoreWebView2PermissionState = CoreWebView2PermissionState(0i32);
    pub const Allow: CoreWebView2PermissionState = CoreWebView2PermissionState(1i32);
    pub const Deny: CoreWebView2PermissionState = CoreWebView2PermissionState(2i32);
}
impl ::std::convert::From<i32> for CoreWebView2PermissionState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CoreWebView2PermissionState {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2PermissionState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2PermissionState;i4)",
    );
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CoreWebView2PhysicalKeyStatus {
    pub RepeatCount: u32,
    pub ScanCode: u32,
    pub IsExtendedKey: i32,
    pub IsMenuKeyDown: i32,
    pub WasKeyDown: i32,
    pub IsKeyReleased: i32,
}
impl CoreWebView2PhysicalKeyStatus {}
impl ::std::default::Default for CoreWebView2PhysicalKeyStatus {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CoreWebView2PhysicalKeyStatus {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CoreWebView2PhysicalKeyStatus")
            .field("RepeatCount", &self.RepeatCount)
            .field("ScanCode", &self.ScanCode)
            .field("IsExtendedKey", &self.IsExtendedKey)
            .field("IsMenuKeyDown", &self.IsMenuKeyDown)
            .field("WasKeyDown", &self.WasKeyDown)
            .field("IsKeyReleased", &self.IsKeyReleased)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CoreWebView2PhysicalKeyStatus {
    fn eq(&self, other: &Self) -> bool {
        self.RepeatCount == other.RepeatCount
            && self.ScanCode == other.ScanCode
            && self.IsExtendedKey == other.IsExtendedKey
            && self.IsMenuKeyDown == other.IsMenuKeyDown
            && self.WasKeyDown == other.WasKeyDown
            && self.IsKeyReleased == other.IsKeyReleased
    }
}
impl ::std::cmp::Eq for CoreWebView2PhysicalKeyStatus {}
unsafe impl ::windows::runtime::Abi for CoreWebView2PhysicalKeyStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2PhysicalKeyStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"struct(Microsoft.Web.WebView2.Core.CoreWebView2PhysicalKeyStatus;u4;u4;i4;i4;i4;i4)",
    );
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
pub struct CoreWebView2PointerEventKind(pub i32);
impl CoreWebView2PointerEventKind {
    pub const Activate: CoreWebView2PointerEventKind = CoreWebView2PointerEventKind(587i32);
    pub const Down: CoreWebView2PointerEventKind = CoreWebView2PointerEventKind(582i32);
    pub const Enter: CoreWebView2PointerEventKind = CoreWebView2PointerEventKind(585i32);
    pub const Leave: CoreWebView2PointerEventKind = CoreWebView2PointerEventKind(586i32);
    pub const Up: CoreWebView2PointerEventKind = CoreWebView2PointerEventKind(583i32);
    pub const Update: CoreWebView2PointerEventKind = CoreWebView2PointerEventKind(581i32);
}
impl ::std::convert::From<i32> for CoreWebView2PointerEventKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CoreWebView2PointerEventKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2PointerEventKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2PointerEventKind;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CoreWebView2PointerInfo(::windows::runtime::IInspectable);
impl CoreWebView2PointerInfo {
    pub fn PointerKind(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn SetPointerKind(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn PointerId(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn SetPointerId(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn FrameId(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn SetFrameId(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn PointerFlags(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn SetPointerFlags(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn PointerDeviceRect(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Windows::Foundation::Rect =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn SetPointerDeviceRect<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Windows::Foundation::Rect>,
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
    #[cfg(feature = "undation")]
    pub fn DisplayRect(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Windows::Foundation::Rect =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn SetDisplayRect<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Windows::Foundation::Rect>,
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
    #[cfg(feature = "undation")]
    pub fn PixelLocation(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Windows::Foundation::Point =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn SetPixelLocation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Windows::Foundation::Point>,
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
    #[cfg(feature = "undation")]
    pub fn HimetricLocation(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Windows::Foundation::Point =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn SetHimetricLocation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Windows::Foundation::Point>,
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
    #[cfg(feature = "undation")]
    pub fn PixelLocationRaw(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Windows::Foundation::Point =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn SetPixelLocationRaw<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Windows::Foundation::Point>,
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
    #[cfg(feature = "undation")]
    pub fn HimetricLocationRaw(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Windows::Foundation::Point =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn SetHimetricLocationRaw<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Windows::Foundation::Point>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Time(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn SetTime(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).27)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn HistoryCount(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn SetHistoryCount(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).29)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn InputData(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetInputData(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).31)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyStates(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn SetKeyStates(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).33)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn PerformanceCount(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u64>(result__)
        }
    }
    pub fn SetPerformanceCount(&self, value: u64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).35)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn ButtonChangeKind(&self) -> ::windows::runtime::Result<i32> {
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
    pub fn SetButtonChangeKind(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).37)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn PenFlags(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).38)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn SetPenFlags(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).39)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn PenMask(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).40)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn SetPenMask(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).41)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn PenPressure(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).42)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn SetPenPressure(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).43)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn PenRotation(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).44)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn SetPenRotation(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).45)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn PenTiltX(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).46)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetPenTiltX(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).47)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn PenTiltY(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).48)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetPenTiltY(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).49)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn TouchFlags(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).50)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn SetTouchFlags(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).51)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn TouchMask(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).52)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn SetTouchMask(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).53)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn TouchContact(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Windows::Foundation::Rect =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).54)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn SetTouchContact<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Windows::Foundation::Rect>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).55)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn TouchContactRaw(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Windows::Foundation::Rect =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).56)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn SetTouchContactRaw<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Windows::Foundation::Rect>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).57)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn TouchOrientation(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).58)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn SetTouchOrientation(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).59)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn TouchPressure(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).60)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn SetTouchPressure(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).61)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2PointerInfo {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2PointerInfo;{c3860e0d-c018-5a84-bc06-9f8f7b275dff})" ) ;
}
unsafe impl ::windows::runtime::Interface for CoreWebView2PointerInfo {
    type Vtable = ICoreWebView2PointerInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3280342541,
        49176,
        23172,
        [188, 6, 159, 143, 123, 39, 93, 255],
    );
}
impl ::windows::runtime::RuntimeName for CoreWebView2PointerInfo {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2PointerInfo";
}
impl ::std::convert::From<CoreWebView2PointerInfo> for ::windows::runtime::IUnknown {
    fn from(value: CoreWebView2PointerInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreWebView2PointerInfo> for ::windows::runtime::IUnknown {
    fn from(value: &CoreWebView2PointerInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CoreWebView2PointerInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CoreWebView2PointerInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CoreWebView2PointerInfo> for ::windows::runtime::IInspectable {
    fn from(value: CoreWebView2PointerInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreWebView2PointerInfo> for ::windows::runtime::IInspectable {
    fn from(value: &CoreWebView2PointerInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CoreWebView2PointerInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CoreWebView2PointerInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreWebView2PointerInfo {}
unsafe impl ::std::marker::Sync for CoreWebView2PointerInfo {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CoreWebView2ProcessFailedEventArgs(::windows::runtime::IInspectable);
impl CoreWebView2ProcessFailedEventArgs {
    pub fn ProcessFailedKind(&self) -> ::windows::runtime::Result<CoreWebView2ProcessFailedKind> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2ProcessFailedKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2ProcessFailedKind>(result__)
        }
    }
    pub fn Reason(&self) -> ::windows::runtime::Result<CoreWebView2ProcessFailedReason> {
        let this =
            &::windows::runtime::Interface::cast::<ICoreWebView2ProcessFailedEventArgs2>(self)?;
        unsafe {
            let mut result__: CoreWebView2ProcessFailedReason = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2ProcessFailedReason>(result__)
        }
    }
    pub fn ExitCode(&self) -> ::windows::runtime::Result<i32> {
        let this =
            &::windows::runtime::Interface::cast::<ICoreWebView2ProcessFailedEventArgs2>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn ProcessDescription(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this =
            &::windows::runtime::Interface::cast::<ICoreWebView2ProcessFailedEventArgs2>(self)?;
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
    #[cfg(feature = "undation_Collections")]
    pub fn FrameInfosForFailedProcess(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::Collections::IVectorView<
            CoreWebView2FrameInfo,
        >,
    > {
        let this =
            &::windows::runtime::Interface::cast::<ICoreWebView2ProcessFailedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Collections::IVectorView<
                CoreWebView2FrameInfo,
            >>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2ProcessFailedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2ProcessFailedEventArgs;{25a8f8c9-d944-539d-afa3-24172b48ef47})" ) ;
}
unsafe impl ::windows::runtime::Interface for CoreWebView2ProcessFailedEventArgs {
    type Vtable = ICoreWebView2ProcessFailedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        631830729,
        55620,
        21405,
        [175, 163, 36, 23, 43, 72, 239, 71],
    );
}
impl ::windows::runtime::RuntimeName for CoreWebView2ProcessFailedEventArgs {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2ProcessFailedEventArgs";
}
impl ::std::convert::From<CoreWebView2ProcessFailedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: CoreWebView2ProcessFailedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreWebView2ProcessFailedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &CoreWebView2ProcessFailedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CoreWebView2ProcessFailedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CoreWebView2ProcessFailedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CoreWebView2ProcessFailedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: CoreWebView2ProcessFailedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreWebView2ProcessFailedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &CoreWebView2ProcessFailedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CoreWebView2ProcessFailedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CoreWebView2ProcessFailedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreWebView2ProcessFailedEventArgs {}
unsafe impl ::std::marker::Sync for CoreWebView2ProcessFailedEventArgs {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CoreWebView2ProcessFailedKind(pub i32);
impl CoreWebView2ProcessFailedKind {
    pub const BrowserProcessExited: CoreWebView2ProcessFailedKind =
        CoreWebView2ProcessFailedKind(0i32);
    pub const RenderProcessExited: CoreWebView2ProcessFailedKind =
        CoreWebView2ProcessFailedKind(1i32);
    pub const RenderProcessUnresponsive: CoreWebView2ProcessFailedKind =
        CoreWebView2ProcessFailedKind(2i32);
    pub const FrameRenderProcessExited: CoreWebView2ProcessFailedKind =
        CoreWebView2ProcessFailedKind(3i32);
    pub const UtilityProcessExited: CoreWebView2ProcessFailedKind =
        CoreWebView2ProcessFailedKind(4i32);
    pub const SandboxHelperProcessExited: CoreWebView2ProcessFailedKind =
        CoreWebView2ProcessFailedKind(5i32);
    pub const GpuProcessExited: CoreWebView2ProcessFailedKind = CoreWebView2ProcessFailedKind(6i32);
    pub const PpapiPluginProcessExited: CoreWebView2ProcessFailedKind =
        CoreWebView2ProcessFailedKind(7i32);
    pub const PpapiBrokerProcessExited: CoreWebView2ProcessFailedKind =
        CoreWebView2ProcessFailedKind(8i32);
    pub const UnknownProcessExited: CoreWebView2ProcessFailedKind =
        CoreWebView2ProcessFailedKind(9i32);
}
impl ::std::convert::From<i32> for CoreWebView2ProcessFailedKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CoreWebView2ProcessFailedKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2ProcessFailedKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2ProcessFailedKind;i4)",
    );
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
pub struct CoreWebView2ProcessFailedReason(pub i32);
impl CoreWebView2ProcessFailedReason {
    pub const Unexpected: CoreWebView2ProcessFailedReason = CoreWebView2ProcessFailedReason(0i32);
    pub const Unresponsive: CoreWebView2ProcessFailedReason = CoreWebView2ProcessFailedReason(1i32);
    pub const Terminated: CoreWebView2ProcessFailedReason = CoreWebView2ProcessFailedReason(2i32);
    pub const Crashed: CoreWebView2ProcessFailedReason = CoreWebView2ProcessFailedReason(3i32);
    pub const LaunchFailed: CoreWebView2ProcessFailedReason = CoreWebView2ProcessFailedReason(4i32);
    pub const OutOfMemory: CoreWebView2ProcessFailedReason = CoreWebView2ProcessFailedReason(5i32);
}
impl ::std::convert::From<i32> for CoreWebView2ProcessFailedReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CoreWebView2ProcessFailedReason {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2ProcessFailedReason {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2ProcessFailedReason;i4)",
    );
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
pub struct CoreWebView2ScriptDialogKind(pub i32);
impl CoreWebView2ScriptDialogKind {
    pub const Alert: CoreWebView2ScriptDialogKind = CoreWebView2ScriptDialogKind(0i32);
    pub const Confirm: CoreWebView2ScriptDialogKind = CoreWebView2ScriptDialogKind(1i32);
    pub const Prompt: CoreWebView2ScriptDialogKind = CoreWebView2ScriptDialogKind(2i32);
    pub const Beforeunload: CoreWebView2ScriptDialogKind = CoreWebView2ScriptDialogKind(3i32);
}
impl ::std::convert::From<i32> for CoreWebView2ScriptDialogKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CoreWebView2ScriptDialogKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2ScriptDialogKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2ScriptDialogKind;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CoreWebView2ScriptDialogOpeningEventArgs(::windows::runtime::IInspectable);
impl CoreWebView2ScriptDialogOpeningEventArgs {
    pub fn Uri(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn Kind(&self) -> ::windows::runtime::Result<CoreWebView2ScriptDialogKind> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2ScriptDialogKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2ScriptDialogKind>(result__)
        }
    }
    pub fn Message(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn DefaultText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn ResultText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn SetResultText<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
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
    pub fn Accept(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn GetDeferral(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2ScriptDialogOpeningEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2ScriptDialogOpeningEventArgs;{a4315212-c7eb-568a-86e4-c61e31ba6cda})" ) ;
}
unsafe impl ::windows::runtime::Interface for CoreWebView2ScriptDialogOpeningEventArgs {
    type Vtable = ICoreWebView2ScriptDialogOpeningEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2754695698,
        51179,
        22154,
        [134, 228, 198, 30, 49, 186, 108, 218],
    );
}
impl ::windows::runtime::RuntimeName for CoreWebView2ScriptDialogOpeningEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2ScriptDialogOpeningEventArgs";
}
impl ::std::convert::From<CoreWebView2ScriptDialogOpeningEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: CoreWebView2ScriptDialogOpeningEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreWebView2ScriptDialogOpeningEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &CoreWebView2ScriptDialogOpeningEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CoreWebView2ScriptDialogOpeningEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CoreWebView2ScriptDialogOpeningEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CoreWebView2ScriptDialogOpeningEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: CoreWebView2ScriptDialogOpeningEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreWebView2ScriptDialogOpeningEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &CoreWebView2ScriptDialogOpeningEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CoreWebView2ScriptDialogOpeningEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CoreWebView2ScriptDialogOpeningEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreWebView2ScriptDialogOpeningEventArgs {}
unsafe impl ::std::marker::Sync for CoreWebView2ScriptDialogOpeningEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CoreWebView2Settings(::windows::runtime::IInspectable);
impl CoreWebView2Settings {
    pub fn IsScriptEnabled(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn SetIsScriptEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsWebMessageEnabled(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn SetIsWebMessageEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn AreDefaultScriptDialogsEnabled(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn SetAreDefaultScriptDialogsEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsStatusBarEnabled(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn SetIsStatusBarEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn AreDevToolsEnabled(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn SetAreDevToolsEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn AreDefaultContextMenusEnabled(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn SetAreDefaultContextMenusEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn AreHostObjectsAllowed(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAreHostObjectsAllowed(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsZoomControlEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsZoomControlEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).21)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsBuiltInErrorPageEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsBuiltInErrorPageEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn UserAgent(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Settings2>(self)?;
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
    pub fn SetUserAgent<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Settings2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn AreBrowserAcceleratorKeysEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Settings3>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAreBrowserAcceleratorKeysEnabled(
        &self,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Settings3>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsPasswordAutosaveEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Settings4>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsPasswordAutosaveEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Settings4>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsGeneralAutofillEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Settings4>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsGeneralAutofillEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Settings4>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsPinchZoomEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Settings5>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsPinchZoomEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Settings5>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn HostObjectDispatchAdapter(
        &self,
    ) -> ::windows::runtime::Result<ICoreWebView2DispatchAdapter> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Settings_Manual>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ICoreWebView2DispatchAdapter>(result__)
        }
    }
    pub fn SetHostObjectDispatchAdapter<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ICoreWebView2DispatchAdapter>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreWebView2Settings_Manual>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2Settings {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2Settings;{003b325e-74cd-52dd-8024-ebb8be38e48e})" ) ;
}
unsafe impl ::windows::runtime::Interface for CoreWebView2Settings {
    type Vtable = ICoreWebView2Settings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3879518,
        29901,
        21213,
        [128, 36, 235, 184, 190, 56, 228, 142],
    );
}
impl ::windows::runtime::RuntimeName for CoreWebView2Settings {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2Settings";
}
impl ::std::convert::From<CoreWebView2Settings> for ::windows::runtime::IUnknown {
    fn from(value: CoreWebView2Settings) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreWebView2Settings> for ::windows::runtime::IUnknown {
    fn from(value: &CoreWebView2Settings) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CoreWebView2Settings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &CoreWebView2Settings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CoreWebView2Settings> for ::windows::runtime::IInspectable {
    fn from(value: CoreWebView2Settings) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreWebView2Settings> for ::windows::runtime::IInspectable {
    fn from(value: &CoreWebView2Settings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CoreWebView2Settings
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CoreWebView2Settings
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreWebView2Settings {}
unsafe impl ::std::marker::Sync for CoreWebView2Settings {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CoreWebView2SourceChangedEventArgs(::windows::runtime::IInspectable);
impl CoreWebView2SourceChangedEventArgs {
    pub fn IsNewDocument(&self) -> ::windows::runtime::Result<bool> {
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
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2SourceChangedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2SourceChangedEventArgs;{ca437b2c-6a18-5552-b749-b198f8cc34d9})" ) ;
}
unsafe impl ::windows::runtime::Interface for CoreWebView2SourceChangedEventArgs {
    type Vtable = ICoreWebView2SourceChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3393420076,
        27160,
        21842,
        [183, 73, 177, 152, 248, 204, 52, 217],
    );
}
impl ::windows::runtime::RuntimeName for CoreWebView2SourceChangedEventArgs {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2SourceChangedEventArgs";
}
impl ::std::convert::From<CoreWebView2SourceChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: CoreWebView2SourceChangedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreWebView2SourceChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &CoreWebView2SourceChangedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CoreWebView2SourceChangedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CoreWebView2SourceChangedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CoreWebView2SourceChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: CoreWebView2SourceChangedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreWebView2SourceChangedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &CoreWebView2SourceChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CoreWebView2SourceChangedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CoreWebView2SourceChangedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreWebView2SourceChangedEventArgs {}
unsafe impl ::std::marker::Sync for CoreWebView2SourceChangedEventArgs {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CoreWebView2WebErrorStatus(pub i32);
impl CoreWebView2WebErrorStatus {
    pub const Unknown: CoreWebView2WebErrorStatus = CoreWebView2WebErrorStatus(0i32);
    pub const CertificateCommonNameIsIncorrect: CoreWebView2WebErrorStatus =
        CoreWebView2WebErrorStatus(1i32);
    pub const CertificateExpired: CoreWebView2WebErrorStatus = CoreWebView2WebErrorStatus(2i32);
    pub const ClientCertificateContainsErrors: CoreWebView2WebErrorStatus =
        CoreWebView2WebErrorStatus(3i32);
    pub const CertificateRevoked: CoreWebView2WebErrorStatus = CoreWebView2WebErrorStatus(4i32);
    pub const CertificateIsInvalid: CoreWebView2WebErrorStatus = CoreWebView2WebErrorStatus(5i32);
    pub const ServerUnreachable: CoreWebView2WebErrorStatus = CoreWebView2WebErrorStatus(6i32);
    pub const Timeout: CoreWebView2WebErrorStatus = CoreWebView2WebErrorStatus(7i32);
    pub const ErrorHttpInvalidServerResponse: CoreWebView2WebErrorStatus =
        CoreWebView2WebErrorStatus(8i32);
    pub const ConnectionAborted: CoreWebView2WebErrorStatus = CoreWebView2WebErrorStatus(9i32);
    pub const ConnectionReset: CoreWebView2WebErrorStatus = CoreWebView2WebErrorStatus(10i32);
    pub const Disconnected: CoreWebView2WebErrorStatus = CoreWebView2WebErrorStatus(11i32);
    pub const CannotConnect: CoreWebView2WebErrorStatus = CoreWebView2WebErrorStatus(12i32);
    pub const HostNameNotResolved: CoreWebView2WebErrorStatus = CoreWebView2WebErrorStatus(13i32);
    pub const OperationCanceled: CoreWebView2WebErrorStatus = CoreWebView2WebErrorStatus(14i32);
    pub const RedirectFailed: CoreWebView2WebErrorStatus = CoreWebView2WebErrorStatus(15i32);
    pub const UnexpectedError: CoreWebView2WebErrorStatus = CoreWebView2WebErrorStatus(16i32);
}
impl ::std::convert::From<i32> for CoreWebView2WebErrorStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CoreWebView2WebErrorStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2WebErrorStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2WebErrorStatus;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CoreWebView2WebMessageReceivedEventArgs(::windows::runtime::IInspectable);
impl CoreWebView2WebMessageReceivedEventArgs {
    pub fn Source(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn WebMessageAsJson(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn TryGetWebMessageAsString(
        &self,
    ) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2WebMessageReceivedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2WebMessageReceivedEventArgs;{eb066159-b725-5d5b-adc8-f5d7b9290304})" ) ;
}
unsafe impl ::windows::runtime::Interface for CoreWebView2WebMessageReceivedEventArgs {
    type Vtable = ICoreWebView2WebMessageReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3943063897,
        46885,
        23899,
        [173, 200, 245, 215, 185, 41, 3, 4],
    );
}
impl ::windows::runtime::RuntimeName for CoreWebView2WebMessageReceivedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2WebMessageReceivedEventArgs";
}
impl ::std::convert::From<CoreWebView2WebMessageReceivedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: CoreWebView2WebMessageReceivedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreWebView2WebMessageReceivedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &CoreWebView2WebMessageReceivedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CoreWebView2WebMessageReceivedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CoreWebView2WebMessageReceivedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CoreWebView2WebMessageReceivedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: CoreWebView2WebMessageReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreWebView2WebMessageReceivedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &CoreWebView2WebMessageReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CoreWebView2WebMessageReceivedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CoreWebView2WebMessageReceivedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreWebView2WebMessageReceivedEventArgs {}
unsafe impl ::std::marker::Sync for CoreWebView2WebMessageReceivedEventArgs {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CoreWebView2WebResourceContext(pub i32);
impl CoreWebView2WebResourceContext {
    pub const All: CoreWebView2WebResourceContext = CoreWebView2WebResourceContext(0i32);
    pub const Document: CoreWebView2WebResourceContext = CoreWebView2WebResourceContext(1i32);
    pub const Stylesheet: CoreWebView2WebResourceContext = CoreWebView2WebResourceContext(2i32);
    pub const Image: CoreWebView2WebResourceContext = CoreWebView2WebResourceContext(3i32);
    pub const Media: CoreWebView2WebResourceContext = CoreWebView2WebResourceContext(4i32);
    pub const Font: CoreWebView2WebResourceContext = CoreWebView2WebResourceContext(5i32);
    pub const Script: CoreWebView2WebResourceContext = CoreWebView2WebResourceContext(6i32);
    pub const XmlHttpRequest: CoreWebView2WebResourceContext = CoreWebView2WebResourceContext(7i32);
    pub const Fetch: CoreWebView2WebResourceContext = CoreWebView2WebResourceContext(8i32);
    pub const TextTrack: CoreWebView2WebResourceContext = CoreWebView2WebResourceContext(9i32);
    pub const EventSource: CoreWebView2WebResourceContext = CoreWebView2WebResourceContext(10i32);
    pub const Websocket: CoreWebView2WebResourceContext = CoreWebView2WebResourceContext(11i32);
    pub const Manifest: CoreWebView2WebResourceContext = CoreWebView2WebResourceContext(12i32);
    pub const SignedExchange: CoreWebView2WebResourceContext =
        CoreWebView2WebResourceContext(13i32);
    pub const Ping: CoreWebView2WebResourceContext = CoreWebView2WebResourceContext(14i32);
    pub const CspViolationReport: CoreWebView2WebResourceContext =
        CoreWebView2WebResourceContext(15i32);
    pub const Other: CoreWebView2WebResourceContext = CoreWebView2WebResourceContext(16i32);
}
impl ::std::convert::From<i32> for CoreWebView2WebResourceContext {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CoreWebView2WebResourceContext {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2WebResourceContext {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2WebResourceContext;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CoreWebView2WebResourceRequest(::windows::runtime::IInspectable);
impl CoreWebView2WebResourceRequest {
    pub fn Uri(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn SetUri<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
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
    pub fn Method(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn SetMethod<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
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
    #[cfg(feature = "orage_Streams")]
    pub fn Content(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Storage::Streams::IRandomAccessStream,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Storage::Streams::IRandomAccessStream>(
                result__,
            )
        }
    }
    #[cfg(feature = "orage_Streams")]
    pub fn SetContent<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Storage::Streams::IRandomAccessStream,
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
    pub fn Headers(&self) -> ::windows::runtime::Result<CoreWebView2HttpRequestHeaders> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2HttpRequestHeaders>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2WebResourceRequest {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2WebResourceRequest;{5c742259-67d2-5df2-8382-0f201b4d7197})" ) ;
}
unsafe impl ::windows::runtime::Interface for CoreWebView2WebResourceRequest {
    type Vtable = ICoreWebView2WebResourceRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1551114841,
        26578,
        24050,
        [131, 130, 15, 32, 27, 77, 113, 151],
    );
}
impl ::windows::runtime::RuntimeName for CoreWebView2WebResourceRequest {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2WebResourceRequest";
}
impl ::std::convert::From<CoreWebView2WebResourceRequest> for ::windows::runtime::IUnknown {
    fn from(value: CoreWebView2WebResourceRequest) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreWebView2WebResourceRequest> for ::windows::runtime::IUnknown {
    fn from(value: &CoreWebView2WebResourceRequest) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CoreWebView2WebResourceRequest
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CoreWebView2WebResourceRequest
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CoreWebView2WebResourceRequest> for ::windows::runtime::IInspectable {
    fn from(value: CoreWebView2WebResourceRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreWebView2WebResourceRequest> for ::windows::runtime::IInspectable {
    fn from(value: &CoreWebView2WebResourceRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CoreWebView2WebResourceRequest
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CoreWebView2WebResourceRequest
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreWebView2WebResourceRequest {}
unsafe impl ::std::marker::Sync for CoreWebView2WebResourceRequest {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CoreWebView2WebResourceRequestedEventArgs(::windows::runtime::IInspectable);
impl CoreWebView2WebResourceRequestedEventArgs {
    pub fn Request(&self) -> ::windows::runtime::Result<CoreWebView2WebResourceRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2WebResourceRequest>(result__)
        }
    }
    pub fn Response(&self) -> ::windows::runtime::Result<CoreWebView2WebResourceResponse> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2WebResourceResponse>(result__)
        }
    }
    pub fn SetResponse<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, CoreWebView2WebResourceResponse>,
    >(
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
    pub fn ResourceContext(&self) -> ::windows::runtime::Result<CoreWebView2WebResourceContext> {
        let this = self;
        unsafe {
            let mut result__: CoreWebView2WebResourceContext = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2WebResourceContext>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn GetDeferral(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2WebResourceRequestedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2WebResourceRequestedEventArgs;{577f1fc4-c943-54a9-9700-bd469b48bd41})" ) ;
}
unsafe impl ::windows::runtime::Interface for CoreWebView2WebResourceRequestedEventArgs {
    type Vtable = ICoreWebView2WebResourceRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1467948996,
        51523,
        21673,
        [151, 0, 189, 70, 155, 72, 189, 65],
    );
}
impl ::windows::runtime::RuntimeName for CoreWebView2WebResourceRequestedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2WebResourceRequestedEventArgs";
}
impl ::std::convert::From<CoreWebView2WebResourceRequestedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: CoreWebView2WebResourceRequestedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreWebView2WebResourceRequestedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &CoreWebView2WebResourceRequestedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CoreWebView2WebResourceRequestedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CoreWebView2WebResourceRequestedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CoreWebView2WebResourceRequestedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: CoreWebView2WebResourceRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreWebView2WebResourceRequestedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &CoreWebView2WebResourceRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CoreWebView2WebResourceRequestedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CoreWebView2WebResourceRequestedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreWebView2WebResourceRequestedEventArgs {}
unsafe impl ::std::marker::Sync for CoreWebView2WebResourceRequestedEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CoreWebView2WebResourceResponse(::windows::runtime::IInspectable);
impl CoreWebView2WebResourceResponse {
    #[cfg(feature = "orage_Streams")]
    pub fn Content(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Storage::Streams::IRandomAccessStream,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Storage::Streams::IRandomAccessStream>(
                result__,
            )
        }
    }
    #[cfg(feature = "orage_Streams")]
    pub fn SetContent<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Storage::Streams::IRandomAccessStream,
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
    pub fn Headers(&self) -> ::windows::runtime::Result<CoreWebView2HttpResponseHeaders> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2HttpResponseHeaders>(result__)
        }
    }
    pub fn StatusCode(&self) -> ::windows::runtime::Result<i32> {
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
    pub fn SetStatusCode(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn ReasonPhrase(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetReasonPhrase<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2WebResourceResponse {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2WebResourceResponse;{14621923-e485-5f44-8f5d-bd4243bc398f})" ) ;
}
unsafe impl ::windows::runtime::Interface for CoreWebView2WebResourceResponse {
    type Vtable = ICoreWebView2WebResourceResponse_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        341973283,
        58501,
        24388,
        [143, 93, 189, 66, 67, 188, 57, 143],
    );
}
impl ::windows::runtime::RuntimeName for CoreWebView2WebResourceResponse {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2WebResourceResponse";
}
impl ::std::convert::From<CoreWebView2WebResourceResponse> for ::windows::runtime::IUnknown {
    fn from(value: CoreWebView2WebResourceResponse) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreWebView2WebResourceResponse> for ::windows::runtime::IUnknown {
    fn from(value: &CoreWebView2WebResourceResponse) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CoreWebView2WebResourceResponse
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CoreWebView2WebResourceResponse
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CoreWebView2WebResourceResponse> for ::windows::runtime::IInspectable {
    fn from(value: CoreWebView2WebResourceResponse) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreWebView2WebResourceResponse> for ::windows::runtime::IInspectable {
    fn from(value: &CoreWebView2WebResourceResponse) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CoreWebView2WebResourceResponse
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CoreWebView2WebResourceResponse
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreWebView2WebResourceResponse {}
unsafe impl ::std::marker::Sync for CoreWebView2WebResourceResponse {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CoreWebView2WebResourceResponseReceivedEventArgs(::windows::runtime::IInspectable);
impl CoreWebView2WebResourceResponseReceivedEventArgs {
    pub fn Request(&self) -> ::windows::runtime::Result<CoreWebView2WebResourceRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2WebResourceRequest>(result__)
        }
    }
    pub fn Response(&self) -> ::windows::runtime::Result<CoreWebView2WebResourceResponseView> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2WebResourceResponseView>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2WebResourceResponseReceivedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2WebResourceResponseReceivedEventArgs;{12424671-9711-54f4-bcdf-5f307add6ec2})" ) ;
}
unsafe impl ::windows::runtime::Interface for CoreWebView2WebResourceResponseReceivedEventArgs {
    type Vtable = ICoreWebView2WebResourceResponseReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        306333297,
        38673,
        21748,
        [188, 223, 95, 48, 122, 221, 110, 194],
    );
}
impl ::windows::runtime::RuntimeName for CoreWebView2WebResourceResponseReceivedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2WebResourceResponseReceivedEventArgs";
}
impl ::std::convert::From<CoreWebView2WebResourceResponseReceivedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: CoreWebView2WebResourceResponseReceivedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreWebView2WebResourceResponseReceivedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &CoreWebView2WebResourceResponseReceivedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CoreWebView2WebResourceResponseReceivedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CoreWebView2WebResourceResponseReceivedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CoreWebView2WebResourceResponseReceivedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: CoreWebView2WebResourceResponseReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreWebView2WebResourceResponseReceivedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &CoreWebView2WebResourceResponseReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CoreWebView2WebResourceResponseReceivedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CoreWebView2WebResourceResponseReceivedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreWebView2WebResourceResponseReceivedEventArgs {}
unsafe impl ::std::marker::Sync for CoreWebView2WebResourceResponseReceivedEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CoreWebView2WebResourceResponseView(::windows::runtime::IInspectable);
impl CoreWebView2WebResourceResponseView {
    pub fn Headers(&self) -> ::windows::runtime::Result<CoreWebView2HttpResponseHeaders> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWebView2HttpResponseHeaders>(result__)
        }
    }
    pub fn StatusCode(&self) -> ::windows::runtime::Result<i32> {
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
    pub fn ReasonPhrase(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    #[cfg(all(feature = "undation", feature = "orage_Streams"))]
    pub fn GetContentAsync(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::IAsyncOperation<
            super::super::super::super::Windows::Storage::Streams::IRandomAccessStream,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::IAsyncOperation<
                super::super::super::super::Windows::Storage::Streams::IRandomAccessStream,
            >>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2WebResourceResponseView {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2WebResourceResponseView;{33ee060b-b578-5698-b541-fef87fe7fe72})" ) ;
}
unsafe impl ::windows::runtime::Interface for CoreWebView2WebResourceResponseView {
    type Vtable = ICoreWebView2WebResourceResponseView_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        871237131,
        46456,
        22168,
        [181, 65, 254, 248, 127, 231, 254, 114],
    );
}
impl ::windows::runtime::RuntimeName for CoreWebView2WebResourceResponseView {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2WebResourceResponseView";
}
impl ::std::convert::From<CoreWebView2WebResourceResponseView> for ::windows::runtime::IUnknown {
    fn from(value: CoreWebView2WebResourceResponseView) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreWebView2WebResourceResponseView> for ::windows::runtime::IUnknown {
    fn from(value: &CoreWebView2WebResourceResponseView) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CoreWebView2WebResourceResponseView
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CoreWebView2WebResourceResponseView
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CoreWebView2WebResourceResponseView>
    for ::windows::runtime::IInspectable
{
    fn from(value: CoreWebView2WebResourceResponseView) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreWebView2WebResourceResponseView>
    for ::windows::runtime::IInspectable
{
    fn from(value: &CoreWebView2WebResourceResponseView) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CoreWebView2WebResourceResponseView
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CoreWebView2WebResourceResponseView
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreWebView2WebResourceResponseView {}
unsafe impl ::std::marker::Sync for CoreWebView2WebResourceResponseView {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CoreWebView2WindowFeatures(::windows::runtime::IInspectable);
impl CoreWebView2WindowFeatures {
    pub fn HasPosition(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn HasSize(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn Left(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn Top(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn Height(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn Width(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn ShouldDisplayMenuBar(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn ShouldDisplayStatus(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn ShouldDisplayToolbar(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn ShouldDisplayScrollBars(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreWebView2WindowFeatures {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Web.WebView2.Core.CoreWebView2WindowFeatures;{ee8686d6-056f-5e06-824f-4e2a24c1c1d6})" ) ;
}
unsafe impl ::windows::runtime::Interface for CoreWebView2WindowFeatures {
    type Vtable = ICoreWebView2WindowFeatures_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4001793750,
        1391,
        24070,
        [130, 79, 78, 42, 36, 193, 193, 214],
    );
}
impl ::windows::runtime::RuntimeName for CoreWebView2WindowFeatures {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2WindowFeatures";
}
impl ::std::convert::From<CoreWebView2WindowFeatures> for ::windows::runtime::IUnknown {
    fn from(value: CoreWebView2WindowFeatures) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreWebView2WindowFeatures> for ::windows::runtime::IUnknown {
    fn from(value: &CoreWebView2WindowFeatures) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CoreWebView2WindowFeatures
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CoreWebView2WindowFeatures
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CoreWebView2WindowFeatures> for ::windows::runtime::IInspectable {
    fn from(value: CoreWebView2WindowFeatures) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreWebView2WindowFeatures> for ::windows::runtime::IInspectable {
    fn from(value: &CoreWebView2WindowFeatures) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CoreWebView2WindowFeatures
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CoreWebView2WindowFeatures
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreWebView2WindowFeatures {}
unsafe impl ::std::marker::Sync for CoreWebView2WindowFeatures {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ICoreWebView2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2 {
    type Vtable = ICoreWebView2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        977229210,
        58857,
        21304,
        [187, 103, 78, 176, 80, 74, 79, 20],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2_abi(
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
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
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
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        uri: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        htmlcontent: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        javascript: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        id: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        javascript: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(all(feature = "undation", feature = "orage_Streams"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        imageformat: CoreWebView2CapturePreviewImageFormat,
        imagestream: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "undation", feature = "orage_Streams")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        webmessageasjson: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        webmessageasstring: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        methodname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        parametersasjson: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        eventname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        rawobject: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        uri: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        resourcecontext: CoreWebView2WebResourceContext,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        uri: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        resourcecontext: CoreWebView2WebResourceContext,
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
pub struct ICoreWebView2AcceleratorKeyPressedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2AcceleratorKeyPressedEventArgs {
    type Vtable = ICoreWebView2AcceleratorKeyPressedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1101357312,
        37541,
        22993,
        [158, 113, 146, 34, 227, 58, 227, 139],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2AcceleratorKeyPressedEventArgs_abi(
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
        result__: *mut CoreWebView2KeyEventKind,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut CoreWebView2PhysicalKeyStatus,
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
pub struct ICoreWebView2ClientCertificate(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2ClientCertificate {
    type Vtable = ICoreWebView2ClientCertificate_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        152779250,
        26847,
        21172,
        [143, 176, 253, 53, 97, 175, 65, 242],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ClientCertificate_abi(
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
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
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
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "undation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation_Collections"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut CoreWebView2ClientCertificateKind,
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
#[doc(hidden)]
pub struct ICoreWebView2ClientCertificateRequestedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2ClientCertificateRequestedEventArgs {
    type Vtable = ICoreWebView2ClientCertificateRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2468903765,
        12793,
        21920,
        [182, 139, 217, 132, 29, 126, 27, 244],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ClientCertificateRequestedEventArgs_abi(
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
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "undation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation_Collections"))] usize,
    #[cfg(feature = "undation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation_Collections"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
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
pub struct ICoreWebView2CompositionController(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2CompositionController {
    type Vtable = ICoreWebView2CompositionController_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1337505715,
        18990,
        22407,
        [148, 185, 204, 72, 196, 211, 100, 215],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2CompositionController_abi(
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
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        eventkind: CoreWebView2MouseEventKind,
        virtualkeys: CoreWebView2MouseEventVirtualKeys,
        mousedata: u32,
        point: super::super::super::super::Windows::Foundation::Point,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        eventkind: CoreWebView2PointerEventKind,
        pointerinfo: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "_Core")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "_Core"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ICoreWebView2CompositionController2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2CompositionController2 {
    type Vtable = ICoreWebView2CompositionController2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2364498361,
        64085,
        21629,
        [170, 230, 123, 202, 237, 66, 73, 162],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2CompositionController2_abi(
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
pub struct ICoreWebView2CompositionControllerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2CompositionControllerStatics {
    type Vtable = ICoreWebView2CompositionControllerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1307618079,
        32554,
        22331,
        [184, 26, 185, 181, 49, 34, 71, 54],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2CompositionControllerStatics_abi(
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
pub struct ICoreWebView2ContentLoadingEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2ContentLoadingEventArgs {
    type Vtable = ICoreWebView2ContentLoadingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1828279155,
        37996,
        23982,
        [155, 62, 15, 226, 61, 90, 162, 159],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ContentLoadingEventArgs_abi(
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
        result__: *mut u64,
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
pub struct ICoreWebView2Controller(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2Controller {
    type Vtable = ICoreWebView2Controller_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2777158172,
        21439,
        22798,
        [128, 229, 41, 215, 41, 203, 215, 67],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Controller_abi(
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
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::Rect,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::super::super::super::Windows::Foundation::Rect,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bounds: super::super::super::super::Windows::Foundation::Rect,
        zoomfactor: f64,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        reason: CoreWebView2MoveFocusReason,
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
#[doc(hidden)]
pub struct ICoreWebView2Controller2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2Controller2 {
    type Vtable = ICoreWebView2Controller2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        6931467,
        11914,
        20799,
        [157, 157, 224, 194, 182, 79, 114, 0],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Controller2_abi(
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
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ICoreWebView2Controller3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2Controller3 {
    type Vtable = ICoreWebView2Controller3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3854230036,
        31002,
        23827,
        [155, 118, 162, 87, 217, 253, 162, 172],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Controller3_abi(
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
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
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
        result__: *mut CoreWebView2BoundsMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: CoreWebView2BoundsMode,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::super::Windows::Foundation::EventRegistrationToken,
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
pub struct ICoreWebView2ControllerFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2ControllerFactory {
    type Vtable = ICoreWebView2ControllerFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        655043675,
        50089,
        21464,
        [165, 202, 38, 46, 169, 234, 98, 232],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ControllerFactory_abi(
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
pub struct ICoreWebView2ControllerWindowReference(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2ControllerWindowReference {
    type Vtable = ICoreWebView2ControllerWindowReference_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        267246292,
        18595,
        23748,
        [159, 97, 231, 173, 253, 30, 156, 118],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ControllerWindowReference_abi(
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
        result__: *mut u64,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "_Core")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "_Core"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ICoreWebView2ControllerWindowReferenceStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2ControllerWindowReferenceStatics {
    type Vtable = ICoreWebView2ControllerWindowReferenceStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3723946993,
        60358,
        23092,
        [144, 8, 102, 28, 58, 46, 183, 103],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ControllerWindowReferenceStatics_abi(
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
        windowhandle: u64,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "_Core")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        corewindow: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "_Core"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ICoreWebView2Cookie(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2Cookie {
    type Vtable = ICoreWebView2Cookie_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1391882494,
        36002,
        23213,
        [174, 219, 37, 247, 144, 59, 112, 56],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Cookie_abi(
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
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
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
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
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
        result__: *mut CoreWebView2CookieSameSiteKind,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: CoreWebView2CookieSameSiteKind,
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
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ICoreWebView2CookieManager(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2CookieManager {
    type Vtable = ICoreWebView2CookieManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1083766038,
        44490,
        21859,
        [170, 165, 215, 175, 253, 132, 122, 163],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2CookieManager_abi(
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
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        domain: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        path: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cookieparam: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cookie: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cookie: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        uri: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        domain: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        path: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
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
#[doc(hidden)]
pub struct ICoreWebView2DOMContentLoadedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2DOMContentLoadedEventArgs {
    type Vtable = ICoreWebView2DOMContentLoadedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3295989923,
        9388,
        23036,
        [183, 139, 218, 117, 98, 166, 160, 82],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2DOMContentLoadedEventArgs_abi(
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
        result__: *mut u64,
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
pub struct ICoreWebView2DevToolsProtocolEventReceivedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2DevToolsProtocolEventReceivedEventArgs {
    type Vtable = ICoreWebView2DevToolsProtocolEventReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3064247325,
        64792,
        23034,
        [146, 58, 197, 117, 85, 217, 96, 206],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2DevToolsProtocolEventReceivedEventArgs_abi(
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
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ICoreWebView2DevToolsProtocolEventReceiver(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2DevToolsProtocolEventReceiver {
    type Vtable = ICoreWebView2DevToolsProtocolEventReceiver_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2997010041,
        26108,
        21815,
        [135, 21, 61, 146, 191, 49, 9, 11],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2DevToolsProtocolEventReceiver_abi(
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
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::super::Windows::Foundation::EventRegistrationToken,
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
pub struct ICoreWebView2DispatchAdapter(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2DispatchAdapter {
    type Vtable = ICoreWebView2DispatchAdapter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2022220845,
        6387,
        22886,
        [128, 203, 140, 194, 83, 81, 189, 10],
    );
}
impl ICoreWebView2DispatchAdapter {
    pub fn WrapNamedObject<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ICoreWebView2DispatchAdapter>,
    >(
        &self,
        name: Param0,
        adapter: Param1,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                name.into_param().abi(),
                adapter.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    pub fn WrapObject<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
        Param1: ::windows::runtime::IntoParam<'a, ICoreWebView2DispatchAdapter>,
    >(
        &self,
        unwrapped: Param0,
        adapter: Param1,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                unwrapped.into_param().abi(),
                adapter.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    pub fn UnwrapObject<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        wrapped: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                wrapped.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    pub fn Clean(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ICoreWebView2DispatchAdapter {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{7888a42d-18f3-5966-80cb-8cc25351bd0a}");
}
impl ::std::convert::From<ICoreWebView2DispatchAdapter> for ::windows::runtime::IUnknown {
    fn from(value: ICoreWebView2DispatchAdapter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ICoreWebView2DispatchAdapter> for ::windows::runtime::IUnknown {
    fn from(value: &ICoreWebView2DispatchAdapter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ICoreWebView2DispatchAdapter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ICoreWebView2DispatchAdapter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ICoreWebView2DispatchAdapter> for ::windows::runtime::IInspectable {
    fn from(value: ICoreWebView2DispatchAdapter) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ICoreWebView2DispatchAdapter> for ::windows::runtime::IInspectable {
    fn from(value: &ICoreWebView2DispatchAdapter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ICoreWebView2DispatchAdapter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ICoreWebView2DispatchAdapter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2DispatchAdapter_abi(
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
        adapter: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        unwrapped: ::windows::runtime::RawPtr,
        adapter: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wrapped: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
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
#[doc(hidden)]
pub struct ICoreWebView2DownloadOperation(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2DownloadOperation {
    type Vtable = ICoreWebView2DownloadOperation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2951167595,
        59232,
        23046,
        [155, 246, 30, 116, 60, 19, 205, 45],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2DownloadOperation_abi(
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
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i64,
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
        result__: *mut CoreWebView2DownloadState,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut CoreWebView2DownloadInterruptReason,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
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
#[doc(hidden)]
pub struct ICoreWebView2DownloadStartingEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2DownloadStartingEventArgs {
    type Vtable = ICoreWebView2DownloadStartingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1171882682,
        37462,
        23349,
        [176, 35, 38, 164, 56, 89, 145, 16],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2DownloadStartingEventArgs_abi(
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
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
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
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
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
pub struct ICoreWebView2Environment(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2Environment {
    type Vtable = ICoreWebView2Environment_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3637278769,
        46979,
        21867,
        [185, 206, 137, 156, 30, 149, 213, 133],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Environment_abi(
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
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        parentwindow: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "orage_Streams")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        content: ::windows::runtime::RawPtr,
        statuscode: i32,
        reasonphrase: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        headers: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "orage_Streams"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ICoreWebView2Environment2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2Environment2 {
    type Vtable = ICoreWebView2Environment2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        191055464,
        4119,
        24519,
        [153, 33, 241, 245, 24, 102, 168, 192],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Environment2_abi(
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
    #[cfg(feature = "orage_Streams")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        uri: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        method: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        postdata: ::windows::runtime::RawPtr,
        headers: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "orage_Streams"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ICoreWebView2Environment3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2Environment3 {
    type Vtable = ICoreWebView2Environment3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1580463212,
        49337,
        20774,
        [136, 64, 23, 249, 193, 27, 58, 138],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Environment3_abi(
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
        parentwindow: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
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
pub struct ICoreWebView2Environment4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2Environment4 {
    type Vtable = ICoreWebView2Environment4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1840682970,
        61117,
        22552,
        [135, 144, 31, 229, 126, 243, 25, 226],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Environment4_abi(
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
pub struct ICoreWebView2EnvironmentOptions(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2EnvironmentOptions {
    type Vtable = ICoreWebView2EnvironmentOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        634838073,
        98,
        22325,
        [139, 9, 166, 245, 53, 241, 158, 151],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2EnvironmentOptions_abi(
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
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
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
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
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
pub struct ICoreWebView2EnvironmentOptions_Manual(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2EnvironmentOptions_Manual {
    type Vtable = ICoreWebView2EnvironmentOptions_Manual_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        521159747,
        60051,
        23095,
        [183, 145, 52, 230, 163, 17, 114, 237],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2EnvironmentOptions_Manual_abi(
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
pub struct ICoreWebView2EnvironmentStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2EnvironmentStatics {
    type Vtable = ICoreWebView2EnvironmentStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        238286852,
        61963,
        22069,
        [132, 145, 22, 42, 170, 39, 81, 123],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2EnvironmentStatics_abi(
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
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        browserexecutablefolder: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        userdatafolder: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        options: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        browserexecutablefolder: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        browserversionstring1: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        browserversionstring2: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
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
#[doc(hidden)]
pub struct ICoreWebView2Frame(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2Frame {
    type Vtable = ICoreWebView2Frame_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        50318329,
        6631,
        23480,
        [130, 115, 52, 100, 32, 251, 21, 3],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Frame_abi(
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
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
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
#[doc(hidden)]
pub struct ICoreWebView2FrameCreatedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2FrameCreatedEventArgs {
    type Vtable = ICoreWebView2FrameCreatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1383793080,
        64621,
        21827,
        [141, 206, 150, 205, 253, 179, 44, 129],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2FrameCreatedEventArgs_abi(
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
pub struct ICoreWebView2FrameInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2FrameInfo {
    type Vtable = ICoreWebView2FrameInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4189597190,
        29683,
        20795,
        [188, 44, 68, 93, 222, 219, 169, 118],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2FrameInfo_abi(
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
#[doc(hidden)]
pub struct ICoreWebView2HttpHeadersCollectionIterator(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2HttpHeadersCollectionIterator {
    type Vtable = ICoreWebView2HttpHeadersCollectionIterator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2918343918,
        55680,
        24392,
        [166, 14, 135, 5, 222, 4, 102, 8],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2HttpHeadersCollectionIterator_abi(
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
pub struct ICoreWebView2HttpRequestHeaders(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2HttpRequestHeaders {
    type Vtable = ICoreWebView2HttpRequestHeaders_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3693225671,
        13589,
        21947,
        [188, 178, 87, 183, 143, 134, 185, 29],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2HttpRequestHeaders_abi(
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
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
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
pub struct ICoreWebView2HttpResponseHeaders(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2HttpResponseHeaders {
    type Vtable = ICoreWebView2HttpResponseHeaders_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4090725353,
        29823,
        21876,
        [134, 98, 154, 107, 146, 12, 236, 212],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2HttpResponseHeaders_abi(
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
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
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
#[doc(hidden)]
pub struct ICoreWebView2MoveFocusRequestedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2MoveFocusRequestedEventArgs {
    type Vtable = ICoreWebView2MoveFocusRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        774443067,
        60637,
        23581,
        [178, 136, 63, 6, 109, 96, 137, 25],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2MoveFocusRequestedEventArgs_abi(
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
        result__: *mut CoreWebView2MoveFocusReason,
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
pub struct ICoreWebView2NavigationCompletedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2NavigationCompletedEventArgs {
    type Vtable = ICoreWebView2NavigationCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1214636600,
        874,
        22116,
        [149, 163, 68, 126, 196, 76, 244, 152],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2NavigationCompletedEventArgs_abi(
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
        result__: *mut CoreWebView2WebErrorStatus,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u64,
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
pub struct ICoreWebView2NavigationStartingEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2NavigationStartingEventArgs {
    type Vtable = ICoreWebView2NavigationStartingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1418535891,
        65187,
        22038,
        [189, 5, 174, 8, 6, 108, 134, 211],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2NavigationStartingEventArgs_abi(
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
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u64,
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
pub struct ICoreWebView2NewWindowRequestedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2NewWindowRequestedEventArgs {
    type Vtable = ICoreWebView2NewWindowRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3873444794,
        44744,
        21294,
        [154, 201, 21, 144, 175, 123, 37, 236],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2NewWindowRequestedEventArgs_abi(
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
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
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
pub struct ICoreWebView2PermissionRequestedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2PermissionRequestedEventArgs {
    type Vtable = ICoreWebView2PermissionRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        294378907,
        52977,
        22800,
        [146, 158, 193, 163, 33, 50, 130, 57],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2PermissionRequestedEventArgs_abi(
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
        result__: *mut CoreWebView2PermissionKind,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut CoreWebView2PermissionState,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: CoreWebView2PermissionState,
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
pub struct ICoreWebView2PointerInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2PointerInfo {
    type Vtable = ICoreWebView2PointerInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3280342541,
        49176,
        23172,
        [188, 6, 159, 143, 123, 39, 93, 255],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2PointerInfo_abi(
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
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::Rect,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::super::super::super::Windows::Foundation::Rect,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::Rect,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::super::super::super::Windows::Foundation::Rect,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::Point,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::super::super::super::Windows::Foundation::Point,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::Point,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::super::super::super::Windows::Foundation::Point,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::Point,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::super::super::super::Windows::Foundation::Point,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::Point,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::super::super::super::Windows::Foundation::Point,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: u64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::Rect,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::super::super::super::Windows::Foundation::Rect,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::Rect,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::super::super::super::Windows::Foundation::Rect,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: u32,
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
pub struct ICoreWebView2ProcessFailedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2ProcessFailedEventArgs {
    type Vtable = ICoreWebView2ProcessFailedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        631830729,
        55620,
        21405,
        [175, 163, 36, 23, 43, 72, 239, 71],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ProcessFailedEventArgs_abi(
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
        result__: *mut CoreWebView2ProcessFailedKind,
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
pub struct ICoreWebView2ProcessFailedEventArgs2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2ProcessFailedEventArgs2 {
    type Vtable = ICoreWebView2ProcessFailedEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3319384402,
        46166,
        24007,
        [159, 118, 253, 233, 103, 72, 74, 245],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ProcessFailedEventArgs2_abi(
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
        result__: *mut CoreWebView2ProcessFailedReason,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "undation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ICoreWebView2ScriptDialogOpeningEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2ScriptDialogOpeningEventArgs {
    type Vtable = ICoreWebView2ScriptDialogOpeningEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2754695698,
        51179,
        22154,
        [134, 228, 198, 30, 49, 186, 108, 218],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ScriptDialogOpeningEventArgs_abi(
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
        result__: *mut CoreWebView2ScriptDialogKind,
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
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
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
pub struct ICoreWebView2Settings(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2Settings {
    type Vtable = ICoreWebView2Settings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3879518,
        29901,
        21213,
        [128, 36, 235, 184, 190, 56, 228, 142],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Settings_abi(
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
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ICoreWebView2Settings2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2Settings2 {
    type Vtable = ICoreWebView2Settings2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        930952320,
        64946,
        22247,
        [186, 222, 80, 125, 53, 40, 135, 233],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Settings2_abi(
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
#[doc(hidden)]
pub struct ICoreWebView2Settings3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2Settings3 {
    type Vtable = ICoreWebView2Settings3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1377832705,
        21257,
        23342,
        [160, 60, 61, 38, 119, 89, 25, 64],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Settings3_abi(
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
pub struct ICoreWebView2Settings4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2Settings4 {
    type Vtable = ICoreWebView2Settings4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3601421808,
        56047,
        23146,
        [166, 246, 199, 47, 14, 222, 118, 32],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Settings4_abi(
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
pub struct ICoreWebView2Settings5(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2Settings5 {
    type Vtable = ICoreWebView2Settings5_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2948868899,
        18489,
        23923,
        [172, 247, 224, 51, 86, 49, 171, 245],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Settings5_abi(
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
pub struct ICoreWebView2Settings_Manual(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2Settings_Manual {
    type Vtable = ICoreWebView2Settings_Manual_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        173247623,
        57344,
        20764,
        [135, 202, 222, 211, 65, 61, 3, 218],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Settings_Manual_abi(
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
pub struct ICoreWebView2SourceChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2SourceChangedEventArgs {
    type Vtable = ICoreWebView2SourceChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3393420076,
        27160,
        21842,
        [183, 73, 177, 152, 248, 204, 52, 217],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2SourceChangedEventArgs_abi(
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
pub struct ICoreWebView2WebMessageReceivedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2WebMessageReceivedEventArgs {
    type Vtable = ICoreWebView2WebMessageReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3943063897,
        46885,
        23899,
        [173, 200, 245, 215, 185, 41, 3, 4],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2WebMessageReceivedEventArgs_abi(
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
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
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
#[doc(hidden)]
pub struct ICoreWebView2WebResourceRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2WebResourceRequest {
    type Vtable = ICoreWebView2WebResourceRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1551114841,
        26578,
        24050,
        [131, 130, 15, 32, 27, 77, 113, 151],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2WebResourceRequest_abi(
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
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "orage_Streams")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "orage_Streams"))] usize,
    #[cfg(feature = "orage_Streams")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "orage_Streams"))] usize,
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
pub struct ICoreWebView2WebResourceRequestedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2WebResourceRequestedEventArgs {
    type Vtable = ICoreWebView2WebResourceRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1467948996,
        51523,
        21673,
        [151, 0, 189, 70, 155, 72, 189, 65],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2WebResourceRequestedEventArgs_abi(
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
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut CoreWebView2WebResourceContext,
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
pub struct ICoreWebView2WebResourceResponse(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2WebResourceResponse {
    type Vtable = ICoreWebView2WebResourceResponse_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        341973283,
        58501,
        24388,
        [143, 93, 189, 66, 67, 188, 57, 143],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2WebResourceResponse_abi(
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
    #[cfg(feature = "orage_Streams")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "orage_Streams"))] usize,
    #[cfg(feature = "orage_Streams")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "orage_Streams"))] usize,
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
        value: i32,
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
#[doc(hidden)]
pub struct ICoreWebView2WebResourceResponseReceivedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2WebResourceResponseReceivedEventArgs {
    type Vtable = ICoreWebView2WebResourceResponseReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        306333297,
        38673,
        21748,
        [188, 223, 95, 48, 122, 221, 110, 194],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2WebResourceResponseReceivedEventArgs_abi(
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
pub struct ICoreWebView2WebResourceResponseView(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2WebResourceResponseView {
    type Vtable = ICoreWebView2WebResourceResponseView_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        871237131,
        46456,
        22168,
        [181, 65, 254, 248, 127, 231, 254, 114],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2WebResourceResponseView_abi(
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
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "undation", feature = "orage_Streams"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "undation", feature = "orage_Streams")))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ICoreWebView2WindowFeatures(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2WindowFeatures {
    type Vtable = ICoreWebView2WindowFeatures_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4001793750,
        1391,
        24070,
        [130, 79, 78, 42, 36, 193, 193, 214],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2WindowFeatures_abi(
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
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
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
pub struct ICoreWebView2_2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2_2 {
    type Vtable = ICoreWebView2_2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1468838195,
        10355,
        21512,
        [189, 158, 56, 155, 190, 159, 167, 250],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2_2_abi(
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
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        request: ::windows::runtime::RawPtr,
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
pub struct ICoreWebView2_3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2_3 {
    type Vtable = ICoreWebView2_3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2831641319,
        24944,
        24062,
        [143, 0, 121, 205, 118, 169, 180, 217],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2_3_abi(
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
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hostname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        folderpath: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        accesskind: CoreWebView2HostResourceAccessKind,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hostname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
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
pub struct ICoreWebView2_4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2_4 {
    type Vtable = ICoreWebView2_4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1254462926,
        5378,
        22389,
        [178, 200, 34, 193, 26, 54, 156, 37],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2_4_abi(
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
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::super::Windows::Foundation::EventRegistrationToken,
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
pub struct ICoreWebView2_5(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWebView2_5 {
    type Vtable = ICoreWebView2_5_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3714774595,
        8716,
        24006,
        [176, 168, 34, 196, 30, 71, 37, 149],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2_5_abi(
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
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
);
