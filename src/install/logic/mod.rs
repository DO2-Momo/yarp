use crate::install::Device;

///
/// Convert bytes to megabytes
///
pub fn to_mb(size: u128) -> u64 {
    return (size as u64) / 1000000; 
}

///
/// Calculate partition sizes
/// TODO: Refactor
///
pub fn calculate_partitions(
    device: &Device,
    swap: u64,
    root: f32,
    home: f32,
    has_home:bool) -> Vec<u64>
{
    let mut sizes = Vec::<u64>::new();
    let size: u64 = to_mb(device.size);
    let efi: u64 = 100;

    sizes.push(0);
    sizes.push(efi);
    sizes.push(swap + sizes[sizes.len()-1]);

    if !has_home {
        sizes.push(size - (swap + efi));
        return sizes;
    }

    sizes.push((root * (size - (swap + efi)) as f32) as u64 + sizes[sizes.len()-1]);
    sizes.push((home * (size - (swap + efi)) as f32) as u64 + sizes[sizes.len()-1]);

    return sizes;
}
