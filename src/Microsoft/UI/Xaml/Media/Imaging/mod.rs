#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct BitmapCreateOptions(pub u32);
impl BitmapCreateOptions {
    pub const None: BitmapCreateOptions = BitmapCreateOptions(0u32);
    pub const IgnoreImageCache: BitmapCreateOptions = BitmapCreateOptions(8u32);
}
impl ::std::convert::From<u32> for BitmapCreateOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BitmapCreateOptions {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for BitmapCreateOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Media.Imaging.BitmapCreateOptions;u4)",
    );
}
impl ::std::ops::BitOr for BitmapCreateOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for BitmapCreateOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for BitmapCreateOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for BitmapCreateOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for BitmapCreateOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct BitmapImage(::windows::runtime::IInspectable);
impl BitmapImage {
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
            BitmapImage,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn CreateOptions(&self) -> ::windows::runtime::Result<BitmapCreateOptions> {
        let this = self;
        unsafe {
            let mut result__: BitmapCreateOptions = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<BitmapCreateOptions>(result__)
        }
    }
    pub fn SetCreateOptions(&self, value: BitmapCreateOptions) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn UriSource(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::super::Windows::Foundation::Uri>
    {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::super::Windows::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn SetUriSource<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::super::Windows::Foundation::Uri,
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
    pub fn DecodePixelWidth(&self) -> ::windows::runtime::Result<i32> {
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
    pub fn SetDecodePixelWidth(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn DecodePixelHeight(&self) -> ::windows::runtime::Result<i32> {
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
    pub fn SetDecodePixelHeight(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn DecodePixelType(&self) -> ::windows::runtime::Result<DecodePixelType> {
        let this = self;
        unsafe {
            let mut result__: DecodePixelType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<DecodePixelType>(result__)
        }
    }
    pub fn SetDecodePixelType(&self, value: DecodePixelType) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAnimatedBitmap(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn IsPlaying(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn AutoPlay(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn SetAutoPlay(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn DownloadProgress<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, DownloadProgressEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = self;
        unsafe {
            let mut result__ : super::super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            ( :: windows :: runtime :: Interface :: vtable ( this ) .20 ) ( :: std :: mem :: transmute_copy ( this ) , handler . into_param ( ) . abi ( ) , & mut result__ ) . from_abi :: < super::super::super::super::super::Windows::Foundation:: EventRegistrationToken > ( result__ )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveDownloadProgress<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::super::Windows::Foundation::EventRegistrationToken,
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
    #[cfg(feature = "undation")]
    pub fn ImageOpened<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::RoutedEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = self;
        unsafe {
            let mut result__ : super::super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            ( :: windows :: runtime :: Interface :: vtable ( this ) .22 ) ( :: std :: mem :: transmute_copy ( this ) , handler . into_param ( ) . abi ( ) , & mut result__ ) . from_abi :: < super::super::super::super::super::Windows::Foundation:: EventRegistrationToken > ( result__ )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveImageOpened<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn ImageFailed<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::ExceptionRoutedEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = self;
        unsafe {
            let mut result__ : super::super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            ( :: windows :: runtime :: Interface :: vtable ( this ) .24 ) ( :: std :: mem :: transmute_copy ( this ) , handler . into_param ( ) . abi ( ) , & mut result__ ) . from_abi :: < super::super::super::super::super::Windows::Foundation:: EventRegistrationToken > ( result__ )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveImageFailed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Play(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn CreateInstanceWithUriSource<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::super::Windows::Foundation::Uri,
        >,
    >(
        urisource: Param0,
    ) -> ::windows::runtime::Result<BitmapImage> {
        Self::IBitmapImageFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                urisource.into_param().abi(),
                &mut result__,
            )
            .from_abi::<BitmapImage>(result__)
        })
    }
    pub fn CreateOptionsProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn UriSourceProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn DecodePixelWidthProperty() -> ::windows::runtime::Result<super::super::DependencyProperty>
    {
        Self::IBitmapImageStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn DecodePixelHeightProperty(
    ) -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn DecodePixelTypeProperty() -> ::windows::runtime::Result<super::super::DependencyProperty>
    {
        Self::IBitmapImageStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IsAnimatedBitmapProperty() -> ::windows::runtime::Result<super::super::DependencyProperty>
    {
        Self::IBitmapImageStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IsPlayingProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn AutoPlayProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
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
    pub fn PixelWidth(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IBitmapSource>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn PixelHeight(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IBitmapSource>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "orage_Streams")]
    pub fn SetSource<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::super::Windows::Storage::Streams::IRandomAccessStream,
        >,
    >(
        &self,
        streamsource: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBitmapSource>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                streamsource.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "undation", feature = "orage_Streams"))]
    pub fn SetSourceAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::super::Windows::Storage::Streams::IRandomAccessStream,
        >,
    >(
        &self,
        streamsource: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::super::Windows::Foundation::IAsyncAction,
    > {
        let this = &::windows::runtime::Interface::cast::<IBitmapSource>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                streamsource.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::super::Windows::Foundation::IAsyncAction>(
                result__,
            )
        }
    }
    pub fn IBitmapImageFactory<
        R,
        F: FnOnce(&IBitmapImageFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BitmapImage, IBitmapImageFactory> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IBitmapImageStatics<
        R,
        F: FnOnce(&IBitmapImageStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BitmapImage, IBitmapImageStatics> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BitmapImage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Media.Imaging.BitmapImage;{5cc29916-a411-5bc2-a3c5-a00d99a59da8})",
    );
}
unsafe impl ::windows::runtime::Interface for BitmapImage {
    type Vtable = IBitmapImage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1556257046,
        42001,
        23490,
        [163, 197, 160, 13, 153, 165, 157, 168],
    );
}
impl ::windows::runtime::RuntimeName for BitmapImage {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.BitmapImage";
}
impl ::std::convert::From<BitmapImage> for ::windows::runtime::IUnknown {
    fn from(value: BitmapImage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&BitmapImage> for ::windows::runtime::IUnknown {
    fn from(value: &BitmapImage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BitmapImage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &BitmapImage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<BitmapImage> for ::windows::runtime::IInspectable {
    fn from(value: BitmapImage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&BitmapImage> for ::windows::runtime::IInspectable {
    fn from(value: &BitmapImage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BitmapImage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a BitmapImage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<BitmapImage> for BitmapSource {
    fn from(value: BitmapImage) -> Self {
        ::std::convert::Into::<BitmapSource>::into(&value)
    }
}
impl ::std::convert::From<&BitmapImage> for BitmapSource {
    fn from(value: &BitmapImage) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, BitmapSource> for BitmapImage {
    fn into_param(self) -> ::windows::runtime::Param<'a, BitmapSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<BitmapSource>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, BitmapSource> for &BitmapImage {
    fn into_param(self) -> ::windows::runtime::Param<'a, BitmapSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<BitmapSource>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<BitmapImage> for super::ImageSource {
    fn from(value: BitmapImage) -> Self {
        ::std::convert::Into::<super::ImageSource>::into(&value)
    }
}
impl ::std::convert::From<&BitmapImage> for super::ImageSource {
    fn from(value: &BitmapImage) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for BitmapImage {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::ImageSource>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for &BitmapImage {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::ImageSource>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<BitmapImage> for super::super::DependencyObject {
    fn from(value: BitmapImage) -> Self {
        ::std::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::std::convert::From<&BitmapImage> for super::super::DependencyObject {
    fn from(value: &BitmapImage) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for BitmapImage {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::super::DependencyObject>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for &BitmapImage {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::super::DependencyObject>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
unsafe impl ::std::marker::Send for BitmapImage {}
unsafe impl ::std::marker::Sync for BitmapImage {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct BitmapSource(::windows::runtime::IInspectable);
impl BitmapSource {
    pub fn PixelWidth(&self) -> ::windows::runtime::Result<i32> {
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
    pub fn PixelHeight(&self) -> ::windows::runtime::Result<i32> {
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
    #[cfg(feature = "orage_Streams")]
    pub fn SetSource<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::super::Windows::Storage::Streams::IRandomAccessStream,
        >,
    >(
        &self,
        streamsource: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                streamsource.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "undation", feature = "orage_Streams"))]
    pub fn SetSourceAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::super::Windows::Storage::Streams::IRandomAccessStream,
        >,
    >(
        &self,
        streamsource: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::super::Windows::Foundation::IAsyncAction,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                streamsource.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::super::Windows::Foundation::IAsyncAction>(
                result__,
            )
        }
    }
    pub fn PixelWidthProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::IBitmapSourceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn PixelHeightProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::IBitmapSourceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
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
    pub fn IBitmapSourceStatics<
        R,
        F: FnOnce(&IBitmapSourceStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BitmapSource, IBitmapSourceStatics> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BitmapSource {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Media.Imaging.BitmapSource;{8424269d-9b82-534f-8fea-af5b5ef96bf2})",
    );
}
unsafe impl ::windows::runtime::Interface for BitmapSource {
    type Vtable = IBitmapSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2216961693,
        39810,
        21327,
        [143, 234, 175, 91, 94, 249, 107, 242],
    );
}
impl ::windows::runtime::RuntimeName for BitmapSource {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.BitmapSource";
}
impl ::std::convert::From<BitmapSource> for ::windows::runtime::IUnknown {
    fn from(value: BitmapSource) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&BitmapSource> for ::windows::runtime::IUnknown {
    fn from(value: &BitmapSource) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BitmapSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &BitmapSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<BitmapSource> for ::windows::runtime::IInspectable {
    fn from(value: BitmapSource) -> Self {
        value.0
    }
}
impl ::std::convert::From<&BitmapSource> for ::windows::runtime::IInspectable {
    fn from(value: &BitmapSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BitmapSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a BitmapSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<BitmapSource> for super::ImageSource {
    fn from(value: BitmapSource) -> Self {
        ::std::convert::Into::<super::ImageSource>::into(&value)
    }
}
impl ::std::convert::From<&BitmapSource> for super::ImageSource {
    fn from(value: &BitmapSource) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for BitmapSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::ImageSource>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for &BitmapSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::ImageSource>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<BitmapSource> for super::super::DependencyObject {
    fn from(value: BitmapSource) -> Self {
        ::std::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::std::convert::From<&BitmapSource> for super::super::DependencyObject {
    fn from(value: &BitmapSource) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for BitmapSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::super::DependencyObject>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for &BitmapSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::super::DependencyObject>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
unsafe impl ::std::marker::Send for BitmapSource {}
unsafe impl ::std::marker::Sync for BitmapSource {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DecodePixelType(pub i32);
impl DecodePixelType {
    pub const Physical: DecodePixelType = DecodePixelType(0i32);
    pub const Logical: DecodePixelType = DecodePixelType(1i32);
}
impl ::std::convert::From<i32> for DecodePixelType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DecodePixelType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DecodePixelType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Media.Imaging.DecodePixelType;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct DownloadProgressEventArgs(::windows::runtime::IInspectable);
impl DownloadProgressEventArgs {
    pub fn Progress(&self) -> ::windows::runtime::Result<i32> {
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
    pub fn SetProgress(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DownloadProgressEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Media.Imaging.DownloadProgressEventArgs;{9a0ea80b-1a17-50d5-83f3-377738212619})" ) ;
}
unsafe impl ::windows::runtime::Interface for DownloadProgressEventArgs {
    type Vtable = IDownloadProgressEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2584651787,
        6679,
        20693,
        [131, 243, 55, 119, 56, 33, 38, 25],
    );
}
impl ::windows::runtime::RuntimeName for DownloadProgressEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.DownloadProgressEventArgs";
}
impl ::std::convert::From<DownloadProgressEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: DownloadProgressEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DownloadProgressEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &DownloadProgressEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for DownloadProgressEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &DownloadProgressEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<DownloadProgressEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: DownloadProgressEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DownloadProgressEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &DownloadProgressEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for DownloadProgressEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a DownloadProgressEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for DownloadProgressEventArgs {}
unsafe impl ::std::marker::Sync for DownloadProgressEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct DownloadProgressEventHandler(::windows::runtime::IUnknown);
impl DownloadProgressEventHandler {
    pub fn new<
        F: FnMut(
                &::std::option::Option<::windows::runtime::IInspectable>,
                &::std::option::Option<DownloadProgressEventArgs>,
            ) -> ::windows::runtime::Result<()>
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = DownloadProgressEventHandler_box::<F> {
            vtable: &DownloadProgressEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
        Param1: ::windows::runtime::IntoParam<'a, DownloadProgressEventArgs>,
    >(
        &self,
        sender: Param0,
        e: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).3)(
                ::std::mem::transmute_copy(this),
                sender.into_param().abi(),
                e.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DownloadProgressEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"delegate({9a8e4af5-b124-5205-8ae9-3496e063c569})",
    );
}
unsafe impl ::windows::runtime::Interface for DownloadProgressEventHandler {
    type Vtable = DownloadProgressEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2593016565,
        45348,
        20997,
        [138, 233, 52, 150, 224, 99, 197, 105],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct DownloadProgressEventHandler_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sender: ::windows::runtime::RawPtr,
        e: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct DownloadProgressEventHandler_box<
    F: FnMut(
            &::std::option::Option<::windows::runtime::IInspectable>,
            &::std::option::Option<DownloadProgressEventArgs>,
        ) -> ::windows::runtime::Result<()>
        + 'static,
> {
    vtable: *const DownloadProgressEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<
        F: FnMut(
                &::std::option::Option<::windows::runtime::IInspectable>,
                &::std::option::Option<DownloadProgressEventArgs>,
            ) -> ::windows::runtime::Result<()>
            + 'static,
    > DownloadProgressEventHandler_box<F>
{
    const VTABLE: DownloadProgressEventHandler_abi = DownloadProgressEventHandler_abi(
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
        *interface = if iid == &<DownloadProgressEventHandler as ::windows::runtime::Interface>::IID
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
        sender: ::windows::runtime::RawPtr,
        e: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ( ( * this ) . invoke ) ( & * ( & sender as * const < :: windows :: runtime :: IInspectable as :: windows :: runtime :: Abi > :: Abi as * const < :: windows :: runtime :: IInspectable as :: windows :: runtime :: Abi > :: DefaultType ) , & * ( & e as * const < DownloadProgressEventArgs as :: windows :: runtime :: Abi > :: Abi as * const < DownloadProgressEventArgs as :: windows :: runtime :: Abi > :: DefaultType ) , ) . into ( )
    }
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IBitmapImage(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBitmapImage {
    type Vtable = IBitmapImage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1556257046,
        42001,
        23490,
        [163, 197, 160, 13, 153, 165, 157, 168],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapImage_abi ( pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , iid : & :: windows :: runtime :: GUID , interface : * mut :: windows :: runtime :: RawPtr ) -> :: windows :: runtime :: HRESULT , pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr ) -> u32 , pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr ) -> u32 , pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , count : * mut u32 , values : * mut * mut :: windows :: runtime :: GUID ) -> :: windows :: runtime :: HRESULT , pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , value : * mut :: windows :: runtime :: RawPtr ) -> :: windows :: runtime :: HRESULT , pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , value : * mut i32 ) -> :: windows :: runtime :: HRESULT , pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , result__ : * mut BitmapCreateOptions ) -> :: windows :: runtime :: HRESULT , pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , value : BitmapCreateOptions ) -> :: windows :: runtime :: HRESULT , #[cfg(feature = "undation")] pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , result__ : * mut :: windows :: runtime :: RawPtr ) -> :: windows :: runtime :: HRESULT , #[cfg(not(feature = "undation"))] usize , #[cfg(feature = "undation")] pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , value : :: windows :: runtime :: RawPtr ) -> :: windows :: runtime :: HRESULT , #[cfg(not(feature = "undation"))] usize , pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , result__ : * mut i32 ) -> :: windows :: runtime :: HRESULT , pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , value : i32 ) -> :: windows :: runtime :: HRESULT , pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , result__ : * mut i32 ) -> :: windows :: runtime :: HRESULT , pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , value : i32 ) -> :: windows :: runtime :: HRESULT , pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , result__ : * mut DecodePixelType ) -> :: windows :: runtime :: HRESULT , pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , value : DecodePixelType ) -> :: windows :: runtime :: HRESULT , pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , result__ : * mut bool ) -> :: windows :: runtime :: HRESULT , pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , result__ : * mut bool ) -> :: windows :: runtime :: HRESULT , pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , result__ : * mut bool ) -> :: windows :: runtime :: HRESULT , pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , value : bool ) -> :: windows :: runtime :: HRESULT , #[cfg(feature = "undation")] pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , handler : :: windows :: runtime :: RawPtr , result__ : * mut super::super::super::super::super::Windows::Foundation:: EventRegistrationToken ) -> :: windows :: runtime :: HRESULT , #[cfg(not(feature = "undation"))] usize , #[cfg(feature = "undation")] pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , token : super::super::super::super::super::Windows::Foundation:: EventRegistrationToken ) -> :: windows :: runtime :: HRESULT , #[cfg(not(feature = "undation"))] usize , #[cfg(feature = "undation")] pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , handler : :: windows :: runtime :: RawPtr , result__ : * mut super::super::super::super::super::Windows::Foundation:: EventRegistrationToken ) -> :: windows :: runtime :: HRESULT , #[cfg(not(feature = "undation"))] usize , #[cfg(feature = "undation")] pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , token : super::super::super::super::super::Windows::Foundation:: EventRegistrationToken ) -> :: windows :: runtime :: HRESULT , #[cfg(not(feature = "undation"))] usize , #[cfg(feature = "undation")] pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , handler : :: windows :: runtime :: RawPtr , result__ : * mut super::super::super::super::super::Windows::Foundation:: EventRegistrationToken ) -> :: windows :: runtime :: HRESULT , #[cfg(not(feature = "undation"))] usize , #[cfg(feature = "undation")] pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , token : super::super::super::super::super::Windows::Foundation:: EventRegistrationToken ) -> :: windows :: runtime :: HRESULT , #[cfg(not(feature = "undation"))] usize , pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , ) -> :: windows :: runtime :: HRESULT , pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , ) -> :: windows :: runtime :: HRESULT , ) where ;
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IBitmapImageFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBitmapImageFactory {
    type Vtable = IBitmapImageFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4030193897,
        61993,
        21038,
        [149, 201, 218, 34, 17, 161, 75, 5],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapImageFactory_abi(
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
        urisource: ::windows::runtime::RawPtr,
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
pub struct IBitmapImageStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBitmapImageStatics {
    type Vtable = IBitmapImageStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1271886249,
        6295,
        20956,
        [142, 63, 44, 92, 121, 109, 28, 217],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapImageStatics_abi(
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
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IBitmapSource(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBitmapSource {
    type Vtable = IBitmapSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2216961693,
        39810,
        21327,
        [143, 234, 175, 91, 94, 249, 107, 242],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapSource_abi(
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
    #[cfg(feature = "orage_Streams")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        streamsource: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "orage_Streams"))] usize,
    #[cfg(all(feature = "undation", feature = "orage_Streams"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        streamsource: ::windows::runtime::RawPtr,
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
pub struct IBitmapSourceFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBitmapSourceFactory {
    type Vtable = IBitmapSourceFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        59961381,
        6248,
        22646,
        [173, 103, 18, 233, 74, 141, 165, 191],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapSourceFactory_abi(
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
pub struct IBitmapSourceStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBitmapSourceStatics {
    type Vtable = IBitmapSourceStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4020466782,
        17408,
        24331,
        [189, 199, 63, 41, 17, 163, 215, 25],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapSourceStatics_abi(
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
pub struct IDownloadProgressEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDownloadProgressEventArgs {
    type Vtable = IDownloadProgressEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2584651787,
        6679,
        20693,
        [131, 243, 55, 119, 56, 33, 38, 25],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadProgressEventArgs_abi(
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
        value: i32,
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
pub struct IRenderTargetBitmap(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRenderTargetBitmap {
    type Vtable = IRenderTargetBitmap_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3473948797,
        64139,
        22435,
        [149, 116, 113, 5, 41, 174, 11, 4],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRenderTargetBitmap_abi(
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
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        scaledwidth: i32,
        scaledheight: i32,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
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
pub struct IRenderTargetBitmapStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRenderTargetBitmapStatics {
    type Vtable = IRenderTargetBitmapStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2213028580,
        40836,
        22918,
        [147, 176, 228, 247, 1, 156, 54, 125],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRenderTargetBitmapStatics_abi(
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
pub struct ISoftwareBitmapSource(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISoftwareBitmapSource {
    type Vtable = ISoftwareBitmapSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2796333058,
        7972,
        23070,
        [191, 8, 120, 26, 133, 237, 85, 17],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoftwareBitmapSource_abi(
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
    #[cfg(all(feature = "undation", feature = "aphics_Imaging"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        softwarebitmap: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "undation", feature = "aphics_Imaging")))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISurfaceImageSource(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISurfaceImageSource {
    type Vtable = ISurfaceImageSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2886176156,
        53472,
        24569,
        [183, 62, 152, 232, 46, 76, 141, 54],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurfaceImageSource_abi(
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
pub struct ISurfaceImageSourceFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISurfaceImageSourceFactory {
    type Vtable = ISurfaceImageSourceFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        161640146,
        4531,
        24305,
        [172, 86, 32, 208, 100, 204, 202, 52],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurfaceImageSourceFactory_abi(
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
        pixelwidth: i32,
        pixelheight: i32,
        baseinterface: ::windows::runtime::RawPtr,
        innerinterface: *mut ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pixelwidth: i32,
        pixelheight: i32,
        isopaque: bool,
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
pub struct ISvgImageSource(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISvgImageSource {
    type Vtable = ISvgImageSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3585482044,
        46733,
        21410,
        [176, 123, 186, 106, 223, 221, 88, 135],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISvgImageSource_abi ( pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , iid : & :: windows :: runtime :: GUID , interface : * mut :: windows :: runtime :: RawPtr ) -> :: windows :: runtime :: HRESULT , pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr ) -> u32 , pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr ) -> u32 , pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , count : * mut u32 , values : * mut * mut :: windows :: runtime :: GUID ) -> :: windows :: runtime :: HRESULT , pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , value : * mut :: windows :: runtime :: RawPtr ) -> :: windows :: runtime :: HRESULT , pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , value : * mut i32 ) -> :: windows :: runtime :: HRESULT , #[cfg(feature = "undation")] pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , result__ : * mut :: windows :: runtime :: RawPtr ) -> :: windows :: runtime :: HRESULT , #[cfg(not(feature = "undation"))] usize , #[cfg(feature = "undation")] pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , value : :: windows :: runtime :: RawPtr ) -> :: windows :: runtime :: HRESULT , #[cfg(not(feature = "undation"))] usize , pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , result__ : * mut f64 ) -> :: windows :: runtime :: HRESULT , pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , value : f64 ) -> :: windows :: runtime :: HRESULT , pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , result__ : * mut f64 ) -> :: windows :: runtime :: HRESULT , pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , value : f64 ) -> :: windows :: runtime :: HRESULT , #[cfg(feature = "undation")] pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , handler : :: windows :: runtime :: RawPtr , result__ : * mut super::super::super::super::super::Windows::Foundation:: EventRegistrationToken ) -> :: windows :: runtime :: HRESULT , #[cfg(not(feature = "undation"))] usize , #[cfg(feature = "undation")] pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , token : super::super::super::super::super::Windows::Foundation:: EventRegistrationToken ) -> :: windows :: runtime :: HRESULT , #[cfg(not(feature = "undation"))] usize , #[cfg(feature = "undation")] pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , handler : :: windows :: runtime :: RawPtr , result__ : * mut super::super::super::super::super::Windows::Foundation:: EventRegistrationToken ) -> :: windows :: runtime :: HRESULT , #[cfg(not(feature = "undation"))] usize , #[cfg(feature = "undation")] pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , token : super::super::super::super::super::Windows::Foundation:: EventRegistrationToken ) -> :: windows :: runtime :: HRESULT , #[cfg(not(feature = "undation"))] usize , #[cfg(all(feature = "undation", feature = "orage_Streams"))] pub unsafe extern "system" fn ( this : :: windows :: runtime :: RawPtr , streamsource : :: windows :: runtime :: RawPtr , result__ : * mut :: windows :: runtime :: RawPtr ) -> :: windows :: runtime :: HRESULT , #[cfg(not(all(feature = "undation", feature = "orage_Streams")))] usize , ) where ;
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISvgImageSourceFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISvgImageSourceFactory {
    type Vtable = ISvgImageSourceFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        797271871,
        44132,
        22285,
        [155, 218, 148, 250, 8, 46, 234, 217],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISvgImageSourceFactory_abi(
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
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        urisource: ::windows::runtime::RawPtr,
        baseinterface: ::windows::runtime::RawPtr,
        innerinterface: *mut ::windows::runtime::RawPtr,
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
pub struct ISvgImageSourceFailedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISvgImageSourceFailedEventArgs {
    type Vtable = ISvgImageSourceFailedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1994809976,
        30724,
        21561,
        [165, 13, 20, 197, 186, 137, 103, 20],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISvgImageSourceFailedEventArgs_abi(
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
        result__: *mut SvgImageSourceLoadStatus,
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
pub struct ISvgImageSourceOpenedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISvgImageSourceOpenedEventArgs {
    type Vtable = ISvgImageSourceOpenedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        479748309,
        14544,
        23329,
        [141, 72, 7, 47, 30, 37, 78, 57],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISvgImageSourceOpenedEventArgs_abi(
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
pub struct ISvgImageSourceStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISvgImageSourceStatics {
    type Vtable = ISvgImageSourceStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3819769960,
        62662,
        21779,
        [167, 119, 41, 128, 240, 186, 65, 189],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISvgImageSourceStatics_abi(
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
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IVirtualSurfaceImageSource(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVirtualSurfaceImageSource {
    type Vtable = IVirtualSurfaceImageSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3841955494,
        65246,
        22684,
        [160, 7, 65, 120, 181, 59, 103, 57],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVirtualSurfaceImageSource_abi(
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
pub struct IVirtualSurfaceImageSourceFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVirtualSurfaceImageSourceFactory {
    type Vtable = IVirtualSurfaceImageSourceFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        139005740,
        1192,
        20529,
        [185, 199, 112, 112, 96, 215, 205, 72],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVirtualSurfaceImageSourceFactory_abi(
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
        pixelwidth: i32,
        pixelheight: i32,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pixelwidth: i32,
        pixelheight: i32,
        isopaque: bool,
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
pub struct IWriteableBitmap(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWriteableBitmap {
    type Vtable = IWriteableBitmap_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2026382505,
        3651,
        24350,
        [147, 188, 208, 70, 204, 168, 43, 126],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IWriteableBitmap_abi(
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
pub struct IWriteableBitmapFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWriteableBitmapFactory {
    type Vtable = IWriteableBitmapFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        652763609,
        45184,
        20779,
        [150, 196, 128, 5, 14, 126, 8, 209],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IWriteableBitmapFactory_abi(
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
        pixelwidth: i32,
        pixelheight: i32,
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
pub struct IXamlRenderingBackgroundTask(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlRenderingBackgroundTask {
    type Vtable = IXamlRenderingBackgroundTask_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2013724684,
        41040,
        20769,
        [172, 116, 51, 34, 213, 53, 142, 57],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlRenderingBackgroundTask_abi(
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
pub struct IXamlRenderingBackgroundTaskFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlRenderingBackgroundTaskFactory {
    type Vtable = IXamlRenderingBackgroundTaskFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        542263203,
        40958,
        22938,
        [162, 26, 113, 129, 68, 42, 157, 117],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlRenderingBackgroundTaskFactory_abi(
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
pub struct IXamlRenderingBackgroundTaskOverrides(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlRenderingBackgroundTaskOverrides {
    type Vtable = IXamlRenderingBackgroundTaskOverrides_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        410202679,
        12875,
        22464,
        [137, 178, 88, 117, 71, 42, 204, 128],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlRenderingBackgroundTaskOverrides_abi(
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
    #[cfg(feature = "plicationModel_Background")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        taskinstance: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "plicationModel_Background"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RenderTargetBitmap(::windows::runtime::IInspectable);
impl RenderTargetBitmap {
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
            RenderTargetBitmap,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn PixelWidth(&self) -> ::windows::runtime::Result<i32> {
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
    pub fn PixelHeight(&self) -> ::windows::runtime::Result<i32> {
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
    #[cfg(feature = "undation")]
    pub fn RenderAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UIElement>>(
        &self,
        element: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::super::Windows::Foundation::IAsyncAction,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::super::Windows::Foundation::IAsyncAction>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RenderToSizeAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::UIElement>,
    >(
        &self,
        element: Param0,
        scaledwidth: i32,
        scaledheight: i32,
    ) -> ::windows::runtime::Result<
        super::super::super::super::super::Windows::Foundation::IAsyncAction,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                scaledwidth,
                scaledheight,
                &mut result__,
            )
            .from_abi::<super::super::super::super::super::Windows::Foundation::IAsyncAction>(
                result__,
            )
        }
    }
    #[cfg(all(feature = "undation", feature = "orage_Streams"))]
    pub fn GetPixelsAsync(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::super::super::Windows::Foundation::IAsyncOperation<
            super::super::super::super::super::Windows::Storage::Streams::IBuffer,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::super::Windows::Foundation::IAsyncOperation<
                super::super::super::super::super::Windows::Storage::Streams::IBuffer,
            >>(result__)
        }
    }
    pub fn PixelWidthProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::IRenderTargetBitmapStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn PixelHeightProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::IRenderTargetBitmapStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
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
    pub fn IRenderTargetBitmapStatics<
        R,
        F: FnOnce(&IRenderTargetBitmapStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            RenderTargetBitmap,
            IRenderTargetBitmapStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RenderTargetBitmap {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Media.Imaging.RenderTargetBitmap;{cf10407d-fa8b-57a3-9574-710529ae0b04})" ) ;
}
unsafe impl ::windows::runtime::Interface for RenderTargetBitmap {
    type Vtable = IRenderTargetBitmap_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3473948797,
        64139,
        22435,
        [149, 116, 113, 5, 41, 174, 11, 4],
    );
}
impl ::windows::runtime::RuntimeName for RenderTargetBitmap {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.RenderTargetBitmap";
}
impl ::std::convert::From<RenderTargetBitmap> for ::windows::runtime::IUnknown {
    fn from(value: RenderTargetBitmap) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RenderTargetBitmap> for ::windows::runtime::IUnknown {
    fn from(value: &RenderTargetBitmap) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RenderTargetBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &RenderTargetBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RenderTargetBitmap> for ::windows::runtime::IInspectable {
    fn from(value: RenderTargetBitmap) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RenderTargetBitmap> for ::windows::runtime::IInspectable {
    fn from(value: &RenderTargetBitmap) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RenderTargetBitmap
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RenderTargetBitmap
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<RenderTargetBitmap> for super::ImageSource {
    fn from(value: RenderTargetBitmap) -> Self {
        ::std::convert::Into::<super::ImageSource>::into(&value)
    }
}
impl ::std::convert::From<&RenderTargetBitmap> for super::ImageSource {
    fn from(value: &RenderTargetBitmap) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for RenderTargetBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::ImageSource>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for &RenderTargetBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::ImageSource>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<RenderTargetBitmap> for super::super::DependencyObject {
    fn from(value: RenderTargetBitmap) -> Self {
        ::std::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::std::convert::From<&RenderTargetBitmap> for super::super::DependencyObject {
    fn from(value: &RenderTargetBitmap) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for RenderTargetBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::super::DependencyObject>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for &RenderTargetBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::super::DependencyObject>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
unsafe impl ::std::marker::Send for RenderTargetBitmap {}
unsafe impl ::std::marker::Sync for RenderTargetBitmap {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SoftwareBitmapSource(::windows::runtime::IInspectable);
impl SoftwareBitmapSource {
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
            SoftwareBitmapSource,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(all(feature = "undation", feature = "aphics_Imaging"))]
    pub fn SetBitmapAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::super::Windows::Graphics::Imaging::SoftwareBitmap,
        >,
    >(
        &self,
        softwarebitmap: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::super::Windows::Foundation::IAsyncAction,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                softwarebitmap.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::super::Windows::Foundation::IAsyncAction>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::super::super::super::Windows::Foundation::IClosable,
        >(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
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
unsafe impl ::windows::runtime::RuntimeType for SoftwareBitmapSource {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Media.Imaging.SoftwareBitmapSource;{a6aca802-1f24-5a1e-bf08-781a85ed5511})" ) ;
}
unsafe impl ::windows::runtime::Interface for SoftwareBitmapSource {
    type Vtable = ISoftwareBitmapSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2796333058,
        7972,
        23070,
        [191, 8, 120, 26, 133, 237, 85, 17],
    );
}
impl ::windows::runtime::RuntimeName for SoftwareBitmapSource {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.SoftwareBitmapSource";
}
impl ::std::convert::From<SoftwareBitmapSource> for ::windows::runtime::IUnknown {
    fn from(value: SoftwareBitmapSource) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SoftwareBitmapSource> for ::windows::runtime::IUnknown {
    fn from(value: &SoftwareBitmapSource) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SoftwareBitmapSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SoftwareBitmapSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<SoftwareBitmapSource> for ::windows::runtime::IInspectable {
    fn from(value: SoftwareBitmapSource) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SoftwareBitmapSource> for ::windows::runtime::IInspectable {
    fn from(value: &SoftwareBitmapSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for SoftwareBitmapSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SoftwareBitmapSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "undation")]
impl ::std::convert::TryFrom<SoftwareBitmapSource>
    for super::super::super::super::super::Windows::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: SoftwareBitmapSource) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "undation")]
impl ::std::convert::TryFrom<&SoftwareBitmapSource>
    for super::super::super::super::super::Windows::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SoftwareBitmapSource) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "undation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::super::super::super::Windows::Foundation::IClosable,
    > for SoftwareBitmapSource
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::super::super::super::Windows::Foundation::IClosable,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "undation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::super::super::super::Windows::Foundation::IClosable,
    > for &SoftwareBitmapSource
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::super::super::super::Windows::Foundation::IClosable,
    > {
        :: std :: convert :: TryInto :: < super::super::super::super::super::Windows::Foundation:: IClosable > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
impl ::std::convert::From<SoftwareBitmapSource> for super::ImageSource {
    fn from(value: SoftwareBitmapSource) -> Self {
        ::std::convert::Into::<super::ImageSource>::into(&value)
    }
}
impl ::std::convert::From<&SoftwareBitmapSource> for super::ImageSource {
    fn from(value: &SoftwareBitmapSource) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for SoftwareBitmapSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::ImageSource>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for &SoftwareBitmapSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::ImageSource>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<SoftwareBitmapSource> for super::super::DependencyObject {
    fn from(value: SoftwareBitmapSource) -> Self {
        ::std::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::std::convert::From<&SoftwareBitmapSource> for super::super::DependencyObject {
    fn from(value: &SoftwareBitmapSource) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject>
    for SoftwareBitmapSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::super::DependencyObject>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject>
    for &SoftwareBitmapSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::super::DependencyObject>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
unsafe impl ::std::marker::Send for SoftwareBitmapSource {}
unsafe impl ::std::marker::Sync for SoftwareBitmapSource {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SurfaceImageSource(::windows::runtime::IInspectable);
impl SurfaceImageSource {
    pub fn CreateInstanceWithDimensions(
        pixelwidth: i32,
        pixelheight: i32,
    ) -> ::windows::runtime::Result<SurfaceImageSource> {
        Self::ISurfaceImageSourceFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                pixelwidth,
                pixelheight,
                ::std::ptr::null_mut(),
                &mut ::std::option::Option::<::windows::runtime::IInspectable>::None as *mut _ as _,
                &mut result__,
            )
            .from_abi::<SurfaceImageSource>(result__)
        })
    }
    pub fn CreateInstanceWithDimensionsAndOpacity(
        pixelwidth: i32,
        pixelheight: i32,
        isopaque: bool,
    ) -> ::windows::runtime::Result<SurfaceImageSource> {
        Self::ISurfaceImageSourceFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                pixelwidth,
                pixelheight,
                isopaque,
                ::std::ptr::null_mut(),
                &mut ::std::option::Option::<::windows::runtime::IInspectable>::None as *mut _ as _,
                &mut result__,
            )
            .from_abi::<SurfaceImageSource>(result__)
        })
    }
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
    pub fn ISurfaceImageSourceFactory<
        R,
        F: FnOnce(&ISurfaceImageSourceFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            SurfaceImageSource,
            ISurfaceImageSourceFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SurfaceImageSource {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Media.Imaging.SurfaceImageSource;{ac078d9c-d0e0-5ff9-b73e-98e82e4c8d36})" ) ;
}
unsafe impl ::windows::runtime::Interface for SurfaceImageSource {
    type Vtable = ISurfaceImageSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2886176156,
        53472,
        24569,
        [183, 62, 152, 232, 46, 76, 141, 54],
    );
}
impl ::windows::runtime::RuntimeName for SurfaceImageSource {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.SurfaceImageSource";
}
impl ::std::convert::From<SurfaceImageSource> for ::windows::runtime::IUnknown {
    fn from(value: SurfaceImageSource) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SurfaceImageSource> for ::windows::runtime::IUnknown {
    fn from(value: &SurfaceImageSource) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SurfaceImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SurfaceImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<SurfaceImageSource> for ::windows::runtime::IInspectable {
    fn from(value: SurfaceImageSource) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SurfaceImageSource> for ::windows::runtime::IInspectable {
    fn from(value: &SurfaceImageSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for SurfaceImageSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SurfaceImageSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<SurfaceImageSource> for super::ImageSource {
    fn from(value: SurfaceImageSource) -> Self {
        ::std::convert::Into::<super::ImageSource>::into(&value)
    }
}
impl ::std::convert::From<&SurfaceImageSource> for super::ImageSource {
    fn from(value: &SurfaceImageSource) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for SurfaceImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::ImageSource>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for &SurfaceImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::ImageSource>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<SurfaceImageSource> for super::super::DependencyObject {
    fn from(value: SurfaceImageSource) -> Self {
        ::std::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::std::convert::From<&SurfaceImageSource> for super::super::DependencyObject {
    fn from(value: &SurfaceImageSource) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for SurfaceImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::super::DependencyObject>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for &SurfaceImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::super::DependencyObject>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
unsafe impl ::std::marker::Send for SurfaceImageSource {}
unsafe impl ::std::marker::Sync for SurfaceImageSource {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SvgImageSource(::windows::runtime::IInspectable);
impl SvgImageSource {
    #[cfg(feature = "undation")]
    pub fn UriSource(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::super::Windows::Foundation::Uri>
    {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::super::Windows::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn SetUriSource<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::super::Windows::Foundation::Uri,
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
    pub fn RasterizePixelWidth(&self) -> ::windows::runtime::Result<f64> {
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
    pub fn SetRasterizePixelWidth(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn RasterizePixelHeight(&self) -> ::windows::runtime::Result<f64> {
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
    pub fn SetRasterizePixelHeight(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn Opened<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::super::Windows::Foundation::TypedEventHandler<
                SvgImageSource,
                SvgImageSourceOpenedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = self;
        unsafe {
            let mut result__ : super::super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            ( :: windows :: runtime :: Interface :: vtable ( this ) .12 ) ( :: std :: mem :: transmute_copy ( this ) , handler . into_param ( ) . abi ( ) , & mut result__ ) . from_abi :: < super::super::super::super::super::Windows::Foundation:: EventRegistrationToken > ( result__ )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveOpened<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn OpenFailed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::super::Windows::Foundation::TypedEventHandler<
                SvgImageSource,
                SvgImageSourceFailedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = self;
        unsafe {
            let mut result__ : super::super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            ( :: windows :: runtime :: Interface :: vtable ( this ) .14 ) ( :: std :: mem :: transmute_copy ( this ) , handler . into_param ( ) . abi ( ) , & mut result__ ) . from_abi :: < super::super::super::super::super::Windows::Foundation:: EventRegistrationToken > ( result__ )
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveOpenFailed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "undation", feature = "orage_Streams"))]
    pub fn SetSourceAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::super::Windows::Storage::Streams::IRandomAccessStream,
        >,
    >(
        &self,
        streamsource: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::super::Windows::Foundation::IAsyncOperation<
            SvgImageSourceLoadStatus,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                streamsource.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::super::Windows::Foundation::IAsyncOperation<
                SvgImageSourceLoadStatus,
            >>(result__)
        }
    }
    pub fn UriSourceProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::ISvgImageSourceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn RasterizePixelWidthProperty(
    ) -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::ISvgImageSourceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn RasterizePixelHeightProperty(
    ) -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::ISvgImageSourceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn new() -> ::windows::runtime::Result<SvgImageSource> {
        Self::ISvgImageSourceFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                ::std::ptr::null_mut(),
                &mut ::std::option::Option::<::windows::runtime::IInspectable>::None as *mut _ as _,
                &mut result__,
            )
            .from_abi::<SvgImageSource>(result__)
        })
    }
    #[cfg(feature = "undation")]
    pub fn CreateInstanceWithUriSource<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::super::Windows::Foundation::Uri,
        >,
    >(
        urisource: Param0,
    ) -> ::windows::runtime::Result<SvgImageSource> {
        Self::ISvgImageSourceFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                urisource.into_param().abi(),
                ::std::ptr::null_mut(),
                &mut ::std::option::Option::<::windows::runtime::IInspectable>::None as *mut _ as _,
                &mut result__,
            )
            .from_abi::<SvgImageSource>(result__)
        })
    }
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
    pub fn ISvgImageSourceStatics<
        R,
        F: FnOnce(&ISvgImageSourceStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            SvgImageSource,
            ISvgImageSourceStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISvgImageSourceFactory<
        R,
        F: FnOnce(&ISvgImageSourceFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            SvgImageSource,
            ISvgImageSourceFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SvgImageSource {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Media.Imaging.SvgImageSource;{d5b61d3c-b68d-53a2-b07b-ba6adfdd5887})" ) ;
}
unsafe impl ::windows::runtime::Interface for SvgImageSource {
    type Vtable = ISvgImageSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3585482044,
        46733,
        21410,
        [176, 123, 186, 106, 223, 221, 88, 135],
    );
}
impl ::windows::runtime::RuntimeName for SvgImageSource {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.SvgImageSource";
}
impl ::std::convert::From<SvgImageSource> for ::windows::runtime::IUnknown {
    fn from(value: SvgImageSource) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SvgImageSource> for ::windows::runtime::IUnknown {
    fn from(value: &SvgImageSource) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SvgImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SvgImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<SvgImageSource> for ::windows::runtime::IInspectable {
    fn from(value: SvgImageSource) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SvgImageSource> for ::windows::runtime::IInspectable {
    fn from(value: &SvgImageSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SvgImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SvgImageSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<SvgImageSource> for super::ImageSource {
    fn from(value: SvgImageSource) -> Self {
        ::std::convert::Into::<super::ImageSource>::into(&value)
    }
}
impl ::std::convert::From<&SvgImageSource> for super::ImageSource {
    fn from(value: &SvgImageSource) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for SvgImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::ImageSource>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for &SvgImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::ImageSource>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<SvgImageSource> for super::super::DependencyObject {
    fn from(value: SvgImageSource) -> Self {
        ::std::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::std::convert::From<&SvgImageSource> for super::super::DependencyObject {
    fn from(value: &SvgImageSource) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for SvgImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::super::DependencyObject>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for &SvgImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::super::DependencyObject>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
unsafe impl ::std::marker::Send for SvgImageSource {}
unsafe impl ::std::marker::Sync for SvgImageSource {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SvgImageSourceFailedEventArgs(::windows::runtime::IInspectable);
impl SvgImageSourceFailedEventArgs {
    pub fn Status(&self) -> ::windows::runtime::Result<SvgImageSourceLoadStatus> {
        let this = self;
        unsafe {
            let mut result__: SvgImageSourceLoadStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SvgImageSourceLoadStatus>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SvgImageSourceFailedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Media.Imaging.SvgImageSourceFailedEventArgs;{76e66278-7804-5439-a50d-14c5ba896714})" ) ;
}
unsafe impl ::windows::runtime::Interface for SvgImageSourceFailedEventArgs {
    type Vtable = ISvgImageSourceFailedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1994809976,
        30724,
        21561,
        [165, 13, 20, 197, 186, 137, 103, 20],
    );
}
impl ::windows::runtime::RuntimeName for SvgImageSourceFailedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.SvgImageSourceFailedEventArgs";
}
impl ::std::convert::From<SvgImageSourceFailedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SvgImageSourceFailedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SvgImageSourceFailedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SvgImageSourceFailedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for SvgImageSourceFailedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &SvgImageSourceFailedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<SvgImageSourceFailedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SvgImageSourceFailedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SvgImageSourceFailedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &SvgImageSourceFailedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for SvgImageSourceFailedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SvgImageSourceFailedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SvgImageSourceFailedEventArgs {}
unsafe impl ::std::marker::Sync for SvgImageSourceFailedEventArgs {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct SvgImageSourceLoadStatus(pub i32);
impl SvgImageSourceLoadStatus {
    pub const Success: SvgImageSourceLoadStatus = SvgImageSourceLoadStatus(0i32);
    pub const NetworkError: SvgImageSourceLoadStatus = SvgImageSourceLoadStatus(1i32);
    pub const InvalidFormat: SvgImageSourceLoadStatus = SvgImageSourceLoadStatus(2i32);
    pub const Other: SvgImageSourceLoadStatus = SvgImageSourceLoadStatus(3i32);
}
impl ::std::convert::From<i32> for SvgImageSourceLoadStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SvgImageSourceLoadStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SvgImageSourceLoadStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Media.Imaging.SvgImageSourceLoadStatus;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SvgImageSourceOpenedEventArgs(::windows::runtime::IInspectable);
impl SvgImageSourceOpenedEventArgs {}
unsafe impl ::windows::runtime::RuntimeType for SvgImageSourceOpenedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Media.Imaging.SvgImageSourceOpenedEventArgs;{1c9860d5-38d0-5b21-8d48-072f1e254e39})" ) ;
}
unsafe impl ::windows::runtime::Interface for SvgImageSourceOpenedEventArgs {
    type Vtable = ISvgImageSourceOpenedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        479748309,
        14544,
        23329,
        [141, 72, 7, 47, 30, 37, 78, 57],
    );
}
impl ::windows::runtime::RuntimeName for SvgImageSourceOpenedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.SvgImageSourceOpenedEventArgs";
}
impl ::std::convert::From<SvgImageSourceOpenedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SvgImageSourceOpenedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SvgImageSourceOpenedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SvgImageSourceOpenedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for SvgImageSourceOpenedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &SvgImageSourceOpenedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<SvgImageSourceOpenedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SvgImageSourceOpenedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SvgImageSourceOpenedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &SvgImageSourceOpenedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for SvgImageSourceOpenedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SvgImageSourceOpenedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SvgImageSourceOpenedEventArgs {}
unsafe impl ::std::marker::Sync for SvgImageSourceOpenedEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct VirtualSurfaceImageSource(::windows::runtime::IInspectable);
impl VirtualSurfaceImageSource {
    pub fn CreateInstanceWithDimensions(
        pixelwidth: i32,
        pixelheight: i32,
    ) -> ::windows::runtime::Result<VirtualSurfaceImageSource> {
        Self::IVirtualSurfaceImageSourceFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                pixelwidth,
                pixelheight,
                &mut result__,
            )
            .from_abi::<VirtualSurfaceImageSource>(result__)
        })
    }
    pub fn CreateInstanceWithDimensionsAndOpacity(
        pixelwidth: i32,
        pixelheight: i32,
        isopaque: bool,
    ) -> ::windows::runtime::Result<VirtualSurfaceImageSource> {
        Self::IVirtualSurfaceImageSourceFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                pixelwidth,
                pixelheight,
                isopaque,
                &mut result__,
            )
            .from_abi::<VirtualSurfaceImageSource>(result__)
        })
    }
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
    pub fn IVirtualSurfaceImageSourceFactory<
        R,
        F: FnOnce(&IVirtualSurfaceImageSourceFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            VirtualSurfaceImageSource,
            IVirtualSurfaceImageSourceFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for VirtualSurfaceImageSource {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Media.Imaging.VirtualSurfaceImageSource;{e4ff96a6-fede-589c-a007-4178b53b6739})" ) ;
}
unsafe impl ::windows::runtime::Interface for VirtualSurfaceImageSource {
    type Vtable = IVirtualSurfaceImageSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3841955494,
        65246,
        22684,
        [160, 7, 65, 120, 181, 59, 103, 57],
    );
}
impl ::windows::runtime::RuntimeName for VirtualSurfaceImageSource {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.VirtualSurfaceImageSource";
}
impl ::std::convert::From<VirtualSurfaceImageSource> for ::windows::runtime::IUnknown {
    fn from(value: VirtualSurfaceImageSource) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&VirtualSurfaceImageSource> for ::windows::runtime::IUnknown {
    fn from(value: &VirtualSurfaceImageSource) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for VirtualSurfaceImageSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &VirtualSurfaceImageSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<VirtualSurfaceImageSource> for ::windows::runtime::IInspectable {
    fn from(value: VirtualSurfaceImageSource) -> Self {
        value.0
    }
}
impl ::std::convert::From<&VirtualSurfaceImageSource> for ::windows::runtime::IInspectable {
    fn from(value: &VirtualSurfaceImageSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for VirtualSurfaceImageSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a VirtualSurfaceImageSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<VirtualSurfaceImageSource> for SurfaceImageSource {
    fn from(value: VirtualSurfaceImageSource) -> Self {
        ::std::convert::Into::<SurfaceImageSource>::into(&value)
    }
}
impl ::std::convert::From<&VirtualSurfaceImageSource> for SurfaceImageSource {
    fn from(value: &VirtualSurfaceImageSource) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SurfaceImageSource> for VirtualSurfaceImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, SurfaceImageSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SurfaceImageSource>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SurfaceImageSource> for &VirtualSurfaceImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, SurfaceImageSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SurfaceImageSource>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<VirtualSurfaceImageSource> for super::ImageSource {
    fn from(value: VirtualSurfaceImageSource) -> Self {
        ::std::convert::Into::<super::ImageSource>::into(&value)
    }
}
impl ::std::convert::From<&VirtualSurfaceImageSource> for super::ImageSource {
    fn from(value: &VirtualSurfaceImageSource) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for VirtualSurfaceImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::ImageSource>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for &VirtualSurfaceImageSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::ImageSource>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<VirtualSurfaceImageSource> for super::super::DependencyObject {
    fn from(value: VirtualSurfaceImageSource) -> Self {
        ::std::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::std::convert::From<&VirtualSurfaceImageSource> for super::super::DependencyObject {
    fn from(value: &VirtualSurfaceImageSource) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject>
    for VirtualSurfaceImageSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::super::DependencyObject>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject>
    for &VirtualSurfaceImageSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::super::DependencyObject>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
unsafe impl ::std::marker::Send for VirtualSurfaceImageSource {}
unsafe impl ::std::marker::Sync for VirtualSurfaceImageSource {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WriteableBitmap(::windows::runtime::IInspectable);
impl WriteableBitmap {
    #[cfg(feature = "orage_Streams")]
    pub fn PixelBuffer(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::super::super::Windows::Storage::Streams::IBuffer,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::super::Windows::Storage::Streams::IBuffer>(
                result__,
            )
        }
    }
    pub fn Invalidate(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn CreateInstanceWithDimensions(
        pixelwidth: i32,
        pixelheight: i32,
    ) -> ::windows::runtime::Result<WriteableBitmap> {
        Self::IWriteableBitmapFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                pixelwidth,
                pixelheight,
                &mut result__,
            )
            .from_abi::<WriteableBitmap>(result__)
        })
    }
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
    pub fn PixelWidth(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IBitmapSource>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn PixelHeight(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IBitmapSource>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "orage_Streams")]
    pub fn SetSource<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::super::Windows::Storage::Streams::IRandomAccessStream,
        >,
    >(
        &self,
        streamsource: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBitmapSource>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                streamsource.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "undation", feature = "orage_Streams"))]
    pub fn SetSourceAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::super::Windows::Storage::Streams::IRandomAccessStream,
        >,
    >(
        &self,
        streamsource: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::super::Windows::Foundation::IAsyncAction,
    > {
        let this = &::windows::runtime::Interface::cast::<IBitmapSource>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                streamsource.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::super::Windows::Foundation::IAsyncAction>(
                result__,
            )
        }
    }
    pub fn IWriteableBitmapFactory<
        R,
        F: FnOnce(&IWriteableBitmapFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            WriteableBitmap,
            IWriteableBitmapFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WriteableBitmap {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Media.Imaging.WriteableBitmap;{78c824a9-0e43-5f1e-93bc-d046cca82b7e})" ) ;
}
unsafe impl ::windows::runtime::Interface for WriteableBitmap {
    type Vtable = IWriteableBitmap_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2026382505,
        3651,
        24350,
        [147, 188, 208, 70, 204, 168, 43, 126],
    );
}
impl ::windows::runtime::RuntimeName for WriteableBitmap {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.WriteableBitmap";
}
impl ::std::convert::From<WriteableBitmap> for ::windows::runtime::IUnknown {
    fn from(value: WriteableBitmap) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&WriteableBitmap> for ::windows::runtime::IUnknown {
    fn from(value: &WriteableBitmap) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for WriteableBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &WriteableBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<WriteableBitmap> for ::windows::runtime::IInspectable {
    fn from(value: WriteableBitmap) -> Self {
        value.0
    }
}
impl ::std::convert::From<&WriteableBitmap> for ::windows::runtime::IInspectable {
    fn from(value: &WriteableBitmap) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for WriteableBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WriteableBitmap
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<WriteableBitmap> for BitmapSource {
    fn from(value: WriteableBitmap) -> Self {
        ::std::convert::Into::<BitmapSource>::into(&value)
    }
}
impl ::std::convert::From<&WriteableBitmap> for BitmapSource {
    fn from(value: &WriteableBitmap) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, BitmapSource> for WriteableBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, BitmapSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<BitmapSource>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, BitmapSource> for &WriteableBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, BitmapSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<BitmapSource>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<WriteableBitmap> for super::ImageSource {
    fn from(value: WriteableBitmap) -> Self {
        ::std::convert::Into::<super::ImageSource>::into(&value)
    }
}
impl ::std::convert::From<&WriteableBitmap> for super::ImageSource {
    fn from(value: &WriteableBitmap) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for WriteableBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::ImageSource>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ImageSource> for &WriteableBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ImageSource> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::ImageSource>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<WriteableBitmap> for super::super::DependencyObject {
    fn from(value: WriteableBitmap) -> Self {
        ::std::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::std::convert::From<&WriteableBitmap> for super::super::DependencyObject {
    fn from(value: &WriteableBitmap) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for WriteableBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::super::DependencyObject>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for &WriteableBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::super::DependencyObject>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
unsafe impl ::std::marker::Send for WriteableBitmap {}
unsafe impl ::std::marker::Sync for WriteableBitmap {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct XamlRenderingBackgroundTask(::windows::runtime::IInspectable);
impl XamlRenderingBackgroundTask {
    #[cfg(feature = "plicationModel_Background")]    pub fn OnRun < 'a , Param0 : :: windows :: runtime :: IntoParam < 'a , super::super::super::super::super::Windows::ApplicationModel::Background:: IBackgroundTaskInstance > , > ( & self , taskinstance : Param0 , ) -> :: windows :: runtime :: Result < ( ) >{
        let this =
            &::windows::runtime::Interface::cast::<IXamlRenderingBackgroundTaskOverrides>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                taskinstance.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for XamlRenderingBackgroundTask {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Media.Imaging.XamlRenderingBackgroundTask;{7807000c-a050-5121-ac74-3322d5358e39})" ) ;
}
unsafe impl ::windows::runtime::Interface for XamlRenderingBackgroundTask {
    type Vtable = IXamlRenderingBackgroundTask_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2013724684,
        41040,
        20769,
        [172, 116, 51, 34, 213, 53, 142, 57],
    );
}
impl ::windows::runtime::RuntimeName for XamlRenderingBackgroundTask {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.XamlRenderingBackgroundTask";
}
impl ::std::convert::From<XamlRenderingBackgroundTask> for ::windows::runtime::IUnknown {
    fn from(value: XamlRenderingBackgroundTask) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&XamlRenderingBackgroundTask> for ::windows::runtime::IUnknown {
    fn from(value: &XamlRenderingBackgroundTask) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for XamlRenderingBackgroundTask
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &XamlRenderingBackgroundTask
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<XamlRenderingBackgroundTask> for ::windows::runtime::IInspectable {
    fn from(value: XamlRenderingBackgroundTask) -> Self {
        value.0
    }
}
impl ::std::convert::From<&XamlRenderingBackgroundTask> for ::windows::runtime::IInspectable {
    fn from(value: &XamlRenderingBackgroundTask) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for XamlRenderingBackgroundTask
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a XamlRenderingBackgroundTask
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for XamlRenderingBackgroundTask {}
unsafe impl ::std::marker::Sync for XamlRenderingBackgroundTask {}
