use pyo3::prelude::*;

#[pymodule]
mod _core {
    use pyo3::prelude::*;

    /// Hex bytes only, one chunk per line.
    #[pyfunction]
    fn hexdump_plain(data: &[u8], k: usize) -> PyResult<String> {
        if k == 0 {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "bytes per line must be at least 1",
            ));
        }

        let mut out = String::new();

        for (n, chunk) in data.chunks(k).enumerate() {
            let offset = n * k;
            let hex_part = chunk
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<Vec<_>>()
                .join(" ");

            out.push_str(&format!("{:08x}  {}\n", offset, hex_part));
        }

        Ok(out)
    }

    /// Offset + hex + ASCII column, like `hexdump -C`.
    #[pyfunction]
    fn hexdump_canonical(data: &[u8], k: usize) -> PyResult<String> {
        if k == 0 {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "bytes per line must be at least 1",
            ));
        }

        let mut out = String::new();
        let width = k * 3 - 1 + usize::from(k > 8);

        for (n, chunk) in data.chunks(k).enumerate() {
            let offset = n * k;
            let mut hex_part = String::new();

            for (i, b) in chunk.iter().enumerate() {
                if i > 0 {
                    hex_part.push(' ');
                    if i == 8 {
                        hex_part.push(' ');
                    }
                }
                hex_part.push_str(&format!("{:02x}", b));
            }

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

        Ok(out)
    }

    #[pyfunction]
    fn hexdump_octal(data: &[u8], k: usize) -> PyResult<String> {
        if k == 0 {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "bytes per line must be at least 1",
            ));
        }

        let mut out = String::new();

        for (n, chunk) in data.chunks(k).enumerate() {
            let offset = n * k;
            let oct_part = chunk
                .iter()
                .map(|b| format!("{:03o}", b))
                .collect::<Vec<_>>()
                .join(" ");

            out.push_str(&format!("{:07o} {}\n", offset, oct_part));
        }

        out.push_str(&format!("{:07o}\n", data.len()));
        Ok(out)
    }
}
