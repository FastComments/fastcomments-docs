Iako FastComments podrška može pomoći s migracijama, većina njih može se izvesti i pratiti jednostavno bez ikakve intervencije
osoblja podrške.

Izvorno podržavamo uvoz izvoza sljedećih pružatelja usluga:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- Cusdis
- WordPress (putem dodatka ili XML ili CSV izvoza)
- AnyComment (putem WordPress Import/Export)

Navigacijom [ovdje](https://fastcomments.com/auth/my-account/manage-data/import) možemo učitati datoteku koja sadrži podatke za migraciju.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; alt='Stranica za uvoz FastComments s odabirom pružatelja usluge i poljima za učitavanje datoteke izvoza'; title='Obrazac stranice za uvoz' app-screenshot-end]

### Praćenje uvoza

FastComments koristi sustav obrade poslova za obradu uvoza i izvoza. Nakon što sustav preuzme vaš posao, on će
povremeno prijavljivati status posla u sučelju za uvoz ili izvoz.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; alt='Stranica uvoza koja prikazuje posao uvoza u tijeku i status koji prijavljuje sustav obrade poslova'; title='Status posla uvoza' app-screenshot-end]

Imajte na umu da status uvoza i izvoza mogu vidjeti svi administratori u računu.

Ako vaš posao ne uspije, neće se automatski ponovno pokrenuti. Uvoz će se morati ponovno pokušati. Ako bilo koji uvoz ili izvoz ne uspije,
naši administratori sustava automatski su obavješteni. Ako uočimo problem, kontaktirat ćemo vas kako bismo vidjeli možemo li pomoći.

### Ponovno pokretanje uvoza

Tijekom nekih migracija potrebno je pokrenuti uvoz više puta. Na primjer, uobičajeno je prvo izvršiti migraciju
za testiranje, a zatim ponovno pokrenuti uvoz s najnovijim podacima prije prebacivanja prekidača.

Ponovni uvoz istog sadržaja **neće stvoriti duplikate**.

### Sigurnost podataka i isteka

Datoteke uvoza nisu na bilo koji način dostupne putem vanjskih zahtjeva, a datoteke uvoza brišu se iz našeg sustava čim
se uvoz dovrši.