FastComments se integriše sa Drupal-ovim sistemom korisnika putem SSO, odnosno jedinstvene prijave. Vaši korisnici se prijavljuju na vaš Drupal sajt, a modul automatski prosleđuje njihov identitet na FastComments. Nema dodatnih naloga za kreiranje, nema početne sinhronizacije koju treba pokrenuti.

Modul podržava tri SSO režima, podešena pod `Administration > Configuration > Content > FastComments`.

### Nijedno

Bez SSO. Korisnici komentarišu kao gosti ili kreiraju FastComments nalog. Koristite ovo ako je vaš sajt javni i ne trebate vezati komentare za Drupal korisnike.

### Jednostavno

Prosleđuje ime Drupal korisnika, e-mail i avatar na FastComments bez provjere na serverskoj strani. Nije potreban API Secret. Pogodno za interne ili niskorizične sajtove.

### Sigurno (preporučeno)

Koristi [HMAC-SHA256](https://en.wikipedia.org/wiki/HMAC) za verifikaciju identiteta svakog korisnika sa FastComments. Ovo je režim koji želite kada imate konfigurisan API Secret, i jedini je režim koji sprječava posjetilaca da se predstavlja kao drugi korisnik.

Identitet korisnika se prosleđuje FastComments svaki put kada korisnik pregleda nit komentara. Nema početne ili kontinuirane sinhronizacije koja treba da se pokrene.

<sup>(Neobavezno)</sup> Dodajte svoje administratore u [Korisnici i administratori](https://fastcomments.com/auth/my-account/users) i moderatore u [Moderatori komentara](https://fastcomments.com/auth/my-account/moderate-comments/moderators) kako biste poboljšali njihovo iskustvo i omogućili praćenje statistike za moderatore.

Za dublji uvid u kako SSO funkcioniše, pogledajte [odjeljak SSO](/guide-customizations-and-configuration.html#sso) u dokumentaciji za prilagođavanje.

---