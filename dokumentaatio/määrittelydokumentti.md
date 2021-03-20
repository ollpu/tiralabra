# Äänen taajuuden seuraus

### Määrittelydokumentti

Työn tavoitteena on toteuttaa algoritmi, jota voidaan käyttää reaaliajassa äänen
perustaajuuden seuraamiseen. Ensisijainen käyttötarkoitus on aaltomuotoa
näyttävän kuvaajan (oskilloskooppin) vakauttaminen, jotta kuvaajaa on
helpompi seurata silmämääräisesti.

Algoritmi lukee jatkuvasti äänisignaalia (esim. 44.1 kHz
näytteenottotaajuudella), ja analysoi taajuutta tietyn ajan välein "paloissa".
Palojen on oltava riittävän suuria, jotta niistä havaitaan taajuus, mutta myös
riittävän pieniä, että analyysi voidaan tehdä tarpeeksi usein tehokkaasti.
Oskilloskoopin kanssa palojen kokoluokka voisi olla likimain yhtäsuuri, kuin
kuvaajassa näkyvän datan määrä.

Ohjelma toteutetaan Rust-kielellä. Toiminnan demonstrointia varten teen
yksinkertainen graafinen käyttöliittymän, joka näyttää oskilloskooppikuvaajan.

Algoritmi tulee perustumaan YIN-algoritmiin, joka hyödyntää autokorrelaatiota
ja tämän jälkikäsittelyä päättelemään äänen perustaajuuden. Oskilloskoopin
vakautusta varten ei kuitenkaan välttämättä tarvita absoluuttista taajuutta,
vaan mikä tahansa moninkerta kelpaa myös, joten yksityiskohdat pitää ehkä
tuunata vähän erilaisiksi. Jos aika riittää, voin toteuttaa molemmat, eli myös
absoluuttisen perustaajuuden raportoinnin.

Autokorrelaatio voidaan toteuttaa tehokkaasti nopean Fourier-muunnoksen avulla
(FFT). Reaaliaikaista toimintaa varten voi olla tarvetta
myös ns. circular buffer -tietorakenteelle, tai muulle vastaavalle tavalle
siirtää dataa säikeiden välillä ilman lukkoja. Nämä tulen toteuttamaan itse.

Algoritmin laatua ja tarkkuutta voi parantaa jonkin verran ylinäytteistämällä
(oversampling) signaali ensin. Tähän on monia tapoja, mutta siihen voidaan myös
hyödyntää FFT:tä. Tämä on mahdollista toteuttaa, jos aikaa jää yli.

FFT:lle tavoitteena on tyypillinen 2-kantainen Cooley-Tukey algoritmi, joka
toimii `O(n log n)` ajassa, kunhan muunnettava taulukko on kahden potenssin
kokoinen.  Autokorrelaation kanssa taulukkoon voidaan lisätä nollia perään
muuttamatta tulosta, eli tämä riittää. Koska operaatiota toistetaan jatkuvasti,
voi olla hyödyllistä esilaskea ns. twiddle-kertoimet.

Muuten tehokkuudelle ei ole tarkkaa tavoitetta, mutta ohjelman pitäisi
ainakin voida tehdä analyysi n. 1000 näytteen kokoiselle palalle ääntä 60 kertaa
sekunnissa tyypillisellä tietokoneella ilman suurempia vaikeuksia.
Ylipäätään algoritmi olisi hyvä saada niin tehokkaaksi kuin järkevästi
mahdollista, koska muitakin ääntä käsitteleviä ohjelmia voidaan käyttää
samanaikaisesti. Tämä vaatii kattavaa suorituskykytestausta. Muistia tarvitaan vain
lineaarinen määrä, mutta välivaiheita varten tarvitaan varmaankin muutamia
etukäteen varattuja taulukoita.

Paperi, jossa YIN-algoritmi esitellään:
https://asa.scitation.org/doi/abs/10.1121/1.1458024

Saatavilla myös http://recherche.ircam.fr/equipes/pcm/cheveign/ps/2002_JASA_YIN_proof.pdf


**Koulutusohjelma:** Tietojenkäsittelytieteen kandidaatti  
**Projektin kieli:** Suomi

