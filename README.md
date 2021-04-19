# Tiralabra: Äänen taajuuden seuraus

[![CI](https://github.com/ollpu/tiralabra/actions/workflows/coverage.yml/badge.svg)](https://github.com/ollpu/tiralabra/actions/workflows/coverage.yml)
[![codecov](https://codecov.io/gh/ollpu/tiralabra/branch/main/graph/badge.svg?token=NXYLTIWRUU)](https://codecov.io/gh/ollpu/tiralabra)

## Suorittaminen

Valmis binääri on ladattavissa Linuxille ja Windowsille [julkaisuista](https://github.com/ollpu/tiralabra/releases).

Jos haluat kääntää demon itse, asenna ensin Rust ja Cargo-paketinhallinta: https://rustup.rs/

Aja sitten demo suorittamalla (grafiikkakirjastojen kääntämisessä kestää jonkin aikaa)

```sh
cargo run --release --example demo
```

"Vakauta"-valintaruudulla itse algoritmin saa päälle tai pois. Vaimenemisajan liu'ut
vaikuttavat siihen, kuinka nopeasti näytettävä signaali vaihtuu. Muistille on oma liukunsa,
jotta algoritmi saadaan muistamaan suurpiirteisen aaltomuodon pidempään.


Testit voidaan ajaa suorittamalla

```sh
cargo test --no-default-features
```

## Dokumentaatio

#### [Koodin dokumentaatio](https://ollpu.github.io/tiralabra)
#### [Määrittelydokumentti](dokumentaatio/määrittelydokumentti.md)
#### [Toteutusdokumentti](dokumentaatio/toteutusdokumentti.pdf)
#### [Testausdokumentti](dokumentaatio/testausdokumentti.md)

## Viikkoraportit

#### [Viikkoraportti 1](dokumentaatio/viikkoraportti1.md)
#### [Viikkoraportti 2](dokumentaatio/viikkoraportti2.md)
#### [Viikkoraportti 3](dokumentaatio/viikkoraportti3.md)
#### [Viikkoraportti 4](dokumentaatio/viikkoraportti4.md)
