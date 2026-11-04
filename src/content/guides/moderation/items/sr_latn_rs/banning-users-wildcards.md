Moguće je banovati korisnike koji koriste određene email provajdere pomoću džoker znakova.

Na primer, ako ustanovite da su svi komentari sa adrese **@bademail.com** spam, možete jednostavno banovati
celog tog email provajdera unosom "*@bademail.com" u polje za email prilikom dodavanja banovanog korisnika.

Obratite pažnju na "*" pre @ u email adresi.

### Poddomeni

Ban domena pokriva i sve poddomene tog domena. Banovanje `*@bademail.com` banuje i
`someone@mail.bademail.com` i `someone@eu.mail.bademail.com`, tako da nije potrebno dodavati poseban ban za svaki poddomen.

Ako želite da banujete samo određeni poddomen, unesite taj poddomen umesto toga, na primer `*@mail.bademail.com`. Taj ban
ne utiče na `someone@bademail.com`.

### Banovanje domena iz komentara

Ne morate sami da kucate šablon. Kada banujete korisnika iz komentara na stranici Moderate Comments, dijalog za banovanje
ima polje za potvrdu "Ban All @domain Users" koje kreira isti ban `*@domain` za email domen komentatora.

### Podržani šabloni

Jedini podržani oblik džoker znaka je jedan `*` umesto celog dela imena, praćen znakom `@` i domenom. Ostali oblici
se odbijaju kada pokušate da ih sačuvate:

- `*@*.bademail.com` nije potreban, jer `*@bademail.com` već pokriva poddomene.
- `name*@bademail.com` i `*bademail.com` nisu podržani.