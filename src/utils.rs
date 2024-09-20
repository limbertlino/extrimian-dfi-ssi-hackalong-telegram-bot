use image::Luma;
use qrcode::QrCode;

pub fn generate_qr(data: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let code = QrCode::new(data)?;

    let image = code.render::<Luma<u8>>().build();

    image.save(path)?;
    Ok(())
}
