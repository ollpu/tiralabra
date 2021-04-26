# Viikkoraportti 5

Tällä viikolla toteutin FFT-algoritmiin optimoinnin, joka esilaskee twiddle-kertoimet.
Optimointi on projektissa hyödyllinen, koska se tekee jatkuvasti samankokoisia muunnoksia,
eli esilaskettuja arvoja voi käyttää uudelleen. Varsinainen algoritmi nopeutui 70-75%,
tästä tarkemmin testausdokumentissa.

Lisäksi toteutin ensimmäisen oman tietorakenteen, eli rinkipuskurin (ring buffer).
Tätä käytetään datan siirtämiseen mikrofonin syötteestä pääsäikeelle. Se hyödyntää
atomisia indeksejä, eikä joudu missään tilanteessa odottamaan tai tekemään järjestelmäkutsuja.

Optimoinnin toteutukseen ja tulosten koostamiseen kului noin 6 tuntia. Tietorakenteen
toteuttamiseen ja testaamiseen puolestaan noin 4 tuntia, eli yhteensä 10 tuntia.

Algoritmiikan puolelta tekemistä olisi vielä parabolisen interpolaation parissa
(näytteenottoaikaa tarkempi tulos), sekä absoluuttisen taajuuden raportoinnin toteuttamisessa.

Jatkan seuraavalla viikolla parantamalla dokumentaatiota ja lisäämällä demoon testisignaalin generoinnin.
Ehtiessä voisin pureutua noihin algoritmisiin ominaisuuksiin.
