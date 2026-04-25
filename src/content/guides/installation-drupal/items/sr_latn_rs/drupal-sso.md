FastComments se integriše sa Drupal-ovim korisničkim sistemom putem SSO (single-sign-on). Vaši korisnici se prijavljuju na vaš Drupal sajt, i modul automatski prosleđuje njihov identitet FastComments-u. Nema dodatnih naloga za kreiranje, nema početne sinhronizacije za pokretanje.

Modul podržava tri SSO moda, podešena pod `Administration > Configuration > Content > FastComments`.

### Nema

Nema SSO. Korisnici komentarišu kao gosti ili kreiraju FastComments nalog. Koristite ovo ako je vaš sajt javan i ne treba vam vezivanje komentara za Drupal korisnike.

### Jednostavno

Prosleđuje Drupal korisnikovo ime, email i avatar FastComments-u bez verifikacije na serverskoj strani. Nije potreban API Secret. Pogodno za interne ili niskorizične sajtove.

### Sigurno (preporučeno)

Koristi [HMAC-SHA256](https://en.wikipedia.org/wiki/HMAC) da verifikuje svaki korisnički identitet sa FastComments-om. Ovo je režim koji želite kada imate konfigurisani API Secret, i jedini režim koji sprečava posetioca da se lažno predstavlja kao drugi korisnik.

Identitet korisnika se prosleđuje FastComments-u svaki put kada korisnik pregleda nit komentara. Ne postoji inicijalna ili kontinuirana sinhronizacija koja treba da se pokreće.

<sup>(Opcionalno)</sup> Dodajte svoje administratore u [Korisnici i administratori](https://fastcomments.com/auth/my-account/users) i moderatore u [Moderatori komentara](https://fastcomments.com/auth/my-account/moderate-comments/moderators) kako biste poboljšali njihovo iskustvo i omogućili praćenje statistike za moderatore.

Za detaljniji prikaz kako SSO funkcioniše, pogledajte [SSO odeljak](/guide-customizations-and-configuration.html#sso) dokumentacije o prilagođavanjima i konfiguraciji.