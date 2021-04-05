//! Design sketch in Finnish:
//!
//! Algoritmi, joka etsii pidemmästä äänenpätkästä A sen kohdan, jossa
//! lyhyempi äänenpätkä B esiintyy kaikista lähimpänä.
//!
//! Käyttötarkoituksena on oskilloskoopin näkymän vakautus. Silloin algoritmille
//! annettaisiin pätkä A uutta signaalia, ja pätkä B, joka vastaa viimeksi näytettyä
//! kuvaajaa. Algoritmi etsii uudesta signaalista sellaisen kohdan, jonka näyttämällä
//! kuvaaja muuttuu mahdollisimman vähän. Vakautettua kuvaajaa on toivottavasti helpompi
//! seurata, koska se ei liiku jatkuvasti taajuudesta riippuvalla tavalla.
//!
//! Olkoon signaalit `A[0..n]` ja `B[0..m]`, `n >= 2m`.
//! Algoritmi etsii sellaisen aikasiirroksen t, jolla summa x:n yli
//!
//! `w(x) * (A[x+t] - B[x])^2`
//!
//! on minimaalinen. Tässä `w(x)` on painofunktio, jonka avulla voidaan esimerkiksi
//! painottaa oskilloskoopin näkymän keskikohtia enemmän kuin reunoja.
//!
//! Jos tämä summa esitetään muodossa
//!
//! `w(x) * A[x+t]^2 - 2(w(x) * B[x]) * A[x+t] + w(x) * B[x]^2`,
//!
//! nähdään, että se voidaan laskea kahtena ristikorrelaationa (summat x:n yli muotoa
//! `f(x) * g(x+t)`) ja yhtenä suorana tulona (summa x:n yli muotoa `f(x) * g(x)`).

use crate::fft;

pub struct Prepared {
    base_size: usize,
    fft: fft::Prepared,

}

impl Prepared {
    pub fn new(max_size: usize) -> Self {
        let base_size = max_size.next_power_of_two();
        let fft_size = base_size * 2;
        Prepared {
            base_size,
            fft: fft::Prepared::new(fft_size),
        }
    }
}
