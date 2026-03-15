---
Idite na **Administracija > Konfiguracija > Sadržaj > FastComments** (`/admin/config/content/fastcomments`).

### Postavke

- **Tenant ID** (obavezno) - Vaš FastComments Tenant ID. Pronađite ga pod [Postavke > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Potrebno za Secure SSO, verifikaciju webhookova i sinkronizaciju stranica. Nalazi se pod [Postavke > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **SSO Mode** - Integracija Single Sign-On:
  - **None** - Nema SSO, korisnici komentiraju kao gosti ili kreiraju FastComments račune.
  - **Simple** - Prosljeđuje Drupal podatke o korisniku (ime, email, avatar) FastCommentsu bez verifikacije na strani poslužitelja.
  - **Secure** - Koristi HMAC-SHA256 verifikaciju za sigurno autentificiranje Drupal korisnika s FastCommentsom (preporučeno).
- **Commenting Style** - Tip widgeta za prikaz:
  - **Live Comments** - Više-nitni komentari u stvarnom vremenu.
  - **Streaming Chat** - Sučelje za chat uživo.
  - **Collab Chat** - Suradničko označavanje odabranog teksta na glavnom području sadržaja.
  - **Collab Chat + Comments** - Oba: collab chat i standardni komentari.
- **CDN URL** - FastComments CDN URL (zadano: `https://cdn.fastcomments.com`).
- **Site URL** - FastComments site URL (zadano: `https://fastcomments.com`).
- **Email notifications** - Pošaljite e-poštu autorima sadržaja kada je objavljen novi komentar na njihovom sadržaju.

### Dodavanje komentara vrstama sadržaja

Dodajte polje **FastComments** vašim vrstama sadržaja putem **Struktura > Vrste sadržaja > [type] > Upravljanje poljima**. Polje ima prekidač statusa i opcionalni prilagođeni identifikator za svaki entitet.

### EU rezidencija podataka

Za smještaj podataka u EU, ažurirajte:
- **CDN URL** na `https://cdn-eu.fastcomments.com`
- **Site URL** na `https://eu.fastcomments.com`
---