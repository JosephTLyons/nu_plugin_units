use super::{ConversionFunction, ConversionFunctionMap, Values};
use std::{collections::HashMap, convert::identity};
use unit_conversions::data_storage::*;

pub struct DataStorage;

impl Values for DataStorage {
    fn name() -> &'static str {
        "data-storage"
    }
    fn hash_map() -> ConversionFunctionMap {
        HashMap::from_iter([
            (
                "bits",
                HashMap::from_iter([
                    ("bits", identity as ConversionFunction),
                    ("gigabits", bits::to_gigabits),
                    ("gigabytes", bits::to_gigabytes),
                    ("kibibits", bits::to_kibibits),
                    ("kilobits", bits::to_kilobits),
                    ("mebibits", bits::to_mebibits),
                    ("megabits", bits::to_megabits),
                    ("megabytes", bits::to_megabytes),
                    ("terabits", bits::to_terabits),
                    ("terabytes", bits::to_terabytes),
                ]),
            ),
            (
                "gigabits",
                HashMap::from_iter([
                    ("bits", gigabits::to_bits as ConversionFunction),
                    ("gigabits", identity),
                    ("gigabytes", gigabits::to_gigabytes),
                    ("kibibits", gigabits::to_kibibits),
                    ("kilobits", gigabits::to_kilobits),
                    ("mebibits", gigabits::to_mebibits),
                    ("megabits", gigabits::to_megabits),
                    ("megabytes", gigabits::to_megabytes),
                    ("terabits", gigabits::to_terabits),
                    ("terabytes", gigabits::to_terabytes),
                ]),
            ),
            (
                "gigabytes",
                HashMap::from_iter([
                    ("bits", gigabytes::to_bits as ConversionFunction),
                    ("gigabits", gigabytes::to_gigabits),
                    ("gigabytes", identity),
                    ("kibibits", gigabytes::to_kibibits),
                    ("kilobits", gigabytes::to_kilobits),
                    ("mebibits", gigabytes::to_mebibits),
                    ("megabits", gigabytes::to_megabits),
                    ("megabytes", gigabytes::to_megabytes),
                    ("terabits", gigabytes::to_terabits),
                    ("terabytes", gigabytes::to_terabytes),
                ]),
            ),
            (
                "kibibits",
                HashMap::from_iter([
                    ("bits", kibibits::to_bits as ConversionFunction),
                    ("gigabits", kibibits::to_gigabits),
                    ("gigabytes", kibibits::to_gigabytes),
                    ("kibibits", identity),
                    ("kilobits", kibibits::to_kilobits),
                    ("mebibits", kibibits::to_mebibits),
                    ("megabits", kibibits::to_megabits),
                    ("megabytes", kibibits::to_megabytes),
                    ("terabits", kibibits::to_terabits),
                    ("terabytes", kibibits::to_terabytes),
                ]),
            ),
            (
                "kilobits",
                HashMap::from_iter([
                    ("bits", kilobits::to_bits as ConversionFunction),
                    ("gigabits", kilobits::to_gigabits),
                    ("gigabytes", kilobits::to_gigabytes),
                    ("kibibits", kilobits::to_kibibits),
                    ("kilobits", identity),
                    ("mebibits", kilobits::to_mebibits),
                    ("megabits", kilobits::to_megabits),
                    ("megabytes", kilobits::to_megabytes),
                    ("terabits", kilobits::to_terabits),
                    ("terabytes", kilobits::to_terabytes),
                ]),
            ),
            (
                "mebibits",
                HashMap::from_iter([
                    ("bits", mebibits::to_bits as ConversionFunction),
                    ("gigabits", mebibits::to_gigabits),
                    ("gigabytes", mebibits::to_gigabytes),
                    ("kibibits", mebibits::to_kibibits),
                    ("kilobits", mebibits::to_kilobits),
                    ("mebibits", identity),
                    ("megabits", mebibits::to_megabits),
                    ("megabytes", mebibits::to_megabytes),
                    ("terabits", mebibits::to_terabits),
                    ("terabytes", mebibits::to_terabytes),
                ]),
            ),
            (
                "megabits",
                HashMap::from_iter([
                    ("bits", megabits::to_bits as ConversionFunction),
                    ("gigabits", megabits::to_gigabits),
                    ("gigabytes", megabits::to_gigabytes),
                    ("kibibits", megabits::to_kibibits),
                    ("kilobits", megabits::to_kilobits),
                    ("mebibits", megabits::to_mebibits),
                    ("megabits", identity),
                    ("megabytes", megabits::to_megabytes),
                    ("terabits", megabits::to_terabits),
                    ("terabytes", megabits::to_terabytes),
                ]),
            ),
            (
                "megabytes",
                HashMap::from_iter([
                    ("bits", megabytes::to_bits as ConversionFunction),
                    ("gigabits", megabytes::to_gigabits),
                    ("gigabytes", megabytes::to_gigabytes),
                    ("kibibits", megabytes::to_kibibits),
                    ("kilobits", megabytes::to_kilobits),
                    ("mebibits", megabytes::to_mebibits),
                    ("megabits", megabytes::to_megabits),
                    ("megabytes", identity),
                    ("terabits", megabytes::to_terabits),
                    ("terabytes", megabytes::to_terabytes),
                ]),
            ),
            (
                "terabits",
                HashMap::from_iter([
                    ("bits", terabits::to_bits as ConversionFunction),
                    ("gigabits", terabits::to_gigabits),
                    ("gigabytes", terabits::to_gigabytes),
                    ("kibibits", terabits::to_kibibits),
                    ("kilobits", terabits::to_kilobits),
                    ("mebibits", terabits::to_mebibits),
                    ("megabits", terabits::to_megabits),
                    ("megabytes", terabits::to_megabytes),
                    ("terabits", identity),
                    ("terabytes", terabits::to_terabytes),
                ]),
            ),
            (
                "terabytes",
                HashMap::from_iter([
                    ("bits", terabytes::to_bits as ConversionFunction),
                    ("gigabits", terabytes::to_gigabits),
                    ("gigabytes", terabytes::to_gigabytes),
                    ("kibibits", terabytes::to_kibibits),
                    ("kilobits", terabytes::to_kilobits),
                    ("mebibits", terabytes::to_mebibits),
                    ("megabits", terabytes::to_megabits),
                    ("megabytes", terabytes::to_megabytes),
                    ("terabits", terabytes::to_terabits),
                    ("terabytes", identity),
                ]),
            ),
        ])
    }
}
