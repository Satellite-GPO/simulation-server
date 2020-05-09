use std::f64::consts::PI;

pub fn integration_factors_etc(doy: u32, latitude: f64) -> ([f64; 4], [f64; 2]) {
    let declination = ( -23.44 * (2.0 * PI * (doy as f64 + 10.0) / 365.25).cos() )
        .to_radians();
    let s = latitude.sin() * declination.sin(); // what do they mean?
    let c = latitude.cos() * declination.cos();
    let mut aob = s / c;
    let day_length = if aob < -0.9999 {
        aob = -0.9999;
        0.0
    } else if aob > 0.9999 {
        aob = 0.9999;
        24.0
    } else {
        12.0 * (1.0 + 2.0 * aob.asin() / PI)
    };

    let mut ds0 = 0.0;
    let factors = if day_length <= 0.001 {
        [0.0, 0.0, 0.0, 0.001]
    } else {
        let dsinb = 3600.0 * (day_length * s + 24.0 * c * (1.0 - aob * aob).sqrt() / PI);
        let sc = 1350.0 * (1.0 + 0.033 * (2.0 * PI * doy as f64 / 365.25).cos());
        ds0 = sc * dsinb / 1e6;

        let altitude = (s + c).asin().max(0.0);
        let alt_noon = PI / 2.0 - altitude;

        let sunrise = 12.0 - day_length / 2.0;
        let sunset = 12.0 + day_length / 2.0;

        let step:f64 = 5.0 / 60.0;
        let mut n = 0;
        let mut sum0 = 0_f64;
        let mut sum1 = 0_f64;
        let mut sum2 = 0_f64;

        let mut time = sunrise + step;
        while time < sunset {
            let hour_angle = 2.0 * PI * (time - 12.0) / 24.0;
            let sin_altitude = s + c * hour_angle.cos();

            if sin_altitude > 0.0 {
                let alt = PI / 2.0 - sin_altitude.asin();
                let p0 = (PI * (alt - alt_noon) / (PI / 2.0 - alt_noon) / 2.0).cos();

                sum0 += sin_altitude;
                sum1 += p0 * sin_altitude / alt_noon.cos();
                sum2 += p0;
                n += 1;
            }

            time += step;
        }

        [
            alt_noon.min( (89 as f64).to_radians() ),
            1.0 / (sum1 * step / day_length),
            1.0 / (sum2 * step / day_length),
            sc * sum0 / n as f64 + 0.001
        ]
    };

    (factors, [day_length, ds0])
}

// TODO: irradiance for whole year depends on 2 temperatures because
// we think how to implement temperature properly
pub fn ground_radiation_for_year(latitude: f64, t_min: f64, t_max: f64) -> Vec<f64> {
    let daily_t_min: Vec<_> = (0..366).map(|_| t_min).collect();
    let daily_t_max: Vec<_> = (0..366).map(|_| t_max).collect();

    let mut s_factors = Vec::with_capacity(366);
    let mut day_length = Vec::with_capacity(366);
    let mut rad_above = Vec::with_capacity(366);
    let mut daily_vap = Vec::with_capacity(366);
    let mut daily_solar_rad = Vec::with_capacity(366);

    for doy in 0..366 {
        let (factors, mid) = integration_factors_etc(doy, latitude);
        s_factors.push(factors);
        day_length.push(mid[0]);
        rad_above.push(mid[1]);

        daily_vap.push(
            0.92 * 6.11 * (17.27 * t_min / (t_min + 237.3)).exp() // OR.CO.: 0.92: parameter may differ with time and location
        );

        let daily_dtair = daily_t_max[doy as usize]
            - daily_t_min[doy as usize];

        let f = if daily_dtair > 0.5 { daily_dtair } else { 0.5 };

        let p = 1.06
            * ( 1.0 - ( -0.06 * f.powf(1.25) ).exp() )
            * ( 1.0 - 0.02 * daily_vap[doy as usize] );
        daily_solar_rad.push(p * rad_above[doy as usize]); // OR.CO.: estimated daily solar radiation on the ground (MJ/m2/day)
    }

    daily_solar_rad
}