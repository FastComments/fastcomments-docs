Idite na **Administracija > Konfiguracija > Sadržaj > FastComments** (`/admin/config/content/fastcomments`).

### Podešavanja

- **Tenant ID** (obavezno) - Vaš FastComments Tenant ID. Pronađite ovo pod [Podešavanja > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Potreban za Secure SSO, verifikaciju webhook-a i sinhronizaciju stranica. Nalazi se pod [Podešavanja > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **SSO Mode** - Integracija Single Sign-On:
  - **None** - Bez SSO, korisnici komentarišu kao gosti ili kreiraju FastComments naloge.
  - **Simple** - Prosleđuje podatke o Drupal korisniku (ime, email, avatar) FastComments bez verifikacije na serverskoj strani.
  - **Secure** - Koristi HMAC-SHA256 verifikaciju za sigurno autentifikovanje Drupal korisnika sa FastComments (preporučeno).
- **Commenting Style** - Tip widgeta koji će se prikazati:
  - **Live Comments** - Ugnježdeni komentari u realnom vremenu.
  - **Streaming Chat** - Interfejs za chat uživo.
  - **Collab Chat** - Kolaborativno označavanje teksta (anotacija) na glavnom sadržaju.
  - **Collab Chat + Comments** - I kolaborativni chat i standardni komentari.
- **CDN URL** - FastComments CDN URL (podrazumevano: `https://cdn.fastcomments.com`).
- **Site URL** - FastComments URL sajta (podrazumevano: `https://fastcomments.com`).
- **Email notifications** - Pošalji e-mail autorima sadržaja kada je novi komentar objavljen na njihovom sadržaju.

### Dodavanje komentara u tipove sadržaja

Dodajte polje **FastComments** vašim tipovima sadržaja putem **Struktura > Tipovi sadržaja > [type] > Upravljanje poljima**. Polje ima prekidač statusa i opcionu prilagođenu identifikaciju po entitetu.

### EU rezidencija podataka

Za EU rezidenciju podataka, ažurirajte:
- **CDN URL** na `https://cdn-eu.fastcomments.com`
- **Site URL** na `https://eu.fastcomments.com`