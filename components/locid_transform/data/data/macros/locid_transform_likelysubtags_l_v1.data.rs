// @generated
/// Implement `DataProvider<LikelySubtagsForLanguageV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_locid_transform_likelysubtags_l_v1 {
    ($ provider : ty) => {
        #[clippy::msrv = "1.67"]
        const _: () = <$provider>::MUST_USE_MAKE_PROVIDER_MACRO;
        #[clippy::msrv = "1.67"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_LOCID_TRANSFORM_LIKELYSUBTAGS_L_V1: &'static <icu::locid_transform::provider::LikelySubtagsForLanguageV1Marker as icu_provider::DataMarker>::Yokeable = &icu::locid_transform::provider::LikelySubtagsForLanguageV1 {
                language_script: unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"az\0Araben\0Shawff\0Adlmhi\0Latnkk\0Arabky\0Arabky\0Latnmn\0Mongpa\0Arabsd\0Devasd\0Khojsd\0Sindtg\0Arabuz\0ArabyueHanszh\0Bopozh\0Hanbzh\0Hant") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"IR\0GB\0GN\0IN\0CN\0CN\0TR\0CN\0PK\0IN\0IN\0IN\0PK\0AF\0CN\0TW\0TW\0TW\0") })
                },
                language_region: unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"az\0IQ\0az\0IR\0az\0RU\0ha\0CM\0ha\0SD\0kk\0AF\0kk\0CN\0kk\0IR\0kk\0MN\0ky\0CN\0ky\0TR\0mn\0CN\0ms\0CC\0pa\0PK\0sd\0IN\0sr\0ME\0sr\0RO\0sr\0RU\0sr\0TR\0tg\0PK\0uz\0AF\0uz\0CN\0yueCN\0zh\0AU\0zh\0BN\0zh\0GB\0zh\0GF\0zh\0HK\0zh\0ID\0zh\0MO\0zh\0PA\0zh\0PF\0zh\0PH\0zh\0SR\0zh\0TH\0zh\0TW\0zh\0US\0zh\0VN\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ArabArabCyrlArabArabArabArabArabArabArabLatnMongArabArabDevaLatnLatnLatnLatnArabArabCyrlHansHantHantHantHantHantHantHantHantHantHantHantHantHantHantHant") })
                },
                language: unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"af\0am\0ar\0as\0astaz\0be\0bg\0bgcbhobn\0br\0brxbs\0ca\0cebchrcs\0cv\0cy\0da\0de\0doidsbel\0en\0es\0et\0eu\0fa\0ff\0fi\0filfo\0fr\0ga\0gd\0gl\0gu\0ha\0he\0hi\0hr\0hsbhu\0hy\0ia\0id\0ig\0is\0it\0ja\0jv\0ka\0keakgpkk\0km\0kn\0ko\0kokks\0ky\0lo\0lt\0lv\0maimi\0mk\0ml\0mn\0mnimr\0ms\0my\0ne\0nl\0nn\0no\0or\0pa\0pcmpl\0ps\0pt\0qu\0rajrm\0ro\0ru\0sa\0satsc\0sd\0si\0sk\0sl\0so\0sq\0sr\0su\0sv\0sw\0ta\0te\0tg\0th\0ti\0tk\0to\0tr\0tt\0uk\0ur\0uz\0vi\0wo\0xh\0yo\0yrlyuezh\0zu\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"LatnZA\0EthiET\0ArabEG\0BengIN\0LatnES\0LatnAZ\0CyrlBY\0CyrlBG\0DevaIN\0DevaIN\0BengBD\0LatnFR\0DevaIN\0LatnBA\0LatnES\0LatnPH\0CherUS\0LatnCZ\0CyrlRU\0LatnGB\0LatnDK\0LatnDE\0DevaIN\0LatnDE\0GrekGR\0LatnUS\0LatnES\0LatnEE\0LatnES\0ArabIR\0LatnSN\0LatnFI\0LatnPH\0LatnFO\0LatnFR\0LatnIE\0LatnGB\0LatnES\0GujrIN\0LatnNG\0HebrIL\0DevaIN\0LatnHR\0LatnDE\0LatnHU\0ArmnAM\0Latn001LatnID\0LatnNG\0LatnIS\0LatnIT\0JpanJP\0LatnID\0GeorGE\0LatnCV\0LatnBR\0CyrlKZ\0KhmrKH\0KndaIN\0KoreKR\0DevaIN\0ArabIN\0CyrlKG\0LaooLA\0LatnLT\0LatnLV\0DevaIN\0LatnNZ\0CyrlMK\0MlymIN\0CyrlMN\0BengIN\0DevaIN\0LatnMY\0MymrMM\0DevaNP\0LatnNL\0LatnNO\0LatnNO\0OryaIN\0GuruIN\0LatnNG\0LatnPL\0ArabAF\0LatnBR\0LatnPE\0DevaIN\0LatnCH\0LatnRO\0CyrlRU\0DevaIN\0OlckIN\0LatnIT\0ArabPK\0SinhLK\0LatnSK\0LatnSI\0LatnSO\0LatnAL\0CyrlRS\0LatnID\0LatnSE\0LatnTZ\0TamlIN\0TeluIN\0CyrlTJ\0ThaiTH\0EthiET\0LatnTM\0LatnTO\0LatnTR\0CyrlRU\0CyrlUA\0ArabPK\0LatnUZ\0LatnVN\0LatnSN\0LatnZA\0LatnNG\0LatnBR\0HantHK\0HansCN\0LatnZA\0") })
                },
                und: (icu::locid::subtags::language!("en"), icu::locid::subtags::script!("Latn"), icu::locid::subtags::region!("US")),
            };
        }
        #[clippy::msrv = "1.67"]
        impl icu_provider::DataProvider<icu::locid_transform::provider::LikelySubtagsForLanguageV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu::locid_transform::provider::LikelySubtagsForLanguageV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_LOCID_TRANSFORM_LIKELYSUBTAGS_L_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu::locid_transform::provider::LikelySubtagsForLanguageV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
