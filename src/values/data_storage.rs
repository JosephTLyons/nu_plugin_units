use super::{conversion, BaseConversionFunction, BaseConversionFunctionsMap, Values};
use std::{collections::HashMap, convert::identity};
use unit_conversions::data_storage::*;

pub struct DataStorage;

impl Values for DataStorage {
    fn name() -> &'static str {
        "data-storage"
    }
    fn base_conversion_functions() -> BaseConversionFunctionsMap {
        HashMap::from_iter([
            conversion(
                "bits",
                identity as BaseConversionFunction,
                identity as BaseConversionFunction,
            ),
            conversion(
                "gigabits",
                gigabits::to_bits as BaseConversionFunction,
                bits::to_gigabits as BaseConversionFunction,
            ),
            conversion(
                "gigabytes",
                gigabytes::to_bits as BaseConversionFunction,
                bits::to_gigabytes as BaseConversionFunction,
            ),
            conversion(
                "gigabytes",
                gigabytes::to_bits as BaseConversionFunction,
                bits::to_gigabytes as BaseConversionFunction,
            ),
            conversion(
                "kibibits",
                kibibits::to_bits as BaseConversionFunction,
                bits::to_kibibits as BaseConversionFunction,
            ),
            conversion(
                "kilobits",
                kilobits::to_bits as BaseConversionFunction,
                bits::to_kilobits as BaseConversionFunction,
            ),
            conversion(
                "mebibits",
                mebibits::to_bits as BaseConversionFunction,
                bits::to_mebibits as BaseConversionFunction,
            ),
            conversion(
                "megabits",
                megabits::to_bits as BaseConversionFunction,
                bits::to_megabits as BaseConversionFunction,
            ),
            conversion(
                "megabytes",
                megabytes::to_bits as BaseConversionFunction,
                bits::to_megabytes as BaseConversionFunction,
            ),
            conversion(
                "terabits",
                terabits::to_bits as BaseConversionFunction,
                bits::to_terabits as BaseConversionFunction,
            ),
            conversion(
                "terabytes",
                terabytes::to_bits as BaseConversionFunction,
                bits::to_terabytes as BaseConversionFunction,
            ),
        ])
    }
}
