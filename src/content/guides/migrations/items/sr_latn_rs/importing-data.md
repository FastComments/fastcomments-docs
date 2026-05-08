Iako FastComments podrška može pomoći pri migracijama, većinu njih je moguće izvršiti i pratiti lako bez ikakve intervencije
osoblja podrške.

Nativno podržavamo uvoz iz izvoznih fajlova sledećih provajdera:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- WordPress (via the plugin)
- AnyComment (Via WordPress Import/Export)

By navigating [here](https://fastcomments.com/auth/my-account/manage-data/import) we can upload the file containing the data to migrate.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; title='The Import Page Form' app-screenshot-end]

### Praćenje uvoza

FastComments koristi sistem obrade zadataka za obradu uvoza i izvoza. Kada sistem preuzme vaš zadatak, povremeno će izveštavati o statusu zadatka u UI za uvoz ili izvoz.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; title='Import Job Status' app-screenshot-end]

Imajte na umu da su statusi uvoza i izvoza vidljivi svim administratorima na nalogu.

Ako vaš zadatak ne uspe, on neće biti automatski ponovo pokrenut. Uvoz će morati biti ponovo pokušan. Ako bilo koji uvoz ili izvoz ne uspe,
naši sistemski administratori će automatski biti obavešteni. Ako identifikujemo problem, obratićemo vam se da vidimo možemo li pomoći.

### Ponovno pokretanje uvoza

Tokom nekih migracija, neophodno je pokretati uvoz više puta. Na primer, uobičajeno je izvršiti prvu migraciju radi testiranja, a zatim ponovo pokrenuti uvoz sa najnovijim podacima pre nego što se izvrši prebacivanje.

Ponovni uvoz istog sadržaja **neće napraviti duplikate**.

### Bezbednost podataka i istek

Fajlovi za uvoz nisu dostupni spoljnim zahtevima ni na koji način, i fajlovi za uvoz se brišu iz našeg sistema čim uvoz bude završen.