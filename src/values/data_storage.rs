use super::{conversion, BaseConversionFunctionsMap, Values};
use std::{collections::HashMap, convert::identity};
use unit_conversions::data_storage::*;

pub struct DataStorage;

impl Values for DataStorage {
    fn name() -> &'static str {
        "data-storage"
    }
    fn base_conversion_functions() -> BaseConversionFunctionsMap {
        HashMap::from_iter([
            conversion("bits", identity, identity),
            conversion("gigabits", gigabits::to_bits, bits::to_gigabits),
            conversion("gigabytes", gigabytes::to_bits, bits::to_gigabytes),
            conversion("gigabytes", gigabytes::to_bits, bits::to_gigabytes),
            conversion("kibibits", kibibits::to_bits, bits::to_kibibits),
            conversion("kilobits", kilobits::to_bits, bits::to_kilobits),
            conversion("mebibits", mebibits::to_bits, bits::to_mebibits),
            conversion("megabits", megabits::to_bits, bits::to_megabits),
            conversion("megabytes", megabytes::to_bits, bits::to_megabytes),
            conversion("terabits", terabits::to_bits, bits::to_terabits),
            conversion("terabytes", terabytes::to_bits, bits::to_terabytes),
        ])
    }
}
