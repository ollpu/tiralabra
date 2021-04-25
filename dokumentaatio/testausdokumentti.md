# Äänen taajuuden seuraus

### Suorituskykytestaus

Suorituskykytestaus tehdään käyttäen Criterion-kirjastoa. Se suorittaa määriteltyä
testiä useaan kertaan ja tekee jonkin verran tilastollista analyysiä.

Jotkin kuvaajat on koostettu Python-skriptien (matplotlib) avulla, jotka löytyvät `suorituskyky`-hakemiston alta.

Kaikki suorituskykytestit voidaan ajaa siirtymällä hakemiston `suorituskyky`, ja ajamalla

```
cargo bench
```

Tulokset kootaan kansioon `suorituskyky/target/criterion/`.

#### [FFT](/suorituskyky/benches/fft_performance.rs)

FFT:lle on kirjoitettu kaksi erilaista suorituskykytestiä.

Molemmissa aloitetaan kokoamalla satunnaisia kompleksilukuja taulukkoon
(molemmat komponentit välillä `[-1, 1]`) ja toistamalla sitten Fourier-muunnosta
tälle taulukolle.

Molemmat testit suoritetaan erikseen kaikilla taulukon koilla, jotka ovat kahden potensseja välillä 64...16384.

##### copy-and-fft

Tässä versiossa, ennen jokaista toistoa, taulukko on kopioidaan alkuperäisestä uudestaan,
koska FFT-toteutus ylikirjoittaa taulukon. Tämä vaikuttaa hieman mitattuun suoritusaikaan. 

##### fft-and-ifft

Tässä versiossa taulukko palautetaan ennalleen suorittamalla käänteismuunnos. Samalla testataan
käänteismuunnoksen suorituskykyä - ideaalitapauksessa se ei ole juurikaan hitaampi kuin varsinainen
muunnos.

*sijoita lopulliset tulokset tähän*

#### Twiddle-kertoimien optimointi

Toteutin commitissa [`c61e5f`](https://github.com/ollpu/tiralabra/commit/c61e5fa048774b5c045c3f6e9a0a6dd175942291) FFT-
algoritmiin optimoinnin, jossa esilasketaan sini- ja kosinifunktioita käyttävät kertoimet. Suorituskykytesti
ei ota esilaskentaa huomioon, sillä normaalissa käytössä esilaskenta tehdään uudestaan vain harvoin.
Alla molempien suorituskykytestien tulokset ennen ("no-memory-fft") ja jälkeen ("new"). Kaikilla
koilla ja molemmissa testeissä algoritmi nopeutui noin 70 %.

![](suorituskykykuvaajat/fft_copy_vertailu.png)
![](suorituskykykuvaajat/fft_both_vertailu.png)

#### Vanha kuvaaja

![](suorituskykykuvaajat/fft1.svg)
