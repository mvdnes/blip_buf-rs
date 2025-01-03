extern crate blip_buf;

use blip_buf::BlipBuf;

const CLOCK_RATE: f64 = 1000000.0;
const SAMPLE_RATE: u32 = 4800;

struct Lfsr13 {
    state: u32,
}

impl Lfsr13 {
    fn new() -> Self {
        Lfsr13 {
            state: (1 << 12) | 1,
        }
    }

    fn next(&mut self) -> u32 {
        let lfsr = self.state;
        let bit = (lfsr ^ (lfsr >> 1) ^ (lfsr >> 2) ^ (lfsr >> 5)) & 1;
        self.state = (lfsr >> 1) | (bit << 12);
        return self.state;
    }
}

#[test]
fn lfsr() {
    let mut blip = BlipBuf::new(SAMPLE_RATE / 10);
    blip.set_rates(CLOCK_RATE, SAMPLE_RATE as f64);

    let period = 20;
    let mut time = 0;

    let mut lfsr = Lfsr13::new();
    let mut oldval = lfsr.next();

    for n in 0..60 {
        let clocks = CLOCK_RATE as i32 / 60;
        while time < clocks {
            let newval = lfsr.next();
            let delta = (newval as i32) - (oldval as i32);
            blip.add_delta(time as u32, delta);
            oldval = newval;

            time = time + period;
        }

        // Add those clocks to buffer and adjust time for next frame
        time = time - clocks;
        blip.end_frame(clocks as u32);

        // Read any output samples now available
        while blip.samples_avail() > 0 {
            let temp = &mut [0i16; 1024];
            let count = blip.read_samples(temp, false);
            let target_count = if n == 0 { 79 } else { 80 };
            assert_eq!(&temp[..count], &RESULTS[n][..target_count]);
        }
    }
}

const RESULTS: [[i16; 80]; 60] = [
    [
        0, -5, 9, -40, 49, -160, 122, -909, -2788, -2819, -2968, -1558, -2579, -2794, -1487, -843,
        -1301, -1618, -1810, -1729, -2141, -3004, -2200, -1951, -1104, -786, -2043, -2319, -2467,
        -2809, -2047, -1823, -2516, -2764, -2553, -852, -2144, -3070, -2024, -2001, -2120, 523,
        -2183, -3377, -545, -1252, -2291, -2598, -1261, -2284, -2007, -1900, -2649, -1998, -883,
        -837, -2558, -2489, -1455, -938, -942, -2079, -1099, -3020, -2459, -779, -998, -986, -1070,
        -1865, -2384, -1353, -2040, -1334, -1745, -2446, -1052, -1553, -1274, 0,
    ],
    [
        -448, -1340, -1002, -1240, -2787, -2806, -2994, -2326, -3977, -3259, -2517, -1613, -840,
        -2052, -2535, -3101, -2143, -1521, -1079, -2295, -1882, -1166, -384, -440, -1157, -1817,
        -3407, -2390, -1664, -480, -2028, -1050, 109, -2134, -2172, -1697, -1161, -835, -908,
        -1613, -1472, -1539, -2593, -2127, -2011, -878, -464, -1227, -2982, -2395, -1027, -2143,
        -1132, 353, -515, -1519, -1757, -1353, -446, 265, -906, -2203, -1922, -2, -1441, -1990,
        -2468, -3055, -2245, -744, 964, -682, -1912, -3146, -3221, -1536, -2786, -2880, -1285,
        -1737,
    ],
    [
        -2825, -1461, -452, -1155, 447, -1501, -566, -1284, -1366, -70, -2539, -797, -1376, -2610,
        -2315, -675, 313, -210, -1887, -2220, -2275, -1252, -2312, -1956, -1476, -2706, -915,
        -1422, -1877, -1795, -1449, -1053, -1625, -2317, -3458, -2603, -532, -1262, -2031, -451,
        -246, -2121, -2788, -1578, -428, -866, -686, -86, 863, -234, -1215, -1649, -2008, -1147,
        -795, -2089, -1901, -2952, -2694, 77, 523, -963, -1719, -2560, -3011, -1080, -908, -776,
        -545, -871, -1529, -2461, -2005, -2869, -2584, -782, -1650, -1353, -767, -261,
    ],
    [
        -430, -1523, -1113, -3292, -4280, -2548, -2392, -944, -2222, -2745, -1913, -1792, -2004,
        -1518, -1909, -2253, -1696, -1132, -1669, -1222, -1278, -1628, -1558, -1890, -514, -1592,
        -892, -2020, -1397, 608, -2358, -2676, -1705, -2119, -265, -988, -2565, -1628, -2300, -727,
        -211, -873, -229, -2857, -2426, -1426, -1872, -719, -654, -1552, -2176, -1332, -272, -2137,
        -2194, -2029, -1803, -615, -1896, 147, 764, -1091, -145, -966, -1136, -1214, -1079, -200,
        329, 98, -852, -391, -131, 500, -438, -1956, -1103, -1317, -21, 521,
    ],
    [
        -663, -1489, -1161, -2424, -2161, -473, -395, -658, -1481, -2291, -957, -333, -805, -1425,
        -238, 387, -1905, -584, -986, -1492, -978, -1630, 221, -956, -1235, 495, 391, -568, 752,
        758, -1392, -620, -1568, -1325, 668, -714, -2132, -2787, -2659, -1309, -16, 203, -303,
        -885, -518, -666, -1011, 299, -466, -1134, -1029, -1962, -1293, -1740, -1121, -105, -1539,
        -434, -192, -1587, -1901, -236, 411, 314, -1287, -1291, -458, -2196, -1314, 1487, -1438,
        -2182, 830, -515, -1901, -1840, -2137, -1441, -37, 591,
    ],
    [
        -950, -2519, -1167, -1474, -1822, -2209, -1979, -1105, -1529, -2349, -2609, -1197, -1619,
        -876, -1254, -2092, -204, -1863, -3062, -924, -672, -2747, -995, 202, -1554, -1233, -1871,
        -1497, 425, 118, 328, 363, -234, 1358, 940, -1115, 130, 186, -516, -103, -1765, -1387,
        -297, -1214, -1025, -296, 693, 285, -1871, -2213, 44, 1040, 833, 1116, -561, -1198, -130,
        -1036, -2771, -738, -35, -1265, -620, 179, -575, -2337, -1059, -485, -777, -597, 335, -65,
        -2161, -1591, -926, -818, -1756, -2248, 1420, 914,
    ],
    [
        -1767, -392, -780, -1416, -3079, -2849, -1213, -1170, -1079, -1594, -1415, -1379, -1910,
        610, 955, 576, 513, 14, 496, 605, -171, -568, 246, -1104, -773, 1117, 905, -122, -1699,
        -1391, -634, -1490, -1863, -430, 318, -511, -1068, -1464, -216, -656, -1901, -2358, -2050,
        -1683, -2000, -1476, -52, 527, 1090, 1496, -423, -625, 17, -1731, -1362, -937, -1306,
        -1294, -404, -1234, -1909, -1803, -904, 206, 71, -401, -271, 236, -828, -1489, 265, 538,
        452, 739, 323, 830, 1190, 1267, -981, -3295,
    ],
    [
        -1152, -568, -710, -501, -1799, 510, 1063, -1279, -1369, -1163, -1229, 8, 447, -291, -362,
        -362, -2694, -1625, 474, -419, -513, 216, -60, -269, -904, -161, -489, -866, -694, -1135,
        -506, 305, -999, -2243, -2292, -1679, -585, -334, -1654, -301, 348, -672, -704, -724, 366,
        -617, -1194, -205, -90, -719, -1066, -231, 266, -1210, -1137, -1185, -1858, -1665, -2078,
        -185, -96, -910, -184, -294, 12, -1371, -243, 296, -1466, -1381, -1573, -1859, -2536, -901,
        -439, -523, 312, -222, -732, -1887,
    ],
    [
        770, 1289, -378, 364, 765, 989, -445, -944, -77, -51, -585, -320, -1146, -1140, -723, -592,
        522, -73, -1094, -906, -735, -1181, -1752, -1703, -1464, 520, 387, 538, 297, -528, -489,
        -780, -819, -1975, -1613, -1746, -2517, -1761, -939, -656, 260, 90, 39, -341, -496, 167, 9,
        -547, -5, -872, -1072, -423, -2092, -1216, -671, -846, -1072, -631, -946, -877, -100,
        -2442, -1202, 508, -1968, -1397, -104, -1074, 82, -48, -425, -745, -822, 375, -649, -2407,
        -2125, -1683, -269, -2208,
    ],
    [
        -2644, 159, 315, 1115, 685, -2417, -950, -35, -824, 61, -464, 476, 544, -1400, -651, -92,
        -1255, -1209, -943, 279, -880, -491, -521, -551, 475, -1421, -1875, 619, -402, -2090, -495,
        109, -872, -1031, 352, -82, 155, -176, -1009, -1611, 323, -94, -1769, -1042, -769, -401,
        -210, -366, -1375, -2508, -1791, -870, 804, -364, -967, -150, 1007, 1396, -784, -531, -638,
        -1724, -561, -201, -1226, -1779, -44, 674, -1459, -1272, -85, -657, -541, -933, -2900,
        -2500, -1017, -1694, -289, -427,
    ],
    [
        -1460, -272, 631, 551, -12, -108, -254, -215, -1331, -928, -407, 66, 935, -73, -780, -779,
        -1219, -848, -132, -812, -1092, -1342, 270, 116, -1554, -881, -235, -949, 1336, 862, -2376,
        327, 690, -441, -1262, -82, -299, -877, -165, -1068, -821, 221, 898, -513, -1346, -208,
        289, 807, -481, 137, -726, -1752, 450, 499, 486, 392, -2, -1058, -147, -327, -326, 174,
        -1165, 16, 149, -173, 854, 439, 57, 618, -1088, -1358, -1696, -966, -1997, -2451, -1149,
        -784, 642, -340,
    ],
    [
        -962, -1691, -1144, -343, 280, -487, -923, 81, 601, 1205, 253, 18, -1795, -1477, -602, 610,
        -11, -594, 1636, -153, -1041, -516, -30, 408, 499, -58, -367, 24, -1090, -1019, -780, -43,
        864, 445, -1143, -1704, 114, -473, -549, 1429, 1153, 41, -542, -225, 386, 1522, 822, -542,
        -1145, 845, 513, -829, -853, -1854, -1294, -211, 1941, 1303, -442, -1400, -2376, -672,
        -937, -2040, -466, -156, -1396, -1016, 848, -26, 1292, 509, -122, 745, -899, 1399, -881,
        -311, 353, -1175,
    ],
    [
        -1445, -62, 1269, 1385, -266, -1010, -1210, -381, -596, -1333, -24, -1479, -370, 165, -772,
        -632, -525, 71, -258, -916, -1977, -2136, 127, 356, -921, 24, 1192, -406, -1653, -1056,
        534, 383, 304, 715, 1784, 1410, 54, -369, -900, -466, 450, -644, -946, -1332, -2179, 305,
        1791, 634, -521, -1071, -2122, -658, 316, 90, 567, 265, -126, -1252, -1117, -1363, -2059,
        -80, -224, -621, 170, 543, 899, -288, -178, -1202, -3526, -1908, -1526, -320, -507, -1906,
        -1095, -769, -946, -723,
    ],
    [
        -601, -1310, -890, -274, -400, -591, 24, -784, -329, -1135, 248, -259, -237, -398, -1226,
        1447, -206, -2164, -731, -1236, 52, 677, -1438, -933, -1075, -624, 1075, -68, 860, -1008,
        -2084, -490, -921, -222, 492, -335, -1089, -930, 637, -504, -1557, -932, -1263, 208, -612,
        -2, 2133, 123, 454, 386, -356, -156, -394, 463, 1061, 1267, 243, 280, 712, 1156, 1109,
        -883, -480, -350, 257, 1519, 671, -501, -335, -1001, -1764, -6, 561, 308, -220, -1356,
        -667, 547, 235, -398,
    ],
    [
        -68, 1558, -477, -426, 440, -881, 15, -864, 518, 604, -750, 864, 1496, 458, 923, 2111,
        -102, -141, -159, -1075, 1120, 815, -990, -1769, -2049, -1011, 455, 1069, 717, 67, 75, 385,
        -328, 720, 851, -364, -89, -952, -792, -616, -903, 756, -396, -207, 818, -317, -1297, -27,
        1113, 1246, 108, -877, 327, -765, -1437, 1774, 740, -2002, 880, 1082, -975, -1053, -1318,
        -1060, 264, 1317, 577, -1597, -923, -380, -1104, -1218, -1556, -480, -603, -1257, -2058,
        -840, -633, -595,
    ],
    [
        15, -1526, 47, -204, -2409, -1008, 453, -1513, -1286, 1084, -378, -718, -782, -1272, 660,
        1092, 784, 1300, 448, 1464, 2240, 17, 190, 1252, 144, 663, -472, -1184, 310, -202, -549,
        140, 1061, 1410, -468, -1793, -100, 1685, 1489, 1831, 832, -651, 259, 262, -1813, -1035,
        910, -382, -259, 622, 665, -1346, -1013, 233, -81, -46, 617, 1105, -979, -1362, -351, -201,
        -572, -1915, 750, 2560, -758, -231, 172, -535, -1833, -2678, -964, -464, -492, -702, -1045,
        -476, -1448, 344,
    ],
    [
        1816, 1171, 1287, 702, 922, 1289, 795, -33, 710, 126, -729, 1321, 1717, 956, -663, -1110,
        -120, -481, -1365, -388, 906, 401, -296, -856, -106, 429, -1049, -1609, -1713, -1055,
        -1362, -1174, 36, 1077, 1344, 2221, 878, -350, 700, -587, -1157, -320, -611, -837, -81,
        -174, -1303, -1256, -805, 569, 748, 334, 113, 756, 244, -999, 173, 1282, 857, 1360, 929,
        1192, 1633, 1934, 642, -2418, -1565, 223, -404, 323, -1094, 52, 1974, -23, -1068, -506,
        -858, 141, 975, 584,
    ],
    [
        -32, 517, -1465, -1934, 632, 573, -224, 654, 543, 420, -281, 72, 328, -371, -148, -489,
        -366, 751, 120, -1468, -1842, -1435, -459, 354, -793, -548, 1067, 22, -130, -384, 609, 422,
        -767, -29, 518, 2, -536, -148, 868, -208, -886, -444, -1305, -1135, -1597, -467, 791, -436,
        154, 177, 530, -411, -511, 1114, -570, -980, -1018, -1189, -1988, -1133, 210, -264, 677,
        429, 52, -1267, 52, 2246, 339, 556, 1089, 1543, 578, -549, 86, 577, -53, 101, -332, -881,
    ],
    [
        -281, -312, 653, 825, -465, -552, -312, -466, -1235, -1210, -1319, 351, 1110, 748, 1082,
        14, 16, -319, -228, -1183, -1450, -1036, -1962, -1699, -664, -376, 418, 696, 415, 329,
        -178, 417, 608, -16, 213, 81, -954, 134, -1209, -1313, -221, -389, -574, -434, -224, -796,
        420, -1234, -1778, 938, -695, -1672, 279, -463, 5, 653, -25, -110, -610, 459, 430, -1653,
        -1878, -1537, -212, -783, -2779, -185, 823, 1076, 1740, -1199, -1518, 581, -443, 276, 105,
        384, 1291, -458,
    ],
    [
        -831, 482, -595, -848, -801, 409, 24, -518, 179, -502, 803, -246, -1809, 214, 868, -1616,
        -701, 478, -38, -940, 429, 493, 392, 424, -315, -1259, -69, 933, -1161, -957, -439, -182,
        150, 86, -511, -1977, -1732, -916, 737, 686, -750, -6, 840, 2035, 241, -531, 76, -1227,
        -701, 267, -473, -1447, -454, 1230, -397, -1361, 85, -69, -342, -202, -1930, -2693, -882,
        -1166, -597, 389, -1073, -382, 745, 1074, 431, 306, 51, 252, -647, -914, -114, 98, 1140,
        740,
    ],
    [
        -372, -399, -742, -794, 115, -212, -699, -1026, -78, 958, -895, -947, 79, -482, 586, 2237,
        -1595, -457, 1346, 213, -782, -312, 463, -693, 113, -453, -737, 150, 1177, 451, -1068,
        -268, 452, 1077, 321, -20, 367, -1666, 150, 931, 780, 725, 579, -530, -270, 298, -315, 630,
        -560, -240, 688, 31, 834, 1107, 214, 974, -195, -1113, -1272, -990, -1023, -2463, -1082,
        -788, 628, 432, -545, -1151, -1225, -227, 371, 295, -811, 143, 614, 1512, 811, 481, -875,
        -1617,
    ],
    [
        -499, 402, 915, -680, 1453, 1008, -796, -382, 65, 584, 796, 491, -147, 299, -389, -954,
        -489, -190, 1013, 961, -209, -1644, -143, 274, -647, 1152, 1767, 657, -186, -94, 356, 1513,
        1526, 171, -934, 350, 1369, -414, -427, -1358, -1324, -430, 1544, 2210, 207, -700, -2055,
        -1081, -168, -1738, -807, 281, -717, -1251, 717, 593, 842, 1597, -301, 1295, -573, 1070,
        417, -817, 873, -495, -1278, -402, 1189, 1797, 609, -690, -896, -555, 87, -1225, 3, -693,
        -860, 655,
    ],
    [
        -385, -412, -396, 162, 201, -458, -1304, -2182, -406, 892, -378, -330, 1350, 521, -1184,
        -1241, 348, 822, 458, 773, 1642, 2067, 619, 5, -520, -542, 530, 104, -850, -694, -2002,
        -482, 1883, 1407, -52, -564, -1681, -1172, 538, 268, 707, 610, 310, -666, -1108, -801,
        -1906, -558, 387, -534, 201, 589, 1158, 369, -190, -179, -2932, -2374, -1261, -699, 229,
        -1516, -1185, -590, -631, -705, -232, -968, -870, -276, 41, -559, 258, -425, -185, -730,
        -125, 442, -369, 272,
    ],
    [
        -1212, 837, 1163, -1886, -905, -793, -469, 1157, -623, -1149, -538, -1020, 1100, 436, 720,
        229, -2087, -657, -497, -467, 684, 184, -659, -1015, 402, 428, -1381, -792, -1096, -94,
        158, -591, 2095, 1065, 215, 934, -132, 74, -230, 341, 1080, 1525, 807, 275, 846, 1099,
        1618, -104, -668, -9, -44, 1522, 1282, -2, -324, -317, -1666, -439, 753, 575, 244, -845,
        -989, 529, 586, 31, -306, 1458, 633, -815, 804, -499, -47, -350, -62, 1274, -462, 400,
        1724, 1024,
    ],
    [
        598, 2253, 919, -336, 405, -957, 522, 1534, -339, -1368, -1902, -1306, 180, 1178, 1067,
        460, 100, 605, 16, 369, 1329, 48, -1, -395, -917, -245, -940, 504, 379, -492, 908, 354,
        -992, -482, 1092, 1410, 876, -715, 141, 80, -1554, 834, 2042, -1550, -130, 1798, -333,
        -943, -1023, -1135, -64, 1247, 1275, -833, -1320, -62, -843, -913, -1464, -655, -274, -824,
        -1767, -1244, -252, -740, 299, -997, -574, 598, -1800, -1654, 502, -535, -1800, 785, 482,
        -754, -366, -1226,
    ],
    [
        87, 1413, 848, 1420, 872, 1000, 2529, 965, -173, 1399, 580, 590, 328, -1186, 37, 326, -489,
        55, 849, 1682, 461, -1510, -776, 1499, 1748, 1819, 1603, -279, -15, 715, -983, -1673, 810,
        251, -371, 484, 1046, -516, -1384, 195, 175, 44, 417, 1356, -39, -1474, -428, -105, -115,
        -1510, -506, 2896, 416, -696, 510, -231, -1068, -2626, -1433, -298, -410, -364, -975, -383,
        -1035, -488, 1881, 1443, 1421, 1040, 864, 1382, 1199, 268, 486, 793, -707, 749, 1959, 1415,
    ],
    [
        53, -1107, -279, -27, -1083, -779, 766, 851, -2, -544, -470, 731, -482, -1327, -1695,
        -1067, -1099, -1220, -348, 1027, 1287, 2155, 1727, -162, 529, 233, -1214, -336, -354, -691,
        -316, 275, -954, -1165, -987, 291, 934, 626, 195, 668, 751, -605, -375, 1343, 1021, 1355,
        1232, 1111, 1646, 1986, 1538, -1417, -2291, 175, -206, 303, -411, -718, 1888, 979, -965,
        -468, -680, -202, 942, 971, 116, 559, -444, -2225, -31, 1062, -81, 483, 785, 572, 102,
        -123, 580, -122,
    ],
    [
        -121, -193, -474, 543, 713, -926, -1722, -1530, -730, 345, -145, -961, 941, 533, -53, -217,
        272, 915, -447, -296, 594, 335, -296, -336, 766, 469, -841, -332, -929, -1147, -1270,
        -1087, 886, -4, -49, 376, 459, 259, -848, 997, 219, -966, -821, -1022, -1584, -1656, 173,
        -97, 439, 763, 272, -657, -863, 2175, 1093, 334, 1087, 1521, 1215, -278, -138, 667, 263,
        63, 90, -794, -340, -218, 317, 1149, 4, -542, -273, -221, -921, -1158, -1243, -313, 1318,
        768,
    ],
    [
        1252, 422, 64, -94, -168, -631, -1524, -896, -1567, -1886, -859, -343, 158, 881, 508, 561,
        -18, 259, 758, 301, 66, 503, -775, -80, -436, -1594, -299, -239, -387, -521, -13, -698,
        142, -210, -2161, 353, 374, -1821, -131, 60, -385, 823, 186, 86, -464, 77, 883, -913,
        -1943, -1543, -706, 49, -2507, -1163, 941, 859, 1914, 87, -2034, 344, -13, -22, 445, 104,
        1322, 375, -1054, 346, -51, -823, -758, -12, 598, -622, 284, -372, 450, 554, -1536, -666,
    ],
    [
        1343, -889, -1243, 360, 398, -773, -36, 792, 348, 634, 34, -906, -738, 1172, -447, -1188,
        -427, -239, 166, 213, -96, -1466, -1946, -1102, 160, 1232, -466, -219, 530, 1922, 1162,
        -603, 176, -714, -1080, 222, -33, -1144, -1010, 1019, 496, -1373, -291, 254, -329, -24,
        -1122, -2764, -1385, -757, -1079, 522, -590, -760, 517, 1172, 717, 390, 197, 269, -134,
        -1012, -217, 33, 867, 1199, -61, -350, -498, -848, -96, 137, -563, -814, -621, 1082, -192,
        -1142, -104, -69,
    ],
    [
        -215, 2431, -234, -1490, 1353, 660, -385, -711, 626, -387, -133, -12, -773, -134, 977,
        1038, -686, -642, 395, 906, 915, -238, 741, -1167, -647, 1048, 827, 837, 749, -39, -579,
        453, -233, 457, 66, -661, 733, 239, 500, 1333, 443, 762, 544, -1018, -1041, -1240, -567,
        -2216, -1525, -772, 115, 906, -312, -798, -1375, -426, 194, 642, -572, -181, 540, 1312,
        1270, 540, -107, -1715, -711, 12, 1201, -337, 602, 1793, -454, -512, -38, 493, 819, 748,
        40, 169,
    ],
    [
        112, -953, -493, -382, 742, 1164, 407, -1351, -796, 589, -526, 457, 1957, 1095, 85, -164,
        236, 1132, 1854, 687, -614, -362, 1567, 145, -425, -894, -1498, -694, 793, 2516, 884, -360,
        -1591, -1695, -2, -1238, -1362, 237, -202, -1302, 39, 1071, 350, 1955, 21, 922, 238, 108,
        1443, -1046, 680, 125, -1123, -843, 790, 1784, 1276, -410, -752, -817, 205, -842, -516,
        -12, -1262, 522, 32, -438, -357, -19, 381, -190, -883, -2028, -1201, 880, 166, -622, 945,
        1173,
    ],
    [
        -704, -1388, -193, 968, 529, 702, 1293, 2247, 1161, 169, -267, -636, 216, 582, -717, -538,
        -1577, -1353, 1416, 1884, 399, -368, -1203, -1677, 247, 434, 554, 790, 460, -192, -1135,
        -685, -1540, -1284, 530, -331, -51, 537, 1038, 881, -223, 185, -1973, -3001, -1264, -1114,
        335, -925, -1472, -641, -517, -732, -253, -635, -993, -440, 125, -408, 24, -17, -385, -303,
        -656, 728, -348, 340, -772, -189, 1845, -1107, -1466, -479, -906, 940, 247, -1353, -421,
        -1089, 459, 1005,
    ],
    [
        319, 980, -1644, -1248, -231, -686, 459, 538, -363, -993, -168, 908, -949, -1031, -855,
        -648, 553, -732, 1282, 1938, 65, 1005, 194, 8, -64, 58, 937, 1465, 1244, 284, 737, 997,
        1624, 706, -833, 15, -199, 1083, 1642, 461, -378, -50, -1306, -1071, 622, 707, 443, -375,
        -1199, 122, 759, 287, -338, 827, 1482, -819, 478, 104, -429, 99, -570, 1280, 125, -186,
        1547, 1457, 488, 1790, 1827, -338, 423, -509, -300, 1706, 342, -1088, -1748, -1633, -291,
        1006,
    ],
    [
        1232, 728, 140, 498, 361, 0, 1309, 560, -126, -14, -953, -293, -730, -136, 902, -538, 550,
        809, -585, -919, 741, 1395, 1304, -291, -322, 525, -1207, -366, 2458, -435, -1235, 1787,
        470, -937, -877, -1178, -494, 908, 1546, 22, -1570, -227, -520, -879, -1261, -1047, -167,
        -588, -1405, -1684, -271, -685, 42, -312, -1178, 713, -920, -2150, -24, 258, -1828, -108,
        1122, -640, -329, -958, -609, 1319, 1023, 1219, 1266, 655, 2238, 1850, -222, 1004, 1083,
        363, 784, -874,
    ],
    [
        -522, 582, -334, -157, 569, 1557, 1167, -990, -1358, 890, 1903, 1690, 1979, 312, -349, 719,
        -167, -1922, 89, 822, -419, 215, 1017, 282, -1496, -236, 353, 56, 231, 1157, 780, -1326,
        -776, -103, 4, -922, -1444, 2211, 1759, -952, 414, 37, -599, -2259, -2058, -415, -366,
        -278, -788, -623, -573, -1126, 1386, 1754, 1364, 1307, 801, 1278, 1392, 620, 209, 1025,
        -313, -13, 1887, 1684, 663, -920, -631, 134, -715, -1104, 321, 1082, 256, -305, -710, 531,
        112, -1145,
    ],
    [
        -1604, -1307, -932, -1254, -738, 681, 1269, 1822, 2244, 330, 102, 760, -988, -638, -204,
        -573, -571, 320, -497, -1186, -1081, -196, 923, 792, 320, 438, 951, -102, -784, 962, 1252,
        1154, 1449, 1026, 1529, 1889, 1973, -255, -2598, -477, 134, -25, 202, -1112, 1177, 1766,
        -580, -690, -474, -553, 681, 1128, 396, 309, 327, -2009, -974, 1142, 262, 149, 884, 608,
        403, -241, 497, 181, -208, -33, -476, 141, 962, -331, -1586, -1643, -1036, 55, 321, -1004,
        326, 1001,
    ],
    [
        -28, -60, -91, 1002, 34, -563, 424, 549, -82, -435, 389, 901, -573, -517, -552, -1234,
        -1039, -1461, 419, 535, -296, 431, 319, 631, -751, 350, 920, -851, -774, -965, -1247,
        -1934, -313, 169, 70, 912, 380, -124, -1294, 1339, 1900, 216, 950, 1352, 1583, 156, -361,
        502, 539, -2, 264, -556, -566, -142, -20, 1094, 515, -517, -334, -163, -602, -1183, -1132,
        -907, 1076, 959, 1097, 871, 35, 75, -221, -254, -1411, -1062, -1181, -1962, -1217, -389,
        -110, 805,
    ],
    [
        643, 587, 212, 46, 709, 558, -4, 534, -318, -543, 125, -1548, -691, -133, -311, -537, -103,
        -407, -357, 438, -1901, -701, 1044, -1427, -892, 424, -551, 594, 480, 94, -221, -312, 887,
        -115, -1888, -1615, -1180, 239, -1674, -2154, 655, 824, 1611, 1212, -1900, -470, 477, -327,
        561, 37, 963, 1054, -895, -171, 411, -758, -717, -460, 769, -381, -12, -24, -76, 964, -918,
        -1405, 1088, 104, -1613, -27, 590, -383, -564, 826, 396, 627, 303, -529, -1145, 778,
    ],
    [
        395, -1298, -579, -303, 63, 256, 100, -899, -2045, -1336, -421, 1258, 111, -515, 302, 1449,
        1864, -318, -85, -174, -1273, -121, 254, -767, -1335, 385, 1132, -1000, -840, 359, -209,
        -103, -480, -2448, -2076, -578, -1255, 133, 23, -1027, 152, 1060, 989, 421, 326, 174, 221,
        -897, -509, 20, 483, 1361, 363, -356, -354, -795, -434, 289, -387, -671, -928, 673, 548,
        -1134, -477, 182, -537, 1724, 1307, -1967, 715, 1109, -24, -854, 313, 120, -479, 242, -660,
        -425,
    ],
    [
        613, 1302, -98, -951, 184, 682, 1207, -76, 522, -309, -1370, 833, 895, 878, 783, 396, -667,
        233, 69, 52, 568, -777, 390, 540, 206, 1231, 830, 432, 1005, -697, -979, -1317, -594,
        -1604, -2087, -776, -420, 1013, 43, -586, -1315, -782, 23, 646, -105, -563, 445, 959, 1575,
        622, 390, -1420, -1125, -245, 962, 367, -250, 1992, 225, -686, -162, 322, 762, 856, 303,
        -17, 380, -729, -673, -429, 297, 1213, 801, -779, -1366, 452, -115, -217, 1765, 1507,
    ],
    [
        393, -199, 114, 719, 1859, 1173, -192, -813, 1168, 868, -492, -511, -1515, -966, 109, 2263,
        1654, -102, -1056, -2049, -355, -592, -1715, -149, 179, -1060, -705, 1170, 304, 1602, 855,
        186, 1083, -586, 1717, -540, -10, 683, -848, -1131, 241, 1580, 1708, 63, -694, -895, -76,
        -269, -1029, 289, -1160, -77, 485, -461, -323, -220, 377, 55, -603, -1660, -1841, 416, 671,
        -613, 311, 1498, -88, -1349, -767, 827, 687, 600, 1008, 2075, 1720, 357, -69, -602, -179,
        744,
    ],
    [
        -338, -657, -1026, -1894, 573, 2083, 937, -227, -773, -1833, -387, 606, 374, 854, 553, 166,
        -961, -838, -1068, -1783, 189, 68, -346, 448, 818, 1183, 0, 96, -900, -3247, -1645, -1250,
        -57, -216, -1632, -827, -496, -670, -455, -323, -1038, -623, -8, -123, -329, 298, -515,
        -59, -867, 504, 19, 19, -117, -974, 1699, 88, -1906, -472, -971, 295, 951, -1168, -682,
        -807, -384, 1337, 193, 1114, -726, -1837, -238, -662, 22, 750, -73, -830, -687, 884, -233,
        -1310,
    ],
    [
        -679, -1017, 450, -349, 223, 2385, 384, 691, 644, -111, 94, -151, 702, 1301, 1515, 494,
        519, 954, 1392, 1363, -632, -248, -106, 483, 1758, 922, -256, -100, -749, -1534, 218, 799,
        546, 24, -1116, -446, 779, 473, -159, 151, 1790, -226, -211, 681, -651, 243, -631, 731,
        849, -525, 1078, 1726, 693, 1134, 2343, 140, 74, 79, -858, 1329, 1054, -758, -1541, -1829,
        -801, 666, 1290, 942, 291, 290, 609, -108, 926, 1080, -144, 128, -727, -584, -393, -697,
    ],
    [
        966, -168, -6, 1034, -91, -1085, 170, 1322, 1459, 335, -671, 532, -538, -1242, 1963, 982,
        -1798, 1064, 1307, -760, -847, -1109, -861, 458, 1519, 798, -1382, -733, -169, -901, -1011,
        -1358, -284, -397, -1049, -1858, -652, -427, -405, 223, -1324, 229, 14, -2206, -830, 653,
        -1299, -1111, 1276, -167, -529, -580, -1085, 838, 1291, 972, 1496, 644, 1640, 2441, 224,
        366, 1447, 337, 850, -267, -1003, 492, -5, -365, 321, 1239, 1603, -264, -1608, 66, 1866,
        1676, 2014, 1033,
    ],
    [
        -465, 433, 457, -1617, -875, 1093, -192, -84, 796, 855, -1153, -847, 412, 100, 131, 787,
        1291, -785, -1193, -177, -26, -385, -1739, 892, 2751, -565, -71, 353, -357, -1643, -2511,
        -804, -289, -322, -523, -877, -302, -1275, 491, 1989, 1341, 1459, 875, 1086, 1458, 972,
        136, 870, 309, -570, 1474, 1887, 1132, -485, -950, 41, -307, -1200, -238, 1066, 573, -129,
        -690, 44, 600, -880, -1443, -1556, -895, -1199, -1019, 184, 1235, 1497, 2379, 1054, -194,
        855, -413, -1007,
    ],
    [
        -164, -451, -681, 68, -8, -1145, -1101, -660, 717, 905, 492, 264, 906, 408, -843, 309,
        1436, 1008, 1511, 1083, 1338, 1781, 2085, 815, -2252, -1439, 374, -256, 473, -935, 177,
        2124, 145, -925, -358, -712, 278, 1119, 738, 112, 664, -1299, -1805, 762, 729, -83, 793,
        689, 565, -133, 207, 476, -228, -7, -343, -232, 886, 275, -1319, -1702, -1301, -329, 494,
        -642, -426, 1206, 168, 8, -247, 737, 570, -629, 99, 656, 144, -398, -21, 1002, -60, -755,
    ],
    [
        -307, -1167, -1003, -1460, -354, 930, -299, 280, 311, 660, -265, -395, 1246, -425, -852,
        -885, -1055, -1853, -1020, 340, -137, 801, 563, 184, -1129, 152, 2378, 478, 675, 1213,
        1669, 719, -422, 205, 705, 76, 224, -198, -759, -158, -189, 767, 957, -335, -432, -189,
        -338, -1109, -1088, -1200, 456, 1238, 865, 1209, 140, 137, -197, -107, -1053, -1336, -911,
        -1837, -1588, -550, -260, 528, 818, 532, 452, -61, 529, 728, 105, 324, 208, -841, 247,
        -1079, -1208, -107,
    ],
    [
        -274, -457, -325, -104, -685, 529, -1098, -1683, 1043, -559, -1571, 385, -341, 104, 769,
        88, 7, -501, 559, 554, -1530, -1773, -1430, -115, -650, -2673, -98, 935, 1175, 1857, -1064,
        -1430, 689, -331, 375, 219, 480, 1401, -335, -736, 589, -480, -743, -698, 504, 142, -422,
        289, -401, 901, -123, -1706, 295, 988, -1503, -613, 579, 75, -840, 521, 600, 490, 530,
        -206, -1152, 12, 1044, -1049, -864, -340, -85, 250, 187, -401, -1870, -1640, -824, 821,
        799, -651,
    ],
    [
        86, 927, 2133, 357, -440, 178, -1122, -616, 364, -367, -1347, -375, 1325, -282, -1272, 173,
        33, -250, -102, -1818, -2607, -798, -1065, -519, 490, -973, -300, 832, 1170, 528, 399, 144,
        345, -545, -829, -25, 185, 1225, 842, -277, -308, -647, -709, 201, -114, -607, -934, -5,
        1055, -794, -866, 165, -386, 649, 2340, -1484, -398, 1438, 309, -688, -239, 558, -605, 194,
        -357, -655, 227, 1259, 552, -978, -193, 535, 1159, 419, 55, 465, -1580, 216, 1018, 863,
    ],
    [
        811, 667, -437, -197, 387, -236, 711, -465, -171, 774, 115, 907, 1196, 295, 1052, -97,
        -1032, -1186, -917, -928, -2384, -1011, -711, 697, 524, -462, -1064, -1152, -153, 445, 384,
        -731, 215, 688, 1587, 899, 560, -779, -1547, -425, 467, 1004, -603, 1512, 1106, -717, -309,
        137, 658, 872, 573, -69, 371, -302, -882, -414, -123, 1082, 1041, -119, -1569, -84, 358,
        -576, 1210, 1847, 739, -109, -25, 424, 1577, 1608, 256, -859, 403, 1451, -334, -354, -1277,
        -1259,
    ],
    [
        -367, 1597, 2293, 291, -621, -1977, -1026, -89, -1661, -752, 353, -635, -1188, 773, 674,
        896, 1682, -234, 1362, -491, 1116, 513, -762, 942, -415, -1209, -347, 1248, 1866, 693,
        -620, -826, -497, 160, -1155, 59, -610, -809, 723, -312, -347, -332, 223, 271, -388, -1228,
        -2118, -361, 961, -302, -277, 1408, 602, -1111, -1184, 399, 890, 521, 833, 1697, 2138, 692,
        71, -452, -484, 586, 179, -788, -624, -1936, -444, 1938, 1482, 17, -497, -1610, -1127, 596,
        331, 765,
    ],
    [
        675, 376, -595, -1051, -734, -1842, -516, 454, -473, 255, 647, 1216, 441, -135, -105,
        -2855, -2333, -1200, -654, 295, -1447, -1135, -532, -570, -649, -172, -904, -816, -223,
        103, -502, 312, -360, -132, -665, -83, 510, -317, 335, -1152, 869, 1243, -1819, -862, -729,
        -428, 1213, -549, -1102, -476, -972, 1145, 502, 764, 307, -2029, -615, -435, -421, 735,
        246, -599, -963, 443, 498, -1323, -742, -1039, -55, 225, -549, 2134, 1139, 259, 992, -75,
        126, -175, 386, 1129,
    ],
    [
        1577, 870, 324, 896, 1147, 1672, -35, -624, 46, -2, 1565, 1343, 58, -277, -257, -1612,
        -405, 802, 628, 299, -785, -949, 571, 640, 87, -261, 1495, 705, -774, 852, -439, -7, -290,
        -30, 1329, -404, 433, 1771, 1082, 639, 2294, 990, -294, 458, -903, 549, 1591, -278, -1315,
        -1854, -1269, 216, 1223, 1118, 513, 146, 651, 69, 404, 1379, 104, 42, -339, -875, -197,
        -893, 536, 439, -453, 949, 412, -939, -451, 1132, 1455, 934, -665, 175, 139, -1507,
    ],
    [
        849, 2107, -1487, -115, 1850, -272, -901, -976, -1095, -32, 1284, 1329, -771, -1288, -18,
        -795, -867, -1419, -621, -228, -775, -1718, -1214, -207, -700, 340, -940, -550, 649, -1740,
        -1628, 535, -472, -1767, 810, 541, -714, -321, -1182, 110, 1456, 890, 1458, 922, 1029,
        2568, 1027, -140, 1434, 631, 625, 383, -1144, 65, 375, -448, 90, 882, 1721, 518, -1461,
        -754, 1527, 1792, 1853, 1655, -230, 14, 758, -925, -1648, 836, 304, -336, 516, 1087, -458,
        -1354, 226, 217,
    ],
    [
        82, 449, 1392, 20, -1437, -397, -68, -73, -1460, -498, 2927, 484, -671, 549, -189, -1018,
        -2585, -1412, -262, -373, -324, -937, -350, -988, -472, 1911, 1485, 1456, 1082, 896, 1417,
        1241, 309, 514, 838, -668, 766, 1995, 1457, 103, -1071, -252, 15, -1041, -755, 791, 893,
        38, -504, -444, 765, -434, -1289, -1661, -1038, -1061, -1188, -325, 1054, 1319, 2182, 1776,
        -121, 553, 282, -1180, -308, -317, -655, -291, 313, -912, -1133, -959, 313, 968, 663, 229,
        696, 791,
    ],
    [
        -563, -357, 1370, 1057, 1383, 1269, 1140, 1676, 2015, 1584, -1359, -2274, 197, -169, 328,
        -363, -703, 1908, 1031, -929, -441, -645, -181, 968, 1008, 150, 585, -391, -2197, -21,
        1099, -45, 505, 819, 604, 140, -98, 611, -86, -93, -158, -447, 564, 752, -883, -1691,
        -1504, -708, 370, -102, -938, 960, 574, -24, -185, 291, 950, -409, -275, 621, 369, -263,
        -312, 787, 511, -810, -306, -891, -1122, -1235, -1072, 908, 35, -28, 406, 483, 299, -824,
        1013, 264,
    ],
    [
        -939, -791, -992, -1548, -1639, 193, -64, 457, 795, 301, -616, -853, 2189, 1139, 355, 1112,
        1544, 1253, -244, -118, 692, 297, 86, 123, -765, -318, -191, 333, 1178, 41, -518, -249,
        -193, -888, -1133, -1215, -305, 1343, 796, 1277, 458, 88, -64, -144, -595, -1499, -872,
        -1533, -1865, -842, -319, 176, 907, 535, 588, 11, 278, 783, 333, 87, 532, -743, -66, -395,
        -1574, -281, -212, -360, -499, 13, -669, 154, -165, -2139, 354, 419, -1795, -123, 95, -370,
    ],
    [
        845, 215, 110, -437, 90, 911, -872, -1922, -1521, -697, 84, -2470, -1167, 962, 878, 1934,
        140, -2018, 352, 21, -9, 473, 122, 1339, 416, -1033, 359, -17, -801, -735, -2, 629, -598,
        302, -345, 459, 590, -1503, -665, 1367, -846, -1234, 376, 426, -744, -28, 817, 369, 658,
        62, -876, -731, 1191, -407, -1172, -408, -220, 185, 235, -68, -1433, -1930, -1086, 165,
        1260, -435, -206, 544, 1933, 1203, -580, 193, -680, -1068, 237, -4, -1116, -1001, 1027,
        535,
    ],
    [
        -1350, -283, 279, -308, -4, -1084, -2742, -1381, -729, -1068, 539, -557, -750, 529, 1191,
        744, 410, 219, 286, -105, -993, -203, 52, 878, 1225, -34, -331, -475, -829, -85, 162, -542,
        -791, -614, 1097, -155, -1127, -93, -43, -210, 2442, -178, -1492, 1363, 689, -358, -700,
        642, -359, -123, 14, -755, -123, 987, 1066, -656, -634, 410, 918, 943, -222, 760, -1132,
        -650, 1065, 845, 856, 768, -10, -568, 469, -210, 466, 98, -651, 746, 262, 509, 1351, 468,
    ],
];