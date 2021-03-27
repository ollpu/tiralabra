# Viikkoraportti 2

Kuten olin suunnitellut, alustin tällä viikolla projektin rakenteen sekä toteutin
FFT-algoritmin.

Projektilla on nyt [GitHub workflow](https://github.com/ollpu/tiralabra/actions/workflows/coverage.yml),
joka suorittaa testit ja raportoi testien kattavuuden Codecoviin. Tämän pystyttämisessä
kesti muutama tunti, kun valmiita esimerkkejä ei ollut kovin montaa. Testikattavuuden
raportointi vaikuttaisi kuitenkin toimivan riittävän hyvin Rustin kanssa - kattavuus
onkin nyt vain 77 %, koska en kirjoittanut kaikille kompleksilukuluokan metodeille testejä.

Koodin muotoiluun ja tyylin seurantaan otin käyttöön rustfmt:n ja Clippyn. Näiden
konfiguraatiotiedostot löytyvät projektin juuresta, tosin oletusasetukset
ovat tyydyttävät.

Koodin dokumentaatio julkaistaan GitHub workflown avulla osoitteeseen
https://ollpu.github.io/tiralabra/.

FFT:n toteutus löytyy (toistaiseksi) tiedostosta [`src/fft.rs`](/src/fft.rs).
En vielä lähtenyt optimoimaan toteutusta esimerkiksi esilaskemalla twiddle-kertoimia,
mutta kirjoitin muutamia [yksikkötestejä](/src/test/test_fft.rs), jotta optimointia ja
refaktorointia on helppo lähteä tekemään myöhemmin.

Lisäksi pohdin hieman tarkemmin ydinalgoritmin toteutusta. Kirjoitus löytyy
[täältä](https://ollpu.github.io/tiralabra/tiralabra/correlation_match/index.html).
(Kirjoitan tämän myöhemmin selkeämmin - pelkkä markdown ei oikein sovellu matemaattiseen
notaatioon.) Tämän toteuttaminen on varmaankin seuraava työnvaihe.

Ajankäyttö: yheensä noin 12 tuntia.

**Kysymys:** Koodia kirjoittaessa askarrutti, että kun projektin kielenä on suomi,
niin pitäisikö myös esim. funktiot ja muuttujat nimetä suomekielisesti? Tämä ei tunnu
järkevältä, koska koodiin tulee väkisin sekaan englanninkielisiä standardikirjaston kutsuja
ja muita vastaavia, eikä ääkköset varsinaisesti toimi. Tein siis nyt niin, että koodin
kommentit on suomeksi, mutta koodin rakenteiden nimet englanniksi.
