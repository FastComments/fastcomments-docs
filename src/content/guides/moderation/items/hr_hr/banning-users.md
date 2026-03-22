Postoje dva načina na koja možete zabraniti korisnicima komentiranje na vašoj stranici pomoću FastComments.

Prvi je način: ako već znate njihovu e-poštu, možete je unijeti na stranicu <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">stranica zabranjenih korisnika</a>.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; title='The Banned Users Page' app-screenshot-end]

Ova stranica dostupna je putem Moderiranje komentara -> Zabranjeni korisnici

Kada želimo zabraniti korisnika, možemo odabrati tip, ili Trajna ili Trajna shadow zabrana:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; title='Banning a User' app-screenshot-end]

Drugi način da zabranite korisnika je klikom na gumb za zabranu koji se nalazi na svakom komentaru na stranici za moderiranje komentara.

Kada kliknete gumb za zabranu, prikazat će se neke opcije, gdje možemo odrediti tip zabrane i trajanje.

### Email Aliases

Prilikom zabrane korisnika po e-pošti, FastComments automatski zanemaruje `+` alias-e. Na primjer, zabrana `user+alias@gmail.com` također će zabraniti `user@gmail.com` i bilo koju drugu `+` varijaciju te adrese, kao što je `user+other@gmail.com`.

### Shadow Bans

Shadow-ban je tip zabrane koji čini da izgleda kao da je korisnikov komentar ili glas uspješno spremljen, kada zapravo nije. To može biti poželjno u određenim situacijama.

### Banning Via IP Address

Ako tenant ne želi isključiti ovu mogućnost, FastComments podržava zabranu putem IP-a pohranjivanjem heširane verzije IP adrese komentatora.