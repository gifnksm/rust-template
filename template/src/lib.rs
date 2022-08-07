{%- if crate_type == "lib" -%}
//! {{ project-description }}
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! {{ project-name }} = "0.0.0"
//! ```

#![doc(html_root_url = "https://docs.rs/{{project-name}}/0.0.0")]

{% endif -%}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
