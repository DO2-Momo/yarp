use crate::install::Device;

///
/// Convert bytes to megabytes
///
pub fn to_mb(size: u128) -> u128 {
    return size / 1000000;
}

///
/// Calculate partition sizes
/// TODO: Refactor
///
pub fn calculate_partitions(
    device: &Device,
    swap: u128,
    root_home_ratio: f64,
    has_home:bool,
    is_legacy: bool) -> Vec<u128>
{
    let mut sizes = Vec::<u128>::new();
    let size = to_mb(device.size);
    let efi: u128 = 105;

    sizes.push(if is_legacy { 4000 } else { 0 });
    sizes.push(efi + sizes[sizes.len()-1]);
    sizes.push(swap + sizes[sizes.len()-1]);

    if !has_home {
        sizes.push(size);
        return sizes;
    }

    let rest = size - sizes[sizes.len()-1];

    sizes.push((root_home_ratio * rest as f64) as u128 + sizes[sizes.len()-1]);
    sizes.push(size);

    return sizes;
}
