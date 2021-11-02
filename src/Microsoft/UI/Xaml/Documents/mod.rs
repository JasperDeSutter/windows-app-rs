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
pub struct Block(::windows::runtime::IInspectable);
impl Block {
    pub fn TextAlignment(&self) -> ::windows::runtime::Result<super::TextAlignment> {
        let this = self;
        unsafe {
            let mut result__: super::TextAlignment = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::TextAlignment>(result__)
        }
    }
    pub fn SetTextAlignment(&self, value: super::TextAlignment) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn HorizontalTextAlignment(&self) -> ::windows::runtime::Result<super::TextAlignment> {
        let this = self;
        unsafe {
            let mut result__: super::TextAlignment = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::TextAlignment>(result__)
        }
    }
    pub fn SetHorizontalTextAlignment(
        &self,
        value: super::TextAlignment,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn LineHeight(&self) -> ::windows::runtime::Result<f64> {
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
    pub fn SetLineHeight(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn LineStackingStrategy(&self) -> ::windows::runtime::Result<super::LineStackingStrategy> {
        let this = self;
        unsafe {
            let mut result__: super::LineStackingStrategy = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::LineStackingStrategy>(result__)
        }
    }
    pub fn SetLineStackingStrategy(
        &self,
        value: super::LineStackingStrategy,
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
    pub fn Margin(&self) -> ::windows::runtime::Result<super::Thickness> {
        let this = self;
        unsafe {
            let mut result__: super::Thickness = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Thickness>(result__)
        }
    }
    pub fn SetMargin<'a, Param0: ::windows::runtime::IntoParam<'a, super::Thickness>>(
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
    pub fn TextAlignmentProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IBlockStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn HorizontalTextAlignmentProperty() -> ::windows::runtime::Result<super::DependencyProperty>
    {
        Self::IBlockStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn LineHeightProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IBlockStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn LineStackingStrategyProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IBlockStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn MarginProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IBlockStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Core::CoreDispatcher>
    {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
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
    pub fn FontSize(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::runtime::Result<super::Media::FontFamily> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Media::FontFamily>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn FontWeight(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::FontWeight> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::FontWeight =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::FontWeight>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::UI::Text::FontWeight,
        >,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn FontStyle(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::FontStyle> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::FontStyle =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::FontStyle>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetFontStyle(
        &self,
        value: super::super::super::super::Windows::UI::Text::FontStyle,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn FontStretch(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::FontStretch>
    {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::FontStretch =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::FontStretch>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetFontStretch(
        &self,
        value: super::super::super::super::Windows::UI::Text::FontStretch,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::runtime::Result<super::Media::Brush> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, Param0: ::windows::runtime::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
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
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn TextDecorations(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::TextDecorations>
    {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::TextDecorations =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::TextDecorations>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetTextDecorations(
        &self,
        value: super::super::super::super::Windows::UI::Text::TextDecorations,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn ContentStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ContentEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).32)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetAccessKey<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).34)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(
        &self,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).36)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).38)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).40)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::runtime::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).41)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).42)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).43)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).44)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).46)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn XamlRoot(&self) -> ::windows::runtime::Result<super::XamlRoot> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).47)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    pub fn SetXamlRoot<'a, Param0: ::windows::runtime::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).48)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn AccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).49)(
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
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).50)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn AccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).51)(
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
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).52)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn AccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).53)(
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
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).54)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FindName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).55)(
                ::std::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElementOverrides>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn IBlockStatics<R, F: FnOnce(&IBlockStatics) -> ::windows::runtime::Result<R>>(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Block, IBlockStatics> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Block {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Block;{8149d507-672f-5fd5-a10a-351389ba9659})",
    );
}
unsafe impl ::windows::runtime::Interface for Block {
    type Vtable = IBlock_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2169099527,
        26415,
        24533,
        [161, 10, 53, 19, 137, 186, 150, 89],
    );
}
impl ::windows::runtime::RuntimeName for Block {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Block";
}
impl ::std::convert::From<Block> for ::windows::runtime::IUnknown {
    fn from(value: Block) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Block> for ::windows::runtime::IUnknown {
    fn from(value: &Block) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Block {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Block {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<Block> for ::windows::runtime::IInspectable {
    fn from(value: Block) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Block> for ::windows::runtime::IInspectable {
    fn from(value: &Block) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Block {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Block {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<Block> for TextElement {
    fn from(value: Block) -> Self {
        ::std::convert::Into::<TextElement>::into(&value)
    }
}
impl ::std::convert::From<&Block> for TextElement {
    fn from(value: &Block) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for Block {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<TextElement>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for &Block {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<TextElement>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<Block> for super::DependencyObject {
    fn from(value: Block) -> Self {
        ::std::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::std::convert::From<&Block> for super::DependencyObject {
    fn from(value: &Block) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for Block {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::DependencyObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for &Block {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::DependencyObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for Block {}
unsafe impl ::std::marker::Sync for Block {}
#[cfg(feature = "undation_Collections")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct BlockCollection(::windows::runtime::IInspectable);
#[cfg(feature = "undation_Collections")]
impl BlockCollection {
    #[cfg(feature = "undation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::runtime::Result<Block> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                index,
                &mut result__,
            )
            .from_abi::<Block>(result__)
        }
    }
    #[cfg(feature = "undation_Collections")]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
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
    #[cfg(feature = "undation_Collections")]
    pub fn GetView(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::Collections::IVectorView<Block>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .8 ) ( :: std :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < super::super::super::super::Windows::Foundation::Collections:: IVectorView :: < Block > > ( result__ )
        }
    }
    #[cfg(feature = "undation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::runtime::IntoParam<'a, Block>>(
        &self,
        value: Param0,
        index: &mut u32,
    ) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
                index,
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "undation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::runtime::IntoParam<'a, Block>>(
        &self,
        index: u32,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                index,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::runtime::IntoParam<'a, Block>>(
        &self,
        index: u32,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                index,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                index,
            )
            .ok()
        }
    }
    #[cfg(feature = "undation_Collections")]
    pub fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, Block>>(
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
    #[cfg(feature = "undation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "undation_Collections")]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "undation_Collections")]
    pub fn GetMany(
        &self,
        startindex: u32,
        items: &mut [<Block as ::windows::runtime::Abi>::DefaultType],
    ) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                startindex,
                items.len() as u32,
                ::std::mem::transmute_copy(&items),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "undation_Collections")]
    pub fn ReplaceAll(
        &self,
        items: &[<Block as ::windows::runtime::Abi>::DefaultType],
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                items.len() as u32,
                ::std::mem::transmute(items.as_ptr()),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation_Collections")]
    pub fn First(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::Collections::IIterator<Block>,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::super::super::Windows::Foundation::Collections::IIterable<Block>,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .6 ) ( :: std :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < super::super::super::super::Windows::Foundation::Collections:: IIterator :: < Block > > ( result__ )
        }
    }
}
#[cfg(feature = "undation_Collections")]
unsafe impl ::windows::runtime::RuntimeType for BlockCollection {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Documents.BlockCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Microsoft.UI.Xaml.Documents.Block;{8149d507-672f-5fd5-a10a-351389ba9659})))" ) ;
}
#[cfg(feature = "undation_Collections")]
unsafe impl ::windows::runtime::Interface for BlockCollection {
    type Vtable = super::super::super::super::Windows::Foundation::Collections::IVector_abi<Block>;
    const IID : :: windows :: runtime :: GUID = :: windows :: runtime :: GUID :: from_signature ( < super::super::super::super::Windows::Foundation::Collections:: IVector :: < Block > as :: windows :: runtime :: RuntimeType > :: SIGNATURE ) ;
}
#[cfg(feature = "undation_Collections")]
impl ::windows::runtime::RuntimeName for BlockCollection {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.BlockCollection";
}
#[cfg(feature = "undation_Collections")]
impl ::std::convert::From<BlockCollection> for ::windows::runtime::IUnknown {
    fn from(value: BlockCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "undation_Collections")]
impl ::std::convert::From<&BlockCollection> for ::windows::runtime::IUnknown {
    fn from(value: &BlockCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "undation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BlockCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "undation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &BlockCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "undation_Collections")]
impl ::std::convert::From<BlockCollection> for ::windows::runtime::IInspectable {
    fn from(value: BlockCollection) -> Self {
        value.0
    }
}
#[cfg(feature = "undation_Collections")]
impl ::std::convert::From<&BlockCollection> for ::windows::runtime::IInspectable {
    fn from(value: &BlockCollection) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "undation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BlockCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "undation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a BlockCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "undation_Collections")]
impl ::std::convert::From<BlockCollection>
    for super::super::super::super::Windows::Foundation::Collections::IVector<Block>
{
    fn from(value: BlockCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "undation_Collections")]
impl ::std::convert::From<&BlockCollection>
    for super::super::super::super::Windows::Foundation::Collections::IVector<Block>
{
    fn from(value: &BlockCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "undation_Collections")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::super::super::Windows::Foundation::Collections::IVector<Block>,
    > for BlockCollection
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::super::super::Windows::Foundation::Collections::IVector<Block>,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::super::super::Windows::Foundation::Collections::IVector<Block>,
        >::into(self))
    }
}
#[cfg(feature = "undation_Collections")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::super::super::Windows::Foundation::Collections::IVector<Block>,
    > for &BlockCollection
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::super::super::Windows::Foundation::Collections::IVector<Block>,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::super::super::Windows::Foundation::Collections::IVector<Block>,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "undation_Collections")]
impl ::std::convert::TryFrom<BlockCollection>
    for super::super::super::super::Windows::Foundation::Collections::IIterable<Block>
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: BlockCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "undation_Collections")]
impl ::std::convert::TryFrom<&BlockCollection>
    for super::super::super::super::Windows::Foundation::Collections::IIterable<Block>
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &BlockCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "undation_Collections")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::super::super::Windows::Foundation::Collections::IIterable<Block>,
    > for BlockCollection
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::super::super::Windows::Foundation::Collections::IIterable<Block>,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "undation_Collections")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::super::super::Windows::Foundation::Collections::IIterable<Block>,
    > for &BlockCollection
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::super::super::Windows::Foundation::Collections::IIterable<Block>,
    > {
        ::std::convert::TryInto::<
            super::super::super::super::Windows::Foundation::Collections::IIterable<Block>,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "undation_Collections")]
unsafe impl ::std::marker::Send for BlockCollection {}
#[cfg(feature = "undation_Collections")]
unsafe impl ::std::marker::Sync for BlockCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for BlockCollection {
    type Item = Block;
    type IntoIter =
        super::super::super::super::Windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for &BlockCollection {
    type Item = Block;
    type IntoIter =
        super::super::super::super::Windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::super::Windows::Foundation::Collections::VectorIterator::new(
            ::std::convert::TryInto::try_into(self).ok(),
        )
    }
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct Bold(::windows::runtime::IInspectable);
impl Bold {
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
            Bold,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Core::CoreDispatcher>
    {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[cfg(feature = "undation_Collections")]
    pub fn Inlines(&self) -> ::windows::runtime::Result<InlineCollection> {
        let this = &::windows::runtime::Interface::cast::<ISpan>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InlineCollection>(result__)
        }
    }
    #[cfg(feature = "undation_Collections")]
    pub fn SetInlines<'a, Param0: ::windows::runtime::IntoParam<'a, InlineCollection>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISpan>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
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
    pub fn FontSize(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::runtime::Result<super::Media::FontFamily> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Media::FontFamily>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn FontWeight(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::FontWeight> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::FontWeight =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::FontWeight>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::UI::Text::FontWeight,
        >,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn FontStyle(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::FontStyle> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::FontStyle =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::FontStyle>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetFontStyle(
        &self,
        value: super::super::super::super::Windows::UI::Text::FontStyle,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn FontStretch(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::FontStretch>
    {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::FontStretch =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::FontStretch>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetFontStretch(
        &self,
        value: super::super::super::super::Windows::UI::Text::FontStretch,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::runtime::Result<super::Media::Brush> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, Param0: ::windows::runtime::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
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
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn TextDecorations(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::TextDecorations>
    {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::TextDecorations =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::TextDecorations>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetTextDecorations(
        &self,
        value: super::super::super::super::Windows::UI::Text::TextDecorations,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn ContentStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ContentEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).32)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetAccessKey<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).34)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(
        &self,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).36)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).38)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).40)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::runtime::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).41)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).42)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).43)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).44)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).46)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn XamlRoot(&self) -> ::windows::runtime::Result<super::XamlRoot> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).47)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    pub fn SetXamlRoot<'a, Param0: ::windows::runtime::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).48)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn AccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).49)(
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
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).50)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn AccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).51)(
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
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).52)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn AccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).53)(
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
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).54)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FindName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).55)(
                ::std::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElementOverrides>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Bold {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Bold;{241a5f5a-c164-597f-b0db-fac7431297f2})",
    );
}
unsafe impl ::windows::runtime::Interface for Bold {
    type Vtable = IBold_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        605708122,
        49508,
        22911,
        [176, 219, 250, 199, 67, 18, 151, 242],
    );
}
impl ::windows::runtime::RuntimeName for Bold {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Bold";
}
impl ::std::convert::From<Bold> for ::windows::runtime::IUnknown {
    fn from(value: Bold) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Bold> for ::windows::runtime::IUnknown {
    fn from(value: &Bold) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Bold {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Bold {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<Bold> for ::windows::runtime::IInspectable {
    fn from(value: Bold) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Bold> for ::windows::runtime::IInspectable {
    fn from(value: &Bold) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Bold {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Bold {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<Bold> for Span {
    fn from(value: Bold) -> Self {
        ::std::convert::Into::<Span>::into(&value)
    }
}
impl ::std::convert::From<&Bold> for Span {
    fn from(value: &Bold) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Span> for Bold {
    fn into_param(self) -> ::windows::runtime::Param<'a, Span> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<Span>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Span> for &Bold {
    fn into_param(self) -> ::windows::runtime::Param<'a, Span> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<Span>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<Bold> for Inline {
    fn from(value: Bold) -> Self {
        ::std::convert::Into::<Inline>::into(&value)
    }
}
impl ::std::convert::From<&Bold> for Inline {
    fn from(value: &Bold) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Inline> for Bold {
    fn into_param(self) -> ::windows::runtime::Param<'a, Inline> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<Inline>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Inline> for &Bold {
    fn into_param(self) -> ::windows::runtime::Param<'a, Inline> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<Inline>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<Bold> for TextElement {
    fn from(value: Bold) -> Self {
        ::std::convert::Into::<TextElement>::into(&value)
    }
}
impl ::std::convert::From<&Bold> for TextElement {
    fn from(value: &Bold) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for Bold {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<TextElement>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for &Bold {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<TextElement>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<Bold> for super::DependencyObject {
    fn from(value: Bold) -> Self {
        ::std::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::std::convert::From<&Bold> for super::DependencyObject {
    fn from(value: &Bold) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for Bold {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::DependencyObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for &Bold {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::DependencyObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for Bold {}
unsafe impl ::std::marker::Sync for Bold {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct Glyphs(::windows::runtime::IInspectable);
impl Glyphs {
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
            Glyphs,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn UnicodeString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn SetUnicodeString<
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
    pub fn Indices(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn SetIndices<
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
    #[cfg(feature = "undation")]
    pub fn FontUri(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn SetFontUri<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Windows::Foundation::Uri>,
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
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn StyleSimulations(&self) -> ::windows::runtime::Result<super::Media::StyleSimulations> {
        let this = self;
        unsafe {
            let mut result__: super::Media::StyleSimulations = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::StyleSimulations>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetStyleSimulations(
        &self,
        value: super::Media::StyleSimulations,
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
    pub fn FontRenderingEmSize(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetFontRenderingEmSize(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn OriginX(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetOriginX(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn OriginY(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetOriginY(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Fill(&self) -> ::windows::runtime::Result<super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFill<'a, Param0: ::windows::runtime::IntoParam<'a, super::Media::Brush>>(
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
    pub fn IsColorFontEnabled(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn SetIsColorFontEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn ColorFontPaletteIndex(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetColorFontPaletteIndex(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn UnicodeStringProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IndicesProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn FontUriProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn StyleSimulationsProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn FontRenderingEmSizeProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn OriginXProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn OriginYProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn FillProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IsColorFontEnabledProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ColorFontPaletteIndexProperty() -> ::windows::runtime::Result<super::DependencyProperty>
    {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Composition")]
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Composition::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::Composition::IAnimationObject,
        >(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn GetVisualInternal(
        &self,
    ) -> ::windows::runtime::Result<super::super::Composition::Visual> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::Composition::IVisualElement2,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Composition::Visual>(result__)
        }
    }
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Core::CoreDispatcher>
    {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[cfg(feature = "undation_Collections")]
    pub fn Triggers(&self) -> ::windows::runtime::Result<super::TriggerCollection> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::TriggerCollection>(result__)
        }
    }
    pub fn Resources(&self) -> ::windows::runtime::Result<super::ResourceDictionary> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ResourceDictionary>(result__)
        }
    }
    pub fn SetResources<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ResourceDictionary>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Tag(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    pub fn SetTag<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
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
    pub fn SetLanguage<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ActualWidth(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn ActualHeight(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn Width(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetWidth(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn Height(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetHeight(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn MinWidth(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetMinWidth(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn MaxWidth(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetMaxWidth(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn MinHeight(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetMinHeight(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn MaxHeight(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetMaxHeight(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn HorizontalAlignment(&self) -> ::windows::runtime::Result<super::HorizontalAlignment> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: super::HorizontalAlignment = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::HorizontalAlignment>(result__)
        }
    }
    pub fn SetHorizontalAlignment(
        &self,
        value: super::HorizontalAlignment,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).28)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn VerticalAlignment(&self) -> ::windows::runtime::Result<super::VerticalAlignment> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: super::VerticalAlignment = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::VerticalAlignment>(result__)
        }
    }
    pub fn SetVerticalAlignment(
        &self,
        value: super::VerticalAlignment,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).30)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn Margin(&self) -> ::windows::runtime::Result<super::Thickness> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: super::Thickness = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Thickness>(result__)
        }
    }
    pub fn SetMargin<'a, Param0: ::windows::runtime::IntoParam<'a, super::Thickness>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).32)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).34)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn BaseUri(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::Foundation::Uri> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Uri>(result__)
        }
    }
    pub fn DataContext(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    pub fn SetDataContext<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).37)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).38)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).39)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn FocusVisualMargin(&self) -> ::windows::runtime::Result<super::Thickness> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: super::Thickness = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).40)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Thickness>(result__)
        }
    }
    pub fn SetFocusVisualMargin<'a, Param0: ::windows::runtime::IntoParam<'a, super::Thickness>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).41)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FocusVisualSecondaryThickness(&self) -> ::windows::runtime::Result<super::Thickness> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: super::Thickness = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).42)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Thickness>(result__)
        }
    }
    pub fn SetFocusVisualSecondaryThickness<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Thickness>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).43)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FocusVisualPrimaryThickness(&self) -> ::windows::runtime::Result<super::Thickness> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: super::Thickness = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).44)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Thickness>(result__)
        }
    }
    pub fn SetFocusVisualPrimaryThickness<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Thickness>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).45)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FocusVisualSecondaryBrush(&self) -> ::windows::runtime::Result<super::Media::Brush> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).46)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFocusVisualSecondaryBrush<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Media::Brush>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).47)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FocusVisualPrimaryBrush(&self) -> ::windows::runtime::Result<super::Media::Brush> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).48)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFocusVisualPrimaryBrush<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Media::Brush>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).49)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn AllowFocusWhenDisabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).50)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusWhenDisabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).51)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn Style(&self) -> ::windows::runtime::Result<super::Style> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).52)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Style>(result__)
        }
    }
    pub fn SetStyle<'a, Param0: ::windows::runtime::IntoParam<'a, super::Style>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).53)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Parent(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).54)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn FlowDirection(&self) -> ::windows::runtime::Result<super::FlowDirection> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: super::FlowDirection = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).55)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::FlowDirection>(result__)
        }
    }
    pub fn SetFlowDirection(&self, value: super::FlowDirection) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).56)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn RequestedTheme(&self) -> ::windows::runtime::Result<super::ElementTheme> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: super::ElementTheme = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).57)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ElementTheme>(result__)
        }
    }
    pub fn SetRequestedTheme(&self, value: super::ElementTheme) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).58)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsLoaded(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).59)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn ActualTheme(&self) -> ::windows::runtime::Result<super::ElementTheme> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: super::ElementTheme = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).60)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ElementTheme>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn Loaded<'a, Param0: ::windows::runtime::IntoParam<'a, super::RoutedEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).61)(
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
    pub fn RemoveLoaded<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).62)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn Unloaded<'a, Param0: ::windows::runtime::IntoParam<'a, super::RoutedEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).63)(
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
    pub fn RemoveUnloaded<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).64)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn DataContextChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                super::FrameworkElement,
                super::DataContextChangedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).65)(
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
    pub fn RemoveDataContextChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).66)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn SizeChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::SizeChangedEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).67)(
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
    pub fn RemoveSizeChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).68)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn LayoutUpdated<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventHandler<
                ::windows::runtime::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).69)(
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
    pub fn RemoveLayoutUpdated<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).70)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn Loading<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                super::FrameworkElement,
                ::windows::runtime::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).71)(
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
    pub fn RemoveLoading<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).72)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn ActualThemeChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                super::FrameworkElement,
                ::windows::runtime::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).73)(
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
    pub fn RemoveActualThemeChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).74)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn EffectiveViewportChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                super::FrameworkElement,
                super::EffectiveViewportChangedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).75)(
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
    pub fn RemoveEffectiveViewportChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).76)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FindName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).77)(
                ::std::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Data")]
    pub fn SetBinding<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, super::Data::BindingBase>,
    >(
        &self,
        dp: Param0,
        binding: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).78)(
                ::std::mem::transmute_copy(this),
                dp.into_param().abi(),
                binding.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Data")]
    pub fn GetBindingExpression<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<super::Data::BindingExpression> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).79)(
                ::std::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::Data::BindingExpression>(result__)
        }
    }
    pub fn InvalidateViewport(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElementProtected>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn DesiredSize(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::Foundation::Size> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::Foundation::Size =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Size>(result__)
        }
    }
    pub fn AllowDrop(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowDrop(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn Opacity(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetOpacity(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Clip(&self) -> ::windows::runtime::Result<super::Media::RectangleGeometry> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::RectangleGeometry>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetClip<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Media::RectangleGeometry>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn RenderTransform(&self) -> ::windows::runtime::Result<super::Media::Transform> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Transform>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetRenderTransform<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Media::Transform>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Projection(&self) -> ::windows::runtime::Result<super::Media::Projection> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Projection>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetProjection<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Media::Projection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media_Media3D")]
    pub fn Transform3D(&self) -> ::windows::runtime::Result<super::Media::Media3D::Transform3D> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Media3D::Transform3D>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media_Media3D")]
    pub fn SetTransform3D<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Media::Media3D::Transform3D>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn RenderTransformOrigin(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::Foundation::Point> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::Foundation::Point =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn SetRenderTransformOrigin<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Windows::Foundation::Point>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IsHitTestVisible(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsHitTestVisible(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn Visibility(&self) -> ::windows::runtime::Result<super::Visibility> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Visibility = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Visibility>(result__)
        }
    }
    pub fn SetVisibility(&self, value: super::Visibility) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn RenderSize(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::Foundation::Size> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::Foundation::Size =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Size>(result__)
        }
    }
    pub fn UseLayoutRounding(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetUseLayoutRounding(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).27)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Media_Animation", feature = "undation_Collections"))]
    pub fn Transitions(
        &self,
    ) -> ::windows::runtime::Result<super::Media::Animation::TransitionCollection> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Animation::TransitionCollection>(result__)
        }
    }
    #[cfg(all(feature = "UI_Xaml_Media_Animation", feature = "undation_Collections"))]
    pub fn SetTransitions<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Media::Animation::TransitionCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).29)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CacheMode(&self) -> ::windows::runtime::Result<super::Media::CacheMode> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::CacheMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCacheMode<'a, Param0: ::windows::runtime::IntoParam<'a, super::Media::CacheMode>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).31)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IsTapEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTapEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).33)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsDoubleTapEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsDoubleTapEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).35)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn CanDrag(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetCanDrag(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).37)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsRightTapEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).38)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsRightTapEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).39)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsHoldingEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).40)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsHoldingEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).41)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationMode(&self) -> ::windows::runtime::Result<super::Input::ManipulationModes> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Input::ManipulationModes = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).42)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::ManipulationModes>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetManipulationMode(
        &self,
        value: super::Input::ManipulationModes,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).43)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation_Collections"))]
    pub fn PointerCaptures(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::Collections::IVectorView<
            super::Input::Pointer,
        >,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).44)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Collections::IVectorView<
                super::Input::Pointer,
            >>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub fn ContextFlyout(
        &self,
    ) -> ::windows::runtime::Result<super::Controls::Primitives::FlyoutBase> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Controls::Primitives::FlyoutBase>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub fn SetContextFlyout<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Controls::Primitives::FlyoutBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).46)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CompositeMode(&self) -> ::windows::runtime::Result<super::Media::ElementCompositeMode> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Media::ElementCompositeMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).47)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::ElementCompositeMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCompositeMode(
        &self,
        value: super::Media::ElementCompositeMode,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).48)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Media", feature = "undation_Collections"))]
    pub fn Lights(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::Collections::IVector<
            super::Media::XamlLight,
        >,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).49)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Collections::IVector<
                super::Media::XamlLight,
            >>(result__)
        }
    }
    pub fn CanBeScrollAnchor(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).50)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetCanBeScrollAnchor(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).51)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).52)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(
        &self,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).53)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).54)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).55)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).56)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).57)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).58)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetAccessKey<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).59)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::runtime::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).60)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).61)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).62)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).63)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).64)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).65)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipTarget(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).66)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetKeyTipTarget<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).67)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusKeyboardNavigation(
        &self,
    ) -> ::windows::runtime::Result<super::Input::XYFocusKeyboardNavigationMode> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Input::XYFocusKeyboardNavigationMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).68)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::XYFocusKeyboardNavigationMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusKeyboardNavigation(
        &self,
        value: super::Input::XYFocusKeyboardNavigationMode,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).69)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusUpNavigationStrategy(
        &self,
    ) -> ::windows::runtime::Result<super::Input::XYFocusNavigationStrategy> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).70)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusUpNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).71)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusDownNavigationStrategy(
        &self,
    ) -> ::windows::runtime::Result<super::Input::XYFocusNavigationStrategy> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).72)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusDownNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).73)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusLeftNavigationStrategy(
        &self,
    ) -> ::windows::runtime::Result<super::Input::XYFocusNavigationStrategy> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).74)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusLeftNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).75)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusRightNavigationStrategy(
        &self,
    ) -> ::windows::runtime::Result<super::Input::XYFocusNavigationStrategy> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).76)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusRightNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).77)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation_Collections"))]
    pub fn KeyboardAccelerators(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::Collections::IVector<
            super::Input::KeyboardAccelerator,
        >,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).78)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Collections::IVector<
                super::Input::KeyboardAccelerator,
            >>(result__)
        }
    }
    pub fn KeyboardAcceleratorPlacementTarget(
        &self,
    ) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).79)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetKeyboardAcceleratorPlacementTarget<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).80)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyboardAcceleratorPlacementMode(
        &self,
    ) -> ::windows::runtime::Result<super::Input::KeyboardAcceleratorPlacementMode> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyboardAcceleratorPlacementMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).81)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyboardAcceleratorPlacementMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyboardAcceleratorPlacementMode(
        &self,
        value: super::Input::KeyboardAcceleratorPlacementMode,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).82)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn HighContrastAdjustment(
        &self,
    ) -> ::windows::runtime::Result<super::ElementHighContrastAdjustment> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::ElementHighContrastAdjustment = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).83)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ElementHighContrastAdjustment>(result__)
        }
    }
    pub fn SetHighContrastAdjustment(
        &self,
        value: super::ElementHighContrastAdjustment,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).84)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn TabFocusNavigation(
        &self,
    ) -> ::windows::runtime::Result<super::Input::KeyboardNavigationMode> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyboardNavigationMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).85)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyboardNavigationMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetTabFocusNavigation(
        &self,
        value: super::Input::KeyboardNavigationMode,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).86)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn OpacityTransition(&self) -> ::windows::runtime::Result<super::ScalarTransition> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).87)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ScalarTransition>(result__)
        }
    }
    pub fn SetOpacityTransition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ScalarTransition>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).88)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation_Numerics")]
    pub fn Translation(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::Numerics::Vector3,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::Foundation::Numerics::Vector3 =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).89)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Numerics::Vector3>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation_Numerics")]
    pub fn SetTranslation<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::Numerics::Vector3,
        >,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).90)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn TranslationTransition(&self) -> ::windows::runtime::Result<super::Vector3Transition> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).91)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Vector3Transition>(result__)
        }
    }
    pub fn SetTranslationTransition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Vector3Transition>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).92)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Rotation(&self) -> ::windows::runtime::Result<f32> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).93)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn SetRotation(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).94)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn RotationTransition(&self) -> ::windows::runtime::Result<super::ScalarTransition> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).95)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ScalarTransition>(result__)
        }
    }
    pub fn SetRotationTransition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ScalarTransition>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).96)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation_Numerics")]
    pub fn Scale(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::Numerics::Vector3,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::Foundation::Numerics::Vector3 =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).97)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Numerics::Vector3>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation_Numerics")]
    pub fn SetScale<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::Numerics::Vector3,
        >,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).98)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ScaleTransition(&self) -> ::windows::runtime::Result<super::Vector3Transition> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).99)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Vector3Transition>(result__)
        }
    }
    pub fn SetScaleTransition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Vector3Transition>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).100)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation_Numerics")]
    pub fn TransformMatrix(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::Numerics::Matrix4x4,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::Foundation::Numerics::Matrix4x4 =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).101)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Numerics::Matrix4x4>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation_Numerics")]
    pub fn SetTransformMatrix<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::Numerics::Matrix4x4,
        >,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).102)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation_Numerics")]
    pub fn CenterPoint(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::Numerics::Vector3,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::Foundation::Numerics::Vector3 =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).103)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Numerics::Vector3>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation_Numerics")]
    pub fn SetCenterPoint<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::Numerics::Vector3,
        >,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).104)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation_Numerics")]
    pub fn RotationAxis(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::Numerics::Vector3,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::Foundation::Numerics::Vector3 =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).105)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Numerics::Vector3>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation_Numerics")]
    pub fn SetRotationAxis<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::Numerics::Vector3,
        >,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).106)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation_Numerics")]
    pub fn ActualOffset(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::Numerics::Vector3,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::Foundation::Numerics::Vector3 =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).107)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Numerics::Vector3>(
                result__,
            )
        }
    }
    #[cfg(feature = "undation_Numerics")]
    pub fn ActualSize(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::Numerics::Vector2,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::Foundation::Numerics::Vector2 =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).108)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Numerics::Vector2>(
                result__,
            )
        }
    }
    pub fn XamlRoot(&self) -> ::windows::runtime::Result<super::XamlRoot> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).109)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    pub fn SetXamlRoot<'a, Param0: ::windows::runtime::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).110)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Shadow(&self) -> ::windows::runtime::Result<super::Media::Shadow> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).111)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Shadow>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetShadow<'a, Param0: ::windows::runtime::IntoParam<'a, super::Media::Shadow>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).112)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn RasterizationScale(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).113)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetRasterizationScale(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).114)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn FocusState(&self) -> ::windows::runtime::Result<super::FocusState> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::FocusState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).115)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::FocusState>(result__)
        }
    }
    pub fn UseSystemFocusVisuals(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).116)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetUseSystemFocusVisuals(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).117)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn XYFocusLeft(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).118)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetXYFocusLeft<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).119)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn XYFocusRight(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).120)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetXYFocusRight<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).121)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn XYFocusUp(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).122)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetXYFocusUp<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).123)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn XYFocusDown(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).124)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetXYFocusDown<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).125)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IsTabStop(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).126)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTabStop(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).127)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn TabIndex(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).128)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetTabIndex(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).129)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn KeyUp<'a, Param0: ::windows::runtime::IntoParam<'a, super::Input::KeyEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).130)(
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
    pub fn RemoveKeyUp<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).131)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn KeyDown<'a, Param0: ::windows::runtime::IntoParam<'a, super::Input::KeyEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).132)(
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
    pub fn RemoveKeyDown<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).133)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn GotFocus<'a, Param0: ::windows::runtime::IntoParam<'a, super::RoutedEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).134)(
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
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).135)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn LostFocus<'a, Param0: ::windows::runtime::IntoParam<'a, super::RoutedEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).136)(
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
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).137)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn DragStarting<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::DragStartingEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).138)(
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
    pub fn RemoveDragStarting<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).139)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn DropCompleted<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::DropCompletedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).140)(
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
    pub fn RemoveDropCompleted<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).141)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn CharacterReceived<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::Input::CharacterReceivedRoutedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).142)(
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
    pub fn RemoveCharacterReceived<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).143)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn DragEnter<'a, Param0: ::windows::runtime::IntoParam<'a, super::DragEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).144)(
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
    pub fn RemoveDragEnter<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).145)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn DragLeave<'a, Param0: ::windows::runtime::IntoParam<'a, super::DragEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).146)(
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
    pub fn RemoveDragLeave<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).147)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn DragOver<'a, Param0: ::windows::runtime::IntoParam<'a, super::DragEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).148)(
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
    pub fn RemoveDragOver<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).149)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn Drop<'a, Param0: ::windows::runtime::IntoParam<'a, super::DragEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).150)(
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
    pub fn RemoveDrop<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).151)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn PointerPressed<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::PointerEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).152)(
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
    pub fn RemovePointerPressed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).153)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn PointerMoved<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::PointerEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).154)(
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
    pub fn RemovePointerMoved<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).155)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn PointerReleased<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::PointerEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).156)(
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
    pub fn RemovePointerReleased<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).157)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn PointerEntered<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::PointerEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).158)(
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
    pub fn RemovePointerEntered<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).159)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn PointerExited<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::PointerEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).160)(
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
    pub fn RemovePointerExited<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).161)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn PointerCaptureLost<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::PointerEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).162)(
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
    pub fn RemovePointerCaptureLost<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).163)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn PointerCanceled<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::PointerEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).164)(
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
    pub fn RemovePointerCanceled<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).165)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn PointerWheelChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::PointerEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).166)(
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
    pub fn RemovePointerWheelChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).167)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn Tapped<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::TappedEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).168)(
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
    pub fn RemoveTapped<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).169)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn DoubleTapped<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::DoubleTappedEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).170)(
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
    pub fn RemoveDoubleTapped<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).171)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn Holding<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::HoldingEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).172)(
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
    pub fn RemoveHolding<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).173)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn ContextRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::Input::ContextRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).174)(
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
    pub fn RemoveContextRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).175)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn ContextCanceled<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::RoutedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).176)(
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
    pub fn RemoveContextCanceled<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).177)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn RightTapped<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::RightTappedEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).178)(
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
    pub fn RemoveRightTapped<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).179)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn ManipulationStarting<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::ManipulationStartingEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).180)(
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
    pub fn RemoveManipulationStarting<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).181)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn ManipulationInertiaStarting<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::ManipulationInertiaStartingEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).182)(
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
    pub fn RemoveManipulationInertiaStarting<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).183)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn ManipulationStarted<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::ManipulationStartedEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).184)(
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
    pub fn RemoveManipulationStarted<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).185)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn ManipulationDelta<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::ManipulationDeltaEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).186)(
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
    pub fn RemoveManipulationDelta<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).187)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn ManipulationCompleted<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::ManipulationCompletedEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).188)(
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
    pub fn RemoveManipulationCompleted<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).189)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn AccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).190)(
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
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).191)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn AccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).192)(
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
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).193)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn AccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).194)(
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
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).195)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn ProcessKeyboardAccelerators<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::Input::ProcessKeyboardAcceleratorEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).196)(
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
    pub fn RemoveProcessKeyboardAccelerators<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).197)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn GettingFocus<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::Input::GettingFocusEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).198)(
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
    pub fn RemoveGettingFocus<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).199)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn LosingFocus<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::Input::LosingFocusEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).200)(
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
    pub fn RemoveLosingFocus<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).201)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn NoFocusCandidateFound<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::Input::NoFocusCandidateFoundEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).202)(
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
    pub fn RemoveNoFocusCandidateFound<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).203)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn PreviewKeyDown<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::KeyEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).204)(
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
    pub fn RemovePreviewKeyDown<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).205)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn PreviewKeyUp<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::KeyEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).206)(
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
    pub fn RemovePreviewKeyUp<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).207)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn BringIntoViewRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::BringIntoViewRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).208)(
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
    pub fn RemoveBringIntoViewRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).209)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn Measure<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Windows::Foundation::Size>,
    >(
        &self,
        availablesize: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).210)(
                ::std::mem::transmute_copy(this),
                availablesize.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn Arrange<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Windows::Foundation::Rect>,
    >(
        &self,
        finalrect: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).211)(
                ::std::mem::transmute_copy(this),
                finalrect.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn CapturePointer<'a, Param0: ::windows::runtime::IntoParam<'a, super::Input::Pointer>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).212)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ReleasePointerCapture<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::Pointer>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).213)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ReleasePointerCaptures(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).214)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn AddHandler<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::RoutedEvent>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        routedevent: Param0,
        handler: Param1,
        handledeventstoo: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).215)(
                ::std::mem::transmute_copy(this),
                routedevent.into_param().abi(),
                handler.into_param().abi(),
                handledeventstoo,
            )
            .ok()
        }
    }
    pub fn RemoveHandler<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::RoutedEvent>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        routedevent: Param0,
        handler: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).216)(
                ::std::mem::transmute_copy(this),
                routedevent.into_param().abi(),
                handler.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn TransformToVisual<'a, Param0: ::windows::runtime::IntoParam<'a, super::UIElement>>(
        &self,
        visual: Param0,
    ) -> ::windows::runtime::Result<super::Media::GeneralTransform> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).217)(
                ::std::mem::transmute_copy(this),
                visual.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::Media::GeneralTransform>(result__)
        }
    }
    pub fn InvalidateMeasure(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).218)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn InvalidateArrange(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).219)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn UpdateLayout(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).220)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn CancelDirectManipulations(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).221)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Input",
        feature = "plicationModel_DataTransfer",
        feature = "undation"
    ))]    pub fn StartDragAsync < 'a , Param0 : :: windows :: runtime :: IntoParam < 'a , super::super::Input:: PointerPoint > , > ( & self , pointerpoint : Param0 , ) -> :: windows :: runtime :: Result < super::super::super::super::Windows::Foundation:: IAsyncOperation :: < super::super::super::super::Windows::ApplicationModel::DataTransfer:: DataPackageOperation > >{
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .222 ) ( :: std :: mem :: transmute_copy ( this ) , pointerpoint . into_param ( ) . abi ( ) , & mut result__ ) . from_abi :: < super::super::super::super::Windows::Foundation:: IAsyncOperation :: < super::super::super::super::Windows::ApplicationModel::DataTransfer:: DataPackageOperation > > ( result__ )
        }
    }
    pub fn StartBringIntoView(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).223)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn StartBringIntoViewWithOptions<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::BringIntoViewOptions>,
    >(
        &self,
        options: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).224)(
                ::std::mem::transmute_copy(this),
                options.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn TryInvokeKeyboardAccelerator<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::ProcessKeyboardAcceleratorEventArgs>,
    >(
        &self,
        args: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).225)(
                ::std::mem::transmute_copy(this),
                args.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Focus(&self, value: super::FocusState) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).226)(
                ::std::mem::transmute_copy(this),
                value,
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn StartAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Composition::ICompositionAnimationBase>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).227)(
                ::std::mem::transmute_copy(this),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn StopAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Composition::ICompositionAnimationBase>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).228)(
                ::std::mem::transmute_copy(this),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn ProtectedCursor(&self) -> ::windows::runtime::Result<super::super::Input::InputCursor> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElementProtected>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Input::InputCursor>(result__)
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn SetProtectedCursor<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Input::InputCursor>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElementProtected>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn MeasureOverride<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Windows::Foundation::Size>,
    >(
        &self,
        availablesize: Param0,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::Foundation::Size> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElementOverrides>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::Foundation::Size =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                availablesize.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn ArrangeOverride<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Windows::Foundation::Size>,
    >(
        &self,
        finalsize: Param0,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::Foundation::Size> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElementOverrides>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::Foundation::Size =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                finalsize.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Size>(result__)
        }
    }
    pub fn OnApplyTemplate(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElementOverrides>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn GoToElementStateCore<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        statename: Param0,
        usetransitions: bool,
    ) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElementOverrides>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                statename.into_param().abi(),
                usetransitions,
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub fn OnCreateAutomationPeer(
        &self,
    ) -> ::windows::runtime::Result<super::Automation::Peers::AutomationPeer> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElementOverrides>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Automation::Peers::AutomationPeer>(result__)
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElementOverrides>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(all(feature = "undation", feature = "undation_Collections"))]
    pub fn FindSubElementsForTouchTargeting<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Windows::Foundation::Point>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::super::super::Windows::Foundation::Rect>,
    >(
        &self,
        point: Param0,
        boundingrect: Param1,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::Collections::IIterable<
            super::super::super::super::Windows::Foundation::Collections::IIterable<
                super::super::super::super::Windows::Foundation::Point,
            >,
        >,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElementOverrides>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                point.into_param().abi(),
                boundingrect.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Collections::IIterable<
                super::super::super::super::Windows::Foundation::Collections::IIterable<
                    super::super::super::super::Windows::Foundation::Point,
                >,
            >>(result__)
        }
    }
    #[cfg(feature = "undation_Collections")]
    pub fn GetChildrenInTabFocusOrder(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::Collections::IIterable<
            super::DependencyObject,
        >,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElementOverrides>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Collections::IIterable<
                super::DependencyObject,
            >>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnKeyboardAcceleratorInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::KeyboardAcceleratorInvokedEventArgs>,
    >(
        &self,
        args: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElementOverrides>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                args.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnProcessKeyboardAccelerators<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::ProcessKeyboardAcceleratorEventArgs>,
    >(
        &self,
        args: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElementOverrides>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                args.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn OnBringIntoViewRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::BringIntoViewRequestedEventArgs>,
    >(
        &self,
        e: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElementOverrides>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                e.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn PopulatePropertyInfoOverride<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Composition::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        animationpropertyinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElementOverrides>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animationpropertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IGlyphsStatics<R, F: FnOnce(&IGlyphsStatics) -> ::windows::runtime::Result<R>>(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Glyphs, IGlyphsStatics> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Glyphs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Glyphs;{0fbf8cfe-18e7-5e45-9fa3-d2d0927958f4})",
    );
}
unsafe impl ::windows::runtime::Interface for Glyphs {
    type Vtable = IGlyphs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        264211710,
        6375,
        24133,
        [159, 163, 210, 208, 146, 121, 88, 244],
    );
}
impl ::windows::runtime::RuntimeName for Glyphs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Glyphs";
}
impl ::std::convert::From<Glyphs> for ::windows::runtime::IUnknown {
    fn from(value: Glyphs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Glyphs> for ::windows::runtime::IUnknown {
    fn from(value: &Glyphs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Glyphs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Glyphs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<Glyphs> for ::windows::runtime::IInspectable {
    fn from(value: Glyphs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Glyphs> for ::windows::runtime::IInspectable {
    fn from(value: &Glyphs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Glyphs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Glyphs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::std::convert::TryFrom<Glyphs> for super::super::Composition::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: Glyphs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::std::convert::TryFrom<&Glyphs> for super::super::Composition::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &Glyphs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Composition::IAnimationObject> for Glyphs {
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::Composition::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Composition::IAnimationObject>
    for &Glyphs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::Composition::IAnimationObject> {
        ::std::convert::TryInto::<super::super::Composition::IAnimationObject>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::std::convert::TryFrom<Glyphs> for super::super::Composition::IVisualElement {
    type Error = ::windows::runtime::Error;
    fn try_from(value: Glyphs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::std::convert::TryFrom<&Glyphs> for super::super::Composition::IVisualElement {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &Glyphs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Composition::IVisualElement> for Glyphs {
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::Composition::IVisualElement> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Composition::IVisualElement> for &Glyphs {
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::Composition::IVisualElement> {
        ::std::convert::TryInto::<super::super::Composition::IVisualElement>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::std::convert::TryFrom<Glyphs> for super::super::Composition::IVisualElement2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: Glyphs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::std::convert::TryFrom<&Glyphs> for super::super::Composition::IVisualElement2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &Glyphs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Composition::IVisualElement2> for Glyphs {
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::Composition::IVisualElement2> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Composition::IVisualElement2> for &Glyphs {
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::Composition::IVisualElement2> {
        ::std::convert::TryInto::<super::super::Composition::IVisualElement2>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<Glyphs> for super::FrameworkElement {
    fn from(value: Glyphs) -> Self {
        ::std::convert::Into::<super::FrameworkElement>::into(&value)
    }
}
impl ::std::convert::From<&Glyphs> for super::FrameworkElement {
    fn from(value: &Glyphs) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::FrameworkElement> for Glyphs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::FrameworkElement> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::FrameworkElement>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::FrameworkElement> for &Glyphs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::FrameworkElement> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::FrameworkElement>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<Glyphs> for super::UIElement {
    fn from(value: Glyphs) -> Self {
        ::std::convert::Into::<super::UIElement>::into(&value)
    }
}
impl ::std::convert::From<&Glyphs> for super::UIElement {
    fn from(value: &Glyphs) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::UIElement> for Glyphs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::UIElement> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::UIElement>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::UIElement> for &Glyphs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::UIElement> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::UIElement>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<Glyphs> for super::DependencyObject {
    fn from(value: Glyphs) -> Self {
        ::std::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::std::convert::From<&Glyphs> for super::DependencyObject {
    fn from(value: &Glyphs) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for Glyphs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::DependencyObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for &Glyphs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::DependencyObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for Glyphs {}
unsafe impl ::std::marker::Sync for Glyphs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct Hyperlink(::windows::runtime::IInspectable);
impl Hyperlink {
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
            Hyperlink,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "undation")]
    pub fn NavigateUri(
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
    #[cfg(feature = "undation")]
    pub fn SetNavigateUri<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Windows::Foundation::Uri>,
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
    pub fn UnderlineStyle(&self) -> ::windows::runtime::Result<UnderlineStyle> {
        let this = self;
        unsafe {
            let mut result__: UnderlineStyle = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<UnderlineStyle>(result__)
        }
    }
    pub fn SetUnderlineStyle(&self, value: UnderlineStyle) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn XYFocusLeft(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetXYFocusLeft<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
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
    pub fn XYFocusRight(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetXYFocusRight<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
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
    pub fn XYFocusUp(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetXYFocusUp<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(
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
    pub fn XYFocusDown(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetXYFocusDown<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
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
    pub fn ElementSoundMode(&self) -> ::windows::runtime::Result<super::ElementSoundMode> {
        let this = self;
        unsafe {
            let mut result__: super::ElementSoundMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ElementSoundMode>(result__)
        }
    }
    pub fn SetElementSoundMode(
        &self,
        value: super::ElementSoundMode,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn FocusState(&self) -> ::windows::runtime::Result<super::FocusState> {
        let this = self;
        unsafe {
            let mut result__: super::FocusState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::FocusState>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusUpNavigationStrategy(
        &self,
    ) -> ::windows::runtime::Result<super::Input::XYFocusNavigationStrategy> {
        let this = self;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusUpNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusDownNavigationStrategy(
        &self,
    ) -> ::windows::runtime::Result<super::Input::XYFocusNavigationStrategy> {
        let this = self;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusDownNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusLeftNavigationStrategy(
        &self,
    ) -> ::windows::runtime::Result<super::Input::XYFocusNavigationStrategy> {
        let this = self;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusLeftNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusRightNavigationStrategy(
        &self,
    ) -> ::windows::runtime::Result<super::Input::XYFocusNavigationStrategy> {
        let this = self;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusRightNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).28)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsTabStop(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTabStop(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).30)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn TabIndex(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetTabIndex(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).32)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn Click<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                Hyperlink,
                HyperlinkClickEventArgs,
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
    pub fn RemoveClick<
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
    pub fn GotFocus<'a, Param0: ::windows::runtime::IntoParam<'a, super::RoutedEventHandler>>(
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
            (::windows::runtime::Interface::vtable(this).36)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn LostFocus<'a, Param0: ::windows::runtime::IntoParam<'a, super::RoutedEventHandler>>(
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
            (::windows::runtime::Interface::vtable(this).38)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Focus(&self, value: super::FocusState) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(
                ::std::mem::transmute_copy(this),
                value,
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn NavigateUriProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn UnderlineStyleProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn XYFocusLeftProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn XYFocusRightProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn XYFocusUpProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn XYFocusDownProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ElementSoundModeProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn FocusStateProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn XYFocusUpNavigationStrategyProperty(
    ) -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn XYFocusDownNavigationStrategyProperty(
    ) -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn XYFocusLeftNavigationStrategyProperty(
    ) -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn XYFocusRightNavigationStrategyProperty(
    ) -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IsTabStopProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn TabIndexProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Core::CoreDispatcher>
    {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[cfg(feature = "undation_Collections")]
    pub fn Inlines(&self) -> ::windows::runtime::Result<InlineCollection> {
        let this = &::windows::runtime::Interface::cast::<ISpan>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InlineCollection>(result__)
        }
    }
    #[cfg(feature = "undation_Collections")]
    pub fn SetInlines<'a, Param0: ::windows::runtime::IntoParam<'a, InlineCollection>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISpan>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
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
    pub fn FontSize(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::runtime::Result<super::Media::FontFamily> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Media::FontFamily>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn FontWeight(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::FontWeight> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::FontWeight =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::FontWeight>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::UI::Text::FontWeight,
        >,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn FontStyle(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::FontStyle> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::FontStyle =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::FontStyle>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetFontStyle(
        &self,
        value: super::super::super::super::Windows::UI::Text::FontStyle,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn FontStretch(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::FontStretch>
    {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::FontStretch =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::FontStretch>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetFontStretch(
        &self,
        value: super::super::super::super::Windows::UI::Text::FontStretch,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::runtime::Result<super::Media::Brush> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, Param0: ::windows::runtime::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
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
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn TextDecorations(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::TextDecorations>
    {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::TextDecorations =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::TextDecorations>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetTextDecorations(
        &self,
        value: super::super::super::super::Windows::UI::Text::TextDecorations,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn ContentStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ContentEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).32)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetAccessKey<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).34)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(
        &self,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).36)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).38)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).40)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::runtime::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).41)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).42)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).43)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).44)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).46)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn XamlRoot(&self) -> ::windows::runtime::Result<super::XamlRoot> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).47)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    pub fn SetXamlRoot<'a, Param0: ::windows::runtime::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).48)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn AccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).49)(
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
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).50)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn AccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).51)(
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
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).52)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn AccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).53)(
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
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).54)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FindName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).55)(
                ::std::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElementOverrides>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn IHyperlinkStatics<R, F: FnOnce(&IHyperlinkStatics) -> ::windows::runtime::Result<R>>(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Hyperlink, IHyperlinkStatics> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Hyperlink {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Hyperlink;{ac09bd16-cdfa-54c2-8d03-a474181545b1})",
    );
}
unsafe impl ::windows::runtime::Interface for Hyperlink {
    type Vtable = IHyperlink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2886319382,
        52730,
        21698,
        [141, 3, 164, 116, 24, 21, 69, 177],
    );
}
impl ::windows::runtime::RuntimeName for Hyperlink {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Hyperlink";
}
impl ::std::convert::From<Hyperlink> for ::windows::runtime::IUnknown {
    fn from(value: Hyperlink) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Hyperlink> for ::windows::runtime::IUnknown {
    fn from(value: &Hyperlink) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Hyperlink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Hyperlink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<Hyperlink> for ::windows::runtime::IInspectable {
    fn from(value: Hyperlink) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Hyperlink> for ::windows::runtime::IInspectable {
    fn from(value: &Hyperlink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Hyperlink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Hyperlink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<Hyperlink> for Span {
    fn from(value: Hyperlink) -> Self {
        ::std::convert::Into::<Span>::into(&value)
    }
}
impl ::std::convert::From<&Hyperlink> for Span {
    fn from(value: &Hyperlink) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Span> for Hyperlink {
    fn into_param(self) -> ::windows::runtime::Param<'a, Span> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<Span>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Span> for &Hyperlink {
    fn into_param(self) -> ::windows::runtime::Param<'a, Span> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<Span>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<Hyperlink> for Inline {
    fn from(value: Hyperlink) -> Self {
        ::std::convert::Into::<Inline>::into(&value)
    }
}
impl ::std::convert::From<&Hyperlink> for Inline {
    fn from(value: &Hyperlink) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Inline> for Hyperlink {
    fn into_param(self) -> ::windows::runtime::Param<'a, Inline> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<Inline>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Inline> for &Hyperlink {
    fn into_param(self) -> ::windows::runtime::Param<'a, Inline> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<Inline>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<Hyperlink> for TextElement {
    fn from(value: Hyperlink) -> Self {
        ::std::convert::Into::<TextElement>::into(&value)
    }
}
impl ::std::convert::From<&Hyperlink> for TextElement {
    fn from(value: &Hyperlink) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for Hyperlink {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<TextElement>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for &Hyperlink {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<TextElement>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<Hyperlink> for super::DependencyObject {
    fn from(value: Hyperlink) -> Self {
        ::std::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::std::convert::From<&Hyperlink> for super::DependencyObject {
    fn from(value: &Hyperlink) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for Hyperlink {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::DependencyObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for &Hyperlink {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::DependencyObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for Hyperlink {}
unsafe impl ::std::marker::Sync for Hyperlink {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct HyperlinkClickEventArgs(::windows::runtime::IInspectable);
impl HyperlinkClickEventArgs {
    pub fn OriginalSource(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IRoutedEventArgs>(self)?;
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
unsafe impl ::windows::runtime::RuntimeType for HyperlinkClickEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Documents.HyperlinkClickEventArgs;{f8f89552-873d-5ef5-82bf-c79a9509b07c})" ) ;
}
unsafe impl ::windows::runtime::Interface for HyperlinkClickEventArgs {
    type Vtable = IHyperlinkClickEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4177040722,
        34621,
        24309,
        [130, 191, 199, 154, 149, 9, 176, 124],
    );
}
impl ::windows::runtime::RuntimeName for HyperlinkClickEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.HyperlinkClickEventArgs";
}
impl ::std::convert::From<HyperlinkClickEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: HyperlinkClickEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HyperlinkClickEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &HyperlinkClickEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for HyperlinkClickEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &HyperlinkClickEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<HyperlinkClickEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: HyperlinkClickEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HyperlinkClickEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &HyperlinkClickEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for HyperlinkClickEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a HyperlinkClickEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<HyperlinkClickEventArgs> for super::RoutedEventArgs {
    fn from(value: HyperlinkClickEventArgs) -> Self {
        ::std::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::std::convert::From<&HyperlinkClickEventArgs> for super::RoutedEventArgs {
    fn from(value: &HyperlinkClickEventArgs) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::RoutedEventArgs> for HyperlinkClickEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::RoutedEventArgs> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::RoutedEventArgs> for &HyperlinkClickEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::RoutedEventArgs> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::RoutedEventArgs>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for HyperlinkClickEventArgs {}
unsafe impl ::std::marker::Sync for HyperlinkClickEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IBlock(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBlock {
    type Vtable = IBlock_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2169099527,
        26415,
        24533,
        [161, 10, 53, 19, 137, 186, 150, 89],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBlock_abi(
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
        result__: *mut super::TextAlignment,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::TextAlignment,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::TextAlignment,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::TextAlignment,
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
        result__: *mut super::LineStackingStrategy,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::LineStackingStrategy,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::Thickness,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::Thickness,
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
pub struct IBlockFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBlockFactory {
    type Vtable = IBlockFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        566060828,
        13282,
        22255,
        [190, 55, 161, 40, 232, 152, 69, 44],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBlockFactory_abi(
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
pub struct IBlockStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBlockStatics {
    type Vtable = IBlockStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2198859487,
        39590,
        22221,
        [152, 62, 5, 85, 0, 23, 27, 69],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBlockStatics_abi(
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
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IBold(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBold {
    type Vtable = IBold_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        605708122,
        49508,
        22911,
        [176, 219, 250, 199, 67, 18, 151, 242],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBold_abi(
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
pub struct IGlyphs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGlyphs {
    type Vtable = IGlyphs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        264211710,
        6375,
        24133,
        [159, 163, 210, 208, 146, 121, 88, 244],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlyphs_abi(
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
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::Media::StyleSimulations,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::Media::StyleSimulations,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
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
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
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
pub struct IGlyphsStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGlyphsStatics {
    type Vtable = IGlyphsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2375951386,
        15886,
        20736,
        [142, 222, 224, 8, 3, 79, 248, 174],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlyphsStatics_abi(
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
pub struct IHyperlink(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHyperlink {
    type Vtable = IHyperlink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2886319382,
        52730,
        21698,
        [141, 3, 164, 116, 24, 21, 69, 177],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IHyperlink_abi(
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
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut UnderlineStyle,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: UnderlineStyle,
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
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
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
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::ElementSoundMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::ElementSoundMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::FocusState,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
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
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: i32,
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
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::FocusState,
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
pub struct IHyperlinkClickEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHyperlinkClickEventArgs {
    type Vtable = IHyperlinkClickEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4177040722,
        34621,
        24309,
        [130, 191, 199, 154, 149, 9, 176, 124],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IHyperlinkClickEventArgs_abi(
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
pub struct IHyperlinkStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHyperlinkStatics {
    type Vtable = IHyperlinkStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3778386164,
        31687,
        23225,
        [136, 91, 112, 243, 47, 140, 149, 49],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IHyperlinkStatics_abi(
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
pub struct IInline(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInline {
    type Vtable = IInline_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2168275578,
        35200,
        23161,
        [168, 250, 242, 121, 25, 207, 178, 79],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInline_abi(
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
pub struct IInlineFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInlineFactory {
    type Vtable = IInlineFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4247075382,
        64043,
        23344,
        [137, 168, 159, 87, 120, 113, 236, 7],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInlineFactory_abi(
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
pub struct IInlineUIContainer(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInlineUIContainer {
    type Vtable = IInlineUIContainer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3576282870,
        49242,
        23469,
        [133, 232, 100, 1, 39, 207, 134, 245],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInlineUIContainer_abi(
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
pub struct IItalic(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IItalic {
    type Vtable = IItalic_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3392978621,
        31373,
        23930,
        [143, 223, 83, 142, 138, 104, 15, 108],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IItalic_abi(
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
pub struct ILineBreak(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILineBreak {
    type Vtable = ILineBreak_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        154170777,
        31938,
        24404,
        [177, 6, 114, 134, 32, 193, 111, 118],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineBreak_abi(
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
pub struct IParagraph(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IParagraph {
    type Vtable = IParagraph_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2664844407,
        12957,
        20527,
        [162, 87, 245, 131, 152, 237, 171, 81],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IParagraph_abi(
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
    #[cfg(feature = "undation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation_Collections"))] usize,
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
#[doc(hidden)]
pub struct IParagraphStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IParagraphStatics {
    type Vtable = IParagraphStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1320721073,
        26312,
        24512,
        [170, 95, 72, 200, 9, 44, 235, 95],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IParagraphStatics_abi(
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
pub struct IRun(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRun {
    type Vtable = IRun_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        529551929,
        14283,
        22795,
        [145, 50, 63, 251, 119, 65, 144, 110],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRun_abi(
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
        result__: *mut super::FlowDirection,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::FlowDirection,
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
pub struct IRunStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRunStatics {
    type Vtable = IRunStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        85671003,
        30208,
        20901,
        [128, 197, 147, 235, 80, 253, 104, 79],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRunStatics_abi(
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
pub struct ISpan(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpan {
    type Vtable = ISpan_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2444836173,
        20008,
        22457,
        [191, 251, 53, 102, 194, 163, 194, 161],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpan_abi(
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
    #[cfg(feature = "undation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation_Collections"))] usize,
    #[cfg(feature = "undation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
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
pub struct ISpanFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpanFactory {
    type Vtable = ISpanFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2800253974,
        49525,
        21960,
        [187, 211, 206, 64, 249, 208, 166, 128],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpanFactory_abi(
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
pub struct ITextElement(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextElement {
    type Vtable = ITextElement_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2703407650,
        33599,
        21024,
        [164, 126, 108, 213, 7, 83, 26, 190],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElement_abi(
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
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "_Text")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::UI::Text::FontWeight,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "_Text"))] usize,
    #[cfg(feature = "_Text")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::super::super::super::Windows::UI::Text::FontWeight,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "_Text"))] usize,
    #[cfg(feature = "_Text")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::UI::Text::FontStyle,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "_Text"))] usize,
    #[cfg(feature = "_Text")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::super::super::super::Windows::UI::Text::FontStyle,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "_Text"))] usize,
    #[cfg(feature = "_Text")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::UI::Text::FontStretch,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "_Text"))] usize,
    #[cfg(feature = "_Text")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::super::super::super::Windows::UI::Text::FontStretch,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "_Text"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
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
    #[cfg(feature = "_Text")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::UI::Text::TextDecorations,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "_Text"))] usize,
    #[cfg(feature = "_Text")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::super::super::super::Windows::UI::Text::TextDecorations,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "_Text"))] usize,
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
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::Input::KeyTipPlacementMode,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
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
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "UI_Xaml_Input", feature = "undation")))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "UI_Xaml_Input", feature = "undation")))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "UI_Xaml_Input", feature = "undation")))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::super::Windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
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
pub struct ITextElementFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextElementFactory {
    type Vtable = ITextElementFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3746691989,
        42470,
        23318,
        [142, 136, 159, 124, 191, 162, 52, 177],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElementFactory_abi(
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
pub struct ITextElementOverrides(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextElementOverrides {
    type Vtable = ITextElementOverrides_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1102058368,
        58527,
        24538,
        [140, 114, 172, 193, 172, 30, 145, 223],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElementOverrides_abi(
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
#[doc(hidden)]
pub struct ITextElementStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextElementStatics {
    type Vtable = ITextElementStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3384105241,
        57854,
        23245,
        [186, 199, 201, 215, 244, 19, 179, 92],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElementStatics_abi(
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
pub struct ITextHighlighter(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextHighlighter {
    type Vtable = ITextHighlighter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3075926113,
        7467,
        24431,
        [129, 253, 197, 26, 91, 192, 104, 255],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighter_abi(
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
    #[cfg(feature = "undation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation_Collections"))] usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ITextHighlighterBase(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextHighlighterBase {
    type Vtable = ITextHighlighterBase_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1545710320,
        14871,
        21608,
        [138, 172, 190, 20, 219, 14, 216, 193],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighterBase_abi(
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
pub struct ITextHighlighterBaseFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextHighlighterBaseFactory {
    type Vtable = ITextHighlighterBaseFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3767657569,
        21419,
        22942,
        [170, 234, 128, 10, 220, 114, 218, 79],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighterBaseFactory_abi(
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
pub struct ITextHighlighterFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextHighlighterFactory {
    type Vtable = ITextHighlighterFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1774661919,
        49177,
        23443,
        [181, 17, 129, 65, 133, 67, 186, 183],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighterFactory_abi(
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
pub struct ITextHighlighterStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextHighlighterStatics {
    type Vtable = ITextHighlighterStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1232405626,
        34733,
        20898,
        [151, 124, 231, 113, 222, 79, 64, 53],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighterStatics_abi(
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
pub struct ITextPointer(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextPointer {
    type Vtable = ITextPointer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2217653125,
        60993,
        22832,
        [151, 155, 67, 143, 167, 82, 90, 81],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextPointer_abi(
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
        result__: *mut LogicalDirection,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        direction: LogicalDirection,
        result__: *mut super::super::super::super::Windows::Foundation::Rect,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        offset: i32,
        direction: LogicalDirection,
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
pub struct ITypography(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITypography {
    type Vtable = ITypography_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4196917987,
        48734,
        23841,
        [154, 94, 144, 207, 16, 42, 248, 40],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypography_abi(
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
pub struct ITypographyStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITypographyStatics {
    type Vtable = ITypographyStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1442727221,
        8485,
        21306,
        [173, 168, 39, 190, 43, 158, 17, 147],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypographyStatics_abi(
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
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut super::FontEastAsianLanguage,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: super::FontEastAsianLanguage,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut super::FontEastAsianWidths,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: super::FontEastAsianWidths,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut super::FontCapitals,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: super::FontCapitals,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut super::FontFraction,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: super::FontFraction,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut super::FontNumeralStyle,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: super::FontNumeralStyle,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut super::FontNumeralAlignment,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: super::FontNumeralAlignment,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut super::FontVariants,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: super::FontVariants,
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
pub struct IUnderline(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUnderline {
    type Vtable = IUnderline_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1756032110,
        60017,
        24274,
        [184, 62, 145, 104, 75, 37, 239, 185],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnderline_abi(
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
pub struct Inline(::windows::runtime::IInspectable);
impl Inline {
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Core::CoreDispatcher>
    {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
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
    pub fn FontSize(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::runtime::Result<super::Media::FontFamily> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Media::FontFamily>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn FontWeight(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::FontWeight> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::FontWeight =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::FontWeight>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::UI::Text::FontWeight,
        >,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn FontStyle(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::FontStyle> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::FontStyle =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::FontStyle>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetFontStyle(
        &self,
        value: super::super::super::super::Windows::UI::Text::FontStyle,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn FontStretch(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::FontStretch>
    {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::FontStretch =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::FontStretch>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetFontStretch(
        &self,
        value: super::super::super::super::Windows::UI::Text::FontStretch,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::runtime::Result<super::Media::Brush> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, Param0: ::windows::runtime::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
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
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn TextDecorations(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::TextDecorations>
    {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::TextDecorations =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::TextDecorations>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetTextDecorations(
        &self,
        value: super::super::super::super::Windows::UI::Text::TextDecorations,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn ContentStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ContentEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).32)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetAccessKey<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).34)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(
        &self,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).36)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).38)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).40)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::runtime::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).41)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).42)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).43)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).44)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).46)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn XamlRoot(&self) -> ::windows::runtime::Result<super::XamlRoot> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).47)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    pub fn SetXamlRoot<'a, Param0: ::windows::runtime::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).48)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn AccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).49)(
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
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).50)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn AccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).51)(
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
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).52)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn AccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).53)(
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
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).54)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FindName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).55)(
                ::std::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElementOverrides>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Inline {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Inline;{813d427a-8980-5a79-a8fa-f27919cfb24f})",
    );
}
unsafe impl ::windows::runtime::Interface for Inline {
    type Vtable = IInline_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2168275578,
        35200,
        23161,
        [168, 250, 242, 121, 25, 207, 178, 79],
    );
}
impl ::windows::runtime::RuntimeName for Inline {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Inline";
}
impl ::std::convert::From<Inline> for ::windows::runtime::IUnknown {
    fn from(value: Inline) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Inline> for ::windows::runtime::IUnknown {
    fn from(value: &Inline) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Inline {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Inline {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<Inline> for ::windows::runtime::IInspectable {
    fn from(value: Inline) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Inline> for ::windows::runtime::IInspectable {
    fn from(value: &Inline) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Inline {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Inline {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<Inline> for TextElement {
    fn from(value: Inline) -> Self {
        ::std::convert::Into::<TextElement>::into(&value)
    }
}
impl ::std::convert::From<&Inline> for TextElement {
    fn from(value: &Inline) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for Inline {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<TextElement>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for &Inline {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<TextElement>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<Inline> for super::DependencyObject {
    fn from(value: Inline) -> Self {
        ::std::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::std::convert::From<&Inline> for super::DependencyObject {
    fn from(value: &Inline) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for Inline {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::DependencyObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for &Inline {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::DependencyObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for Inline {}
unsafe impl ::std::marker::Sync for Inline {}
#[cfg(feature = "undation_Collections")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct InlineCollection(::windows::runtime::IInspectable);
#[cfg(feature = "undation_Collections")]
impl InlineCollection {
    #[cfg(feature = "undation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::runtime::Result<Inline> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                index,
                &mut result__,
            )
            .from_abi::<Inline>(result__)
        }
    }
    #[cfg(feature = "undation_Collections")]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
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
    #[cfg(feature = "undation_Collections")]
    pub fn GetView(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::Collections::IVectorView<Inline>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .8 ) ( :: std :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < super::super::super::super::Windows::Foundation::Collections:: IVectorView :: < Inline > > ( result__ )
        }
    }
    #[cfg(feature = "undation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::runtime::IntoParam<'a, Inline>>(
        &self,
        value: Param0,
        index: &mut u32,
    ) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
                index,
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "undation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::runtime::IntoParam<'a, Inline>>(
        &self,
        index: u32,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                index,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::runtime::IntoParam<'a, Inline>>(
        &self,
        index: u32,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                index,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                index,
            )
            .ok()
        }
    }
    #[cfg(feature = "undation_Collections")]
    pub fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, Inline>>(
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
    #[cfg(feature = "undation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "undation_Collections")]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "undation_Collections")]
    pub fn GetMany(
        &self,
        startindex: u32,
        items: &mut [<Inline as ::windows::runtime::Abi>::DefaultType],
    ) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                startindex,
                items.len() as u32,
                ::std::mem::transmute_copy(&items),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "undation_Collections")]
    pub fn ReplaceAll(
        &self,
        items: &[<Inline as ::windows::runtime::Abi>::DefaultType],
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                items.len() as u32,
                ::std::mem::transmute(items.as_ptr()),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation_Collections")]
    pub fn First(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::Collections::IIterator<Inline>,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::super::super::Windows::Foundation::Collections::IIterable<Inline>,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .6 ) ( :: std :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < super::super::super::super::Windows::Foundation::Collections:: IIterator :: < Inline > > ( result__ )
        }
    }
}
#[cfg(feature = "undation_Collections")]
unsafe impl ::windows::runtime::RuntimeType for InlineCollection {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Documents.InlineCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Microsoft.UI.Xaml.Documents.Inline;{813d427a-8980-5a79-a8fa-f27919cfb24f})))" ) ;
}
#[cfg(feature = "undation_Collections")]
unsafe impl ::windows::runtime::Interface for InlineCollection {
    type Vtable = super::super::super::super::Windows::Foundation::Collections::IVector_abi<Inline>;
    const IID : :: windows :: runtime :: GUID = :: windows :: runtime :: GUID :: from_signature ( < super::super::super::super::Windows::Foundation::Collections:: IVector :: < Inline > as :: windows :: runtime :: RuntimeType > :: SIGNATURE ) ;
}
#[cfg(feature = "undation_Collections")]
impl ::windows::runtime::RuntimeName for InlineCollection {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.InlineCollection";
}
#[cfg(feature = "undation_Collections")]
impl ::std::convert::From<InlineCollection> for ::windows::runtime::IUnknown {
    fn from(value: InlineCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "undation_Collections")]
impl ::std::convert::From<&InlineCollection> for ::windows::runtime::IUnknown {
    fn from(value: &InlineCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "undation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InlineCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "undation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InlineCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "undation_Collections")]
impl ::std::convert::From<InlineCollection> for ::windows::runtime::IInspectable {
    fn from(value: InlineCollection) -> Self {
        value.0
    }
}
#[cfg(feature = "undation_Collections")]
impl ::std::convert::From<&InlineCollection> for ::windows::runtime::IInspectable {
    fn from(value: &InlineCollection) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "undation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InlineCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "undation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a InlineCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "undation_Collections")]
impl ::std::convert::From<InlineCollection>
    for super::super::super::super::Windows::Foundation::Collections::IVector<Inline>
{
    fn from(value: InlineCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "undation_Collections")]
impl ::std::convert::From<&InlineCollection>
    for super::super::super::super::Windows::Foundation::Collections::IVector<Inline>
{
    fn from(value: &InlineCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "undation_Collections")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::super::super::Windows::Foundation::Collections::IVector<Inline>,
    > for InlineCollection
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::super::super::Windows::Foundation::Collections::IVector<Inline>,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::super::super::Windows::Foundation::Collections::IVector<Inline>,
        >::into(self))
    }
}
#[cfg(feature = "undation_Collections")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::super::super::Windows::Foundation::Collections::IVector<Inline>,
    > for &InlineCollection
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::super::super::Windows::Foundation::Collections::IVector<Inline>,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::super::super::Windows::Foundation::Collections::IVector<Inline>,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "undation_Collections")]
impl ::std::convert::TryFrom<InlineCollection>
    for super::super::super::super::Windows::Foundation::Collections::IIterable<Inline>
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: InlineCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "undation_Collections")]
impl ::std::convert::TryFrom<&InlineCollection>
    for super::super::super::super::Windows::Foundation::Collections::IIterable<Inline>
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InlineCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "undation_Collections")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::super::super::Windows::Foundation::Collections::IIterable<Inline>,
    > for InlineCollection
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::super::super::Windows::Foundation::Collections::IIterable<Inline>,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "undation_Collections")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::super::super::Windows::Foundation::Collections::IIterable<Inline>,
    > for &InlineCollection
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::super::super::Windows::Foundation::Collections::IIterable<Inline>,
    > {
        ::std::convert::TryInto::<
            super::super::super::super::Windows::Foundation::Collections::IIterable<Inline>,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "undation_Collections")]
unsafe impl ::std::marker::Send for InlineCollection {}
#[cfg(feature = "undation_Collections")]
unsafe impl ::std::marker::Sync for InlineCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for InlineCollection {
    type Item = Inline;
    type IntoIter =
        super::super::super::super::Windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for &InlineCollection {
    type Item = Inline;
    type IntoIter =
        super::super::super::super::Windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::super::Windows::Foundation::Collections::VectorIterator::new(
            ::std::convert::TryInto::try_into(self).ok(),
        )
    }
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct InlineUIContainer(::windows::runtime::IInspectable);
impl InlineUIContainer {
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
            InlineUIContainer,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Child(&self) -> ::windows::runtime::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::UIElement>(result__)
        }
    }
    pub fn SetChild<'a, Param0: ::windows::runtime::IntoParam<'a, super::UIElement>>(
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
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Core::CoreDispatcher>
    {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
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
    pub fn FontSize(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::runtime::Result<super::Media::FontFamily> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Media::FontFamily>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn FontWeight(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::FontWeight> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::FontWeight =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::FontWeight>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::UI::Text::FontWeight,
        >,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn FontStyle(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::FontStyle> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::FontStyle =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::FontStyle>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetFontStyle(
        &self,
        value: super::super::super::super::Windows::UI::Text::FontStyle,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn FontStretch(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::FontStretch>
    {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::FontStretch =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::FontStretch>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetFontStretch(
        &self,
        value: super::super::super::super::Windows::UI::Text::FontStretch,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::runtime::Result<super::Media::Brush> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, Param0: ::windows::runtime::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
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
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn TextDecorations(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::TextDecorations>
    {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::TextDecorations =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::TextDecorations>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetTextDecorations(
        &self,
        value: super::super::super::super::Windows::UI::Text::TextDecorations,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn ContentStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ContentEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).32)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetAccessKey<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).34)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(
        &self,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).36)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).38)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).40)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::runtime::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).41)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).42)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).43)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).44)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).46)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn XamlRoot(&self) -> ::windows::runtime::Result<super::XamlRoot> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).47)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    pub fn SetXamlRoot<'a, Param0: ::windows::runtime::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).48)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn AccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).49)(
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
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).50)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn AccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).51)(
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
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).52)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn AccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).53)(
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
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).54)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FindName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).55)(
                ::std::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElementOverrides>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InlineUIContainer {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.InlineUIContainer;{d529bef6-c05a-5bad-85e8-640127cf86f5})",
    );
}
unsafe impl ::windows::runtime::Interface for InlineUIContainer {
    type Vtable = IInlineUIContainer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3576282870,
        49242,
        23469,
        [133, 232, 100, 1, 39, 207, 134, 245],
    );
}
impl ::windows::runtime::RuntimeName for InlineUIContainer {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.InlineUIContainer";
}
impl ::std::convert::From<InlineUIContainer> for ::windows::runtime::IUnknown {
    fn from(value: InlineUIContainer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InlineUIContainer> for ::windows::runtime::IUnknown {
    fn from(value: &InlineUIContainer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InlineUIContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InlineUIContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<InlineUIContainer> for ::windows::runtime::IInspectable {
    fn from(value: InlineUIContainer) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InlineUIContainer> for ::windows::runtime::IInspectable {
    fn from(value: &InlineUIContainer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InlineUIContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a InlineUIContainer
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<InlineUIContainer> for Inline {
    fn from(value: InlineUIContainer) -> Self {
        ::std::convert::Into::<Inline>::into(&value)
    }
}
impl ::std::convert::From<&InlineUIContainer> for Inline {
    fn from(value: &InlineUIContainer) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Inline> for InlineUIContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, Inline> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<Inline>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Inline> for &InlineUIContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, Inline> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<Inline>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<InlineUIContainer> for TextElement {
    fn from(value: InlineUIContainer) -> Self {
        ::std::convert::Into::<TextElement>::into(&value)
    }
}
impl ::std::convert::From<&InlineUIContainer> for TextElement {
    fn from(value: &InlineUIContainer) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for InlineUIContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<TextElement>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for &InlineUIContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<TextElement>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<InlineUIContainer> for super::DependencyObject {
    fn from(value: InlineUIContainer) -> Self {
        ::std::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::std::convert::From<&InlineUIContainer> for super::DependencyObject {
    fn from(value: &InlineUIContainer) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for InlineUIContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::DependencyObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for &InlineUIContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::DependencyObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for InlineUIContainer {}
unsafe impl ::std::marker::Sync for InlineUIContainer {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct Italic(::windows::runtime::IInspectable);
impl Italic {
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
            Italic,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Core::CoreDispatcher>
    {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[cfg(feature = "undation_Collections")]
    pub fn Inlines(&self) -> ::windows::runtime::Result<InlineCollection> {
        let this = &::windows::runtime::Interface::cast::<ISpan>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InlineCollection>(result__)
        }
    }
    #[cfg(feature = "undation_Collections")]
    pub fn SetInlines<'a, Param0: ::windows::runtime::IntoParam<'a, InlineCollection>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISpan>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
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
    pub fn FontSize(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::runtime::Result<super::Media::FontFamily> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Media::FontFamily>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn FontWeight(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::FontWeight> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::FontWeight =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::FontWeight>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::UI::Text::FontWeight,
        >,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn FontStyle(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::FontStyle> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::FontStyle =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::FontStyle>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetFontStyle(
        &self,
        value: super::super::super::super::Windows::UI::Text::FontStyle,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn FontStretch(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::FontStretch>
    {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::FontStretch =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::FontStretch>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetFontStretch(
        &self,
        value: super::super::super::super::Windows::UI::Text::FontStretch,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::runtime::Result<super::Media::Brush> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, Param0: ::windows::runtime::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
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
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn TextDecorations(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::TextDecorations>
    {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::TextDecorations =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::TextDecorations>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetTextDecorations(
        &self,
        value: super::super::super::super::Windows::UI::Text::TextDecorations,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn ContentStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ContentEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).32)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetAccessKey<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).34)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(
        &self,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).36)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).38)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).40)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::runtime::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).41)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).42)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).43)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).44)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).46)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn XamlRoot(&self) -> ::windows::runtime::Result<super::XamlRoot> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).47)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    pub fn SetXamlRoot<'a, Param0: ::windows::runtime::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).48)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn AccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).49)(
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
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).50)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn AccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).51)(
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
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).52)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn AccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).53)(
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
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).54)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FindName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).55)(
                ::std::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElementOverrides>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Italic {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Italic;{ca3cbebd-7a8d-5d7a-8fdf-538e8a680f6c})",
    );
}
unsafe impl ::windows::runtime::Interface for Italic {
    type Vtable = IItalic_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3392978621,
        31373,
        23930,
        [143, 223, 83, 142, 138, 104, 15, 108],
    );
}
impl ::windows::runtime::RuntimeName for Italic {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Italic";
}
impl ::std::convert::From<Italic> for ::windows::runtime::IUnknown {
    fn from(value: Italic) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Italic> for ::windows::runtime::IUnknown {
    fn from(value: &Italic) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Italic {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Italic {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<Italic> for ::windows::runtime::IInspectable {
    fn from(value: Italic) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Italic> for ::windows::runtime::IInspectable {
    fn from(value: &Italic) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Italic {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Italic {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<Italic> for Span {
    fn from(value: Italic) -> Self {
        ::std::convert::Into::<Span>::into(&value)
    }
}
impl ::std::convert::From<&Italic> for Span {
    fn from(value: &Italic) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Span> for Italic {
    fn into_param(self) -> ::windows::runtime::Param<'a, Span> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<Span>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Span> for &Italic {
    fn into_param(self) -> ::windows::runtime::Param<'a, Span> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<Span>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<Italic> for Inline {
    fn from(value: Italic) -> Self {
        ::std::convert::Into::<Inline>::into(&value)
    }
}
impl ::std::convert::From<&Italic> for Inline {
    fn from(value: &Italic) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Inline> for Italic {
    fn into_param(self) -> ::windows::runtime::Param<'a, Inline> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<Inline>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Inline> for &Italic {
    fn into_param(self) -> ::windows::runtime::Param<'a, Inline> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<Inline>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<Italic> for TextElement {
    fn from(value: Italic) -> Self {
        ::std::convert::Into::<TextElement>::into(&value)
    }
}
impl ::std::convert::From<&Italic> for TextElement {
    fn from(value: &Italic) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for Italic {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<TextElement>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for &Italic {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<TextElement>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<Italic> for super::DependencyObject {
    fn from(value: Italic) -> Self {
        ::std::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::std::convert::From<&Italic> for super::DependencyObject {
    fn from(value: &Italic) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for Italic {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::DependencyObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for &Italic {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::DependencyObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for Italic {}
unsafe impl ::std::marker::Sync for Italic {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct LineBreak(::windows::runtime::IInspectable);
impl LineBreak {
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
            LineBreak,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Core::CoreDispatcher>
    {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
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
    pub fn FontSize(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::runtime::Result<super::Media::FontFamily> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Media::FontFamily>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn FontWeight(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::FontWeight> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::FontWeight =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::FontWeight>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::UI::Text::FontWeight,
        >,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn FontStyle(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::FontStyle> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::FontStyle =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::FontStyle>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetFontStyle(
        &self,
        value: super::super::super::super::Windows::UI::Text::FontStyle,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn FontStretch(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::FontStretch>
    {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::FontStretch =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::FontStretch>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetFontStretch(
        &self,
        value: super::super::super::super::Windows::UI::Text::FontStretch,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::runtime::Result<super::Media::Brush> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, Param0: ::windows::runtime::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
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
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn TextDecorations(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::TextDecorations>
    {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::TextDecorations =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::TextDecorations>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetTextDecorations(
        &self,
        value: super::super::super::super::Windows::UI::Text::TextDecorations,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn ContentStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ContentEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).32)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetAccessKey<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).34)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(
        &self,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).36)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).38)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).40)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::runtime::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).41)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).42)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).43)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).44)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).46)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn XamlRoot(&self) -> ::windows::runtime::Result<super::XamlRoot> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).47)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    pub fn SetXamlRoot<'a, Param0: ::windows::runtime::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).48)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn AccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).49)(
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
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).50)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn AccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).51)(
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
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).52)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn AccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).53)(
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
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).54)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FindName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).55)(
                ::std::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElementOverrides>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LineBreak {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.LineBreak;{09307599-7cc2-5f54-b106-728620c16f76})",
    );
}
unsafe impl ::windows::runtime::Interface for LineBreak {
    type Vtable = ILineBreak_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        154170777,
        31938,
        24404,
        [177, 6, 114, 134, 32, 193, 111, 118],
    );
}
impl ::windows::runtime::RuntimeName for LineBreak {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.LineBreak";
}
impl ::std::convert::From<LineBreak> for ::windows::runtime::IUnknown {
    fn from(value: LineBreak) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&LineBreak> for ::windows::runtime::IUnknown {
    fn from(value: &LineBreak) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LineBreak {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &LineBreak {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<LineBreak> for ::windows::runtime::IInspectable {
    fn from(value: LineBreak) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LineBreak> for ::windows::runtime::IInspectable {
    fn from(value: &LineBreak) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LineBreak {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LineBreak {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<LineBreak> for Inline {
    fn from(value: LineBreak) -> Self {
        ::std::convert::Into::<Inline>::into(&value)
    }
}
impl ::std::convert::From<&LineBreak> for Inline {
    fn from(value: &LineBreak) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Inline> for LineBreak {
    fn into_param(self) -> ::windows::runtime::Param<'a, Inline> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<Inline>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Inline> for &LineBreak {
    fn into_param(self) -> ::windows::runtime::Param<'a, Inline> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<Inline>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<LineBreak> for TextElement {
    fn from(value: LineBreak) -> Self {
        ::std::convert::Into::<TextElement>::into(&value)
    }
}
impl ::std::convert::From<&LineBreak> for TextElement {
    fn from(value: &LineBreak) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for LineBreak {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<TextElement>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for &LineBreak {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<TextElement>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<LineBreak> for super::DependencyObject {
    fn from(value: LineBreak) -> Self {
        ::std::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::std::convert::From<&LineBreak> for super::DependencyObject {
    fn from(value: &LineBreak) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for LineBreak {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::DependencyObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for &LineBreak {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::DependencyObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for LineBreak {}
unsafe impl ::std::marker::Sync for LineBreak {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct LogicalDirection(pub i32);
impl LogicalDirection {
    pub const Backward: LogicalDirection = LogicalDirection(0i32);
    pub const Forward: LogicalDirection = LogicalDirection(1i32);
}
impl ::std::convert::From<i32> for LogicalDirection {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LogicalDirection {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for LogicalDirection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Documents.LogicalDirection;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct Paragraph(::windows::runtime::IInspectable);
impl Paragraph {
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
            Paragraph,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "undation_Collections")]
    pub fn Inlines(&self) -> ::windows::runtime::Result<InlineCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InlineCollection>(result__)
        }
    }
    pub fn TextIndent(&self) -> ::windows::runtime::Result<f64> {
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
    pub fn SetTextIndent(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn TextIndentProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IParagraphStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Core::CoreDispatcher>
    {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn TextAlignment(&self) -> ::windows::runtime::Result<super::TextAlignment> {
        let this = &::windows::runtime::Interface::cast::<IBlock>(self)?;
        unsafe {
            let mut result__: super::TextAlignment = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::TextAlignment>(result__)
        }
    }
    pub fn SetTextAlignment(&self, value: super::TextAlignment) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBlock>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn HorizontalTextAlignment(&self) -> ::windows::runtime::Result<super::TextAlignment> {
        let this = &::windows::runtime::Interface::cast::<IBlock>(self)?;
        unsafe {
            let mut result__: super::TextAlignment = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::TextAlignment>(result__)
        }
    }
    pub fn SetHorizontalTextAlignment(
        &self,
        value: super::TextAlignment,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBlock>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn LineHeight(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<IBlock>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetLineHeight(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBlock>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn LineStackingStrategy(&self) -> ::windows::runtime::Result<super::LineStackingStrategy> {
        let this = &::windows::runtime::Interface::cast::<IBlock>(self)?;
        unsafe {
            let mut result__: super::LineStackingStrategy = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::LineStackingStrategy>(result__)
        }
    }
    pub fn SetLineStackingStrategy(
        &self,
        value: super::LineStackingStrategy,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBlock>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn Margin(&self) -> ::windows::runtime::Result<super::Thickness> {
        let this = &::windows::runtime::Interface::cast::<IBlock>(self)?;
        unsafe {
            let mut result__: super::Thickness = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Thickness>(result__)
        }
    }
    pub fn SetMargin<'a, Param0: ::windows::runtime::IntoParam<'a, super::Thickness>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBlock>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
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
    pub fn FontSize(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::runtime::Result<super::Media::FontFamily> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Media::FontFamily>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn FontWeight(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::FontWeight> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::FontWeight =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::FontWeight>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::UI::Text::FontWeight,
        >,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn FontStyle(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::FontStyle> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::FontStyle =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::FontStyle>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetFontStyle(
        &self,
        value: super::super::super::super::Windows::UI::Text::FontStyle,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn FontStretch(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::FontStretch>
    {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::FontStretch =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::FontStretch>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetFontStretch(
        &self,
        value: super::super::super::super::Windows::UI::Text::FontStretch,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::runtime::Result<super::Media::Brush> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, Param0: ::windows::runtime::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
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
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn TextDecorations(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::TextDecorations>
    {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::TextDecorations =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::TextDecorations>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetTextDecorations(
        &self,
        value: super::super::super::super::Windows::UI::Text::TextDecorations,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn ContentStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ContentEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).32)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetAccessKey<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).34)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(
        &self,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).36)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).38)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).40)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::runtime::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).41)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).42)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).43)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).44)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).46)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn XamlRoot(&self) -> ::windows::runtime::Result<super::XamlRoot> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).47)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    pub fn SetXamlRoot<'a, Param0: ::windows::runtime::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).48)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn AccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).49)(
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
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).50)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn AccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).51)(
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
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).52)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn AccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).53)(
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
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).54)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FindName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).55)(
                ::std::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElementOverrides>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn IParagraphStatics<R, F: FnOnce(&IParagraphStatics) -> ::windows::runtime::Result<R>>(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Paragraph, IParagraphStatics> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Paragraph {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Paragraph;{9ed64c77-329d-502f-a257-f58398edab51})",
    );
}
unsafe impl ::windows::runtime::Interface for Paragraph {
    type Vtable = IParagraph_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2664844407,
        12957,
        20527,
        [162, 87, 245, 131, 152, 237, 171, 81],
    );
}
impl ::windows::runtime::RuntimeName for Paragraph {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Paragraph";
}
impl ::std::convert::From<Paragraph> for ::windows::runtime::IUnknown {
    fn from(value: Paragraph) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Paragraph> for ::windows::runtime::IUnknown {
    fn from(value: &Paragraph) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Paragraph {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Paragraph {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<Paragraph> for ::windows::runtime::IInspectable {
    fn from(value: Paragraph) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Paragraph> for ::windows::runtime::IInspectable {
    fn from(value: &Paragraph) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Paragraph {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Paragraph {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<Paragraph> for Block {
    fn from(value: Paragraph) -> Self {
        ::std::convert::Into::<Block>::into(&value)
    }
}
impl ::std::convert::From<&Paragraph> for Block {
    fn from(value: &Paragraph) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Block> for Paragraph {
    fn into_param(self) -> ::windows::runtime::Param<'a, Block> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<Block>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Block> for &Paragraph {
    fn into_param(self) -> ::windows::runtime::Param<'a, Block> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<Block>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<Paragraph> for TextElement {
    fn from(value: Paragraph) -> Self {
        ::std::convert::Into::<TextElement>::into(&value)
    }
}
impl ::std::convert::From<&Paragraph> for TextElement {
    fn from(value: &Paragraph) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for Paragraph {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<TextElement>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for &Paragraph {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<TextElement>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<Paragraph> for super::DependencyObject {
    fn from(value: Paragraph) -> Self {
        ::std::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::std::convert::From<&Paragraph> for super::DependencyObject {
    fn from(value: &Paragraph) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for Paragraph {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::DependencyObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for &Paragraph {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::DependencyObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for Paragraph {}
unsafe impl ::std::marker::Sync for Paragraph {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct Run(::windows::runtime::IInspectable);
impl Run {
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
            Run,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Text(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn SetText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
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
    pub fn FlowDirection(&self) -> ::windows::runtime::Result<super::FlowDirection> {
        let this = self;
        unsafe {
            let mut result__: super::FlowDirection = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::FlowDirection>(result__)
        }
    }
    pub fn SetFlowDirection(&self, value: super::FlowDirection) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn FlowDirectionProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IRunStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Core::CoreDispatcher>
    {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
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
    pub fn FontSize(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::runtime::Result<super::Media::FontFamily> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Media::FontFamily>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn FontWeight(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::FontWeight> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::FontWeight =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::FontWeight>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::UI::Text::FontWeight,
        >,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn FontStyle(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::FontStyle> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::FontStyle =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::FontStyle>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetFontStyle(
        &self,
        value: super::super::super::super::Windows::UI::Text::FontStyle,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn FontStretch(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::FontStretch>
    {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::FontStretch =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::FontStretch>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetFontStretch(
        &self,
        value: super::super::super::super::Windows::UI::Text::FontStretch,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::runtime::Result<super::Media::Brush> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, Param0: ::windows::runtime::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
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
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn TextDecorations(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::TextDecorations>
    {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::TextDecorations =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::TextDecorations>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetTextDecorations(
        &self,
        value: super::super::super::super::Windows::UI::Text::TextDecorations,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn ContentStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ContentEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).32)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetAccessKey<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).34)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(
        &self,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).36)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).38)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).40)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::runtime::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).41)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).42)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).43)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).44)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).46)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn XamlRoot(&self) -> ::windows::runtime::Result<super::XamlRoot> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).47)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    pub fn SetXamlRoot<'a, Param0: ::windows::runtime::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).48)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn AccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).49)(
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
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).50)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn AccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).51)(
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
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).52)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn AccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).53)(
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
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).54)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FindName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).55)(
                ::std::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElementOverrides>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn IRunStatics<R, F: FnOnce(&IRunStatics) -> ::windows::runtime::Result<R>>(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Run, IRunStatics> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Run {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Run;{1f905239-37cb-590b-9132-3ffb7741906e})",
    );
}
unsafe impl ::windows::runtime::Interface for Run {
    type Vtable = IRun_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        529551929,
        14283,
        22795,
        [145, 50, 63, 251, 119, 65, 144, 110],
    );
}
impl ::windows::runtime::RuntimeName for Run {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Run";
}
impl ::std::convert::From<Run> for ::windows::runtime::IUnknown {
    fn from(value: Run) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Run> for ::windows::runtime::IUnknown {
    fn from(value: &Run) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Run {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Run {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<Run> for ::windows::runtime::IInspectable {
    fn from(value: Run) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Run> for ::windows::runtime::IInspectable {
    fn from(value: &Run) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Run {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Run {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<Run> for Inline {
    fn from(value: Run) -> Self {
        ::std::convert::Into::<Inline>::into(&value)
    }
}
impl ::std::convert::From<&Run> for Inline {
    fn from(value: &Run) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Inline> for Run {
    fn into_param(self) -> ::windows::runtime::Param<'a, Inline> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<Inline>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Inline> for &Run {
    fn into_param(self) -> ::windows::runtime::Param<'a, Inline> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<Inline>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<Run> for TextElement {
    fn from(value: Run) -> Self {
        ::std::convert::Into::<TextElement>::into(&value)
    }
}
impl ::std::convert::From<&Run> for TextElement {
    fn from(value: &Run) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for Run {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<TextElement>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for &Run {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<TextElement>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<Run> for super::DependencyObject {
    fn from(value: Run) -> Self {
        ::std::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::std::convert::From<&Run> for super::DependencyObject {
    fn from(value: &Run) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for Run {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::DependencyObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for &Run {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::DependencyObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for Run {}
unsafe impl ::std::marker::Sync for Run {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct Span(::windows::runtime::IInspectable);
impl Span {
    #[cfg(feature = "undation_Collections")]
    pub fn Inlines(&self) -> ::windows::runtime::Result<InlineCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InlineCollection>(result__)
        }
    }
    #[cfg(feature = "undation_Collections")]
    pub fn SetInlines<'a, Param0: ::windows::runtime::IntoParam<'a, InlineCollection>>(
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
    pub fn new() -> ::windows::runtime::Result<Span> {
        Self::ISpanFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                ::std::ptr::null_mut(),
                &mut ::std::option::Option::<::windows::runtime::IInspectable>::None as *mut _ as _,
                &mut result__,
            )
            .from_abi::<Span>(result__)
        })
    }
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Core::CoreDispatcher>
    {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
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
    pub fn FontSize(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::runtime::Result<super::Media::FontFamily> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Media::FontFamily>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn FontWeight(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::FontWeight> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::FontWeight =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::FontWeight>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::UI::Text::FontWeight,
        >,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn FontStyle(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::FontStyle> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::FontStyle =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::FontStyle>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetFontStyle(
        &self,
        value: super::super::super::super::Windows::UI::Text::FontStyle,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn FontStretch(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::FontStretch>
    {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::FontStretch =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::FontStretch>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetFontStretch(
        &self,
        value: super::super::super::super::Windows::UI::Text::FontStretch,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::runtime::Result<super::Media::Brush> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, Param0: ::windows::runtime::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
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
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn TextDecorations(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::TextDecorations>
    {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::TextDecorations =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::TextDecorations>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetTextDecorations(
        &self,
        value: super::super::super::super::Windows::UI::Text::TextDecorations,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn ContentStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ContentEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).32)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetAccessKey<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).34)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(
        &self,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).36)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).38)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).40)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::runtime::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).41)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).42)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).43)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).44)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).46)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn XamlRoot(&self) -> ::windows::runtime::Result<super::XamlRoot> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).47)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    pub fn SetXamlRoot<'a, Param0: ::windows::runtime::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).48)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn AccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).49)(
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
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).50)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn AccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).51)(
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
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).52)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn AccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).53)(
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
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).54)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FindName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).55)(
                ::std::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElementOverrides>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn ISpanFactory<R, F: FnOnce(&ISpanFactory) -> ::windows::runtime::Result<R>>(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Span, ISpanFactory> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Span {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Span;{91b93d4d-4e28-57b9-bffb-3566c2a3c2a1})",
    );
}
unsafe impl ::windows::runtime::Interface for Span {
    type Vtable = ISpan_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2444836173,
        20008,
        22457,
        [191, 251, 53, 102, 194, 163, 194, 161],
    );
}
impl ::windows::runtime::RuntimeName for Span {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Span";
}
impl ::std::convert::From<Span> for ::windows::runtime::IUnknown {
    fn from(value: Span) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Span> for ::windows::runtime::IUnknown {
    fn from(value: &Span) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Span {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Span {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<Span> for ::windows::runtime::IInspectable {
    fn from(value: Span) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Span> for ::windows::runtime::IInspectable {
    fn from(value: &Span) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Span {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Span {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<Span> for Inline {
    fn from(value: Span) -> Self {
        ::std::convert::Into::<Inline>::into(&value)
    }
}
impl ::std::convert::From<&Span> for Inline {
    fn from(value: &Span) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Inline> for Span {
    fn into_param(self) -> ::windows::runtime::Param<'a, Inline> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<Inline>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Inline> for &Span {
    fn into_param(self) -> ::windows::runtime::Param<'a, Inline> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<Inline>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<Span> for TextElement {
    fn from(value: Span) -> Self {
        ::std::convert::Into::<TextElement>::into(&value)
    }
}
impl ::std::convert::From<&Span> for TextElement {
    fn from(value: &Span) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for Span {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<TextElement>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for &Span {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<TextElement>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<Span> for super::DependencyObject {
    fn from(value: Span) -> Self {
        ::std::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::std::convert::From<&Span> for super::DependencyObject {
    fn from(value: &Span) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for Span {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::DependencyObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for &Span {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::DependencyObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for Span {}
unsafe impl ::std::marker::Sync for Span {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct TextElement(::windows::runtime::IInspectable);
impl TextElement {
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
    pub fn FontSize(&self) -> ::windows::runtime::Result<f64> {
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
    pub fn SetFontSize(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::runtime::Result<super::Media::FontFamily> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Media::FontFamily>,
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
    #[cfg(feature = "_Text")]
    pub fn FontWeight(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::FontWeight> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::FontWeight =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::FontWeight>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::UI::Text::FontWeight,
        >,
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
    #[cfg(feature = "_Text")]
    pub fn FontStyle(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::FontStyle> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::FontStyle =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::FontStyle>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetFontStyle(
        &self,
        value: super::super::super::super::Windows::UI::Text::FontStyle,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn FontStretch(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::FontStretch>
    {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::FontStretch =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::FontStretch>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetFontStretch(
        &self,
        value: super::super::super::super::Windows::UI::Text::FontStretch,
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
    pub fn CharacterSpacing(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::runtime::Result<super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, Param0: ::windows::runtime::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).20)(
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
            (::windows::runtime::Interface::vtable(this).21)(
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
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn TextDecorations(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::TextDecorations>
    {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::TextDecorations =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::TextDecorations>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetTextDecorations(
        &self,
        value: super::super::super::super::Windows::UI::Text::TextDecorations,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn ContentStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ContentEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).32)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetAccessKey<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
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
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(
        &self,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).36)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).38)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).40)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::runtime::Result<super::Input::KeyTipPlacementMode> {
        let this = self;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).41)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).42)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).43)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).44)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).46)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn XamlRoot(&self) -> ::windows::runtime::Result<super::XamlRoot> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).47)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    pub fn SetXamlRoot<'a, Param0: ::windows::runtime::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).48)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn AccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
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
            (::windows::runtime::Interface::vtable(this).49)(
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
    pub fn RemoveAccessKeyDisplayRequested<
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
            (::windows::runtime::Interface::vtable(this).50)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn AccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
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
            (::windows::runtime::Interface::vtable(this).51)(
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
    pub fn RemoveAccessKeyDisplayDismissed<
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
            (::windows::runtime::Interface::vtable(this).52)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn AccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
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
            (::windows::runtime::Interface::vtable(this).53)(
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
    pub fn RemoveAccessKeyInvoked<
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
            (::windows::runtime::Interface::vtable(this).54)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FindName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).55)(
                ::std::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElementOverrides>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn FontSizeProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn FontFamilyProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn FontWeightProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn FontStyleProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn FontStretchProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn CharacterSpacingProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ForegroundProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn LanguageProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IsTextScaleFactorEnabledProperty(
    ) -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn TextDecorationsProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn AllowFocusOnInteractionProperty() -> ::windows::runtime::Result<super::DependencyProperty>
    {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn AccessKeyProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ExitDisplayModeOnAccessKeyInvokedProperty(
    ) -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IsAccessKeyScopeProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn AccessKeyScopeOwnerProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn KeyTipPlacementModeProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn KeyTipHorizontalOffsetProperty() -> ::windows::runtime::Result<super::DependencyProperty>
    {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn KeyTipVerticalOffsetProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Core::CoreDispatcher>
    {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn ITextElementStatics<
        R,
        F: FnOnce(&ITextElementStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<TextElement, ITextElementStatics> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TextElement {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.TextElement;{a122ba22-833f-5220-a47e-6cd507531abe})",
    );
}
unsafe impl ::windows::runtime::Interface for TextElement {
    type Vtable = ITextElement_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2703407650,
        33599,
        21024,
        [164, 126, 108, 213, 7, 83, 26, 190],
    );
}
impl ::windows::runtime::RuntimeName for TextElement {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.TextElement";
}
impl ::std::convert::From<TextElement> for ::windows::runtime::IUnknown {
    fn from(value: TextElement) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&TextElement> for ::windows::runtime::IUnknown {
    fn from(value: &TextElement) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TextElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &TextElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<TextElement> for ::windows::runtime::IInspectable {
    fn from(value: TextElement) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TextElement> for ::windows::runtime::IInspectable {
    fn from(value: &TextElement) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for TextElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a TextElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<TextElement> for super::DependencyObject {
    fn from(value: TextElement) -> Self {
        ::std::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::std::convert::From<&TextElement> for super::DependencyObject {
    fn from(value: &TextElement) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for TextElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::DependencyObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for &TextElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::DependencyObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for TextElement {}
unsafe impl ::std::marker::Sync for TextElement {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct TextHighlighter(::windows::runtime::IInspectable);
impl TextHighlighter {
    #[cfg(feature = "undation_Collections")]
    pub fn Ranges(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::Collections::IVector<TextRange>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .6 ) ( :: std :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < super::super::super::super::Windows::Foundation::Collections:: IVector :: < TextRange > > ( result__ )
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::runtime::Result<super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, Param0: ::windows::runtime::IntoParam<'a, super::Media::Brush>>(
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
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Background(&self) -> ::windows::runtime::Result<super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetBackground<'a, Param0: ::windows::runtime::IntoParam<'a, super::Media::Brush>>(
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
    pub fn ForegroundProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITextHighlighterStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn BackgroundProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITextHighlighterStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn new() -> ::windows::runtime::Result<TextHighlighter> {
        Self::ITextHighlighterFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                ::std::ptr::null_mut(),
                &mut ::std::option::Option::<::windows::runtime::IInspectable>::None as *mut _ as _,
                &mut result__,
            )
            .from_abi::<TextHighlighter>(result__)
        })
    }
    pub fn ITextHighlighterStatics<
        R,
        F: FnOnce(&ITextHighlighterStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            TextHighlighter,
            ITextHighlighterStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ITextHighlighterFactory<
        R,
        F: FnOnce(&ITextHighlighterFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            TextHighlighter,
            ITextHighlighterFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TextHighlighter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.TextHighlighter;{b756e861-1d2b-5f6f-81fd-c51a5bc068ff})",
    );
}
unsafe impl ::windows::runtime::Interface for TextHighlighter {
    type Vtable = ITextHighlighter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3075926113,
        7467,
        24431,
        [129, 253, 197, 26, 91, 192, 104, 255],
    );
}
impl ::windows::runtime::RuntimeName for TextHighlighter {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.TextHighlighter";
}
impl ::std::convert::From<TextHighlighter> for ::windows::runtime::IUnknown {
    fn from(value: TextHighlighter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&TextHighlighter> for ::windows::runtime::IUnknown {
    fn from(value: &TextHighlighter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TextHighlighter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &TextHighlighter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<TextHighlighter> for ::windows::runtime::IInspectable {
    fn from(value: TextHighlighter) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TextHighlighter> for ::windows::runtime::IInspectable {
    fn from(value: &TextHighlighter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for TextHighlighter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a TextHighlighter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for TextHighlighter {}
unsafe impl ::std::marker::Sync for TextHighlighter {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct TextHighlighterBase(::windows::runtime::IInspectable);
impl TextHighlighterBase {
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Core::CoreDispatcher>
    {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TextHighlighterBase {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Documents.TextHighlighterBase;{5c21aaf0-3a17-5468-8aac-be14db0ed8c1})" ) ;
}
unsafe impl ::windows::runtime::Interface for TextHighlighterBase {
    type Vtable = ITextHighlighterBase_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1545710320,
        14871,
        21608,
        [138, 172, 190, 20, 219, 14, 216, 193],
    );
}
impl ::windows::runtime::RuntimeName for TextHighlighterBase {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.TextHighlighterBase";
}
impl ::std::convert::From<TextHighlighterBase> for ::windows::runtime::IUnknown {
    fn from(value: TextHighlighterBase) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&TextHighlighterBase> for ::windows::runtime::IUnknown {
    fn from(value: &TextHighlighterBase) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TextHighlighterBase {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &TextHighlighterBase {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<TextHighlighterBase> for ::windows::runtime::IInspectable {
    fn from(value: TextHighlighterBase) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TextHighlighterBase> for ::windows::runtime::IInspectable {
    fn from(value: &TextHighlighterBase) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for TextHighlighterBase
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a TextHighlighterBase
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<TextHighlighterBase> for super::DependencyObject {
    fn from(value: TextHighlighterBase) -> Self {
        ::std::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::std::convert::From<&TextHighlighterBase> for super::DependencyObject {
    fn from(value: &TextHighlighterBase) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for TextHighlighterBase {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::DependencyObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for &TextHighlighterBase {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::DependencyObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for TextHighlighterBase {}
unsafe impl ::std::marker::Sync for TextHighlighterBase {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct TextPointer(::windows::runtime::IInspectable);
impl TextPointer {
    pub fn Parent(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn VisualParent(&self) -> ::windows::runtime::Result<super::FrameworkElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::FrameworkElement>(result__)
        }
    }
    pub fn LogicalDirection(&self) -> ::windows::runtime::Result<LogicalDirection> {
        let this = self;
        unsafe {
            let mut result__: LogicalDirection = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<LogicalDirection>(result__)
        }
    }
    pub fn Offset(&self) -> ::windows::runtime::Result<i32> {
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
    #[cfg(feature = "undation")]
    pub fn GetCharacterRect(
        &self,
        direction: LogicalDirection,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Windows::Foundation::Rect =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                direction,
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::Foundation::Rect>(result__)
        }
    }
    pub fn GetPositionAtOffset(
        &self,
        offset: i32,
        direction: LogicalDirection,
    ) -> ::windows::runtime::Result<TextPointer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                offset,
                direction,
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TextPointer {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.TextPointer;{842eb385-ee41-5930-979b-438fa7525a51})",
    );
}
unsafe impl ::windows::runtime::Interface for TextPointer {
    type Vtable = ITextPointer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2217653125,
        60993,
        22832,
        [151, 155, 67, 143, 167, 82, 90, 81],
    );
}
impl ::windows::runtime::RuntimeName for TextPointer {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.TextPointer";
}
impl ::std::convert::From<TextPointer> for ::windows::runtime::IUnknown {
    fn from(value: TextPointer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&TextPointer> for ::windows::runtime::IUnknown {
    fn from(value: &TextPointer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TextPointer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &TextPointer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<TextPointer> for ::windows::runtime::IInspectable {
    fn from(value: TextPointer) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TextPointer> for ::windows::runtime::IInspectable {
    fn from(value: &TextPointer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for TextPointer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a TextPointer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for TextPointer {}
unsafe impl ::std::marker::Sync for TextPointer {}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct TextRange {
    pub StartIndex: i32,
    pub Length: i32,
}
impl TextRange {}
impl ::std::default::Default for TextRange {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TextRange {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TextRange")
            .field("StartIndex", &self.StartIndex)
            .field("Length", &self.Length)
            .finish()
    }
}
impl ::std::cmp::PartialEq for TextRange {
    fn eq(&self, other: &Self) -> bool {
        self.StartIndex == other.StartIndex && self.Length == other.Length
    }
}
impl ::std::cmp::Eq for TextRange {}
unsafe impl ::windows::runtime::Abi for TextRange {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for TextRange {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"struct(Microsoft.UI.Xaml.Documents.TextRange;i4;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct Typography(::windows::runtime::IInspectable);
impl Typography {
    pub fn AnnotationAlternatesProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetAnnotationAlternates<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<i32> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        })
    }
    pub fn SetAnnotationAlternates<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: i32,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn EastAsianExpertFormsProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetEastAsianExpertForms<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetEastAsianExpertForms<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn EastAsianLanguageProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetEastAsianLanguage<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<super::FontEastAsianLanguage> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: super::FontEastAsianLanguage = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::FontEastAsianLanguage>(result__)
        })
    }
    pub fn SetEastAsianLanguage<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: super::FontEastAsianLanguage,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn EastAsianWidthsProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetEastAsianWidths<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<super::FontEastAsianWidths> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: super::FontEastAsianWidths = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::FontEastAsianWidths>(result__)
        })
    }
    pub fn SetEastAsianWidths<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: super::FontEastAsianWidths,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StandardLigaturesProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStandardLigatures<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetStandardLigatures<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn ContextualLigaturesProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetContextualLigatures<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetContextualLigatures<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn DiscretionaryLigaturesProperty() -> ::windows::runtime::Result<super::DependencyProperty>
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetDiscretionaryLigatures<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetDiscretionaryLigatures<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn HistoricalLigaturesProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetHistoricalLigatures<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetHistoricalLigatures<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).29)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StandardSwashesProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStandardSwashes<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<i32> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        })
    }
    pub fn SetStandardSwashes<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: i32,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).32)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn ContextualSwashesProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetContextualSwashes<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<i32> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        })
    }
    pub fn SetContextualSwashes<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: i32,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).35)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn ContextualAlternatesProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetContextualAlternates<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetContextualAlternates<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).38)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticAlternatesProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStylisticAlternates<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<i32> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).40)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        })
    }
    pub fn SetStylisticAlternates<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: i32,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).41)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet1Property() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).42)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStylisticSet1<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).43)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetStylisticSet1<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).44)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet2Property() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStylisticSet2<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).46)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetStylisticSet2<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).47)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet3Property() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).48)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStylisticSet3<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).49)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetStylisticSet3<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).50)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet4Property() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).51)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStylisticSet4<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).52)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetStylisticSet4<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).53)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet5Property() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).54)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStylisticSet5<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).55)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetStylisticSet5<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).56)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet6Property() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).57)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStylisticSet6<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).58)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetStylisticSet6<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).59)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet7Property() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).60)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStylisticSet7<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).61)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetStylisticSet7<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).62)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet8Property() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).63)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStylisticSet8<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).64)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetStylisticSet8<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).65)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet9Property() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).66)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStylisticSet9<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).67)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetStylisticSet9<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).68)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet10Property() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).69)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStylisticSet10<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).70)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetStylisticSet10<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).71)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet11Property() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).72)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStylisticSet11<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).73)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetStylisticSet11<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).74)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet12Property() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).75)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStylisticSet12<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).76)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetStylisticSet12<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).77)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet13Property() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).78)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStylisticSet13<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).79)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetStylisticSet13<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).80)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet14Property() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).81)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStylisticSet14<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).82)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetStylisticSet14<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).83)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet15Property() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).84)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStylisticSet15<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).85)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetStylisticSet15<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).86)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet16Property() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).87)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStylisticSet16<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).88)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetStylisticSet16<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).89)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet17Property() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).90)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStylisticSet17<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).91)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetStylisticSet17<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).92)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet18Property() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).93)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStylisticSet18<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).94)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetStylisticSet18<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).95)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet19Property() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).96)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStylisticSet19<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).97)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetStylisticSet19<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).98)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet20Property() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).99)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStylisticSet20<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).100)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetStylisticSet20<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).101)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn CapitalsProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).102)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetCapitals<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::runtime::Result<super::FontCapitals> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: super::FontCapitals = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).103)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::FontCapitals>(result__)
        })
    }
    pub fn SetCapitals<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: super::FontCapitals,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).104)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn CapitalSpacingProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).105)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetCapitalSpacing<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).106)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetCapitalSpacing<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).107)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn KerningProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).108)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetKerning<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).109)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetKerning<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).110)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn CaseSensitiveFormsProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).111)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetCaseSensitiveForms<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).112)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetCaseSensitiveForms<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).113)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn HistoricalFormsProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).114)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetHistoricalForms<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).115)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetHistoricalForms<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).116)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn FractionProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).117)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetFraction<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::runtime::Result<super::FontFraction> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: super::FontFraction = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).118)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::FontFraction>(result__)
        })
    }
    pub fn SetFraction<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: super::FontFraction,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).119)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn NumeralStyleProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).120)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetNumeralStyle<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<super::FontNumeralStyle> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: super::FontNumeralStyle = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).121)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::FontNumeralStyle>(result__)
        })
    }
    pub fn SetNumeralStyle<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: super::FontNumeralStyle,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).122)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn NumeralAlignmentProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).123)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetNumeralAlignment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<super::FontNumeralAlignment> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: super::FontNumeralAlignment = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).124)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::FontNumeralAlignment>(result__)
        })
    }
    pub fn SetNumeralAlignment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: super::FontNumeralAlignment,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).125)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn SlashedZeroProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).126)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetSlashedZero<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).127)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetSlashedZero<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).128)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn MathematicalGreekProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).129)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetMathematicalGreek<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).130)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetMathematicalGreek<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).131)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn VariantsProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).132)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetVariants<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::runtime::Result<super::FontVariants> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: super::FontVariants = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).133)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::FontVariants>(result__)
        })
    }
    pub fn SetVariants<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: super::FontVariants,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).134)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn ITypographyStatics<
        R,
        F: FnOnce(&ITypographyStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Typography, ITypographyStatics> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Typography {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Typography;{fa27e2e3-be5e-5d21-9a5e-90cf102af828})",
    );
}
unsafe impl ::windows::runtime::Interface for Typography {
    type Vtable = ITypography_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4196917987,
        48734,
        23841,
        [154, 94, 144, 207, 16, 42, 248, 40],
    );
}
impl ::windows::runtime::RuntimeName for Typography {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Typography";
}
impl ::std::convert::From<Typography> for ::windows::runtime::IUnknown {
    fn from(value: Typography) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Typography> for ::windows::runtime::IUnknown {
    fn from(value: &Typography) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Typography {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Typography {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<Typography> for ::windows::runtime::IInspectable {
    fn from(value: Typography) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Typography> for ::windows::runtime::IInspectable {
    fn from(value: &Typography) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Typography {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Typography {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for Typography {}
unsafe impl ::std::marker::Sync for Typography {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct Underline(::windows::runtime::IInspectable);
impl Underline {
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
            Underline,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
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
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Core::CoreDispatcher>
    {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[cfg(feature = "undation_Collections")]
    pub fn Inlines(&self) -> ::windows::runtime::Result<InlineCollection> {
        let this = &::windows::runtime::Interface::cast::<ISpan>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InlineCollection>(result__)
        }
    }
    #[cfg(feature = "undation_Collections")]
    pub fn SetInlines<'a, Param0: ::windows::runtime::IntoParam<'a, InlineCollection>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISpan>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
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
    pub fn FontSize(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::runtime::Result<super::Media::FontFamily> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Media::FontFamily>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn FontWeight(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::FontWeight> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::FontWeight =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::FontWeight>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::UI::Text::FontWeight,
        >,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn FontStyle(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::FontStyle> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::FontStyle =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::FontStyle>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetFontStyle(
        &self,
        value: super::super::super::super::Windows::UI::Text::FontStyle,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn FontStretch(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::FontStretch>
    {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::FontStretch =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::FontStretch>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetFontStretch(
        &self,
        value: super::super::super::super::Windows::UI::Text::FontStretch,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::runtime::Result<super::Media::Brush> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, Param0: ::windows::runtime::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
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
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "_Text")]
    pub fn TextDecorations(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::super::Windows::UI::Text::TextDecorations>
    {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Windows::UI::Text::TextDecorations =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::super::Windows::UI::Text::TextDecorations>(result__)
        }
    }
    #[cfg(feature = "_Text")]
    pub fn SetTextDecorations(
        &self,
        value: super::super::super::super::Windows::UI::Text::TextDecorations,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn ContentStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ContentEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).32)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetAccessKey<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).34)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(
        &self,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).36)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).38)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).40)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::runtime::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).41)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).42)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).43)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).44)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).46)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn XamlRoot(&self) -> ::windows::runtime::Result<super::XamlRoot> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).47)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    pub fn SetXamlRoot<'a, Param0: ::windows::runtime::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).48)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn AccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).49)(
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
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).50)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn AccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).51)(
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
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).52)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "UI_Xaml_Input", feature = "undation"))]
    pub fn AccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::super::Windows::Foundation::EventRegistrationToken,
    > {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ : super::super::super::super::Windows::Foundation:: EventRegistrationToken = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).53)(
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
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::super::Windows::Foundation::EventRegistrationToken,
        >,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).54)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FindName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).55)(
                ::std::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElementOverrides>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Underline {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Underline;{68aaec6e-ea71-5ed2-b83e-91684b25efb9})",
    );
}
unsafe impl ::windows::runtime::Interface for Underline {
    type Vtable = IUnderline_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1756032110,
        60017,
        24274,
        [184, 62, 145, 104, 75, 37, 239, 185],
    );
}
impl ::windows::runtime::RuntimeName for Underline {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Underline";
}
impl ::std::convert::From<Underline> for ::windows::runtime::IUnknown {
    fn from(value: Underline) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Underline> for ::windows::runtime::IUnknown {
    fn from(value: &Underline) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Underline {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Underline {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<Underline> for ::windows::runtime::IInspectable {
    fn from(value: Underline) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Underline> for ::windows::runtime::IInspectable {
    fn from(value: &Underline) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Underline {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Underline {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<Underline> for Span {
    fn from(value: Underline) -> Self {
        ::std::convert::Into::<Span>::into(&value)
    }
}
impl ::std::convert::From<&Underline> for Span {
    fn from(value: &Underline) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Span> for Underline {
    fn into_param(self) -> ::windows::runtime::Param<'a, Span> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<Span>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Span> for &Underline {
    fn into_param(self) -> ::windows::runtime::Param<'a, Span> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<Span>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<Underline> for Inline {
    fn from(value: Underline) -> Self {
        ::std::convert::Into::<Inline>::into(&value)
    }
}
impl ::std::convert::From<&Underline> for Inline {
    fn from(value: &Underline) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Inline> for Underline {
    fn into_param(self) -> ::windows::runtime::Param<'a, Inline> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<Inline>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Inline> for &Underline {
    fn into_param(self) -> ::windows::runtime::Param<'a, Inline> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<Inline>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<Underline> for TextElement {
    fn from(value: Underline) -> Self {
        ::std::convert::Into::<TextElement>::into(&value)
    }
}
impl ::std::convert::From<&Underline> for TextElement {
    fn from(value: &Underline) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for Underline {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<TextElement>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for &Underline {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<TextElement>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<Underline> for super::DependencyObject {
    fn from(value: Underline) -> Self {
        ::std::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::std::convert::From<&Underline> for super::DependencyObject {
    fn from(value: &Underline) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for Underline {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::DependencyObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for &Underline {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::DependencyObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for Underline {}
unsafe impl ::std::marker::Sync for Underline {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct UnderlineStyle(pub i32);
impl UnderlineStyle {
    pub const None: UnderlineStyle = UnderlineStyle(0i32);
    pub const Single: UnderlineStyle = UnderlineStyle(1i32);
}
impl ::std::convert::From<i32> for UnderlineStyle {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UnderlineStyle {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UnderlineStyle {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Documents.UnderlineStyle;i4)",
    );
}
