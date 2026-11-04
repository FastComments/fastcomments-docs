Iako FastComments podrška može pomoći sa migracijama, većina njih može lako da se izvrši i prati bez ikakve intervencije
osoblja podrške.

Nativno podržavamo uvoz eksportova od sledećih provajdera:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- Cusdis
- WordPress (putem dodatka, ili XML ili CSV eksporta)
- AnyComment (putem WordPress Import/Export)

Navigacijom [ovde](https://fastcomments.com/auth/my-account/manage-data/import) možemo da otpremimo fajl koji sadrži podatke za migraciju.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; alt='Stranica za uvoz na FastComments sa izborom provajdera i poljima za otpremanje eksport fajla'; title='Forma stranice za uvoz' app-screenshot-end]

### Praćenje uvoza

FastComments koristi sistem za obradu poslova za obradu uvoza i eksporta. Kada sistem preuzme vaš posao, on će
periodično prijavljivati status posla u UI za uvoz ili eksport.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; alt='Stranica za uvoz koja prikazuje posao uvoza u toku i status koji prijavljuje sistem za obradu poslova'; title='Status posla uvoza' app-screenshot-end]

Imajte na umu da status za uvoz i eksport mogu da vide svi administratori na nalogu.

Ako vaš posao ne uspe, neće biti automatski ponovo pokrenut. Uvoz će morati ponovo da se pokuša. Ako bilo koji uvoz ili eksport ne uspe,
naši sistemski administratori su automatski obavešteni. Ako uočimo problem, kontaktiraćemo vas da vidimo kako možemo pomoći.

### Ponovno pokretanje uvoza

Tokom nekih migracija, potrebno je pokrenuti uvoz više puta. Na primer, uobičajeno je da se prvo uradi migracija
radi testiranja, a zatim ponovo pokrene uvoz sa najnovijim podacima pre prebacivanja prekidača.

Ponovni uvoz istog sadržaja **neće stvoriti duplikate**.

### Bezbednost podataka i istek

Fajlovi za uvoz nisu na bilo koji način dostupni spoljnim zahtevima, a fajlovi za uvoz se brišu iz našeg sistema čim
se uvoz završi.