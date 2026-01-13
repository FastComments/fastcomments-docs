FastComments radi s web-lokacijama samo za članove koristeći ono što se zove SSO, odnosno single-sign-on. Vaši članovi se prijavljuju na vašu WordPress stranicu, ali
ne moraju se brinuti o stvaranju računa na FastCommentsu ili prijavljivanju putem društvenih mreža da bi komentirali. Ako su vaši članovi (uključujući administratore) prijavljeni na
vašu WordPress stranicu, moći će komentirati. Vaši administratori i moderatori moći će također moderirati komentare izravno iz objava na vašem WordPress blogu.

<sup>(Opcionalno)</sup> Sjetite se također dodati svoje administratore na [Korisnici i administratori](https://fastcomments.com/auth/my-account/users) i moderatore na [Moderatori komentara](https://fastcomments.com/auth/my-account/moderate-comments/moderators)
kako biste poboljšali njihovo iskustvo i omogućili praćenje statistike za moderatore.

SSO se može omogućiti tako da odete na nadzornu ploču dodatka i kliknete "Postavke SSO".

Prvo ćete morati omogućiti značajku "Svatko se može registrirati" na svojoj stranici.

Sve korisničke informacije besprijekorno i sigurno se prenose na FastComments svaki put kada korisnik pregleda nit komentara putem [HMAC](https://en.wikipedia.org/wiki/HMAC).

Nije potrebno pokretati početnu ili stalnu sinkronizaciju da biste kopirali svoje članove na FastComments poslužitelje. To se automatski radi kada pregledaju niti komentara otvaranjem objave na blogu.

## Imena i Avatari

Dodatak će automatski ažurirati prikazano ime korisnika i avatar na svim njihovim komentarima unutar FastComments kada pregledaju bilo koju nit komentara. Avatari su podržani putem Gravatara ili bilo kojeg dodatka za upravljanje avatarima u WordPressu budući da dodatak poziva `get_avatar_url`.