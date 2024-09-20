use std::path::Path;
use crate::texture;


pub async fn load_string(out_dir: &mut Path, file_name: &str) -> anyhow::Result<String> {
    let path = out_dir.join("res").join(file_name);
    let txt = std::fs::read_to_string(path)?;

    Ok(txt)
}

pub async fn load_binary(out_dir: &mut Path, file_name: &str) -> anyhow::Result<Vec<u8>> {
    let path = out_dir.join("res").join(file_name);
    let data = std::fs::read(path)?;

    Ok(data)
}

pub async fn load_texture(
    out_dir: &mut Path,
    file_name: &str,
    is_normal_map: bool,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
) -> anyhow::Result<texture::Texture> {
    let data = load_binary(out_dir, file_name).await?;
    texture::Texture::from_bytes(device, queue, &data, file_name, is_normal_map)
}
