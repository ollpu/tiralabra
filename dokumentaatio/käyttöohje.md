# Demon käyttöohje

Demoikkunan tulisi näyttää jotakuinkin tältä:

![ikkuna](https://user-images.githubusercontent.com/7241014/117027750-aafbc300-ad05-11eb-9064-d489f2180c9a.png)

Äänilähteenä voit käyttää joko mikrofonia tai testisignaalia. Terminaaliin tulisi ilmestyä joko
teksti `Käytetään äänilaitetta: "laitteen nimi"` tai virheviesti, jos mikrofoni ei toimi.
Vaihda testisignaaliin "Äänilähde"-pudotusvalikosta:

![äänilähde](https://user-images.githubusercontent.com/7241014/118395844-36a21780-b655-11eb-9b32-fe73089a37d4.png)

Mikrofonilla kannattaa kokeilla esimerkiksi hyräilyä. Jos saat järjestelmäasetukset
asetettua siten, että ohjelma monitoroi kaiuttimien uloslähtöä, voit kokeilla mitä
tahansa äänisignaalia.

"Testi: Siniaalto" on vakiotaajuuksinen siniaalto. "Testi: Vaihtuva" on siniaalto,
jonka taajuus vaihtelee noin 200 Hz ympärillä.

Hiiren rullaa käyttämällä kuvaajaa voi siirtää suhteellisesti.

Alareunassa oleva teksti kertoo arvioidun perustaajuuden. Tämän vieressä oleva
harmaa palkki ilmaisee, mikä kohta saadusta signaalista näytetään tällä
hetkellä. Perustaajuudesta riippuen sen on tarkoituskin hyppiä edestakaisin.

"Vakauta"-valintaruudusta voit ottaa algoritmin pois käytöstä. Tällöin alareunan palkki
pysyy paikoillaan, eli näytöllä näkyy aina suhteessa sama kohta.

Näytön ja muistin vaimenemisajan liu'ut säätävät, kuinka pitkään vanhaa signaalia
muistetaan. Suuremmalla näytön vaimenemisajalla näkymä on sulavampi, mutta
se eroaa hieman oikeasti luetusta signaalista. Suuremmalla muistin vaimenemisajalla
algoritmi pystyy muistamaan aiemmin näytettyä kuvaajaa, eli näkymä pysyy vakaana
riippumatta hetkellisistä häiriöistä.

Kun viet hiiren muistin vaimenemisajan liu'un päälle, näkyy näytöllä sinisellä
värillä toinen kuvaaja, joka ilmaisee muistin tämänhetkisen sisällön. Tämä on
siis se taulukko, jota verrataan uuteen signaaliin.
