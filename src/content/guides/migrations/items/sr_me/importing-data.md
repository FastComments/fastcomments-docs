Iako FastComments podrška može pomoći pri migracijama, većina se može obaviti i pratiti lako bez intervencije osoblja podrške.

Izvorno podržavamo uvoz izvezenih podataka od sljedećih provajdera:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- WordPress (via the plugin)
- AnyComment (Via WordPress Import/Export)

Posjetom [ovdje](https://fastcomments.com/auth/my-account/manage-data/import) možemo otpremiti fajl koji sadrži podatke za migraciju.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; title='The Import Page Form' app-screenshot-end]

### Praćenje uvoza

FastComments koristi sistem za obradu poslova za procesuiranje uvoza i izvoza. Kada sistem preuzme vaš posao, periodično će prijavljivati status posla u korisničkom interfejsu za uvoz ili izvoz.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; title='Import Job Status' app-screenshot-end]

Imajte na umu da su statusi uvoza i izvoza vidljivi svim administratorima na nalogu.

Ako vaš posao zakaže, neće se automatski ponovo pokrenuti. Uvoz će morati biti pokušan ponovo. Ako bilo koji uvoz ili izvoz zakaže, naši sistemski administratori će automatski biti obaviješteni. Ako otkrijemo problem, kontaktiraćemo vas da vidimo možemo li pomoći.

### Ponovno pokretanje uvoza

Tokom nekih migracija, potrebno je pokrenuti uvoz više puta. Na primjer, često se izvodi početna migracija radi testiranja, a zatim se uvoz ponovo pokreće s najnovijim podacima prije konačnog prebacivanja.

Ponovni uvoz istog sadržaja **neće stvoriti duplikate**.

### Bezbednost podataka i isteka

Fajlovi za uvoz nisu na bilo koji način dostupni putem vanjskih zahtjeva, i fajlovi za uvoz se brišu iz našeg sistema čim uvoz bude završen.