Postoje dva načina da zabranite korisnicima da komentarišu na vašem sajtu koristeći FastComments.

Prvi način je, ako već znate njihov email, možete ga uneti na <a href="/auth/my-account/moderate-comments/banned-users" target="_blank">stranici zabranjenih korisnika</a>.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; title='Stranica zabranjenih korisnika' app-screenshot-end]

Na ovu stranicu se može pristupiti putem Moderacija komentara -> Zabranjeni korisnici

Kada želimo da zabranimo korisnika, možemo izabrati tip: ili Permanent ili Permanent Shadow Ban:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; title='Zabrana korisnika' app-screenshot-end]

Drugi način da zabranite korisnika je klikom na dugme za zabranu koje se nalazi na svakom komentaru na stranici Moderacija komentara.

Kada kliknete na dugme za zabranu, biće vam prikazane opcije u kojima možemo odrediti tip zabrane i njenog trajanja.

### Shadow banovi

Shadow-ban je tip zabrane koji stvara utisak da je korisnikov komentar ili glas uspešno sačuvan, iako to zapravo nije slučaj. Ovo može biti poželjno u određenim situacijama.

### Zabrana putem IP adrese

Osim ako tenant ne želi da se isključi, FastComments podržava zabranu po IP adresi čuvanjem hashovane verzije IP adrese komentatora.