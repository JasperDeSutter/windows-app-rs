#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[cfg(all(feature = "n32_Foundation", feature = "n32_Storage_Packaging_Appx"))]
#[inline]
pub unsafe fn MddBootstrapInitialize<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::super::Windows::Win32::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<
        'a,
        super::super::super::Windows::Win32::Storage::Packaging::Appx::PACKAGE_VERSION,
    >,
>(
    majorminorversion: u32,
    versiontag: Param1,
    minversion: Param2,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "microsoft.windowsappruntime.bootstrap")]
        extern "system" {
            fn MddBootstrapInitialize(
                majorminorversion: u32,
                versiontag: super::super::super::Windows::Win32::Foundation::PWSTR,
                minversion : super::super::super::Windows::Win32::Storage::Packaging::Appx:: PACKAGE_VERSION,
            ) -> ::windows::runtime::HRESULT;
        }
        MddBootstrapInitialize(
            ::std::mem::transmute(majorminorversion),
            versiontag.into_param().abi(),
            minversion.into_param().abi(),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MddBootstrapShutdown() {
    #[cfg(windows)]
    {
        #[link(name = "microsoft.windowsappruntime.bootstrap")]
        extern "system" {
            fn MddBootstrapShutdown();
        }
        ::std::mem::transmute(MddBootstrapShutdown())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
