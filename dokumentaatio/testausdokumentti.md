# Äänen taajuuden seuraus

### Suorituskykytestaus

Suorituskykytestaus tehdään käyttäen Criterion-kirjastoa. Se suorittaa määriteltyä
testiä useaan kertaan ja tekee jonkin verran tilastollista analyysiä.

Kaikki suorituskykytestit voidaan ajaa siirtymällä kansioon `suorituskyky`, ja ajamalla

```
cargo bench
```

Tulokset kootaan kansioon `suorituskyky/target/criterion/`.

#### [FFT](/suorituskyky/benches/fft_performance.rs)

FFT:n toteutuksesta puuttuu vielä optimointeja. Lisään tähän vertailuja myöhemmin.

Testi suoritetaan kokoamalla ensin satunnaisia kompleksilukuja taulukkoon
(molemmat komponentit välillä `[-1, 1]`) ja toistamalla sitten Fourier-muunnosta
tälle taulukolle.

Ennen jokaista toistoa on kopioitava taulukko uudestaan, koska FFT-toteutus ylikirjoittaa
taulukon. Tämä vaikuttaa hieman mitattuun suoritusaikaan. Pyrin laatimaan myöhemmin toisen testin,
jossa tätä ei tarvitse tehdä.

Testi suoritettiin erikseen kaikilla taulukon koilla, jotka ovat kahden potensseja välillä 64...16384.

![](suorituskykykuvaajat/ff1.svg)
