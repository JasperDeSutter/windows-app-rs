#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[cfg(feature = "UI_Input_Interop")]
pub mod Interop;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CrossSlideThresholds {
    pub SelectionStart: f32,
    pub SpeedBumpStart: f32,
    pub SpeedBumpEnd: f32,
    pub RearrangeStart: f32,
}
impl CrossSlideThresholds {}
impl ::std::default::Default for CrossSlideThresholds {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CrossSlideThresholds {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CrossSlideThresholds")
            .field("SelectionStart", &self.SelectionStart)
            .field("SpeedBumpStart", &self.SpeedBumpStart)
            .field("SpeedBumpEnd", &self.SpeedBumpEnd)
            .field("RearrangeStart", &self.RearrangeStart)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CrossSlideThresholds {
    fn eq(&self, other: &Self) -> bool {
        self.SelectionStart == other.SelectionStart
            && self.SpeedBumpStart == other.SpeedBumpStart
            && self.SpeedBumpEnd == other.SpeedBumpEnd
            && self.RearrangeStart == other.RearrangeStart
    }
}
impl ::std::cmp::Eq for CrossSlideThresholds {}
unsafe impl ::windows::runtime::Abi for CrossSlideThresholds {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CrossSlideThresholds {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"struct(Microsoft.UI.Input.CrossSlideThresholds;f4;f4;f4;f4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CrossSlidingEventArgs(::windows::runtime::IInspectable);
impl CrossSlidingEventArgs {
    pub fn CrossSlidingState(&self) -> ::windows::runtime::Result<CrossSlidingState> {
        let this = self;
        unsafe {
            let mut result__: CrossSlidingState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CrossSlidingState>(result__)
        }
    }
    pub fn PointerDeviceType(&self) -> ::windows::runtime::Result<PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: PointerDeviceType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PointerDeviceType>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn Position(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Windows::Foundation::Point =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::Point>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CrossSlidingEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Input.CrossSlidingEventArgs;{7679641f-ba9f-543c-a7c8-6229a98f89ef})",
    );
}
unsafe impl ::windows::runtime::Interface for CrossSlidingEventArgs {
    type Vtable = ICrossSlidingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1987666975,
        47775,
        21564,
        [167, 200, 98, 41, 169, 143, 137, 239],
    );
}
impl ::windows::runtime::RuntimeName for CrossSlidingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.CrossSlidingEventArgs";
}
impl ::std::convert::From<CrossSlidingEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: CrossSlidingEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CrossSlidingEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &CrossSlidingEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CrossSlidingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CrossSlidingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CrossSlidingEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: CrossSlidingEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CrossSlidingEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &CrossSlidingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CrossSlidingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CrossSlidingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CrossSlidingEventArgs {}
unsafe impl ::std::marker::Sync for CrossSlidingEventArgs {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CrossSlidingState(pub i32);
impl CrossSlidingState {
    pub const Started: CrossSlidingState = CrossSlidingState(0i32);
    pub const Dragging: CrossSlidingState = CrossSlidingState(1i32);
    pub const Selecting: CrossSlidingState = CrossSlidingState(2i32);
    pub const SelectSpeedBumping: CrossSlidingState = CrossSlidingState(3i32);
    pub const SpeedBumping: CrossSlidingState = CrossSlidingState(4i32);
    pub const Rearranging: CrossSlidingState = CrossSlidingState(5i32);
    pub const Completed: CrossSlidingState = CrossSlidingState(6i32);
}
impl ::std::convert::From<i32> for CrossSlidingState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CrossSlidingState {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CrossSlidingState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Input.CrossSlidingState;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct DraggingEventArgs(::windows::runtime::IInspectable);
impl DraggingEventArgs {
    pub fn DraggingState(&self) -> ::windows::runtime::Result<DraggingState> {
        let this = self;
        unsafe {
            let mut result__: DraggingState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<DraggingState>(result__)
        }
    }
    pub fn PointerDeviceType(&self) -> ::windows::runtime::Result<PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: PointerDeviceType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PointerDeviceType>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn Position(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Windows::Foundation::Point =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::Point>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DraggingEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Input.DraggingEventArgs;{3efb1b75-3d3b-550e-963d-0828ca76128a})",
    );
}
unsafe impl ::windows::runtime::Interface for DraggingEventArgs {
    type Vtable = IDraggingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1056643957,
        15675,
        21774,
        [150, 61, 8, 40, 202, 118, 18, 138],
    );
}
impl ::windows::runtime::RuntimeName for DraggingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.DraggingEventArgs";
}
impl ::std::convert::From<DraggingEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: DraggingEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DraggingEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &DraggingEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DraggingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &DraggingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<DraggingEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: DraggingEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DraggingEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &DraggingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DraggingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a DraggingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for DraggingEventArgs {}
unsafe impl ::std::marker::Sync for DraggingEventArgs {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DraggingState(pub i32);
impl DraggingState {
    pub const Started: DraggingState = DraggingState(0i32);
    pub const Continuing: DraggingState = DraggingState(1i32);
    pub const Completed: DraggingState = DraggingState(2i32);
}
impl ::std::convert::From<i32> for DraggingState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DraggingState {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DraggingState {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"enum(Microsoft.UI.Input.DraggingState;i4)");
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct GestureRecognizer(::windows::runtime::IInspectable);
impl GestureRecognizer {
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
            GestureRecognizer,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn AutoProcessInertia(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn SetAutoProcessInertia(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn CrossSlideExact(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn SetCrossSlideExact(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn CrossSlideHorizontally(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn SetCrossSlideHorizontally(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn CrossSlideThresholds(&self) -> ::windows::runtime::Result<CrossSlideThresholds> {
        let this = self;
        unsafe {
            let mut result__: CrossSlideThresholds = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CrossSlideThresholds>(result__)
        }
    }
    pub fn SetCrossSlideThresholds<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, CrossSlideThresholds>,
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
    pub fn GestureSettings(&self) -> ::windows::runtime::Result<GestureSettings> {
        let this = self;
        unsafe {
            let mut result__: GestureSettings = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<GestureSettings>(result__)
        }
    }
    pub fn SetGestureSettings(&self, value: GestureSettings) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsActive(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn IsInertial(&self) -> ::windows::runtime::Result<bool> {
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
    #[cfg(feature = "undation")]
    pub fn PivotCenter(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Windows::Foundation::Point =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn SetPivotCenter<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::Windows::Foundation::Point>,
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
    pub fn PivotRadius(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn SetPivotRadius(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).21)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn InertiaExpansionDeceleration(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn SetInertiaExpansionDeceleration(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn InertiaExpansion(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn SetInertiaExpansion(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn InertiaRotationAngle(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn SetInertiaRotationAngle(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).27)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn InertiaRotationDeceleration(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn SetInertiaRotationDeceleration(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).29)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn InertiaTranslationDeceleration(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn SetInertiaTranslationDeceleration(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).31)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn InertiaTranslationDisplacement(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn SetInertiaTranslationDisplacement(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).33)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn ManipulationExact(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetManipulationExact(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).35)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn MouseWheelParameters(&self) -> ::windows::runtime::Result<MouseWheelParameters> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<MouseWheelParameters>(result__)
        }
    }
    pub fn ShowGestureFeedback(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn SetShowGestureFeedback(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).38)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn CanBeDoubleTap<'a, Param0: ::windows::runtime::IntoParam<'a, PointerPoint>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn CompleteGesture(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).40)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn ProcessDownEvent<'a, Param0: ::windows::runtime::IntoParam<'a, PointerPoint>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).41)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation_Collections")]
    pub fn ProcessMoveEvents<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::Collections::IVector<PointerPoint>,
        >,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).42)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ProcessMouseWheelEvent<'a, Param0: ::windows::runtime::IntoParam<'a, PointerPoint>>(
        &self,
        value: Param0,
        isshiftkeydown: bool,
        iscontrolkeydown: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).43)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
                isshiftkeydown,
                iscontrolkeydown,
            )
            .ok()
        }
    }
    pub fn ProcessInertia(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).44)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn ProcessUpEvent<'a, Param0: ::windows::runtime::IntoParam<'a, PointerPoint>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).45)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn Tapped<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::TypedEventHandler<
                GestureRecognizer,
                TappedEventArgs,
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
            (::windows::runtime::Interface::vtable(this).46)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveTapped<
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
            (::windows::runtime::Interface::vtable(this).47)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn RightTapped<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::TypedEventHandler<
                GestureRecognizer,
                RightTappedEventArgs,
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
            (::windows::runtime::Interface::vtable(this).48)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveRightTapped<
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
            (::windows::runtime::Interface::vtable(this).49)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn Holding<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::TypedEventHandler<
                GestureRecognizer,
                HoldingEventArgs,
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
            (::windows::runtime::Interface::vtable(this).50)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveHolding<
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
            (::windows::runtime::Interface::vtable(this).51)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn Dragging<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::TypedEventHandler<
                GestureRecognizer,
                DraggingEventArgs,
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
            (::windows::runtime::Interface::vtable(this).52)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveDragging<
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
            (::windows::runtime::Interface::vtable(this).53)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn ManipulationStarted<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::TypedEventHandler<
                GestureRecognizer,
                ManipulationStartedEventArgs,
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
            (::windows::runtime::Interface::vtable(this).54)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveManipulationStarted<
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
            (::windows::runtime::Interface::vtable(this).55)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn ManipulationUpdated<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::TypedEventHandler<
                GestureRecognizer,
                ManipulationUpdatedEventArgs,
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
            (::windows::runtime::Interface::vtable(this).56)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveManipulationUpdated<
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
            (::windows::runtime::Interface::vtable(this).57)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn ManipulationInertiaStarting<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::TypedEventHandler<
                GestureRecognizer,
                ManipulationInertiaStartingEventArgs,
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
            (::windows::runtime::Interface::vtable(this).58)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveManipulationInertiaStarting<
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
            (::windows::runtime::Interface::vtable(this).59)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn ManipulationCompleted<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::TypedEventHandler<
                GestureRecognizer,
                ManipulationCompletedEventArgs,
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
            (::windows::runtime::Interface::vtable(this).60)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveManipulationCompleted<
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
            (::windows::runtime::Interface::vtable(this).61)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn CrossSliding<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::TypedEventHandler<
                GestureRecognizer,
                CrossSlidingEventArgs,
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
            (::windows::runtime::Interface::vtable(this).62)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveCrossSliding<
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
            (::windows::runtime::Interface::vtable(this).63)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GestureRecognizer {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Input.GestureRecognizer;{cda89afc-6bd0-595c-ba37-545fce5bf016})",
    );
}
unsafe impl ::windows::runtime::Interface for GestureRecognizer {
    type Vtable = IGestureRecognizer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3450379004,
        27600,
        22876,
        [186, 55, 84, 95, 206, 91, 240, 22],
    );
}
impl ::windows::runtime::RuntimeName for GestureRecognizer {
    const NAME: &'static str = "Microsoft.UI.Input.GestureRecognizer";
}
impl ::std::convert::From<GestureRecognizer> for ::windows::runtime::IUnknown {
    fn from(value: GestureRecognizer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&GestureRecognizer> for ::windows::runtime::IUnknown {
    fn from(value: &GestureRecognizer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GestureRecognizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &GestureRecognizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<GestureRecognizer> for ::windows::runtime::IInspectable {
    fn from(value: GestureRecognizer) -> Self {
        value.0
    }
}
impl ::std::convert::From<&GestureRecognizer> for ::windows::runtime::IInspectable {
    fn from(value: &GestureRecognizer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GestureRecognizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a GestureRecognizer
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for GestureRecognizer {}
unsafe impl ::std::marker::Sync for GestureRecognizer {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct GestureSettings(pub u32);
impl GestureSettings {
    pub const None: GestureSettings = GestureSettings(0u32);
    pub const Tap: GestureSettings = GestureSettings(1u32);
    pub const DoubleTap: GestureSettings = GestureSettings(2u32);
    pub const Hold: GestureSettings = GestureSettings(4u32);
    pub const HoldWithMouse: GestureSettings = GestureSettings(8u32);
    pub const RightTap: GestureSettings = GestureSettings(16u32);
    pub const Drag: GestureSettings = GestureSettings(32u32);
    pub const ManipulationTranslateX: GestureSettings = GestureSettings(64u32);
    pub const ManipulationTranslateY: GestureSettings = GestureSettings(128u32);
    pub const ManipulationTranslateRailsX: GestureSettings = GestureSettings(256u32);
    pub const ManipulationTranslateRailsY: GestureSettings = GestureSettings(512u32);
    pub const ManipulationRotate: GestureSettings = GestureSettings(1024u32);
    pub const ManipulationScale: GestureSettings = GestureSettings(2048u32);
    pub const ManipulationTranslateInertia: GestureSettings = GestureSettings(4096u32);
    pub const ManipulationRotateInertia: GestureSettings = GestureSettings(8192u32);
    pub const ManipulationScaleInertia: GestureSettings = GestureSettings(16384u32);
    pub const CrossSlide: GestureSettings = GestureSettings(32768u32);
    pub const ManipulationMultipleFingerPanning: GestureSettings = GestureSettings(65536u32);
}
impl ::std::convert::From<u32> for GestureSettings {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GestureSettings {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for GestureSettings {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"enum(Microsoft.UI.Input.GestureSettings;u4)");
}
impl ::std::ops::BitOr for GestureSettings {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for GestureSettings {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for GestureSettings {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for GestureSettings {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for GestureSettings {
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
pub struct HoldingEventArgs(::windows::runtime::IInspectable);
impl HoldingEventArgs {
    pub fn HoldingState(&self) -> ::windows::runtime::Result<HoldingState> {
        let this = self;
        unsafe {
            let mut result__: HoldingState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<HoldingState>(result__)
        }
    }
    pub fn PointerDeviceType(&self) -> ::windows::runtime::Result<PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: PointerDeviceType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PointerDeviceType>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn Position(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Windows::Foundation::Point =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::Point>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HoldingEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Input.HoldingEventArgs;{8e449e85-d223-533c-b0b2-bf7c6d10c2db})",
    );
}
unsafe impl ::windows::runtime::Interface for HoldingEventArgs {
    type Vtable = IHoldingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2386861701,
        53795,
        21308,
        [176, 178, 191, 124, 109, 16, 194, 219],
    );
}
impl ::windows::runtime::RuntimeName for HoldingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.HoldingEventArgs";
}
impl ::std::convert::From<HoldingEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: HoldingEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HoldingEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &HoldingEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HoldingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HoldingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<HoldingEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: HoldingEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HoldingEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &HoldingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HoldingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a HoldingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for HoldingEventArgs {}
unsafe impl ::std::marker::Sync for HoldingEventArgs {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct HoldingState(pub i32);
impl HoldingState {
    pub const Started: HoldingState = HoldingState(0i32);
    pub const Completed: HoldingState = HoldingState(1i32);
    pub const Canceled: HoldingState = HoldingState(2i32);
}
impl ::std::convert::From<i32> for HoldingState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HoldingState {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for HoldingState {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"enum(Microsoft.UI.Input.HoldingState;i4)");
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ICrossSlidingEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICrossSlidingEventArgs {
    type Vtable = ICrossSlidingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1987666975,
        47775,
        21564,
        [167, 200, 98, 41, 169, 143, 137, 239],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICrossSlidingEventArgs_abi(
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
        result__: *mut CrossSlidingState,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut PointerDeviceType,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::Windows::Foundation::Point,
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
pub struct IDraggingEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDraggingEventArgs {
    type Vtable = IDraggingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1056643957,
        15675,
        21774,
        [150, 61, 8, 40, 202, 118, 18, 138],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDraggingEventArgs_abi(
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
        result__: *mut DraggingState,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut PointerDeviceType,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::Windows::Foundation::Point,
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
pub struct IGestureRecognizer(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGestureRecognizer {
    type Vtable = IGestureRecognizer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3450379004,
        27600,
        22876,
        [186, 55, 84, 95, 206, 91, 240, 22],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IGestureRecognizer_abi(
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
        result__: *mut CrossSlideThresholds,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: CrossSlideThresholds,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut GestureSettings,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: GestureSettings,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::Windows::Foundation::Point,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::super::super::Windows::Foundation::Point,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f32,
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
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "undation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation_Collections"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
        isshiftkeydown: bool,
        iscontrolkeydown: bool,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
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
pub struct IHoldingEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHoldingEventArgs {
    type Vtable = IHoldingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2386861701,
        53795,
        21308,
        [176, 178, 191, 124, 109, 16, 194, 219],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IHoldingEventArgs_abi(
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
        result__: *mut HoldingState,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut PointerDeviceType,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::Windows::Foundation::Point,
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
pub struct IInputCursor(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInputCursor {
    type Vtable = IInputCursor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        899356153,
        6594,
        22292,
        [132, 50, 117, 23, 104, 38, 64, 107],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputCursor_abi(
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
pub struct IInputCursorFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInputCursorFactory {
    type Vtable = IInputCursorFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        793207931,
        19424,
        21481,
        [190, 126, 195, 141, 84, 89, 219, 107],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputCursorFactory_abi(
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
pub struct IInputDesktopResourceCursor(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInputDesktopResourceCursor {
    type Vtable = IInputDesktopResourceCursor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        502429567,
        31888,
        22780,
        [167, 163, 213, 115, 108, 101, 16, 253],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputDesktopResourceCursor_abi(
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
        result__: *mut u32,
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
pub struct IInputDesktopResourceCursorStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInputDesktopResourceCursorStatics {
    type Vtable = IInputDesktopResourceCursorStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4097891383,
        41142,
        22251,
        [188, 236, 176, 36, 242, 35, 61, 71],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputDesktopResourceCursorStatics_abi(
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
        resourceid: u32,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        modulename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        resourceid: u32,
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
pub struct IInputKeyboardSourceStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInputKeyboardSourceStatics {
    type Vtable = IInputKeyboardSourceStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4108408381,
        35886,
        23501,
        [183, 132, 71, 173, 234, 163, 205, 126],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputKeyboardSourceStatics_abi(
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
    #[cfg(all(feature = "stem", feature = "_Core"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        virtualkey: super::super::super::Windows::System::VirtualKey,
        result__: *mut super::super::super::Windows::UI::Core::CoreVirtualKeyStates,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "stem", feature = "_Core")))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IInputLightDismissAction(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInputLightDismissAction {
    type Vtable = IInputLightDismissAction_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3903034626,
        43104,
        20527,
        [140, 16, 54, 70, 212, 58, 236, 241],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputLightDismissAction_abi(
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
pub struct IInputLightDismissActionStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInputLightDismissActionStatics {
    type Vtable = IInputLightDismissActionStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3986394607,
        25750,
        20841,
        [152, 77, 212, 75, 78, 105, 6, 35],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputLightDismissActionStatics_abi(
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
pub struct IInputLightDismissEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInputLightDismissEventArgs {
    type Vtable = IInputLightDismissEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        126247150,
        1994,
        22536,
        [185, 130, 230, 232, 153, 207, 9, 140],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputLightDismissEventArgs_abi(
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
pub struct IInputObject(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInputObject {
    type Vtable = IInputObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1122876552,
        54150,
        21581,
        [177, 184, 104, 97, 127, 230, 130, 130],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputObject_abi(
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
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IInputObjectFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInputObjectFactory {
    type Vtable = IInputObjectFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4151864258,
        45240,
        22881,
        [154, 87, 174, 25, 157, 69, 33, 6],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputObjectFactory_abi(
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
pub struct IInputPointerSource(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInputPointerSource {
    type Vtable = IInputPointerSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1785472868,
        50164,
        23525,
        [132, 71, 201, 169, 135, 102, 194, 64],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputPointerSource_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut InputPointerSourceDeviceKinds,
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
pub struct IInputSystemCursor(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInputSystemCursor {
    type Vtable = IInputSystemCursor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1509243111,
        50432,
        22955,
        [139, 84, 11, 198, 16, 15, 212, 158],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputSystemCursor_abi(
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
        result__: *mut InputSystemCursorShape,
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
pub struct IInputSystemCursorStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInputSystemCursorStatics {
    type Vtable = IInputSystemCursorStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3548777398,
        27018,
        22548,
        [174, 221, 194, 250, 139, 186, 90, 2],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputSystemCursorStatics_abi(
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
        r#type: InputSystemCursorShape,
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
pub struct IManipulationCompletedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IManipulationCompletedEventArgs {
    type Vtable = IManipulationCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        235030996,
        18148,
        21849,
        [174, 227, 250, 69, 206, 42, 127, 86],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationCompletedEventArgs_abi(
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
        result__: *mut ManipulationDelta,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut PointerDeviceType,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::Windows::Foundation::Point,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ManipulationVelocities,
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
pub struct IManipulationInertiaStartingEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IManipulationInertiaStartingEventArgs {
    type Vtable = IManipulationInertiaStartingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2902060913,
        28181,
        22187,
        [146, 96, 240, 211, 206, 95, 102, 232],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationInertiaStartingEventArgs_abi(
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
        result__: *mut ManipulationDelta,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ManipulationDelta,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut PointerDeviceType,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::Windows::Foundation::Point,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ManipulationVelocities,
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
pub struct IManipulationStartedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IManipulationStartedEventArgs {
    type Vtable = IManipulationStartedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1247897107,
        61169,
        24347,
        [167, 104, 7, 117, 71, 141, 73, 212],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationStartedEventArgs_abi(
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
        result__: *mut ManipulationDelta,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut PointerDeviceType,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::Windows::Foundation::Point,
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
pub struct IManipulationUpdatedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IManipulationUpdatedEventArgs {
    type Vtable = IManipulationUpdatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1080957281,
        3224,
        24512,
        [179, 216, 17, 100, 146, 239, 0, 83],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationUpdatedEventArgs_abi(
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
        result__: *mut ManipulationDelta,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ManipulationDelta,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut PointerDeviceType,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::Windows::Foundation::Point,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ManipulationVelocities,
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
pub struct IMouseWheelParameters(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMouseWheelParameters {
    type Vtable = IMouseWheelParameters_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1838726720,
        7510,
        20945,
        [170, 13, 243, 37, 67, 156, 208, 9],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IMouseWheelParameters_abi(
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
        result__: *mut super::super::super::Windows::Foundation::Point,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::super::super::Windows::Foundation::Point,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::Windows::Foundation::Point,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::super::super::Windows::Foundation::Point,
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
pub struct IPointerEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPointerEventArgs {
    type Vtable = IPointerEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2254117004,
        11989,
        24056,
        [130, 159, 172, 7, 1, 213, 197, 26],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerEventArgs_abi(
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
    #[cfg(feature = "stem")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::Windows::System::VirtualKeyModifiers,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "stem"))] usize,
    #[cfg(feature = "undation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation_Collections"))] usize,
    #[cfg(feature = "undation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        transform: ::windows::runtime::RawPtr,
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
pub struct IPointerPoint(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPointerPoint {
    type Vtable = IPointerPoint_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        222498534,
        9516,
        22948,
        [178, 162, 212, 66, 100, 220, 106, 64],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPoint_abi(
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
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut PointerDeviceType,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::Windows::Foundation::Point,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        transform: ::windows::runtime::RawPtr,
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
pub struct IPointerPointProperties(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPointerPointProperties {
    type Vtable = IPointerPointProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3613453687,
        19216,
        22437,
        [179, 204, 217, 191, 52, 19, 233, 150],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPointProperties_abi(
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
        result__: *mut super::super::super::Windows::Foundation::Rect,
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
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut PointerUpdateKind,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IPointerPointTransform(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPointerPointTransform {
    type Vtable = IPointerPointTransform_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3678900668,
        39245,
        21703,
        [146, 239, 102, 234, 29, 233, 180, 60],
    );
}
impl IPointerPointTransform {
    pub fn Inverse(&self) -> ::windows::runtime::Result<IPointerPointTransform> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IPointerPointTransform>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn TryTransform<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::Windows::Foundation::Point>,
    >(
        &self,
        inpoint: Param0,
        outpoint: &mut super::super::super::Windows::Foundation::Point,
    ) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                inpoint.into_param().abi(),
                outpoint,
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn TryTransformBounds<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::Windows::Foundation::Rect>,
    >(
        &self,
        inrect: Param0,
        outrect: &mut super::super::super::Windows::Foundation::Rect,
    ) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                inrect.into_param().abi(),
                outrect,
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IPointerPointTransform {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{db4791bc-994d-54c7-92ef-66ea1de9b43c}");
}
impl ::std::convert::From<IPointerPointTransform> for ::windows::runtime::IUnknown {
    fn from(value: IPointerPointTransform) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPointerPointTransform> for ::windows::runtime::IUnknown {
    fn from(value: &IPointerPointTransform) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IPointerPointTransform
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IPointerPointTransform
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IPointerPointTransform> for ::windows::runtime::IInspectable {
    fn from(value: IPointerPointTransform) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IPointerPointTransform> for ::windows::runtime::IInspectable {
    fn from(value: &IPointerPointTransform) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for IPointerPointTransform
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IPointerPointTransform
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPointTransform_abi(
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
        inpoint: super::super::super::Windows::Foundation::Point,
        outpoint: *mut super::super::super::Windows::Foundation::Point,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        inrect: super::super::super::Windows::Foundation::Rect,
        outrect: *mut super::super::super::Windows::Foundation::Rect,
        result__: *mut bool,
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
pub struct IPointerPredictor(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPointerPredictor {
    type Vtable = IPointerPredictor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        314638572,
        8448,
        22111,
        [166, 12, 241, 24, 127, 67, 136, 40],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPredictor_abi(
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
        point: ::windows::runtime::RawPtr,
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
#[doc(hidden)]
pub struct IPointerPredictorStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPointerPredictorStatics {
    type Vtable = IPointerPredictorStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2024337200,
        15964,
        21965,
        [143, 133, 101, 172, 9, 177, 169, 135],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPredictorStatics_abi(
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
        inputpointersource: ::windows::runtime::RawPtr,
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
pub struct IRightTappedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRightTappedEventArgs {
    type Vtable = IRightTappedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2415344441,
        34942,
        20644,
        [133, 0, 119, 149, 48, 57, 220, 180],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRightTappedEventArgs_abi(
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
        result__: *mut PointerDeviceType,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::Windows::Foundation::Point,
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
pub struct ITappedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITappedEventArgs {
    type Vtable = ITappedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3282049973,
        24694,
        24079,
        [135, 26, 157, 148, 166, 168, 248, 43],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITappedEventArgs_abi(
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
        result__: *mut PointerDeviceType,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "undation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::Windows::Foundation::Point,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "undation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct InputCursor(::windows::runtime::IInspectable);
impl InputCursor {
    #[cfg(feature = "undation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::super::Windows::Foundation::IClosable,
        >(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InputCursor {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Input.InputCursor;{359b15f9-19c2-5714-8432-75176826406b})",
    );
}
unsafe impl ::windows::runtime::Interface for InputCursor {
    type Vtable = IInputCursor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        899356153,
        6594,
        22292,
        [132, 50, 117, 23, 104, 38, 64, 107],
    );
}
impl ::windows::runtime::RuntimeName for InputCursor {
    const NAME: &'static str = "Microsoft.UI.Input.InputCursor";
}
impl ::std::convert::From<InputCursor> for ::windows::runtime::IUnknown {
    fn from(value: InputCursor) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InputCursor> for ::windows::runtime::IUnknown {
    fn from(value: &InputCursor) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InputCursor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InputCursor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<InputCursor> for ::windows::runtime::IInspectable {
    fn from(value: InputCursor) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InputCursor> for ::windows::runtime::IInspectable {
    fn from(value: &InputCursor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InputCursor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InputCursor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "undation")]
impl ::std::convert::TryFrom<InputCursor> for super::super::super::Windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InputCursor) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "undation")]
impl ::std::convert::TryFrom<&InputCursor> for super::super::super::Windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InputCursor) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "undation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Windows::Foundation::IClosable>
    for InputCursor
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::super::Windows::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "undation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Windows::Foundation::IClosable>
    for &InputCursor
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::super::Windows::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::super::Windows::Foundation::IClosable>::try_into(
            self,
        )
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for InputCursor {}
unsafe impl ::std::marker::Sync for InputCursor {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct InputDesktopResourceCursor(::windows::runtime::IInspectable);
impl InputDesktopResourceCursor {
    pub fn ModuleName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn ResourceId(&self) -> ::windows::runtime::Result<u32> {
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
    pub fn Create(resourceid: u32) -> ::windows::runtime::Result<InputDesktopResourceCursor> {
        Self::IInputDesktopResourceCursorStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                resourceid,
                &mut result__,
            )
            .from_abi::<InputDesktopResourceCursor>(result__)
        })
    }
    pub fn CreateFromModule<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        modulename: Param0,
        resourceid: u32,
    ) -> ::windows::runtime::Result<InputDesktopResourceCursor> {
        Self::IInputDesktopResourceCursorStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                modulename.into_param().abi(),
                resourceid,
                &mut result__,
            )
            .from_abi::<InputDesktopResourceCursor>(result__)
        })
    }
    #[cfg(feature = "undation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::super::Windows::Foundation::IClosable,
        >(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn IInputDesktopResourceCursorStatics<
        R,
        F: FnOnce(&IInputDesktopResourceCursorStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            InputDesktopResourceCursor,
            IInputDesktopResourceCursorStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InputDesktopResourceCursor {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Input.InputDesktopResourceCursor;{1df2777f-7c90-58fc-a7a3-d5736c6510fd})",
    );
}
unsafe impl ::windows::runtime::Interface for InputDesktopResourceCursor {
    type Vtable = IInputDesktopResourceCursor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        502429567,
        31888,
        22780,
        [167, 163, 213, 115, 108, 101, 16, 253],
    );
}
impl ::windows::runtime::RuntimeName for InputDesktopResourceCursor {
    const NAME: &'static str = "Microsoft.UI.Input.InputDesktopResourceCursor";
}
impl ::std::convert::From<InputDesktopResourceCursor> for ::windows::runtime::IUnknown {
    fn from(value: InputDesktopResourceCursor) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InputDesktopResourceCursor> for ::windows::runtime::IUnknown {
    fn from(value: &InputDesktopResourceCursor) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for InputDesktopResourceCursor
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &InputDesktopResourceCursor
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<InputDesktopResourceCursor> for ::windows::runtime::IInspectable {
    fn from(value: InputDesktopResourceCursor) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InputDesktopResourceCursor> for ::windows::runtime::IInspectable {
    fn from(value: &InputDesktopResourceCursor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for InputDesktopResourceCursor
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a InputDesktopResourceCursor
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "undation")]
impl ::std::convert::TryFrom<InputDesktopResourceCursor>
    for super::super::super::Windows::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: InputDesktopResourceCursor) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "undation")]
impl ::std::convert::TryFrom<&InputDesktopResourceCursor>
    for super::super::super::Windows::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InputDesktopResourceCursor) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "undation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Windows::Foundation::IClosable>
    for InputDesktopResourceCursor
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::super::Windows::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "undation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Windows::Foundation::IClosable>
    for &InputDesktopResourceCursor
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::super::Windows::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::super::Windows::Foundation::IClosable>::try_into(
            self,
        )
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<InputDesktopResourceCursor> for InputCursor {
    fn from(value: InputDesktopResourceCursor) -> Self {
        ::std::convert::Into::<InputCursor>::into(&value)
    }
}
impl ::std::convert::From<&InputDesktopResourceCursor> for InputCursor {
    fn from(value: &InputDesktopResourceCursor) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, InputCursor> for InputDesktopResourceCursor {
    fn into_param(self) -> ::windows::runtime::Param<'a, InputCursor> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<InputCursor>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, InputCursor> for &InputDesktopResourceCursor {
    fn into_param(self) -> ::windows::runtime::Param<'a, InputCursor> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<InputCursor>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for InputDesktopResourceCursor {}
unsafe impl ::std::marker::Sync for InputDesktopResourceCursor {}
pub struct InputKeyboardSource {}
impl InputKeyboardSource {
    #[cfg(all(feature = "stem", feature = "_Core"))]
    pub fn GetKeyStateForCurrentThread(
        virtualkey: super::super::super::Windows::System::VirtualKey,
    ) -> ::windows::runtime::Result<super::super::super::Windows::UI::Core::CoreVirtualKeyStates>
    {
        Self::IInputKeyboardSourceStatics(|this| unsafe {
            let mut result__: super::super::super::Windows::UI::Core::CoreVirtualKeyStates =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                virtualkey,
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::UI::Core::CoreVirtualKeyStates>(result__)
        })
    }
    pub fn IInputKeyboardSourceStatics<
        R,
        F: FnOnce(&IInputKeyboardSourceStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            InputKeyboardSource,
            IInputKeyboardSourceStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for InputKeyboardSource {
    const NAME: &'static str = "Microsoft.UI.Input.InputKeyboardSource";
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct InputLightDismissAction(::windows::runtime::IInspectable);
impl InputLightDismissAction {
    #[cfg(feature = "undation")]
    pub fn Dismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::TypedEventHandler<
                InputLightDismissAction,
                InputLightDismissEventArgs,
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
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemoveDismissed<
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
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn GetForWindowId<'a, Param0: ::windows::runtime::IntoParam<'a, super::WindowId>>(
        windowid: Param0,
    ) -> ::windows::runtime::Result<InputLightDismissAction> {
        Self::IInputLightDismissActionStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                windowid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<InputLightDismissAction>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<IInputObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn IInputLightDismissActionStatics<
        R,
        F: FnOnce(&IInputLightDismissActionStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            InputLightDismissAction,
            IInputLightDismissActionStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InputLightDismissAction {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Input.InputLightDismissAction;{e8a39502-a860-502f-8c10-3646d43aecf1})",
    );
}
unsafe impl ::windows::runtime::Interface for InputLightDismissAction {
    type Vtable = IInputLightDismissAction_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3903034626,
        43104,
        20527,
        [140, 16, 54, 70, 212, 58, 236, 241],
    );
}
impl ::windows::runtime::RuntimeName for InputLightDismissAction {
    const NAME: &'static str = "Microsoft.UI.Input.InputLightDismissAction";
}
impl ::std::convert::From<InputLightDismissAction> for ::windows::runtime::IUnknown {
    fn from(value: InputLightDismissAction) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InputLightDismissAction> for ::windows::runtime::IUnknown {
    fn from(value: &InputLightDismissAction) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for InputLightDismissAction
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &InputLightDismissAction
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<InputLightDismissAction> for ::windows::runtime::IInspectable {
    fn from(value: InputLightDismissAction) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InputLightDismissAction> for ::windows::runtime::IInspectable {
    fn from(value: &InputLightDismissAction) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for InputLightDismissAction
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a InputLightDismissAction
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<InputLightDismissAction> for InputObject {
    fn from(value: InputLightDismissAction) -> Self {
        ::std::convert::Into::<InputObject>::into(&value)
    }
}
impl ::std::convert::From<&InputLightDismissAction> for InputObject {
    fn from(value: &InputLightDismissAction) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, InputObject> for InputLightDismissAction {
    fn into_param(self) -> ::windows::runtime::Param<'a, InputObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<InputObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, InputObject> for &InputLightDismissAction {
    fn into_param(self) -> ::windows::runtime::Param<'a, InputObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<InputObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for InputLightDismissAction {}
unsafe impl ::std::marker::Sync for InputLightDismissAction {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct InputLightDismissEventArgs(::windows::runtime::IInspectable);
impl InputLightDismissEventArgs {}
unsafe impl ::windows::runtime::RuntimeType for InputLightDismissEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Input.InputLightDismissEventArgs;{078660ee-07ca-5808-b982-e6e899cf098c})",
    );
}
unsafe impl ::windows::runtime::Interface for InputLightDismissEventArgs {
    type Vtable = IInputLightDismissEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        126247150,
        1994,
        22536,
        [185, 130, 230, 232, 153, 207, 9, 140],
    );
}
impl ::windows::runtime::RuntimeName for InputLightDismissEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.InputLightDismissEventArgs";
}
impl ::std::convert::From<InputLightDismissEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: InputLightDismissEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InputLightDismissEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &InputLightDismissEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for InputLightDismissEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &InputLightDismissEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<InputLightDismissEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: InputLightDismissEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InputLightDismissEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &InputLightDismissEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for InputLightDismissEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a InputLightDismissEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for InputLightDismissEventArgs {}
unsafe impl ::std::marker::Sync for InputLightDismissEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct InputObject(::windows::runtime::IInspectable);
impl InputObject {
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::Dispatching::DispatcherQueue> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Dispatching::DispatcherQueue>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InputObject {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Input.InputObject;{42edbc88-d386-544d-b1b8-68617fe68282})",
    );
}
unsafe impl ::windows::runtime::Interface for InputObject {
    type Vtable = IInputObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1122876552,
        54150,
        21581,
        [177, 184, 104, 97, 127, 230, 130, 130],
    );
}
impl ::windows::runtime::RuntimeName for InputObject {
    const NAME: &'static str = "Microsoft.UI.Input.InputObject";
}
impl ::std::convert::From<InputObject> for ::windows::runtime::IUnknown {
    fn from(value: InputObject) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InputObject> for ::windows::runtime::IUnknown {
    fn from(value: &InputObject) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InputObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InputObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<InputObject> for ::windows::runtime::IInspectable {
    fn from(value: InputObject) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InputObject> for ::windows::runtime::IInspectable {
    fn from(value: &InputObject) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InputObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InputObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for InputObject {}
unsafe impl ::std::marker::Sync for InputObject {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct InputPointerSource(::windows::runtime::IInspectable);
impl InputPointerSource {
    pub fn Cursor(&self) -> ::windows::runtime::Result<InputCursor> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InputCursor>(result__)
        }
    }
    pub fn SetCursor<'a, Param0: ::windows::runtime::IntoParam<'a, InputCursor>>(
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
    pub fn DeviceKinds(&self) -> ::windows::runtime::Result<InputPointerSourceDeviceKinds> {
        let this = self;
        unsafe {
            let mut result__: InputPointerSourceDeviceKinds = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InputPointerSourceDeviceKinds>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn PointerCaptureLost<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::TypedEventHandler<
                InputPointerSource,
                PointerEventArgs,
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
    pub fn RemovePointerCaptureLost<
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
    pub fn PointerEntered<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::TypedEventHandler<
                InputPointerSource,
                PointerEventArgs,
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
    pub fn RemovePointerEntered<
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
    pub fn PointerExited<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::TypedEventHandler<
                InputPointerSource,
                PointerEventArgs,
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
    pub fn RemovePointerExited<
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
    pub fn PointerMoved<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::TypedEventHandler<
                InputPointerSource,
                PointerEventArgs,
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
    pub fn RemovePointerMoved<
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
    pub fn PointerPressed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::TypedEventHandler<
                InputPointerSource,
                PointerEventArgs,
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
    pub fn RemovePointerPressed<
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
    #[cfg(feature = "undation")]
    pub fn PointerReleased<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::TypedEventHandler<
                InputPointerSource,
                PointerEventArgs,
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
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemovePointerReleased<
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
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn PointerRoutedAway<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::TypedEventHandler<
                InputPointerSource,
                PointerEventArgs,
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
            (::windows::runtime::Interface::vtable(this).21)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemovePointerRoutedAway<
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
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn PointerRoutedReleased<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::TypedEventHandler<
                InputPointerSource,
                PointerEventArgs,
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
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemovePointerRoutedReleased<
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
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn PointerRoutedTo<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::TypedEventHandler<
                InputPointerSource,
                PointerEventArgs,
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
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemovePointerRoutedTo<
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
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "undation")]
    pub fn PointerWheelChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Windows::Foundation::TypedEventHandler<
                InputPointerSource,
                PointerEventArgs,
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
            (::windows::runtime::Interface::vtable(this).27)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn RemovePointerWheelChanged<
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
            (::windows::runtime::Interface::vtable(this).28)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<IInputObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Dispatching::DispatcherQueue>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InputPointerSource {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Input.InputPointerSource;{6a6c2764-c3f4-5be5-8447-c9a98766c240})",
    );
}
unsafe impl ::windows::runtime::Interface for InputPointerSource {
    type Vtable = IInputPointerSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1785472868,
        50164,
        23525,
        [132, 71, 201, 169, 135, 102, 194, 64],
    );
}
impl ::windows::runtime::RuntimeName for InputPointerSource {
    const NAME: &'static str = "Microsoft.UI.Input.InputPointerSource";
}
impl ::std::convert::From<InputPointerSource> for ::windows::runtime::IUnknown {
    fn from(value: InputPointerSource) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InputPointerSource> for ::windows::runtime::IUnknown {
    fn from(value: &InputPointerSource) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InputPointerSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InputPointerSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<InputPointerSource> for ::windows::runtime::IInspectable {
    fn from(value: InputPointerSource) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InputPointerSource> for ::windows::runtime::IInspectable {
    fn from(value: &InputPointerSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for InputPointerSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a InputPointerSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<InputPointerSource> for InputObject {
    fn from(value: InputPointerSource) -> Self {
        ::std::convert::Into::<InputObject>::into(&value)
    }
}
impl ::std::convert::From<&InputPointerSource> for InputObject {
    fn from(value: &InputPointerSource) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, InputObject> for InputPointerSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, InputObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<InputObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, InputObject> for &InputPointerSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, InputObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<InputObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for InputPointerSource {}
unsafe impl ::std::marker::Sync for InputPointerSource {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct InputPointerSourceDeviceKinds(pub u32);
impl InputPointerSourceDeviceKinds {
    pub const None: InputPointerSourceDeviceKinds = InputPointerSourceDeviceKinds(0u32);
    pub const Touch: InputPointerSourceDeviceKinds = InputPointerSourceDeviceKinds(1u32);
    pub const Pen: InputPointerSourceDeviceKinds = InputPointerSourceDeviceKinds(2u32);
    pub const Mouse: InputPointerSourceDeviceKinds = InputPointerSourceDeviceKinds(4u32);
}
impl ::std::convert::From<u32> for InputPointerSourceDeviceKinds {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InputPointerSourceDeviceKinds {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InputPointerSourceDeviceKinds {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Input.InputPointerSourceDeviceKinds;u4)",
    );
}
impl ::std::ops::BitOr for InputPointerSourceDeviceKinds {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for InputPointerSourceDeviceKinds {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for InputPointerSourceDeviceKinds {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for InputPointerSourceDeviceKinds {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for InputPointerSourceDeviceKinds {
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
pub struct InputSystemCursor(::windows::runtime::IInspectable);
impl InputSystemCursor {
    pub fn CursorShape(&self) -> ::windows::runtime::Result<InputSystemCursorShape> {
        let this = self;
        unsafe {
            let mut result__: InputSystemCursorShape = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InputSystemCursorShape>(result__)
        }
    }
    pub fn Create(r#type: InputSystemCursorShape) -> ::windows::runtime::Result<InputSystemCursor> {
        Self::IInputSystemCursorStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                r#type,
                &mut result__,
            )
            .from_abi::<InputSystemCursor>(result__)
        })
    }
    #[cfg(feature = "undation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::super::Windows::Foundation::IClosable,
        >(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn IInputSystemCursorStatics<
        R,
        F: FnOnce(&IInputSystemCursorStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            InputSystemCursor,
            IInputSystemCursorStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InputSystemCursor {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Input.InputSystemCursor;{59f538e7-c500-59ab-8b54-0bc6100fd49e})",
    );
}
unsafe impl ::windows::runtime::Interface for InputSystemCursor {
    type Vtable = IInputSystemCursor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1509243111,
        50432,
        22955,
        [139, 84, 11, 198, 16, 15, 212, 158],
    );
}
impl ::windows::runtime::RuntimeName for InputSystemCursor {
    const NAME: &'static str = "Microsoft.UI.Input.InputSystemCursor";
}
impl ::std::convert::From<InputSystemCursor> for ::windows::runtime::IUnknown {
    fn from(value: InputSystemCursor) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InputSystemCursor> for ::windows::runtime::IUnknown {
    fn from(value: &InputSystemCursor) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InputSystemCursor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InputSystemCursor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<InputSystemCursor> for ::windows::runtime::IInspectable {
    fn from(value: InputSystemCursor) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InputSystemCursor> for ::windows::runtime::IInspectable {
    fn from(value: &InputSystemCursor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InputSystemCursor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a InputSystemCursor
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "undation")]
impl ::std::convert::TryFrom<InputSystemCursor>
    for super::super::super::Windows::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: InputSystemCursor) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "undation")]
impl ::std::convert::TryFrom<&InputSystemCursor>
    for super::super::super::Windows::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InputSystemCursor) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "undation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Windows::Foundation::IClosable>
    for InputSystemCursor
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::super::Windows::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "undation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Windows::Foundation::IClosable>
    for &InputSystemCursor
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::super::Windows::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::super::Windows::Foundation::IClosable>::try_into(
            self,
        )
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<InputSystemCursor> for InputCursor {
    fn from(value: InputSystemCursor) -> Self {
        ::std::convert::Into::<InputCursor>::into(&value)
    }
}
impl ::std::convert::From<&InputSystemCursor> for InputCursor {
    fn from(value: &InputSystemCursor) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, InputCursor> for InputSystemCursor {
    fn into_param(self) -> ::windows::runtime::Param<'a, InputCursor> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<InputCursor>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, InputCursor> for &InputSystemCursor {
    fn into_param(self) -> ::windows::runtime::Param<'a, InputCursor> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<InputCursor>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for InputSystemCursor {}
unsafe impl ::std::marker::Sync for InputSystemCursor {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct InputSystemCursorShape(pub i32);
impl InputSystemCursorShape {
    pub const Arrow: InputSystemCursorShape = InputSystemCursorShape(0i32);
    pub const Cross: InputSystemCursorShape = InputSystemCursorShape(1i32);
    pub const Hand: InputSystemCursorShape = InputSystemCursorShape(3i32);
    pub const Help: InputSystemCursorShape = InputSystemCursorShape(4i32);
    pub const IBeam: InputSystemCursorShape = InputSystemCursorShape(5i32);
    pub const SizeAll: InputSystemCursorShape = InputSystemCursorShape(6i32);
    pub const SizeNortheastSouthwest: InputSystemCursorShape = InputSystemCursorShape(7i32);
    pub const SizeNorthSouth: InputSystemCursorShape = InputSystemCursorShape(8i32);
    pub const SizeNorthwestSoutheast: InputSystemCursorShape = InputSystemCursorShape(9i32);
    pub const SizeWestEast: InputSystemCursorShape = InputSystemCursorShape(10i32);
    pub const UniversalNo: InputSystemCursorShape = InputSystemCursorShape(11i32);
    pub const UpArrow: InputSystemCursorShape = InputSystemCursorShape(12i32);
    pub const Wait: InputSystemCursorShape = InputSystemCursorShape(13i32);
    pub const Pin: InputSystemCursorShape = InputSystemCursorShape(14i32);
    pub const Person: InputSystemCursorShape = InputSystemCursorShape(15i32);
    pub const AppStarting: InputSystemCursorShape = InputSystemCursorShape(16i32);
}
impl ::std::convert::From<i32> for InputSystemCursorShape {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InputSystemCursorShape {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InputSystemCursorShape {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Input.InputSystemCursorShape;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ManipulationCompletedEventArgs(::windows::runtime::IInspectable);
impl ManipulationCompletedEventArgs {
    #[cfg(feature = "undation")]
    pub fn Cumulative(&self) -> ::windows::runtime::Result<ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: ManipulationDelta = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ManipulationDelta>(result__)
        }
    }
    pub fn PointerDeviceType(&self) -> ::windows::runtime::Result<PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: PointerDeviceType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PointerDeviceType>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn Position(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Windows::Foundation::Point =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn Velocities(&self) -> ::windows::runtime::Result<ManipulationVelocities> {
        let this = self;
        unsafe {
            let mut result__: ManipulationVelocities = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ManipulationVelocities>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ManipulationCompletedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Input.ManipulationCompletedEventArgs;{0e0249d4-46e4-5559-aee3-fa45ce2a7f56})" ) ;
}
unsafe impl ::windows::runtime::Interface for ManipulationCompletedEventArgs {
    type Vtable = IManipulationCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        235030996,
        18148,
        21849,
        [174, 227, 250, 69, 206, 42, 127, 86],
    );
}
impl ::windows::runtime::RuntimeName for ManipulationCompletedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.ManipulationCompletedEventArgs";
}
impl ::std::convert::From<ManipulationCompletedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ManipulationCompletedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ManipulationCompletedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ManipulationCompletedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ManipulationCompletedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ManipulationCompletedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ManipulationCompletedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ManipulationCompletedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ManipulationCompletedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ManipulationCompletedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ManipulationCompletedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ManipulationCompletedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ManipulationCompletedEventArgs {}
unsafe impl ::std::marker::Sync for ManipulationCompletedEventArgs {}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "undation")]
pub struct ManipulationDelta {
    pub Translation: super::super::super::Windows::Foundation::Point,
    pub Scale: f32,
    pub Rotation: f32,
    pub Expansion: f32,
}
#[cfg(feature = "undation")]
impl ManipulationDelta {}
#[cfg(feature = "undation")]
impl ::std::default::Default for ManipulationDelta {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "undation")]
impl ::std::fmt::Debug for ManipulationDelta {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ManipulationDelta")
            .field("Translation", &self.Translation)
            .field("Scale", &self.Scale)
            .field("Rotation", &self.Rotation)
            .field("Expansion", &self.Expansion)
            .finish()
    }
}
#[cfg(feature = "undation")]
impl ::std::cmp::PartialEq for ManipulationDelta {
    fn eq(&self, other: &Self) -> bool {
        self.Translation == other.Translation
            && self.Scale == other.Scale
            && self.Rotation == other.Rotation
            && self.Expansion == other.Expansion
    }
}
#[cfg(feature = "undation")]
impl ::std::cmp::Eq for ManipulationDelta {}
#[cfg(feature = "undation")]
unsafe impl ::windows::runtime::Abi for ManipulationDelta {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "undation")]
unsafe impl ::windows::runtime::RuntimeType for ManipulationDelta {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"struct(Microsoft.UI.Input.ManipulationDelta;struct(Windows.Foundation.Point;f4;f4);f4;f4;f4)" ) ;
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ManipulationInertiaStartingEventArgs(::windows::runtime::IInspectable);
impl ManipulationInertiaStartingEventArgs {
    #[cfg(feature = "undation")]
    pub fn Cumulative(&self) -> ::windows::runtime::Result<ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: ManipulationDelta = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ManipulationDelta>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn Delta(&self) -> ::windows::runtime::Result<ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: ManipulationDelta = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ManipulationDelta>(result__)
        }
    }
    pub fn PointerDeviceType(&self) -> ::windows::runtime::Result<PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: PointerDeviceType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PointerDeviceType>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn Position(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Windows::Foundation::Point =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn Velocities(&self) -> ::windows::runtime::Result<ManipulationVelocities> {
        let this = self;
        unsafe {
            let mut result__: ManipulationVelocities = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ManipulationVelocities>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ManipulationInertiaStartingEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Input.ManipulationInertiaStartingEventArgs;{acf9ef71-6e15-56ab-9260-f0d3ce5f66e8})" ) ;
}
unsafe impl ::windows::runtime::Interface for ManipulationInertiaStartingEventArgs {
    type Vtable = IManipulationInertiaStartingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2902060913,
        28181,
        22187,
        [146, 96, 240, 211, 206, 95, 102, 232],
    );
}
impl ::windows::runtime::RuntimeName for ManipulationInertiaStartingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.ManipulationInertiaStartingEventArgs";
}
impl ::std::convert::From<ManipulationInertiaStartingEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ManipulationInertiaStartingEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ManipulationInertiaStartingEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ManipulationInertiaStartingEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ManipulationInertiaStartingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ManipulationInertiaStartingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ManipulationInertiaStartingEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: ManipulationInertiaStartingEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ManipulationInertiaStartingEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &ManipulationInertiaStartingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ManipulationInertiaStartingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ManipulationInertiaStartingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ManipulationInertiaStartingEventArgs {}
unsafe impl ::std::marker::Sync for ManipulationInertiaStartingEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ManipulationStartedEventArgs(::windows::runtime::IInspectable);
impl ManipulationStartedEventArgs {
    #[cfg(feature = "undation")]
    pub fn Cumulative(&self) -> ::windows::runtime::Result<ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: ManipulationDelta = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ManipulationDelta>(result__)
        }
    }
    pub fn PointerDeviceType(&self) -> ::windows::runtime::Result<PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: PointerDeviceType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PointerDeviceType>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn Position(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Windows::Foundation::Point =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::Point>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ManipulationStartedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Input.ManipulationStartedEventArgs;{4a616613-eef1-5f1b-a768-0775478d49d4})" ) ;
}
unsafe impl ::windows::runtime::Interface for ManipulationStartedEventArgs {
    type Vtable = IManipulationStartedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1247897107,
        61169,
        24347,
        [167, 104, 7, 117, 71, 141, 73, 212],
    );
}
impl ::windows::runtime::RuntimeName for ManipulationStartedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.ManipulationStartedEventArgs";
}
impl ::std::convert::From<ManipulationStartedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ManipulationStartedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ManipulationStartedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ManipulationStartedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ManipulationStartedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ManipulationStartedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ManipulationStartedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ManipulationStartedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ManipulationStartedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ManipulationStartedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ManipulationStartedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ManipulationStartedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ManipulationStartedEventArgs {}
unsafe impl ::std::marker::Sync for ManipulationStartedEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ManipulationUpdatedEventArgs(::windows::runtime::IInspectable);
impl ManipulationUpdatedEventArgs {
    #[cfg(feature = "undation")]
    pub fn Cumulative(&self) -> ::windows::runtime::Result<ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: ManipulationDelta = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ManipulationDelta>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn Delta(&self) -> ::windows::runtime::Result<ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: ManipulationDelta = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ManipulationDelta>(result__)
        }
    }
    pub fn PointerDeviceType(&self) -> ::windows::runtime::Result<PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: PointerDeviceType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PointerDeviceType>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn Position(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Windows::Foundation::Point =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn Velocities(&self) -> ::windows::runtime::Result<ManipulationVelocities> {
        let this = self;
        unsafe {
            let mut result__: ManipulationVelocities = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ManipulationVelocities>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ManipulationUpdatedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Input.ManipulationUpdatedEventArgs;{406e1961-0c98-5fc0-b3d8-116492ef0053})" ) ;
}
unsafe impl ::windows::runtime::Interface for ManipulationUpdatedEventArgs {
    type Vtable = IManipulationUpdatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1080957281,
        3224,
        24512,
        [179, 216, 17, 100, 146, 239, 0, 83],
    );
}
impl ::windows::runtime::RuntimeName for ManipulationUpdatedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.ManipulationUpdatedEventArgs";
}
impl ::std::convert::From<ManipulationUpdatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ManipulationUpdatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ManipulationUpdatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ManipulationUpdatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ManipulationUpdatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ManipulationUpdatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ManipulationUpdatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ManipulationUpdatedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ManipulationUpdatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ManipulationUpdatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ManipulationUpdatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ManipulationUpdatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ManipulationUpdatedEventArgs {}
unsafe impl ::std::marker::Sync for ManipulationUpdatedEventArgs {}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "undation")]
pub struct ManipulationVelocities {
    pub Linear: super::super::super::Windows::Foundation::Point,
    pub Angular: f32,
    pub Expansion: f32,
}
#[cfg(feature = "undation")]
impl ManipulationVelocities {}
#[cfg(feature = "undation")]
impl ::std::default::Default for ManipulationVelocities {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "undation")]
impl ::std::fmt::Debug for ManipulationVelocities {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ManipulationVelocities")
            .field("Linear", &self.Linear)
            .field("Angular", &self.Angular)
            .field("Expansion", &self.Expansion)
            .finish()
    }
}
#[cfg(feature = "undation")]
impl ::std::cmp::PartialEq for ManipulationVelocities {
    fn eq(&self, other: &Self) -> bool {
        self.Linear == other.Linear
            && self.Angular == other.Angular
            && self.Expansion == other.Expansion
    }
}
#[cfg(feature = "undation")]
impl ::std::cmp::Eq for ManipulationVelocities {}
#[cfg(feature = "undation")]
unsafe impl ::windows::runtime::Abi for ManipulationVelocities {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "undation")]
unsafe impl ::windows::runtime::RuntimeType for ManipulationVelocities {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"struct(Microsoft.UI.Input.ManipulationVelocities;struct(Windows.Foundation.Point;f4;f4);f4;f4)" ) ;
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct MouseWheelParameters(::windows::runtime::IInspectable);
impl MouseWheelParameters {
    #[cfg(feature = "undation")]
    pub fn CharTranslation(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Windows::Foundation::Point =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn SetCharTranslation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::Windows::Foundation::Point>,
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
    pub fn DeltaScale(&self) -> ::windows::runtime::Result<f32> {
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
    pub fn SetDeltaScale(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn DeltaRotationAngle(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn SetDeltaRotationAngle(&self, value: f32) -> ::windows::runtime::Result<()> {
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
    pub fn PageTranslation(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Windows::Foundation::Point =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn SetPageTranslation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::Windows::Foundation::Point>,
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
}
unsafe impl ::windows::runtime::RuntimeType for MouseWheelParameters {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Input.MouseWheelParameters;{6d98be40-1d56-51d1-aa0d-f325439cd009})",
    );
}
unsafe impl ::windows::runtime::Interface for MouseWheelParameters {
    type Vtable = IMouseWheelParameters_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1838726720,
        7510,
        20945,
        [170, 13, 243, 37, 67, 156, 208, 9],
    );
}
impl ::windows::runtime::RuntimeName for MouseWheelParameters {
    const NAME: &'static str = "Microsoft.UI.Input.MouseWheelParameters";
}
impl ::std::convert::From<MouseWheelParameters> for ::windows::runtime::IUnknown {
    fn from(value: MouseWheelParameters) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MouseWheelParameters> for ::windows::runtime::IUnknown {
    fn from(value: &MouseWheelParameters) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MouseWheelParameters {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MouseWheelParameters {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<MouseWheelParameters> for ::windows::runtime::IInspectable {
    fn from(value: MouseWheelParameters) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MouseWheelParameters> for ::windows::runtime::IInspectable {
    fn from(value: &MouseWheelParameters) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for MouseWheelParameters
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a MouseWheelParameters
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MouseWheelParameters {}
unsafe impl ::std::marker::Sync for MouseWheelParameters {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct PointerDeviceType(pub i32);
impl PointerDeviceType {
    pub const Touch: PointerDeviceType = PointerDeviceType(0i32);
    pub const Pen: PointerDeviceType = PointerDeviceType(1i32);
    pub const Mouse: PointerDeviceType = PointerDeviceType(2i32);
    pub const Touchpad: PointerDeviceType = PointerDeviceType(3i32);
}
impl ::std::convert::From<i32> for PointerDeviceType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PointerDeviceType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PointerDeviceType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Input.PointerDeviceType;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct PointerEventArgs(::windows::runtime::IInspectable);
impl PointerEventArgs {
    pub fn CurrentPoint(&self) -> ::windows::runtime::Result<PointerPoint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PointerPoint>(result__)
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
    #[cfg(feature = "stem")]
    pub fn KeyModifiers(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Windows::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Windows::System::VirtualKeyModifiers =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::System::VirtualKeyModifiers>(result__)
        }
    }
    #[cfg(feature = "undation_Collections")]
    pub fn GetIntermediatePoints(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::Windows::Foundation::Collections::IVector<PointerPoint>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::Collections::IVector<
                PointerPoint,
            >>(result__)
        }
    }
    #[cfg(feature = "undation_Collections")]
    pub fn GetIntermediateTransformedPoints<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IPointerPointTransform>,
    >(
        &self,
        transform: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::Windows::Foundation::Collections::IVector<PointerPoint>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                transform.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::Collections::IVector<
                PointerPoint,
            >>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PointerEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Input.PointerEventArgs;{865b188c-2ed5-5df8-829f-ac0701d5c51a})",
    );
}
unsafe impl ::windows::runtime::Interface for PointerEventArgs {
    type Vtable = IPointerEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2254117004,
        11989,
        24056,
        [130, 159, 172, 7, 1, 213, 197, 26],
    );
}
impl ::windows::runtime::RuntimeName for PointerEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.PointerEventArgs";
}
impl ::std::convert::From<PointerEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: PointerEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PointerEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &PointerEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PointerEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PointerEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<PointerEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: PointerEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PointerEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &PointerEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PointerEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a PointerEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PointerEventArgs {}
unsafe impl ::std::marker::Sync for PointerEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct PointerPoint(::windows::runtime::IInspectable);
impl PointerPoint {
    pub fn FrameId(&self) -> ::windows::runtime::Result<u32> {
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
    pub fn IsInContact(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn PointerDeviceType(&self) -> ::windows::runtime::Result<PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: PointerDeviceType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PointerDeviceType>(result__)
        }
    }
    pub fn PointerId(&self) -> ::windows::runtime::Result<u32> {
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
    #[cfg(feature = "undation")]
    pub fn Position(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Windows::Foundation::Point =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::Point>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::runtime::Result<PointerPointProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PointerPointProperties>(result__)
        }
    }
    pub fn Timestamp(&self) -> ::windows::runtime::Result<u64> {
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
    pub fn GetTransformedPoint<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IPointerPointTransform>,
    >(
        &self,
        transform: Param0,
    ) -> ::windows::runtime::Result<PointerPoint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                transform.into_param().abi(),
                &mut result__,
            )
            .from_abi::<PointerPoint>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PointerPoint {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Input.PointerPoint;{0d430ee6-252c-59a4-b2a2-d44264dc6a40})",
    );
}
unsafe impl ::windows::runtime::Interface for PointerPoint {
    type Vtable = IPointerPoint_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        222498534,
        9516,
        22948,
        [178, 162, 212, 66, 100, 220, 106, 64],
    );
}
impl ::windows::runtime::RuntimeName for PointerPoint {
    const NAME: &'static str = "Microsoft.UI.Input.PointerPoint";
}
impl ::std::convert::From<PointerPoint> for ::windows::runtime::IUnknown {
    fn from(value: PointerPoint) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PointerPoint> for ::windows::runtime::IUnknown {
    fn from(value: &PointerPoint) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PointerPoint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PointerPoint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<PointerPoint> for ::windows::runtime::IInspectable {
    fn from(value: PointerPoint) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PointerPoint> for ::windows::runtime::IInspectable {
    fn from(value: &PointerPoint) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PointerPoint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PointerPoint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PointerPoint {}
unsafe impl ::std::marker::Sync for PointerPoint {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct PointerPointProperties(::windows::runtime::IInspectable);
impl PointerPointProperties {
    #[cfg(feature = "undation")]
    pub fn ContactRect(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Windows::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::Rect>(result__)
        }
    }
    pub fn IsBarrelButtonPressed(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn IsCanceled(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn IsEraser(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn IsHorizontalMouseWheel(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn IsInRange(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn IsInverted(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn IsLeftButtonPressed(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn IsMiddleButtonPressed(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn IsPrimary(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn IsRightButtonPressed(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn IsXButton1Pressed(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn IsXButton2Pressed(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn MouseWheelDelta(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn Orientation(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn PointerUpdateKind(&self) -> ::windows::runtime::Result<PointerUpdateKind> {
        let this = self;
        unsafe {
            let mut result__: PointerUpdateKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PointerUpdateKind>(result__)
        }
    }
    pub fn Pressure(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn TouchConfidence(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn Twist(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn XTilt(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn YTilt(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PointerPointProperties {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Input.PointerPointProperties;{d760ed77-4b10-57a5-b3cc-d9bf3413e996})",
    );
}
unsafe impl ::windows::runtime::Interface for PointerPointProperties {
    type Vtable = IPointerPointProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3613453687,
        19216,
        22437,
        [179, 204, 217, 191, 52, 19, 233, 150],
    );
}
impl ::windows::runtime::RuntimeName for PointerPointProperties {
    const NAME: &'static str = "Microsoft.UI.Input.PointerPointProperties";
}
impl ::std::convert::From<PointerPointProperties> for ::windows::runtime::IUnknown {
    fn from(value: PointerPointProperties) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PointerPointProperties> for ::windows::runtime::IUnknown {
    fn from(value: &PointerPointProperties) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for PointerPointProperties
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &PointerPointProperties
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<PointerPointProperties> for ::windows::runtime::IInspectable {
    fn from(value: PointerPointProperties) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PointerPointProperties> for ::windows::runtime::IInspectable {
    fn from(value: &PointerPointProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for PointerPointProperties
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a PointerPointProperties
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PointerPointProperties {}
unsafe impl ::std::marker::Sync for PointerPointProperties {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct PointerPredictor(::windows::runtime::IInspectable);
impl PointerPredictor {
    #[cfg(feature = "undation")]
    pub fn PredictionTime(
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
    pub fn SetPredictionTime<
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
    pub fn GetPredictedPoints<'a, Param0: ::windows::runtime::IntoParam<'a, PointerPoint>>(
        &self,
        point: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::Array<PointerPoint>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<PointerPoint> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                point.into_param().abi(),
                ::windows::runtime::Array::<PointerPoint>::set_abi_len(&mut result__),
                &mut result__ as *mut _ as _,
            )
            .and_then(|| result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::super::Windows::Foundation::IClosable,
        >(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn CreateForInputPointerSource<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, InputPointerSource>,
    >(
        inputpointersource: Param0,
    ) -> ::windows::runtime::Result<PointerPredictor> {
        Self::IPointerPredictorStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                inputpointersource.into_param().abi(),
                &mut result__,
            )
            .from_abi::<PointerPredictor>(result__)
        })
    }
    pub fn IPointerPredictorStatics<
        R,
        F: FnOnce(&IPointerPredictorStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            PointerPredictor,
            IPointerPredictorStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PointerPredictor {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Input.PointerPredictor;{12c100ec-2100-565f-a60c-f1187f438828})",
    );
}
unsafe impl ::windows::runtime::Interface for PointerPredictor {
    type Vtable = IPointerPredictor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        314638572,
        8448,
        22111,
        [166, 12, 241, 24, 127, 67, 136, 40],
    );
}
impl ::windows::runtime::RuntimeName for PointerPredictor {
    const NAME: &'static str = "Microsoft.UI.Input.PointerPredictor";
}
impl ::std::convert::From<PointerPredictor> for ::windows::runtime::IUnknown {
    fn from(value: PointerPredictor) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PointerPredictor> for ::windows::runtime::IUnknown {
    fn from(value: &PointerPredictor) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PointerPredictor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PointerPredictor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<PointerPredictor> for ::windows::runtime::IInspectable {
    fn from(value: PointerPredictor) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PointerPredictor> for ::windows::runtime::IInspectable {
    fn from(value: &PointerPredictor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PointerPredictor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a PointerPredictor
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "undation")]
impl ::std::convert::TryFrom<PointerPredictor>
    for super::super::super::Windows::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: PointerPredictor) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "undation")]
impl ::std::convert::TryFrom<&PointerPredictor>
    for super::super::super::Windows::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PointerPredictor) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "undation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Windows::Foundation::IClosable>
    for PointerPredictor
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::super::Windows::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "undation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Windows::Foundation::IClosable>
    for &PointerPredictor
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::super::Windows::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::super::Windows::Foundation::IClosable>::try_into(
            self,
        )
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for PointerPredictor {}
unsafe impl ::std::marker::Sync for PointerPredictor {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct PointerUpdateKind(pub i32);
impl PointerUpdateKind {
    pub const Other: PointerUpdateKind = PointerUpdateKind(0i32);
    pub const LeftButtonPressed: PointerUpdateKind = PointerUpdateKind(1i32);
    pub const LeftButtonReleased: PointerUpdateKind = PointerUpdateKind(2i32);
    pub const RightButtonPressed: PointerUpdateKind = PointerUpdateKind(3i32);
    pub const RightButtonReleased: PointerUpdateKind = PointerUpdateKind(4i32);
    pub const MiddleButtonPressed: PointerUpdateKind = PointerUpdateKind(5i32);
    pub const MiddleButtonReleased: PointerUpdateKind = PointerUpdateKind(6i32);
    pub const XButton1Pressed: PointerUpdateKind = PointerUpdateKind(7i32);
    pub const XButton1Released: PointerUpdateKind = PointerUpdateKind(8i32);
    pub const XButton2Pressed: PointerUpdateKind = PointerUpdateKind(9i32);
    pub const XButton2Released: PointerUpdateKind = PointerUpdateKind(10i32);
}
impl ::std::convert::From<i32> for PointerUpdateKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PointerUpdateKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PointerUpdateKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Input.PointerUpdateKind;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RightTappedEventArgs(::windows::runtime::IInspectable);
impl RightTappedEventArgs {
    pub fn PointerDeviceType(&self) -> ::windows::runtime::Result<PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: PointerDeviceType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PointerDeviceType>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn Position(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Windows::Foundation::Point =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::Point>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RightTappedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Input.RightTappedEventArgs;{8ff73b39-887e-50a4-8500-77953039dcb4})",
    );
}
unsafe impl ::windows::runtime::Interface for RightTappedEventArgs {
    type Vtable = IRightTappedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2415344441,
        34942,
        20644,
        [133, 0, 119, 149, 48, 57, 220, 180],
    );
}
impl ::windows::runtime::RuntimeName for RightTappedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.RightTappedEventArgs";
}
impl ::std::convert::From<RightTappedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: RightTappedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RightTappedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &RightTappedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RightTappedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &RightTappedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RightTappedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: RightTappedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RightTappedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &RightTappedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RightTappedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RightTappedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RightTappedEventArgs {}
unsafe impl ::std::marker::Sync for RightTappedEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct TappedEventArgs(::windows::runtime::IInspectable);
impl TappedEventArgs {
    pub fn PointerDeviceType(&self) -> ::windows::runtime::Result<PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: PointerDeviceType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PointerDeviceType>(result__)
        }
    }
    #[cfg(feature = "undation")]
    pub fn Position(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Windows::Foundation::Point =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Windows::Foundation::Point>(result__)
        }
    }
    pub fn TapCount(&self) -> ::windows::runtime::Result<u32> {
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
}
unsafe impl ::windows::runtime::RuntimeType for TappedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Input.TappedEventArgs;{c3a01bb5-6076-5e0f-871a-9d94a6a8f82b})",
    );
}
unsafe impl ::windows::runtime::Interface for TappedEventArgs {
    type Vtable = ITappedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3282049973,
        24694,
        24079,
        [135, 26, 157, 148, 166, 168, 248, 43],
    );
}
impl ::windows::runtime::RuntimeName for TappedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.TappedEventArgs";
}
impl ::std::convert::From<TappedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: TappedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&TappedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &TappedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TappedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &TappedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<TappedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: TappedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TappedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &TappedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for TappedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a TappedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for TappedEventArgs {}
unsafe impl ::std::marker::Sync for TappedEventArgs {}
