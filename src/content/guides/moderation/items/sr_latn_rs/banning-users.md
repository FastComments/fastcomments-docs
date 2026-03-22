Postoje dva načina da se korisnicima zabrani komentarisanje na vašem sajtu uz FastComments.

Prvi je da, ako već znate njihov email, možete ga uneti na strani <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">zabranjeni korisnici</a>.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; title='The Banned Users Page' app-screenshot-end]

Na ovu stranicu se može pristupiti putem Moderacija komentara -> Zabranjeni korisnici

Kada idemo da zabrani korisnika, možemo izabrati tip, bilo Permanent ili Permanent Shadow Ban:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; title='Banning a User' app-screenshot-end]

Drugi način za zabranu korisnika je klikom na dugme za zabranu koje se nalazi na svakom komentaru na stranici za moderaciju komentara.

Kada kliknemo dugme za zabranu, biće vam prikazane neke opcije, gde možemo odrediti tip zabrane i trajanje.

### Email aliasi

Kada se korisnik zabrani po emailu, FastComments automatski ignoriše `+` alias-e. Na primer, zabrana `user+alias@gmail.com` će
takođe zabraniti `user@gmail.com` i bilo koju drugu `+` varijaciju te adrese, kao što je `user+other@gmail.com`.

### Shadow banovi

Shadow-ban je tip zabrane koji stvara utisak da je komentar ili glas korisnika uspešno sačuvan, kada zapravo nije. Ovo može biti
poželjno u određenim situacijama.

### Zabrana putem IP adrese

Osim ako tenant ne želi da se isključi, FastComments podržava zabranu putem IP-a čuvanjem heširane verzije IP adrese komentatora.