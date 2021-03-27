//! Tässä moduulissa toteutetaan FFT eli Fast Fourier Transform, sekä sen
//! käänteismuunnos.

use crate::math::*;

/// Etukäteen alustettava rakenne, joka sisältää twiddle-kertoimet tietylle
/// muunnoksen koolle. Toistaiseksi tähän ei tallenneta mitään, vaan kaikki
/// lasketaan varsinaisessa rutiinissa.
pub struct Prepared {
    size: usize,
}

impl Prepared {
    /// Valmistele FFT. Koon on oltava kahden potenssi.
    pub fn new(size: usize) -> Self {
        assert!(size.count_ones() == 1);
        Prepared { size }
    }

    /// Suorita muunnos. Taulukon koon on oltava sama, kuin millä tämä instanssi
    /// valmisteltiin.
    pub fn fft(&self, array: &mut [Complex]) {
        assert!(array.len() == self.size);
        // Lomitus. Indeksit vaihdetaan siten, että niiden binääriesitys käännetään.
        for index in 0..self.size {
            let reversed = index.reverse_bits() >> (self.size.leading_zeros() + 1);
            if reversed > index {
                array.swap(index, reversed);
            }
        }
        // "Perhoskuvio", eli varsinainen lasku. Suoritetaan jokaiselle koolle
        // 2, 4, 8 ... kasvavassa järjestyksessä.
        for half_width in (0..).map(|e| 1 << e).take_while(|w| *w < self.size) {
            for pos in (0..self.size).step_by(2 * half_width) {
                for i in 0..half_width {
                    let l = array[pos + i];
                    let r = array[pos + half_width + i];
                    let r = r * Complex::euler(-(i as Num) * PI / half_width as Num);
                    array[pos + i] = l + r;
                    array[pos + half_width + i] = l - r;
                }
            }
        }
    }

    /// Suorita käänteismuunnos. Taulukon koon on oltava sama, kuin millä tämä
    /// instanssi valmisteltiin.
    pub fn ifft(&self, array: &mut [Complex]) {
        assert!(array.len() == self.size);
        self.fft(array);
        // Käänteismuunnos on muuten sama, paitsi lopputuloksen indeksit
        // vaihdetaan niiden vasataluvuiksi modulo koko, eli käytännössä
        // taulukko käännetään nollan jälkeen ympäri.
        for index in 1..(self.size / 2) {
            array.swap(index, self.size - index);
        }
        // ...ja lopuksi tulos kerrotaan normalisointikertoimella että käänteismuunnos
        // tosiaankin toimii käänteismuunnoksena.
        for z in array.iter_mut() {
            *z = *z / self.size as Num;
        }
    }
}
