pub fn calculate_checksum(spreadsheet: &[&[u32]]) -> u32 {
    spreadsheet.iter().map(|x| row_diff(x)).sum()
}

pub fn row_diff(row: &[u32]) -> u32 {
    row.iter().max().expect("no max!") - row.iter().min().expect("no min!")
}

pub fn part_two_checksum(s: &[&[u32]]) -> u32 {
    s.iter().map(|x| row_division(x)).sum()
}

pub fn row_division(row: &[u32]) -> u32 {
    for item in row {
        for other_item in row {
            if item % other_item == 0 && item != other_item {
                return item / other_item;
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    mod part_one {
        use day_two::*;

        #[test]
        fn it_finds_the_difference_between_min_and_max() {
            assert_eq!(row_diff(&[5, 1, 9, 5]), 8);
            assert_eq!(row_diff(&[7, 5, 3]), 4);
            assert_eq!(row_diff(&[2, 4, 6, 8]), 6);
        }

        #[test]
        fn it_calculates_the_checksum() {
            let spreadsheet: &[&[u32]] = &[&[5, 1, 9, 5], &[7, 5, 3], &[2, 4, 6, 8]];

            assert_eq!(calculate_checksum(spreadsheet), 18);
        }

        #[test]
        fn the_answer_is() {
            assert_eq!(calculate_checksum(PUZZLE_INPUT), 34925);
        }
    }

    mod part_two {
        use day_two::*;

        #[test]
        fn it_finds_the_even_division() {
            assert_eq!(row_division(&[5, 9, 2, 8]), 4);
            assert_eq!(row_division(&[9, 4, 7, 3]), 3);
            assert_eq!(row_division(&[3, 8, 6, 5]), 2);
        }

        #[test]
        fn the_answer_is() {
            assert_eq!(part_two_checksum(PUZZLE_INPUT), 221);
        }
    }
}

pub const PUZZLE_INPUT: &[&[u32]] = &[
    &[
        737,
        1866,
        1565,
        1452,
        1908,
        1874,
        232,
        1928,
        201,
        241,
        922,
        281,
        1651,
        1740,
        1012,
        1001,
    ],
    &[
        339,
        581,
        41,
        127,
        331,
        133,
        51,
        131,
        129,
        95,
        499,
        527,
        518,
        435,
        508,
        494,
    ],
    &[
        1014,
        575,
        1166,
        259,
        152,
        631,
        1152,
        1010,
        182,
        943,
        163,
        158,
        1037,
        1108,
        1092,
        887,
    ],
    &[
        56,
        491,
        409,
        1263,
        1535,
        41,
        1431,
        1207,
        1393,
        700,
        1133,
        53,
        131,
        466,
        202,
        62,
    ],
    &[
        632,
        403,
        118,
        352,
        253,
        672,
        711,
        135,
        116,
        665,
        724,
        780,
        159,
        133,
        90,
        100,
    ],
    &[
        1580,
        85,
        1786,
        1613,
        1479,
        100,
        94,
        1856,
        546,
        76,
        1687,
        1769,
        1284,
        1422,
        1909,
        1548,
    ],
    &[
        479,
        356,
        122,
        372,
        786,
        1853,
        979,
        116,
        530,
        123,
        1751,
        887,
        109,
        1997,
        160,
        1960,
    ],
    &[
        446,
        771,
        72,
        728,
        109,
        369,
        300,
        746,
        86,
        910,
        566,
        792,
        616,
        84,
        338,
        57,
    ],
    &[
        6599,
        2182,
        200,
        2097,
        4146,
        7155,
        7018,
        1815,
        1173,
        4695,
        201,
        7808,
        242,
        3627,
        222,
        7266,
    ],
    &[
        1729,
        600,
        651,
        165,
        1780,
        2160,
        626,
        1215,
        149,
        179,
        1937,
        1423,
        156,
        129,
        634,
        458,
    ],
    &[
        1378,
        121,
        146,
        437,
        1925,
        2692,
        130,
        557,
        2374,
        2538,
        2920,
        2791,
        156,
        317,
        139,
        541,
    ],
    &[
        1631,
        176,
        1947,
        259,
        2014,
        153,
        268,
        752,
        2255,
        347,
        227,
        2270,
        2278,
        544,
        2379,
        349,
    ],
    &[
        184,
        314,
        178,
        242,
        145,
        410,
        257,
        342,
        183,
        106,
        302,
        320,
        288,
        151,
        449,
        127,
    ],
    &[
        175,
        5396,
        1852,
        4565,
        4775,
        665,
        4227,
        171,
        4887,
        181,
        2098,
        4408,
        2211,
        3884,
        2482,
        158,
    ],
    &[
        1717,
        3629,
        244,
        258,
        281,
        3635,
        235,
        4148,
        3723,
        4272,
        3589,
        4557,
        4334,
        4145,
        3117,
        4510,
    ],
    &[
        55,
        258,
        363,
        116,
        319,
        49,
        212,
        44,
        303,
        349,
        327,
        330,
        316,
        297,
        313,
        67,
    ],
];
