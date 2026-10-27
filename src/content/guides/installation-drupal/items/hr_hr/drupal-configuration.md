All settings live under `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## Obavezno

- **Tenant ID** - Vaš FastComments Tenant ID. Pronađite ga pod [Postavke > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Potrebno za Secure SSO, provjeru webhooka i sinkronizaciju stranice. Pronađite ga pod [Postavke > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).

## Pohranjivanje API tajne

Modul čita API Secret iz prvog od ovih izvora koji ima vrijednost:

1. **Key module** - Instalirajte [Key](https://www.drupal.org/project/key), kreirajte *Authentication* ključ i odaberite ga pod **API Secret key** u obrascu za postavke. Ključ može biti podržan varijablom okruženja ili datotekom izvan web korijena.
2. **settings.php** - Dodajte `$config['fastcomments.settings']['api_secret'] = 'your-secret';` u `settings.php` ili `settings.local.php`. Tajna nikada ne ulazi u izvezenu konfiguraciju.
3. **Obrazac za postavke** - Unesite je u polje **API Secret**. Ovo pohranjuje vrijednost u `fastcomments.settings`, pa se ona nalazi u izvozima konfiguracije. Drupal izvještaj o statusu upozorava kada je ovo aktivni izvor.

Obrazac prikazuje koji je izvor aktivan. Ostavite polje prazno da zadržite pohranjenu tajnu. Tick **Remove the API Secret stored in configuration** to delete it after moving the secret to the Key module or `settings.php`.

## Stil komentiranja

Odaberite widget koji odgovara načinu na koji želite da ljudi razgovaraju na vašoj stranici.

- **Live Comments** - Komentari u stvarnom vremenu s grananjem.
- **Streaming Chat** - Sučelje za live chat, pogodno za događaje i prijenose uživo.
- **Collab Chat** - Anotacija odabira teksta u glavnom području sadržaja. Posjetitelji ističu tekst i započinju raspravu u kontekstu.
- **Collab Chat + Comments** - I collab chat i standardni komentari na istoj stranici.

## SSO način

- **None** - Nema SSO. Korisnici komentiraju kao gosti ili kreiraju FastComments račun.
- **Simple** - Prosljeđuje informacije o Drupal korisniku (ime, email, avatar) FastCommentsu bez provjere na poslužitelju.
- **Secure** - Koristi HMAC‑SHA256 za provjeru Drupal korisnika s FastCommentsom. Preporučeno kada imate konfiguriranu API Secret.

Pogledajte odjeljak `Single Sign-On (SSO)` za detalje.

## Ostale postavke

- **CDN URL** - Zadano je `https://cdn.fastcomments.com`.
- **Site URL** - Zadano je `https://fastcomments.com`.
- **Email notifications** - Pošaljite email autoru sadržaja kada se na njegovom sadržaju objavi novi komentar.

Za EU rezidenciju podataka, pogledajte odjeljak `EU Data Residency`.