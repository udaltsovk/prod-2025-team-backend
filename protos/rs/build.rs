use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let protos_dir = "../";
    let protos_src_dir = format!("{protos_dir}/src");
    let protos_rs_dir = format!("{protos_dir}/rs/src");

    tonic_build::configure()
        .out_dir(protos_rs_dir)
        .compile_protos(
            &[
                format!("{protos_src_dir}/admin.proto"),
                format!("{protos_src_dir}/client.proto"),
                format!("{protos_src_dir}/coworking.proto"),
                format!("{protos_src_dir}/image.proto"),
                format!("{protos_src_dir}/mail.proto"),
                format!("{protos_src_dir}/notification.proto"),
                format!("{protos_src_dir}/reservation.proto"),
                format!("{protos_src_dir}/seat-lock.proto"),
            ],
            &[protos_src_dir],
        )?;

    Ok(())
}
