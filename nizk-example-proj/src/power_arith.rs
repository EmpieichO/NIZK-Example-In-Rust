pub fn power_function(a: u32, r: u32, p: u32) -> u32 {
    let a_to_5 = a.pow(5) % p;
    let a_5_sqr = (a_to_5 * a_to_5) % p; // a^10
    let a_5_cube = (a_5_sqr * a_to_5) % p; // a^15
    let a_5_quad = (a_5_sqr * a_5_sqr) % p; // a^20

    let power_res;

    if r >= 80 && r < 120 {
        let a_5_octo = (a_5_quad * a_5_quad) % p; // a^40
        let a_5_ocho = (a_5_octo * a_5_octo) % p; // a^80
        if (r - 80) < 5 {
            power_res = (a_5_ocho * a.pow(r - 80)) % p;
        }
        else if (r - 80) >= 5 && (r - 80) < 10 {
            let a_to_85 = (a_5_ocho * a_to_5) % p;
            power_res = (a_to_85 * a.pow(r - 85)) % p;
        } 
        else if (r - 80) >= 10 && (r - 80) < 15 {
            let a_to_90 = (a_5_ocho * a_5_sqr) % p;
            power_res = (a_to_90 * a.pow(r - 90)) % p;
        }
        else if (r - 80) >= 15 && (r - 80) < 20 {
            let a_to_95 = (a_5_ocho * a_5_cube) % p;
            power_res = (a_to_95 * a.pow(r - 95)) % p;
        }
        else if (r - 80) >= 20 && (r - 80) < 25 {
            let a_to_100 = (a_5_ocho * a_5_quad) % p;
            power_res = (a_to_100 * a.pow(r - 100)) % p;
        } else if (r - 80) >= 25 && (r - 80) < 30 {
            let a_to_100 = (a_5_ocho * a_5_quad) % p;
            let a_to_105 = (a_to_100 * a_to_5) % p;
            power_res = (a_to_105 * a.pow(r - 105)) % p;
        } else if (r - 80) >= 30 && (r - 80) < 35 {
            let a_to_100 = (a_5_ocho * a_5_quad) % p;
            let a_to_110 = (a_to_100 * a_5_sqr) % p;
            power_res = (a_to_110 * a.pow(r - 110)) % p;
        } else {
            let a_to_100 = (a_5_ocho * a_5_quad) % p;
            let a_to_115 = (a_to_100 * a_5_cube) % p;
            power_res = (a_to_115 * a.pow(r - 115)) % p;
        }
    } else if r >= 40 && r < 80 {
        let a_5_octo = (a_5_quad * a_5_quad) % p; // a^40
        if (r - 40) < 5 {
            power_res = (a_5_octo * a.pow(r - 40)) % p;
        } else if (r - 40) >= 5 && (r - 40) < 10 {
            let a_to_45 = (a_5_octo * a_to_5) % p;
            power_res = (a_to_45 * a.pow(r - 45)) % p;
        } else if (r - 40) >= 10 && (r - 40) < 15 {
            let a_to_50 = (a_5_octo * a_5_sqr) % p;
            power_res = (a_to_50 * a.pow(r - 50)) % p;
        } else if (r - 40) >= 15 && (r - 40) < 20 {
            let a_to_55 = (a_5_octo * a_5_cube) % p;
            power_res = (a_to_55 * a.pow(r - 55)) % p;
        } else if (r - 40) >= 20 && (r - 40) < 25 {
            let a_to_60 = (a_5_octo * a_5_quad) % p;
            power_res = (a_to_60 * a.pow(r - 60)) % p;
        } else if (r - 40) >= 25 && (r - 40) < 30 {
            let a_to_60 = (a_5_octo * a_5_quad) % p;
            let a_to_65 = (a_to_60 * a_to_5) % p;
            power_res = (a_to_65 * a.pow(r - 65)) % p;
        } else if (r - 40) >= 30 && (r - 40) < 35 {
            let a_to_60 = (a_5_octo * a_5_quad) % p;
            let a_to_70 = (a_to_60 * a_5_sqr) % p;
            power_res = (a_to_70 * a.pow(r - 70)) % p;
        } else {
            let a_to_60 = (a_5_octo * a_5_quad) % p;
            let a_to_75 = (a_to_60 * a_5_cube) % p;
            power_res = (a_to_75 * a.pow(r - 75)) % p;
        }
    } else if r >= 35 && r < 40 {
        let a_to_35 = (a_5_quad * a_5_cube) % p;
        power_res = (a_to_35 * a.pow(r - 35)) % p;
    } else if r >= 30 && r < 35 {
        let a_to_30 = (a_5_quad * a_5_sqr) % p;
        power_res = (a_to_30 * a.pow(r - 30)) % p;
    } else if r >= 25 && r < 30 {
        let a_to_25 = (a_5_quad * a_to_5) % p;
        power_res = (a_to_25 * a.pow(r - 25)) % p;
    } else if r >= 20 && r < 25 {
        power_res = (a_5_quad * a.pow(r - 20)) % p;
    } else if r >= 15 && r < 20 {
        power_res = (a_5_cube * a.pow(r - 15)) % p;
    } else if r >= 10 && r < 15 {
        power_res = (a_5_sqr * a.pow(r - 10)) % p;
    } else if r >= 5 && r < 10 {
        power_res = (a_to_5 * a.pow(r - 5)) % p;
    } else {
        power_res = a.pow(r) % p;
    }

    return power_res; 
} 
