use super::{Category, ConversionFunction, ConversionFunctionMap};
use std::{collections::HashMap, convert::identity};
use unit_conversions::data_storage::*;

const BITS: &str = "bits";
const GIGABITS: &str = "gigabits";
const GIGABYTES: &str = "gigabytes";
const KIBIBITS: &str = "kibibits";
const KILOBITS: &str = "kilobits";
const MEBIBITS: &str = "mebibits";
const MEGABITS: &str = "megabits";
const MEGABYTES: &str = "megabytes";
const TERABITS: &str = "terabits";
const TERABYTES: &str = "terabytes";

pub struct DataStorage;

impl Category for DataStorage {
    fn name() -> &'static str {
        "data-storage"
    }
    fn conversion_function_map() -> ConversionFunctionMap {
        HashMap::from_iter([
            (
                BITS,
                HashMap::from_iter([
                    (BITS, identity as ConversionFunction),
                    (GIGABITS, bits::to_gigabits),
                    (GIGABYTES, bits::to_gigabytes),
                    (KIBIBITS, bits::to_kibibits),
                    (KILOBITS, bits::to_kilobits),
                    (MEBIBITS, bits::to_mebibits),
                    (MEGABITS, bits::to_megabits),
                    (MEGABYTES, bits::to_megabytes),
                    (TERABITS, bits::to_terabits),
                    (TERABYTES, bits::to_terabytes),
                ]),
            ),
            (
                GIGABITS,
                HashMap::from_iter([
                    (BITS, gigabits::to_bits as ConversionFunction),
                    (GIGABITS, identity),
                    (GIGABYTES, gigabits::to_gigabytes),
                    (KIBIBITS, gigabits::to_kibibits),
                    (KILOBITS, gigabits::to_kilobits),
                    (MEBIBITS, gigabits::to_mebibits),
                    (MEGABITS, gigabits::to_megabits),
                    (MEGABYTES, gigabits::to_megabytes),
                    (TERABITS, gigabits::to_terabits),
                    (TERABYTES, gigabits::to_terabytes),
                ]),
            ),
            (
                GIGABYTES,
                HashMap::from_iter([
                    (BITS, gigabytes::to_bits as ConversionFunction),
                    (GIGABITS, gigabytes::to_gigabits),
                    (GIGABYTES, identity),
                    (KIBIBITS, gigabytes::to_kibibits),
                    (KILOBITS, gigabytes::to_kilobits),
                    (MEBIBITS, gigabytes::to_mebibits),
                    (MEGABITS, gigabytes::to_megabits),
                    (MEGABYTES, gigabytes::to_megabytes),
                    (TERABITS, gigabytes::to_terabits),
                    (TERABYTES, gigabytes::to_terabytes),
                ]),
            ),
            (
                KIBIBITS,
                HashMap::from_iter([
                    (BITS, kibibits::to_bits as ConversionFunction),
                    (GIGABITS, kibibits::to_gigabits),
                    (GIGABYTES, kibibits::to_gigabytes),
                    (KIBIBITS, identity),
                    (KILOBITS, kibibits::to_kilobits),
                    (MEBIBITS, kibibits::to_mebibits),
                    (MEGABITS, kibibits::to_megabits),
                    (MEGABYTES, kibibits::to_megabytes),
                    (TERABITS, kibibits::to_terabits),
                    (TERABYTES, kibibits::to_terabytes),
                ]),
            ),
            (
                KILOBITS,
                HashMap::from_iter([
                    (BITS, kilobits::to_bits as ConversionFunction),
                    (GIGABITS, kilobits::to_gigabits),
                    (GIGABYTES, kilobits::to_gigabytes),
                    (KIBIBITS, kilobits::to_kibibits),
                    (KILOBITS, identity),
                    (MEBIBITS, kilobits::to_mebibits),
                    (MEGABITS, kilobits::to_megabits),
                    (MEGABYTES, kilobits::to_megabytes),
                    (TERABITS, kilobits::to_terabits),
                    (TERABYTES, kilobits::to_terabytes),
                ]),
            ),
            (
                MEBIBITS,
                HashMap::from_iter([
                    (BITS, mebibits::to_bits as ConversionFunction),
                    (GIGABITS, mebibits::to_gigabits),
                    (GIGABYTES, mebibits::to_gigabytes),
                    (KIBIBITS, mebibits::to_kibibits),
                    (KILOBITS, mebibits::to_kilobits),
                    (MEBIBITS, identity),
                    (MEGABITS, mebibits::to_megabits),
                    (MEGABYTES, mebibits::to_megabytes),
                    (TERABITS, mebibits::to_terabits),
                    (TERABYTES, mebibits::to_terabytes),
                ]),
            ),
            (
                MEGABITS,
                HashMap::from_iter([
                    (BITS, megabits::to_bits as ConversionFunction),
                    (GIGABITS, megabits::to_gigabits),
                    (GIGABYTES, megabits::to_gigabytes),
                    (KIBIBITS, megabits::to_kibibits),
                    (KILOBITS, megabits::to_kilobits),
                    (MEBIBITS, megabits::to_mebibits),
                    (MEGABITS, identity),
                    (MEGABYTES, megabits::to_megabytes),
                    (TERABITS, megabits::to_terabits),
                    (TERABYTES, megabits::to_terabytes),
                ]),
            ),
            (
                MEGABYTES,
                HashMap::from_iter([
                    (BITS, megabytes::to_bits as ConversionFunction),
                    (GIGABITS, megabytes::to_gigabits),
                    (GIGABYTES, megabytes::to_gigabytes),
                    (KIBIBITS, megabytes::to_kibibits),
                    (KILOBITS, megabytes::to_kilobits),
                    (MEBIBITS, megabytes::to_mebibits),
                    (MEGABITS, megabytes::to_megabits),
                    (MEGABYTES, identity),
                    (TERABITS, megabytes::to_terabits),
                    (TERABYTES, megabytes::to_terabytes),
                ]),
            ),
            (
                TERABITS,
                HashMap::from_iter([
                    (BITS, terabits::to_bits as ConversionFunction),
                    (GIGABITS, terabits::to_gigabits),
                    (GIGABYTES, terabits::to_gigabytes),
                    (KIBIBITS, terabits::to_kibibits),
                    (KILOBITS, terabits::to_kilobits),
                    (MEBIBITS, terabits::to_mebibits),
                    (MEGABITS, terabits::to_megabits),
                    (MEGABYTES, terabits::to_megabytes),
                    (TERABITS, identity),
                    (TERABYTES, terabits::to_terabytes),
                ]),
            ),
            (
                TERABYTES,
                HashMap::from_iter([
                    (BITS, terabytes::to_bits as ConversionFunction),
                    (GIGABITS, terabytes::to_gigabits),
                    (GIGABYTES, terabytes::to_gigabytes),
                    (KIBIBITS, terabytes::to_kibibits),
                    (KILOBITS, terabytes::to_kilobits),
                    (MEBIBITS, terabytes::to_mebibits),
                    (MEGABITS, terabytes::to_megabits),
                    (MEGABYTES, terabytes::to_megabytes),
                    (TERABITS, terabytes::to_terabits),
                    (TERABYTES, identity),
                ]),
            ),
        ])
    }
}
