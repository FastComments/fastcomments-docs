Postoje dva načina da se zabrani komentarisanje korisnicima na vašem sajtu pomoću FastComments.

Prvi je ako već znate njihovu imejl adresu, možete je uneti na stranici <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">banned users</a>.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; alt='Lista banovanih korisnika pod Moderate Comments, sa banovanim imejl adresama i dugmetom za dodavanje nove zabrane'; title='Stranica Banned Users' app-screenshot-end]

Ovoj stranici se može pristupiti preko Moderate Comments -> Banned Users

Kada banujemo korisnika, možemo izabrati tip, bilo Permanent ili Permanent Shadow Ban:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; alt='Forma za novu zabranu sa poljem za imejl i izborom tipa zabrane Permanent ili Permanent Shadow Ban'; title='Banovanje korisnika' app-screenshot-end]

Drugi način da se banuje korisnik je klikom na dugme za banovanje koje se nalazi na svakom komentaru na stranici Comment Moderation.

Kada kliknemo na dugme za banovanje, biće vam prikazane neke opcije, gde možemo specificirati tip i trajanje zabrane.

### Email Aliases

Prilikom banovanja korisnika po imejlu, FastComments automatski ignoriše `+` aliase. Na primer, banovanje `user+alias@gmail.com` će
takođe banovati `user@gmail.com` i svaku drugu `+` varijantu te adrese, kao što je `user+other@gmail.com`.

### Shadow Bans

Shadow-ban je tip zabrane koji stvara utisak da je komentar ili glas korisnika uspešno sačuvan, iako zapravo nije. Ovo može biti
poželjno u određenim situacijama.

### Banovanje preko IP adrese

Osim ako tenant ne želi da se isključi iz ove opcije, FastComments podržava banovanje preko IP adrese čuvanjem heširane verzije IP adrese komentatora.

### Pretraga banovanih korisnika

Kada vaša lista naraste na stranicu ili dve, možete je suziti pomoću reda za pretragu iznad tabele.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .filter-form'; alt='Red za pretragu na stranici Banned Users sa padajućim menijem Search By, padajućim menijem Match i poljem Value'; title='Pretraga banovanih korisnika' app-screenshot-end]

Postoje tri kontrole:

- **Search By** bira u kom polju se traži: Any Field, Email, Name, Banned By, ili Banned For Saying. Poslednja četiri odgovaraju kolonama istog imena u tabeli.
- **Match** bira kako se poredi. **Contains** pronalazi vašu vrednost bilo gde u polju, a **Equals** podudara celo polje.
- **Value** je tekst koji se traži.

Svako polje se podudara bez obzira na velika i mala slova, tako da pretraga za `SPAMMER@EXAMPLE.COM` pronalazi ban sačuvan kao `spammer@example.com`.

Nekoliko stvari vrednih pažnje:

- **Banned For Saying** pretražuje tekst komentara zbog kojeg je korisnik banovan. Ovako pronalazite sve banovane zbog određene fraze.
- **Banned By** pretražuje ime moderatora koji je izdao zabranu, što je korisno za pregled odluka drugog moderatora.
- Wildcard banovi se čuvaju sa svojim `*`, tako da **Contains** pretraga za `bademail.com` pronalazi ban `*@bademail.com`.
- **Name** podudara ime prikazano u koloni Name, tako da pronalazi korisnika čak i ako je promenio ime od banovanja, i čak i ako ste napravili ban unosom imejl adrese a ime nije zabeleženo u tom trenutku. Ime zabeleženo na banu takođe i dalje podudara, tako da pretraga za staro ili trenutno ime radi.
- **Any Field** pretražuje imejl, ime, moderatora koji je banovao i tekst banovanog komentara zajedno.

Vaša pretraga je deo URL-a stranice, tako da možete deliti filtriranu listu sa drugim moderatorima na isti način na koji delite druge linkove za moderaciju. Listanje kroz rezultate zadržava primenjenu pretragu, pokretanje nove pretrage vas vraća na prvu stranicu, a **Clear** vraća na celu listu.