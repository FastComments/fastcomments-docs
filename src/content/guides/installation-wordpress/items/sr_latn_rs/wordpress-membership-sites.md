FastComments radi sa sajtovima koji su dostupni samo članovima koristeći ono što se zove SSO, ili single-sign-on. Vaši članovi se prijavljuju na vaš WordPress sajt, ali
ne moraju da brinu o kreiranju naloga na FastComments, ili prijavljivanju putem društvenih mreža, da bi komentarisali. Ako su vaši članovi (uključujući administratore) prijavljeni na
vaš WordPress sajt, moći će da komentarišu. Vaši administratori i moderatori moći će da moderiraju komentare direktno iz vaših WordPress blog postova.

<sup>(Opcionalno)</sup> Zapamtite takođe da dodate svoje administratore u [Korisnici i administratori](https://fastcomments.com/auth/my-account/users) i moderatore u [Moderatori komentara](https://fastcomments.com/auth/my-account/moderate-comments/moderators)
kako biste poboljšali njihovo iskustvo i omogućili praćenje statistike za moderatore.

SSO se može omogućiti odlaskom na kontrolnu tablu dodatka i klikom na "SSO Settings".

Prvo ćete morati da omogućite opciju "Svi mogu da se registruju" na vašem sajtu.

Sve informacije o korisniku se besprekorno i bezbedno prenose na FastComments svaki put kada korisnik pregleda nit komentara putem [HMAC](https://en.wikipedia.org/wiki/HMAC).

Ne postoji inicijalna ili kontinuirana sinhronizacija koju treba pokretati da biste kopirali svoje članove na FastComments servere. Ovo se automatski radi kada pregledaju niti komentara otvaranjem blog posta.

## Imena i avatari

Dodatak će automatski ažurirati korisničko prikazno ime i avatar na svim njihovim komentarima unutar FastComments kada pregledaju
bilo koju nit komentara. Avatari su podržani putem Gravatar-a ili bilo kojeg plugina za upravljanje avatarima u okviru WordPress-a, pošto dodatak poziva `get_avatar_url`.