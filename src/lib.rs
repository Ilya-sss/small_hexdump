use pyo3::prelude::*;

#[pymodule]
mod _core {
    use pyo3::prelude::*;

    /// Hex bytes only, one chunk per line.
    #[pyfunction]
    fn hexdump_plain(data: &[u8], k: usize) -> String {
        let mut out = String::new();
        for chunk in data.chunks(k) {
            let hex_part: Vec<String> = chunk.iter().map(|b| format!("{:02x}", b)).collect();
            out.push_str(&hex_part.join(" "));
            out.push('\n');
        }
        out
    }

    /// Offset + hex + ASCII column, like `hexdump -C`.
    #[pyfunction]
    fn hexdump_canonical(data: &[u8], k: usize) -> String {
        let mut out = String::new();
        let width = k * 3 - 1;

        for (n, chunk) in data.chunks(k).enumerate() {
            let offset = n * k;

            let hex_part: Vec<String> = chunk.iter().map(|b| format!("{:02x}", b)).collect();
            let hex_part = hex_part.join(" ");

            let ascii_part: String = chunk
                .iter()
                .map(|&b| {
                    if (32..127).contains(&b) {
                        b as char
                    } else {
                        '.'
                    }
                })
                .collect();

            out.push_str(&format!(
                "{:08x}  {:<width$}  |{}|\n",
                offset, hex_part, ascii_part
            ));
        }
        out
    }
}
