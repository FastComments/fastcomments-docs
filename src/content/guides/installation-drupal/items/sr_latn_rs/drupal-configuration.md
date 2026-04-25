Sva podešavanja se nalaze u `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## Obavezno

- **Tenant ID** - Vaš FastComments Tenant ID. Pronađite ga pod [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Potrebno za Secure SSO, verifikaciju webhook-ova i sinhronizaciju stranica. Nalazi se pod [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).

## Stil komentarisanja

Izaberite widget koji odgovara načinu na koji želite da ljudi komuniciraju na vašem sajtu.

- **Live Comments** - Komentari sa nitima u realnom vremenu.
- **Streaming Chat** - Interfejs za chat uživo, dobar za događaje i livestream-ove.
- **Collab Chat** - Annotacija selektovanog teksta u glavnom sadržaju. Posetioci označe tekst i započnu diskusiju u kontekstu.
- **Collab Chat + Comments** - I collab chat i standardni komentari na istoj stranici.

## SSO Mode

- **None** - Nema SSO. Korisnici komentarišu kao gosti ili kreiraju FastComments nalog.
- **Simple** - Prosleđuje Drupal korisničke podatke (ime, email, avatar) FastComments bez verifikacije na serverskoj strani.
- **Secure** - Koristi HMAC-SHA256 za verifikaciju Drupal korisnika sa FastComments. Preporučeno kada imate konfigurisani API Secret.

Pogledajte odeljak `Single Sign-On (SSO)` za detalje.

## Ostala podešavanja

- **CDN URL** - Podrazumevano je `https://cdn.fastcomments.com`.
- **Site URL** - Podrazumevano je `https://fastcomments.com`.
- **Email notifications** - Pošaljite email autoru sadržaja kada je novi komentar postavljen na njihov sadržaj.

Za EU rezidenciju podataka, pogledajte odeljak `EU Data Residency`.

---