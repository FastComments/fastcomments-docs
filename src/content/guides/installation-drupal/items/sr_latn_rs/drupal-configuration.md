All settings live under `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## Required

- **Tenant ID** – Vaš FastComments Tenant ID. Pronađite ga pod [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** – Potrebno za Secure SSO, verifikaciju webhook‑a i sinhronizaciju stranica. Pronađite ga pod [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).

## Storing the API Secret

The module reads the API Secret from the first of these sources that has a value:

1. **Key module** – Instalirajte [Key](https://www.drupal.org/project/key), kreirajte *Authentication* ključ i odaberite ga pod **API Secret key** na obrascu za podešavanja. Ključ može biti podržan promenljivom okruženja ili fajlom izvan web korena.
2. **settings.php** – Dodajte `$config['fastcomments.settings']['api_secret'] = 'your-secret';` u `settings.php` ili `settings.local.php`. Tajna nikada ne ulazi u izvezeni konfiguracioni fajl.
3. **The settings form** – Unesite je u polje **API Secret**. Ovo čuva vrednost u `fastcomments.settings`, pa se ona pojavljuje u izvezenim konfiguracijama. Drupal izveštaj o statusu upozorava kada je ovo aktivni izvor.

The form shows which source is active. Leaving the field blank keeps the stored secret. Tick **Remove the API Secret stored in configuration** to delete it after moving the secret to the Key module or `settings.php`.

## Commenting Style

Izaberite vidžet koji odgovara načinu na koji želite da ljudi komuniciraju na vašem sajtu.

- **Live Comments** – Komentari u realnom vremenu sa nitima.
- **Streaming Chat** – Interfejs za live chat, dobar za događaje i prenose uživo.
- **Collab Chat** – Anotacija odabira teksta u glavnoj oblasti sadržaja. Posetioci ističu tekst i započinju diskusiju u kontekstu.
- **Collab Chat + Comments** – I collab chat i standardni komentari na istoj stranici.

## SSO Mode

- **None** – Nema SSO. Korisnici komentarišu kao gosti ili kreiraju FastComments nalog.
- **Simple** – Prosleđuje informacije o Drupal korisniku (ime, email, avatar) FastComments‑u bez verifikacije na serveru.
- **Secure** – Koristi HMAC‑SHA256 za verifikaciju Drupal korisnika sa FastComments‑om. Preporučuje se kada je API Secret podešen.

Pogledajte odeljak `Single Sign-On (SSO)` za detalje.

## Other Settings

- **CDN URL** – Podrazumevano je `https://cdn.fastcomments.com`.
- **Site URL** – Podrazumevano je `https://fastcomments.com`.
- **Email notifications** – Pošaljite email autoru sadržaja kada se na njegovom sadržaju objavi novi komentar.

Za EU rezidenciju podataka, pogledajte odeljak `EU Data Residency`.