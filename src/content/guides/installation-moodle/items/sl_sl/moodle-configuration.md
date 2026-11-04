Nastavitvena stran vtičnika je na **Administracija spletnega mesta > Vtičniki > Lokalni vtičniki > FastComments**. Na voljo so naslednje možnosti:

#### Tenant ID

Vaš FastComments Tenant ID. Najdete ga v <a href="https://fastcomments.com/auth/my-account" target="_blank">nadzorni plošči FastComments</a> pod nastavitvami vašega računa.

#### API Secret

Vaš ključ API Secret, ki je potreben za način Secure SSO. Najdete ga na <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">Moj račun > API Secret</a>.

#### SSO Mode

Izberite, kako se uporabniki overjajo. Podrobnosti o vsaki možnosti najdete v razdelku [SSO Modes](#moodle-sso-modes).

- **Secure** (priporočeno) - strežniška overjanja, podpisana z HMAC-SHA256
- **Simple** - podatki o uporabniku na strani odjemalca brez podpisa
- **None** - anonimno komentiranje brez integracije prijave v Moodle

#### Page Contexts

Nadzoruje, kje se prikazujejo komentarji:

- **Course pages** - komentarji na glavnih straneh tečajev
- **Module/activity pages** - komentarji na posameznih dejavnostih in virih
- **Both** - komentarji na vseh vrstah strani

#### Commenting Style

Izberite izkušnjo komentiranja. Posnetke zaslona vsakega načina najdete v razdelku [Commenting Styles](#moodle-commenting-styles).

- **Comments** - standarden gradnik nitkastih komentarjev pod vsebino strani
- **Collab Chat** - razprave o izboru besedila v vrstici z indikatorji prisotnosti
- **Both** - komentarji in collab chat aktivna hkrati

#### CDN URL

URL CDN FastComments. Privzeto je `https://cdn.fastcomments.com`. Če so vaši podatki gostovani v regiji EU, to spremenite na URL CDN za EU.