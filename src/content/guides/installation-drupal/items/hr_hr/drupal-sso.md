FastComments se integrira s Drupalovim sustavom korisnika putem SSO, odnosno single-sign-on. Vaši korisnici se prijavljuju na vaš Drupal sajt, a modul automatski prosljeđuje njihov identitet FastCommentsu. Nema potrebe za dodatnim računima, nema početne sinkronizacije.

Modul podržava tri SSO načina rada, postavljena pod `Administration > Configuration > Content > FastComments`.

### Nijedan

Bez SSO-a. Korisnici komentiraju kao gosti ili naprave FastComments račun. Koristite ovo ako je vaš sajt javan i ne trebate povezivati komentare s Drupal korisnicima.

### Jednostavan

Prosljeđuje Drupalovo korisničko ime, email i avatar FastCommentsu bez provjere na strani servera. Nije potreban API Secret. Dobro za interne ili niskorizične sajtove.

### Siguran (preporučeno)

Koristi [HMAC-SHA256](https://en.wikipedia.org/wiki/HMAC) za verifikaciju svakog korisničkog identiteta s FastCommentsom. Ovo je način koji želite kada imate konfiguriran API Secret, i jedini je način koji sprječava posjetitelja da se lažno predstavlja kao drugi korisnik.

Korisnički identitet se prosljeđuje FastCommentsu svaki put kad korisnik pogleda nit komentara. Ne postoji početna ili kontinuirana sinkronizacija koja treba biti pokrenuta.

<sup>(Neobavezno)</sup> Dodajte svoje administratore u [Korisnici i administratori](https://fastcomments.com/auth/my-account/users) i moderatore u [Moderatori komentara](https://fastcomments.com/auth/my-account/moderate-comments/moderators) kako biste poboljšali njihovo iskustvo i omogućili praćenje statistike za moderatore.

Za dublji uvid u to kako SSO funkcionira, pogledajte [SSO odjeljak](/guide-customizations-and-configuration.html#sso) u dokumentaciji za prilagodbe.