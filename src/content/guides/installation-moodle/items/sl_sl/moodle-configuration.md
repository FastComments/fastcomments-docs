Stran z nastavitvami vtičnika se nahaja na **Administracija spletnega mesta > Vtičniki > Lokalni vtičniki > FastComments**. Na voljo so naslednje možnosti:

#### Tenant ID

Vaš FastComments Tenant ID. Najdete ga v <a href="https://fastcomments.com/auth/my-account" target="_blank">nadzorni plošči FastComments</a> v nastavitvah računa.

#### API Secret

Vaš API Secret ključ, zahtevan za način Secure SSO. Najdete ga na <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">Moj račun > API Secret</a>.

#### SSO Mode

Izberite, kako so uporabniki overjeni. Oglejte si razdelek [SSO Modes](#moodle-sso-modes) za podrobnosti o vsaki možnosti.

- **Secure** (recommended) - strežniško HMAC-SHA256 podpisano overjanje
- **Simple** - odjemalski podatki o uporabniku brez podpisa
- **None** - anonimno komentiranje, brez integracije prijave v Moodle

#### Page Contexts

Nadzoruje, kje se prikažejo komentarji:

- **Course pages** - komentarji na glavnih straneh predmeta
- **Module/activity pages** - komentarji na posameznih dejavnostih in gradivih
- **Both** - komentarji na vseh vrstah strani

#### Commenting Style

Izberite izkušnjo komentiranja. Oglejte si [Commenting Styles](#moodle-commenting-styles) za posnetke zaslona vsakega načina.

- **Comments** - standardni nitni pripomoček za komentarje pod vsebino strani
- **Collab Chat** - razprave ob izbiri besedila z indikatorji prisotnosti
- **Both** - komentarji in Collab Chat hkrati aktivni

#### CDN URL

URL FastComments CDN. Privzeto je `https://cdn.fastcomments.com`. Spremenite ga na EU CDN URL, če so vaši podatki gostovani v regiji EU.