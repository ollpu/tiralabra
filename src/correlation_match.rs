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

