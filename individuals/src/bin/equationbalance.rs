// Pb(NO3)

fn main() {
    let pb = 207.;
    let n = 14.007;
    let o = 15.999;
    let h = 1.008;
    let cl = 35.45;

    let no3_group = n + 3. * o;
    let pbno32 = pb + 2. * no3_group;
    let hcl = h + cl;
    let hno3 = h + no3_group;

    let no3_ratio = 1. - (pb / pbno32);
    let mass_no3 = 15. * no3_ratio;

    let h_per_no3 = h / hno3;

    let mass_hno3 = mass_no3 + (h_per_no3 * h);

    let hcl_per_lead_nitrate = (2. * hcl) / pbno32;

    let hcl_used = hcl_per_lead_nitrate * 15.;

    let hcl_remaining = 15. - hcl_used;

    println!("{mass_hno3} {hcl_remaining}");
}
