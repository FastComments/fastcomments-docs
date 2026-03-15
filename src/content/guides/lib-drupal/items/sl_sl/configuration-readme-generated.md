Pojdite v **Administracija > Konfiguracija > Vsebina > FastComments** (`/admin/config/content/fastcomments`).

### Nastavitve

- **Tenant ID** (required) - Vaš FastComments Tenant ID. Najdete to pod [Nastavitve > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Zahtevano za Secure SSO, preverjanje webhookov in sinhronizacijo strani. Najdete ga pod [Nastavitve > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **SSO Mode** - Integracija Enotne prijave (Single Sign-On):
  - **None** - Brez SSO, uporabniki komentirajo kot gostje ali ustvarijo FastComments račune.
  - **Simple** - Posreduje podatke Drupal uporabnika (ime, e-pošta, avatar) v FastComments brez preverjanja na strežniku.
  - **Secure** - Uporablja HMAC-SHA256 preverjanje za varno overjanje Drupal uporabnikov pri FastComments (priporočeno).
- **Commenting Style** - Vrsta vtičnika za prikaz:
  - **Live Comments** - V realnem času medsebojno povezani komentarji.
  - **Streaming Chat** - Vmesnik za klepet v živo.
  - **Collab Chat** - Sodelovalno označevanje besedila na glavni vsebini.
  - **Collab Chat + Comments** - Tako sodelovalni klepet kot standardni komentarji.
- **CDN URL** - FastComments CDN URL (privzeto: `https://cdn.fastcomments.com`).
- **Site URL** - FastComments site URL (privzeto: `https://fastcomments.com`).
- **Email notifications** - Pošlji e-pošto avtorjem vsebine, ko je na njihovi vsebini objavljen nov komentar.

### Dodajanje komentarjev k tipom vsebine

Dodajte polje **FastComments** k vašim tipom vsebine preko **Struktura > Tipi vsebine > [type] > Uredi polja**. Polje ima stikalo za status in neobvezni prilagojen identifikator za vsako entiteto.

### Rezidenca podatkov v EU

Za rezidenco podatkov v EU posodobite:
- **CDN URL** na `https://cdn-eu.fastcomments.com`
- **Site URL** na `https://eu.fastcomments.com`