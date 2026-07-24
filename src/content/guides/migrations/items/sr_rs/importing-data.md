While FastComments podrška može pomoći pri migracijama, većinu možete izvesti i nadgledati lako bez ikakve intervencije osoblja za podršku.

Nativno podržavamo uvoz i izvoz iz sledećih provajdera:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- Cusdis
- WordPress (preko dodatka)
- AnyComment (preko WordPress uvoza/izvoza)

Navigacijom na [ovde](https://fastcomments.com/auth/my-account/manage-data/import) možete otpremiti fajl koji sadrži podatke za migraciju.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; title='Forma stranice za uvoz' app-screenshot-end]

### Praćenje uvoza

FastComments koristi sistem za obradu poslova prilikom obrade uvoza i izvoza. Kada sistem preuzme vaš posao, periodično će izveštavati o statusu posla u UI‑ju za uvoz ili izvoz.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; title='Status posla uvoza' app-screenshot-end]

Napomena: status uvoza i izvoza je vidljiv svim administratorima naloga.

Ako vaš posao ne uspe, neće se automatski ponovo pokrenuti. Uvoz će morati biti ponovo pokušan. Ako bilo koji uvoz ili izvoz ne uspe, naši sistemski administratori će biti automatski obavešteni. Ako identifikujemo problem, kontaktiraćemo vas da vidimo da li možemo pomoći.

### Ponovno pokretanje uvoza

Tokom nekih migracija, neophodno je pokrenuti uvoz više puta. Na primer, uobičajeno je uraditi prvi prolaz migracije radi testiranja, a zatim ponovo pokrenuti uvoz sa najnovijim podacima pre nego što se pređe na finalnu fazu.

Ponovni uvoz istog sadržaja **neće kreirati duplikate**.

### Bezbednost podataka i isteka

Fajlovi za uvoz nisu dostupni putem spoljašnjih zahteva ni na koji način, a fajlovi za uvoz se brišu iz našeg sistema čim uvoz bude završen.