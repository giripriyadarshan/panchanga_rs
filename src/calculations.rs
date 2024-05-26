use libm::atan2;
use std::f64::consts::PI;

const D2R: f64 = PI / 180_f64;
const R2D: f64 = 180_f64 / PI;

pub(super) fn rev(x: f64) -> f64 {
    (x) - ((x) / 360.0).floor() * 360.0
}

pub(super) fn calculate_ayanamasa(d: f64) -> f64 {
    let t: f64 = (d + 36523.5) / 36525_f64;
    let o: f64 = 259.183275 - 1934.142008333206 * t + 0.0020777778 * t * t;
    let l: f64 = 279.696678 + 36000.76892 * t + 0.0003025 * t * t;
    let ayana: f64 =
        17.23 * ((o) * D2R).sin() + 1.27 * ((l * 2_f64) * D2R).sin() - (5025.64 + 1.11 * t) * t;

    (ayana - 80861.27) / 3600.0
}

pub(super) struct Longitudes {
    pub(super) sun: f64,
    pub(super) moon: f64,
}

impl Longitudes {
    pub(super) fn get(d: f64) -> Self {
        // sun longitude calculations
        let sun_w = 282.9404 + 4.70935e-5 * d;
        let sun_a = 1.00000;
        let sun_e = 0.016709 - 1.151e-9 * d;

        let sun_cap_m: f64 = rev(356.0470 + 0.9856002585 * d);
        let global_ms = sun_cap_m;
        let global_ls = sun_w + sun_cap_m;

        let mut tmp = sun_cap_m * D2R;

        let sun_cap_e = sun_cap_m + R2D * sun_e * tmp.sin() * (sun_a + sun_e * tmp.cos());
        tmp = sun_cap_e * D2R;

        let sun_x = tmp.cos() - sun_e;
        let sun_y = tmp.sin() * (sun_a - sun_e * sun_e).sqrt();

        let sun_v = rev(R2D * atan2(sun_y, sun_x));

        // moon long calculations
        let moon_cap_n = 125.1228 - 0.0529538083 * d;
        let moon_i = 5.1454;
        let moon_w = rev(318.0634 + 0.1643573223 * d);
        let moon_a = 60.2666;
        let moon_e = 0.054900;
        let moon_cap_m = rev(115.3654 + 13.0649929509 * d);
        let global_mm = moon_cap_m;
        let global_lm = moon_cap_n + moon_w + moon_cap_m;

        tmp = moon_cap_m * D2R;

        let mut moon_cap_e = moon_cap_m + R2D * moon_e * tmp.sin() * (1_f64 + moon_e * tmp.cos());
        tmp = moon_cap_e * D2R;

        let mut moon_et = moon_cap_e
            - (moon_cap_e - R2D * moon_e * tmp.sin() - moon_cap_m) / (1_f64 - moon_e * tmp.cos());
        loop {
            moon_cap_e = moon_et;
            tmp = moon_cap_e * D2R;
            moon_et = moon_cap_e
                - (moon_cap_e - R2D * moon_e * tmp.sin() - moon_cap_m)
                    / (1_f64 - moon_e * tmp.cos());
            if moon_cap_e - moon_et < 0.005 {
                break;
            }
        }

        tmp = moon_cap_e * D2R;
        let moon_x = moon_a * (tmp.cos() - moon_e);
        let moon_y = moon_a * (1_f64 - moon_e * moon_e).sqrt() * tmp.sin();

        let moon_r = (moon_x * moon_x + moon_y * moon_y).sqrt();
        let moon_v = rev(R2D * atan2(moon_y, moon_x));

        tmp = D2R * moon_cap_n;
        let tmp1 = D2R * (moon_v + moon_w);
        let tmp2 = D2R * moon_i;
        let moon_xec = moon_r * (tmp.cos() * tmp1.cos() - tmp.sin() * tmp1.sin() * tmp2.cos());
        let moon_yec = moon_r * (tmp.sin() * tmp1.cos() + tmp.cos() * tmp1.sin() * tmp2.cos());
        // let moon_zec = moon_r * tmp1.sin() * tmp2.sin();

        let moon_cap_d = global_lm - global_ls;
        let moon_cap_f = global_lm - moon_cap_n;

        let mut moon_lon = R2D * atan2(moon_yec, moon_xec);

        moon_lon += -1.274 * ((global_mm - 2_f64 * moon_cap_d) * D2R).sin();
        moon_lon += 0.658 * ((2_f64 * moon_cap_d) * D2R).sin();
        moon_lon += -0.186 * ((global_ms) * D2R).sin();
        moon_lon += -0.059 * ((2_f64 * global_mm - 2_f64 * moon_cap_d) * D2R).sin();
        moon_lon += -0.057 * ((global_ms - 2_f64 * moon_cap_d + global_ms) * D2R).sin();
        moon_lon += 0.053 * ((global_mm + 2_f64 * moon_cap_d) * D2R).sin();
        moon_lon += 0.046 * ((2_f64 * moon_cap_d - global_ms) * D2R).sin();
        moon_lon += 0.041 * ((global_mm - global_ms) * D2R).sin();
        moon_lon += -0.035 * ((moon_cap_d) * D2R).sin();
        moon_lon += -0.031 * ((global_mm + global_ms) * D2R).sin();
        moon_lon += -0.015 * ((2_f64 * moon_cap_f - 2_f64 * moon_cap_d) * D2R).sin();
        moon_lon += 0.011 * ((global_mm - 4_f64 * moon_cap_d) * D2R).sin();

        Longitudes {
            sun: rev(sun_v + sun_w),
            moon: rev(moon_lon),
        }
    }
}
