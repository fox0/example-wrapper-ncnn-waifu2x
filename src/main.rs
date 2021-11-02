#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(clippy::redundant_static_lifetimes)]
#[allow(clippy::upper_case_acronyms)]
mod bindings;
mod ncnn;

fn main() {
    println!("{}", ncnn::version());

    let mut net = ncnn::Net::new();
    net.load_param("noise0_scale2.0x_model.param").unwrap();
    net.load_model("noise0_scale2.0x_model.bin").unwrap();
    //todo
}
