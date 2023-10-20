use super::{conversion, BaseConversionFunction, BaseConversionFunctionsMap, Values};
use std::{collections::HashMap, convert::identity};
use unit_conversions::data_transfer_rate::*;

pub struct DataTransferRate;

impl Values for DataTransferRate {
    fn name() -> &'static str {
        "data-transfer-rate"
    }
    fn base_conversion_functions() -> BaseConversionFunctionsMap {
        HashMap::from_iter([
            conversion(
                "bits-per-second",
                identity as BaseConversionFunction,
                identity as BaseConversionFunction,
            ),
            conversion(
                "gigabits-per-second",
                giga_bits_per_second::to_bits_per_second as BaseConversionFunction,
                bits_per_second::to_giga_bits_per_second as BaseConversionFunction,
            ),
            conversion(
                "gigabytes-per-second",
                giga_bytes_per_second::to_bits_per_second,
                bits_per_second::to_giga_bytes_per_second,
            ),
            conversion(
                "kibibits-per-second",
                kibibits_per_second::to_bits_per_second,
                bits_per_second::to_kibibits_per_second,
            ),
            conversion(
                "kilobits-per-second",
                kilo_bits_per_second::to_bits_per_second,
                bits_per_second::to_kilo_bits_per_second,
            ),
            conversion(
                "kilobytes-per-second",
                kilo_bytes_per_second::to_bits_per_second,
                bits_per_second::to_kilo_bytes_per_second,
            ),
            conversion(
                "mebibits-per-second",
                mebibits_per_second::to_bits_per_second,
                bits_per_second::to_mebibits_per_second,
            ),
            conversion(
                "megabits-per-second",
                mega_bits_per_second::to_bits_per_second,
                bits_per_second::to_mega_bits_per_second,
            ),
            conversion(
                "megabytes-per-second",
                mega_bytes_per_second::to_bits_per_second,
                bits_per_second::to_mega_bytes_per_second,
            ),
            conversion(
                "terabits-per-second",
                tera_bits_per_second::to_bits_per_second,
                bits_per_second::to_tera_bits_per_second,
            ),
            conversion(
                "terabytes-per-second",
                tera_bytes_per_second::to_bits_per_second,
                bits_per_second::to_tera_bytes_per_second,
            ),
        ])
    }
}
