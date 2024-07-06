use std::io::Read;

struct RotDecoder<R: Read> {
    input: R,
    rot: u8,
}

// Implement the `Read` trait for `RotDecoder`.
impl<R: Read> Read for RotDecoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let r_bytes = self.input.read(buf)?;
        for idx in 0..r_bytes {
            let c = buf[idx];
            if !c.is_ascii_alphabetic() {
                continue;
            }
            if c > 97 {
                buf[idx] = ((c - 97) + self.rot) % 26 + 97;
            } else {
                buf[idx] = ((c - 65) + self.rot) % 26 + 65;
            }
        }
        return Result::Ok(r_bytes);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joke() {
        let mut rot = RotDecoder {
            input: "Gb trg gb gur bgure fvqr!".as_bytes(),
            rot: 13,
        };
        let mut ret = String::new();

        rot.read_to_string(&mut ret).unwrap();

        assert_eq!(&ret, "To get to the other side!");
    }

    #[test]
    fn binary() {
        let input: Vec<u8> = (0..=255u8).collect();
        let mut rot = RotDecoder::<&[u8]> {
            input: input.as_ref(),
            rot: 13,
        };
        let mut buf = [0u8; 256];

        assert_eq!(rot.read(&mut buf).unwrap(), 256);

        for i in 0..=255 {
            if input[i] != buf[i] {
                assert!(input[i].is_ascii_alphabetic());
                assert!(buf[i].is_ascii_alphabetic());
            }
        }
    }
}
