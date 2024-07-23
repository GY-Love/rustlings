trait Licensed {
    fn licensing_info(&self) -> String {
        "Default license".to_string()
    }
}

struct SomeSoftware;
struct OtherSoftware;

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

fn compare_license_types(software1: &impl Licensed, software2: &impl Licensed) -> bool {
    software1.licensing_info() == software2.licensing_info()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        let some_software = SomeSoftware;
        let other_software = OtherSoftware;
        assert!(compare_license_types(&some_software, &other_software));
    }

    #[test]
    fn compare_license_information_backwards() {
        let some_software = SomeSoftware;
        let other_software = OtherSoftware;
        assert!(compare_license_types(&other_software, &some_software));
    }
}
