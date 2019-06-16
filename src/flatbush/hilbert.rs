use crate::primitives::{Coordinate, Envelope, Rect};

pub struct Hilbert<C: Coordinate> {
    x: C,
    y: C,
    x_scale: C,
    y_scale: C,
}

impl<C> Hilbert<C>
where
    C: Coordinate,
{
    pub fn new(rect: Rect<C>) -> Self {
        let hilbert_max = C::from((1 << 16) - 1).unwrap();
        let delta = rect.max - rect.min;
        Hilbert {
            x: rect.min.x,
            y: rect.min.y,
            x_scale: hilbert_max / delta.x,
            y_scale: hilbert_max / delta.y,
        }
    }

    /**
     * Place empty envelopes at the end.
     */
    pub fn safe_hilbert(&self, env: Envelope<C>) -> u32 {
        match env.rect {
            None => u32::max_value(),
            Some(r) => self.hilbert(r),
        }
    }

    pub fn hilbert(&self, rect: Rect<C>) -> u32 {
        let x = self.x_scale * (rect.min.x - self.x);
        let y = self.y_scale * (rect.min.y - self.y);
        Self::hilbert_normalized(x.floor().to_u32().unwrap(), y.floor().to_u32().unwrap())
    }

    // Fast Hilbert curve algorithm by http://threadlocalmutex.com/
    // Ported from C++ https://github.com/rawrunprotected/hilbert_curves (public domain)
    #[allow(non_snake_case)]
    pub fn hilbert_normalized(x: u32, y: u32) -> u32 {
        let mut a = x ^ y;
        let mut b = 0xFFFF ^ a;
        let mut c = 0xFFFF ^ (x | y);
        let mut d = x & (y ^ 0xFFFF);

        let mut A = a | (b >> 1);
        let mut B = (a >> 1) ^ a;
        let mut C = ((c >> 1) ^ (b & (d >> 1))) ^ c;
        let mut D = ((a & (c >> 1)) ^ (d >> 1)) ^ d;

        a = A;
        b = B;
        c = C;
        d = D;
        A = (a & (a >> 2)) ^ (b & (b >> 2));
        B = (a & (b >> 2)) ^ (b & ((a ^ b) >> 2));
        C ^= (a & (c >> 2)) ^ (b & (d >> 2));
        D ^= (b & (c >> 2)) ^ ((a ^ b) & (d >> 2));

        a = A;
        b = B;
        c = C;
        d = D;
        A = (a & (a >> 4)) ^ (b & (b >> 4));
        B = (a & (b >> 4)) ^ (b & ((a ^ b) >> 4));
        C ^= (a & (c >> 4)) ^ (b & (d >> 4));
        D ^= (b & (c >> 4)) ^ ((a ^ b) & (d >> 4));

        a = A;
        b = B;
        c = C;
        d = D;
        C ^= (a & (c >> 8)) ^ (b & (d >> 8));
        D ^= (b & (c >> 8)) ^ ((a ^ b) & (d >> 8));

        a = C ^ (C >> 1);
        b = D ^ (D >> 1);

        let mut i0 = x ^ y;
        let mut i1 = b | (0xFFFF ^ (i0 | a));

        i0 = (i0 | (i0 << 8)) & 0x00FF00FF;
        i0 = (i0 | (i0 << 4)) & 0x0F0F0F0F;
        i0 = (i0 | (i0 << 2)) & 0x33333333;
        i0 = (i0 | (i0 << 1)) & 0x55555555;

        i1 = (i1 | (i1 << 8)) & 0x00FF00FF;
        i1 = (i1 | (i1 << 4)) & 0x0F0F0F0F;
        i1 = (i1 | (i1 << 2)) & 0x33333333;
        i1 = (i1 | (i1 << 1)) & 0x55555555;

        return ((i1 << 1) | i0) >> 0;
    }
}

#[cfg(test)]
mod tests {
    use super::Hilbert;
    use super::Rect;

    #[test]
    fn normalized() {
        let h = Hilbert::<f32>::hilbert_normalized(12345, 67890);
        assert_eq!(h, 99289669);
    }

    #[test]
    fn hilbert() {
        let total_rect = Rect::from(((1., 2.), (2., 8.)));
        let query_rect = Rect::from(((1.25, 5.), (2., 8.)));
        let h = Hilbert::new(total_rect);
        let result = h.hilbert(query_rect);
        // x = floor(0.25 * 65535) y = floor(0.5 * 65535)
        // or hilbert_normalized(16383, 32767)
        assert_eq!(result, 805306368);
    }
}
