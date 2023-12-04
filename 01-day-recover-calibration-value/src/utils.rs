pub fn replace_in_index(str: &str, index: usize, to: &str) -> String {
    let start_slice = str[0..index].to_string();
    let end_slice = &str[index + to.len()..];

    start_slice + to + end_slice
}

pub fn find_apperance<'a>(
    str: &'a str,
    targets: &[&'a str],
    first: bool,
) -> Option<(&'a str, usize)> {
    let mut appearance: Option<(&str, usize)> = None;

    for &target in targets {
        let appearance_index = if first {
            str.find(target)
        } else {
            str.rfind(target)
        };

        if appearance_index.is_none() {
            continue;
        }

        let key_index = appearance_index.unwrap();
        let is_there_appearance = appearance.is_some();
        let is_minor_index = is_there_appearance && key_index < appearance.unwrap().1;

        if !is_there_appearance || first && is_minor_index || !first && !is_minor_index {
            appearance = Some((target, key_index));
            continue;
        }
    }

    appearance
}
