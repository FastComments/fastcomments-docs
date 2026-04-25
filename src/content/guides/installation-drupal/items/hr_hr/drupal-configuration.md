Sva podešavanja nalaze se pod `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## Obavezno

- **Tenant ID** - Vaš FastComments Tenant ID. Pronađite ga pod [Postavke > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Potrebno za Secure SSO, provjeru webhookova i sinkronizaciju stranica. Nalazi se pod [Postavke > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).

## Stil komentiranja

Pick the widget that matches how you want people to talk on your site.

- **Live Comments** - Komentari u niti u stvarnom vremenu.
- **Streaming Chat** - Sučelje za chat uživo, pogodno za događaje i prijenose uživo.
- **Collab Chat** - Označavanje teksta na glavnom sadržaju. Posjetitelji označe tekst i započnu diskusiju u kontekstu.
- **Collab Chat + Comments** - I Collab Chat i standardni komentari na istoj stranici.

## Način SSO

- **None** - Nema SSO. Korisnici komentiraju kao gosti ili kreiraju FastComments račun.
- **Simple** - Prosljeđuje Drupal korisničke podatke (name, email, avatar) FastComments bez verifikacije na poslužitelju.
- **Secure** - Koristi HMAC-SHA256 za verifikaciju Drupal korisnika s FastComments. Preporučeno kada imate konfiguriran API Secret.

See the `Single Sign-On (SSO)` section for details.

## Ostala podešavanja

- **CDN URL** - Zadano na `https://cdn.fastcomments.com`.
- **Site URL** - Zadano na `https://fastcomments.com`.
- **Email notifications** - Šalje e-poštu autoru sadržaja kada je na njihov sadržaj objavljen novi komentar.

Za rezidenciju podataka u EU, pogledajte odjeljak `EU Data Residency`.

---