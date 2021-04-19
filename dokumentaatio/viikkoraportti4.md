# Viikkoraportti 4

Tällä viikolla pystytin suorituskykytestauksen, ja aloitin totetus- ja testaudokumenttien kirjoittamisen.

Lisäksi parantelin demon käyttöliittymää hieman. Lisäsin repositorioon julkaisun (release),
josta demon voi ladata valmiina binäärinä, jos kääntäminen tuottaa ongelmia.

Suorituskykytestaukseen käytän Criterion-kirjastoa, lähinnä koska sen avulla saa kätevästi piirrettyä kuvaajia. Tähän tutustumiseen ja testien kirjoittamiseen kului noin 3 tuntia.

Toteutusdokumenttiin siirsin aiemmin koodikommenttiin kirjoitetun kaavailun algoritmin toiminnasta,
sekä selostin demon toimintaa. Selostus on tuohon yhteyteen ehkä turhankin yksityiskohtainen,
mutta haluan sen olevan jossain luettavissa. Testausdokumenttiin liitin FFT:n suorituskykytestin tuloksia. Näihin
kului noin 5 tuntia.

Ajankäyttö yhteensä noin 10 tuntia.

Seuraavaksi jatkan suorituskykytestausta, ja toteutan FFT:hen suunnittelemani optimoinnit.
Suorituskykytestauksen avulla voidaan sitten arvioida optimointien hyötyä.
