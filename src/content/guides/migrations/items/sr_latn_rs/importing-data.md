Iako FastComments Support može pomoći sa migracijama, većina se može lako izvršiti i pratiti bez ikakve intervencije
osoblja podrške.

We nativno podržavamo uvoz izvoznih datoteka od sledećih provajdera:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- WordPress (via the plugin)

By navigating [here](https://fastcomments.com/auth/my-account/manage-data/import) we can upload the file containing the data to migrate.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; title='The Import Page Form' app-screenshot-end]

### Praćenje uvoza

FastComments koristi sistem za obradu poslova za obradu uvoza i izvoza. Kada sistem preuzme vaš posao, on će
periodično izveštavati o statusu posla u korisničkom interfejsu za uvoz ili izvoz.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; title='Import Job Status' app-screenshot-end]

Imajte na umu da su statusi uvoza i izvoza vidljivi svim administratorima na nalogu.

Ako vaš posao zakaže, on se neće automatski ponovo pokrenuti. Uvoz će morati biti pokušан ponovo. Ako bilo koji uvoz ili izvoz zakaže,
naši sistemski administratori su automatski obavešteni. Ako identifikujemo problem, kontaktiraćemo vas da vidimo da li možemo pomoći.

### Ponovno pokretanje uvoza

Tokom nekih migracija, neophodno je pokretati uvoz više puta. Na primer, uobičajeno je prvo izvršiti probnu
migraciju za testiranje, a zatim ponovo pokrenuti uvoz sa najnovijim podacima pre nego što izvršite konačno prebacivanje.

Ponovni uvoz istog sadržaja **neće stvoriti duplikate**.

### Sigurnost podataka i rok čuvanja

Datoteke za uvoz nisu dostupne spoljnim zahtevima ni na koji način, i datoteke za uvoz se brišu iz našeg sistema čim
se uvoz završi.

---