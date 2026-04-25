Vse nastavitve najdete v `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## Required

- **Tenant ID** - Vaš FastComments Tenant ID. Najdete ga pod [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Zahtevan za Secure SSO, preverjanje webhookov in sinhronizacijo strani. Najdete ga pod [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).

## Commenting Style

Izberite widget, ki ustreza načinu, kako želite, da ljudje komunicirajo na vašem spletnem mestu.

- **Live Comments** - Komentarji v realnem času, organizirani v niti.
- **Streaming Chat** - Vmesnik za klepet v živo, primeren za dogodke in prenose v živo.
- **Collab Chat** - Označevanje besedila za pripombe na glavnem vsebinskem območju. Obiskovalci označijo besedilo in začnejo razpravo v kontekstu.
- **Collab Chat + Comments** - Tako collab chat kot običajni komentarji na isti strani.

## SSO Mode

- **None** - Brez SSO. Uporabniki komentirajo kot gostje ali ustvarijo FastComments račun.
- **Simple** - Posreduje podatke uporabnika Drupal (name, email, avatar) FastComments brez preverjanja na strežniku.
- **Secure** - Uporablja HMAC-SHA256 za preverjanje uporabnikov Drupala z FastComments. Priporočeno, ko imate konfiguriran API Secret.

Oglejte si razdelek `Single Sign-On (SSO)` za podrobnosti.

## Other Settings

- **CDN URL** - Privzeto `https://cdn.fastcomments.com`.
- **Site URL** - Privzeto `https://fastcomments.com`.
- **Email notifications** - Pošlje e-poštno obvestilo avtorju vsebine, ko je na njegovi vsebini objavljen nov komentar.

Za EU podatkovno rezidenco glejte razdelek `EU Data Residency`.

---