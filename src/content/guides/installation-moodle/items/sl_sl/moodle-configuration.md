Stran z nastavitvami vtičnika se nahaja na **Site Administration > Plugins > Local plugins > FastComments**. Na voljo so naslednje možnosti:

#### ID najemnika

Vaš FastComments Tenant ID. Najdete ga v <a href="https://fastcomments.com/auth/my-account" target="_blank">nadzorni plošči FastComments</a> pod nastavitvami računa.

#### API skrivnost

Vaš API Secret ključ, zahtevan za varen način SSO. Najdete ga na <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">Moj račun > API skrivnost</a>.

#### Način SSO

Izberite, kako so uporabniki overjeni. Oglejte si razdelek [Načini SSO](#items-moodle-sso-modes) za podrobnosti o vsaki možnosti.

- **Varen** (priporočeno) - strežniško overjanje, podpisano z HMAC-SHA256
- **Preprosto** - podatki o uporabniku na odjemalski strani brez podpisa
- **Brez** - anonimno komentiranje, brez integracije prijave Moodle

#### Konteksti strani

Nadzoruje, kje se prikazujejo komentarji:

- **Strani tečajev** - komentarji na glavnih straneh tečajev
- **Strani modulov/aktivnosti** - komentarji pri posameznih aktivnostih in virih
- **Oboje** - komentarji na vseh vrstah strani

#### Slog komentiranja

Izberite izkušnjo komentiranja. Oglejte si [Slogi komentiranja](#items-moodle-commenting-styles) za posnetke zaslona vsakega načina.

- **Komentarji** - standardni nitast pripomoček za komentarje pod vsebino strani
- **Collab Chat** - razprave ob izbiri besedila z indikatorji prisotnosti
- **Oboje** - komentarji in Collab Chat hkrati aktivna

#### CDN URL

CDN URL storitve FastComments. Privzeto je `https://cdn.fastcomments.com`. Spremenite to na CDN URL za EU, če so vaši podatki gostovani v regiji EU.