Site Analytics, ili jednostavno `Analytics` na kontrolnoj tabli, pruža pregled kako vaša zajednica koristi FastComments na svim vašim domenima.

FastComments pruža neke jedinstvene funkcije koje mnoge druge platforme ne nude, poput **uživo** izveštavanja o korisnicima na mreži na svakoj stranici i sortiranja stranica prema broju korisnika na mreži. Da biste to uradili, jednostavno posetite [Analytics Page](https://fastcomments.com/auth/my-account/analytics) i kliknite `Sort by users online` pod `Top Pages`.

I ukupni `Users Online` i `Top Pages` metrike su uživo i prijavljuju se bez kašnjenja.

`Top Pages` će podrazumevano sortirati prema broju komentara na svakoj stranici.

Na kraju, pruža se razbijanje ukupnih metrika vašeg zakupca, po danu, tokom vremena za:

- Page Loads
  - Ovo je broj puta kada je korisnik otvorio stranicu koja sadrži jedan ili više FastComments vidžeta. Ako stranica sadrži više vidžeta, onda će se ovaj broj povećati za broj vidžeta na toj stranici. Ako imate SPA, onda će se ovaj broj povećati svaki put kada aplikacija otvori novu nit komentara. Ovo se takođe odnosi na React Native biblioteku.
  - Ova metrika se takođe koristi za potrebe naplate u Flex planovima.
- Comments Left
  - Ovo uključuje sve komentare, bez obzira na status verifikacije ili odobrenja, ili da li su spam ili ne.
- Votes Left
  - Ovo je za broj ostavljenih glasova. Brojaće samo verifikovane glasove, osim ako je omogućeno anonimno glasanje.
- Accounts Created
  - Ova metrika je za slučajeve kada se doda novi SSO korisnik ili kada komentator prvi put komentariše sa FastComments koristeći vašu stranicu.

Ove metrike su skoro u realnom vremenu, sa kašnjenjem do jednog minuta.
