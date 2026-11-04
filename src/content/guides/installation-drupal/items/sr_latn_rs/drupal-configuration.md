Sva podešavanja se nalaze pod `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## Obavezno

- **Tenant ID** - Vaš FastComments Tenant ID. Pronađite ga pod [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Obavezno za Secure SSO, verifikaciju webhook-ova i sinhronizaciju stranica. Nalazi se pod [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).

## Čuvanje API Secret-a

Modul čita API Secret iz prvog od ovih izvora koji ima vrednost:

1. **Key modul** - Instalirajte [Key](https://www.drupal.org/project/key), napravite *Authentication* ključ i izaberite ga pod **API Secret key** na formi za podešavanja. Ključ može biti podržan promenljivom okruženja ili fajlom van web root-a.
2. **settings.php** - Dodajte `$config['fastcomments.settings']['api_secret'] = 'your-secret';` u `settings.php` ili `settings.local.php`. Tajna nikada ne ulazi u izvezenu konfiguraciju.
3. **Forma za podešavanja** - Unesite je u polje **API Secret**. Ovo čuva vrednost u `fastcomments.settings`, tako da završava u izvozima konfiguracije. Drupal izveštaj o statusu upozorava kada je ovo aktivan izvor.

Forma prikazuje koji je izvor aktivan. Ako ostavite polje prazno, sačuvana tajna ostaje. Štiklirajte **Remove the API Secret stored in configuration** da biste je obrisali nakon premeštanja tajne u Key modul ili `settings.php`.

## Stil komentara

Izaberite vidžet koji odgovara načinu na koji želite da ljudi komuniciraju na vašem sajtu.

- **Live Comments** - Komentari u granama u realnom vremenu.
- **Streaming Chat** - Interfejs za live chat, dobar za događaje i prenose uživo.
- **Collab Chat** - Anotacija izbora teksta na glavnom području sadržaja. Posetioci istaknu tekst i započnu diskusiju u kontekstu.
- **Collab Chat + Comments** - I collab chat i standardni komentari na istoj stranici.

## SSO režim

- **None** - Bez SSO-a. Korisnici komentarišu kao gosti ili naprave FastComments nalog.
- **Simple** - Prosleđuje informacije o Drupal korisniku (ime, email, avatar) FastComments-u bez server-side verifikacije.
- **Secure** - Koristi HMAC-SHA256 za verifikaciju Drupal korisnika sa FastComments-om. Preporučuje se kada imate konfigurisan API Secret.

Pogledajte odeljak `Single Sign-On (SSO)` za detalje.

## Ostala podešavanja

- **CDN URL** - Podrazumevano `https://cdn.fastcomments.com`.
- **Site URL** - Podrazumevano `https://fastcomments.com`.
- **Email obaveštenja** - Šalje email autoru sadržaja kada se objavi novi komentar na njegovom sadržaju.

Za EU data residency, pogledajte odeljak `EU Data Residency`.