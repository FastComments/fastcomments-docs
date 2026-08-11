---
While FastComments podrška može pomoći pri migracijama, većinu je moguće izvesti i pratiti lako bez ikakve intervencije osoblja za podršku.

Izvorno podržavamo uvoz izvoznih podataka od sledećih provajdera:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- Cusdis
- WordPress (via the plugin)
- AnyComment (Via WordPress Import/Export)

Navigacijom [ovde](https://fastcomments.com/auth/my-account/manage-data/import) možemo otpremiti fajl koji sadrži podatke za migraciju.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; alt='FastComments stranica za uvoz sa izborom provajdera i poljima za otpremanje fajla za izvoz'; title='Obrazac stranice za uvoz' app-screenshot-end]

### Praćenje uvoza

FastComments koristi sistem za obradu poslova za obradu uvoza i izvozа. Kada sistem preuzme vaš posao, periodično će izveštavati o statusu posla u UI‑u za uvoz ili izvoz.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; alt='Stranica za uvoz koja prikazuje pokrenuti posao uvoza i status izveštavan od strane sistema za obradu poslova'; title='Status posla uvoza' app-screenshot-end]

Napomena: status uvoza i izvoza je vidljiv svim administratorima naloga.

Ako vaš posao ne uspe, neće se automatski ponovo pokrenuti. Uvoz će morati biti ponovo pokušan. Ako bilo koji uvoz ili izvoz ne uspe, naši sistemski administratori će biti automatski obavešteni. Ako identifikujemo problem, kontaktiraćemo vas da vidimo da li možemo pomoći.

### Ponovno pokretanje uvoza

Tokom nekih migracija, potrebno je pokrenuti uvoz više puta. Na primer, uobičajeno je uraditi prvu fazu migracije za testiranje, a zatim ponovo pokrenuti uvoz sa najnovijim podacima pre prebacivanja.

Ponovni uvoz istog sadržaja **neće kreirati duplikate**.

### Bezbednost podataka i isteka

Fajlovi za uvoz nisu na bilo koji način dostupni putem spoljašnjih zahteva, a fajlovi za uvoz se brišu iz našeg sistema čim se uvoz završi.

---