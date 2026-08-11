---
Postoje dva načina za zabraniti korisnicima komentiranje na vašoj web stranici uz FastComments.

Prvi je ako već znate njihovu e‑mail adresu, možete je unijeti na <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">zabranjeni korisnici</a> stranicu.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; alt='Popis zabranjenih korisnika pod Moderate Comments, s zabranjenim e‑mail adresama i gumbom za dodavanje nove zabrane'; title='Stranica zabranjenih korisnika' app-screenshot-end]

Ovu stranicu možete pristupiti putem Moderate Comments -> Zabranjeni korisnici

Kada želimo zabraniti korisnika, možemo odabrati vrstu, bilo da je Permanent ili Permanent Shadow Ban:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; alt='Obrazac za novu zabranu s poljem za e‑mail i odabirom vrste zabrane: Permanent ili Permanent Shadow Ban'; title='Zabranjivanje korisnika' app-screenshot-end]

Drugi način za zabraniti korisnika je klikom na gumb za zabranu koji se nalazi na svakom komentaru na stranici Comment Moderation.

Kada kliknete gumb za zabranu, bit će vam prikazane neke opcije, gdje možemo odrediti vrstu zabrane i trajanje.

### Email aliasi

Pri zabranjivanju korisnika putem e‑mail adrese, FastComments automatski zanemaruje `+` aliasove. Na primjer, zabranom `user+alias@gmail.com` također će se zabraniti `user@gmail.com` i bilo koja druga `+` varijacija te adrese, poput `user+other@gmail.com`.

### Sjenovne zabrane

Sjenovna zabrana je vrsta zabrane koja čini da se čini da je komentar ili glas korisnika uspješno spremljen, iako to nije bio slučaj. To može biti poželjno u određenim situacijama.

### Zabrana putem IP adrese

Osim ako najmodavac ne želi isključiti ovu opciju, FastComments podržava zabranu putem IP adrese pohranjivanjem hashirane verzije IP adrese komentatora.

---