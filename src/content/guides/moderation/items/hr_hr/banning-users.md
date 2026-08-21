Postoje dva načina za zabranjivanje korisnika da komentiraju na vašoj web stranici uz FastComments.

Prvi je ako već znate njihov e‑mail, možete ga unijeti na <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">zabranjene korisnike</a> stranicu.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; alt='Popis zabranjenih korisnika pod Moderate Comments, s adresama zabranjenih e‑mailova i gumbom za dodavanje nove zabrane'; title='Stranica zabranjenih korisnika' app-screenshot-end]

Ovu stranicu možete pristupiti putem Moderate Comments -> Zabranjeni korisnici

Kada idemo zabraniti korisnika, možemo odabrati vrstu, bilo Permanentna ili Permanentna sjenovita zabrana:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; alt='Obrazac za novu zabranu s poljem za e‑mail i izborom vrste zabrane: Permanentna ili Permanentna sjenovita zabrana'; title='Zabranjivanje korisnika' app-screenshot-end]

Drugi način za zabranjivanje korisnika je klikom na gumb za zabranu koji se nalazi na svakom komentaru na stranici Comment Moderation.

Kada kliknete gumb za zabranu, bit će vam prikazane neke opcije, gdje možemo odrediti vrstu zabrane i trajanje.

### Alias e‑mailova

Pri zabranjivanju korisnika putem e‑maila, FastComments automatski zanemaruje `+` alias-e. Na primjer, zabranom `user+alias@gmail.com` također će se zabraniti `user@gmail.com` i bilo koja druga `+` varijacija te adrese, poput `user+other@gmail.com`.

### Sjenovite zabrane

Sjenovita zabrana je vrsta zabrane koja čini da se čini da je komentar ili glas korisnika uspješno spremljen, iako to nije bio slučaj. To može biti poželjno u određenim situacijama.

### Zabrana putem IP adrese

Osim ako najamnik ne želi isključiti ovu opciju, FastComments podržava zabranu putem IP adrese pohranjivanjem hashirane verzije IP adrese komentatora.

### Pretraživanje zabranjenih korisnika

Kad vaš popis naraste iznad jedne ili dvije stranice, možete ga suziti pomoću retka za pretraživanje iznad tablice.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .filter-form'; alt='Redak za pretraživanje na stranici zabranjenih korisnika s padajućim izbornikom \'Search By\', padajućim izbornikom \'Match\' i unosom \'Value\''; title='Pretraživanje zabranjenih korisnika' app-screenshot-end]

Postoje tri kontrole:

- **Search By** odabire u kojem polju tražiti: Any Field, Email, Name, Banned By ili Banned For Saying. Posljednja četiri odgovaraju stupcima istog naziva u tablici.
- **Match** odabire način usporedbe. **Contains** pronalazi vašu vrijednost bilo gdje u polju, a **Equals** podudara cijelo polje.
- **Value** je tekst koji se traži.

Svako polje se podudara neovisno o veličini slova, pa pretraživanje za `SPAMMER@EXAMPLE.COM` pronalazi zabranu pohranjenu kao `spammer@example.com`.

Nekoliko stvari koje vrijedi znati:

- **Banned For Saying** pretražuje tekst komentara zbog kojeg je korisnik zabranjen. Tako pronalazite sve zabranjene zbog određene fraze.
- **Banned By** pretražuje ime moderatora koji je izdao zabranu, što je korisno za pregled odluka drugog moderatora.
- Zabrane s džokerom pohranjuju se s `*`, pa **Contains** pretraživanje za `bademail.com` pronalazi `*@bademail.com` zabranu.
- **Name** podudara se s imenom prikazanim u stupcu Name, pa pronalazi korisnika čak i ako je promijenio ime od vremena zabrane, i čak i ako ste zabranu kreirali unosom e‑mail adrese i tada nije bilo zapisanog imena. Ime zapisano uz zabranu također se podudara, pa pretraživanje po starom ili trenutnom imenu funkcionira.
- **Any Field** pretražuje e‑mail, ime, moderatora koji je zabranio i tekst zabranjenog komentara zajedno.

Vaša pretraga je dio URL-a stranice, pa možete podijeliti filtrirani popis s drugim moderatorima na isti način kao što dijelite druge moderacijske poveznice. Straničenje kroz rezultate zadržava primijenjenu pretragu, započinjanje nove pretrage vraća vas na prvu stranicu, a **Clear** vraća na cijeli popis.