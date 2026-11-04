Moguće je banirati korisnike koji koriste određene email providere pomoću wildcards.

Na primjer, ako otkrijete da su svi komentari s adrese **@bademail.com** spam, možete jednostavno banirati
cijelog tog email providera unosom "*@bademail.com" u polje za email prilikom dodavanja baniranog korisnika.

Obratite pažnju na "*" ispred @ u email adresi.

### Poddomene

Ban domene također pokriva sve poddomene te domene. Baniranjem `*@bademail.com` baniraju se i
`someone@mail.bademail.com` i `someone@eu.mail.bademail.com`, tako da nije potrebno dodavati poseban ban za svaku poddomenu.

Ako želite banirati samo određenu poddomenu, unesite tu poddomenu, na primjer `*@mail.bademail.com`. Taj ban
ne utječe na `someone@bademail.com`.

### Baniranje domene iz komentara

Ne morate sami upisivati obrazac. Kada banirate korisnika iz komentara na stranici Moderate Comments, dijalog za baniranje
ima potvrdni okvir "Ban All @domain Users" koji stvara isti ban `*@domain` za email domenu komentatora.

### Podržani obrasci

Jedini podršani wildcard oblik je jedan `*` umjesto cijelog dijela imena, nakon čega slijedi `@` i domena. Ostali oblici
se odbijaju kada pokušate spremiti:

- `*@*.bademail.com` nije potreban, jer `*@bademail.com` već pokriva poddomene.
- `name*@bademail.com` i `*bademail.com` nisu podržani.