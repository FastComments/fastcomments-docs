Iako podrška FastComments-a može pomoći pri migracijama, većinu se može izvesti i pratiti jednostavno bez intervencije osoblja podrške.

Nativno podržavamo uvoz iz sljedećih pružatelja usluga:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- WordPress (putem dodatka)
- AnyComment (putem WordPress uvoza/izvoza)

Idući na [ovdje](https://fastcomments.com/auth/my-account/manage-data/import) možemo učitati datoteku koja sadrži podatke za migraciju.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; title='The Import Page Form' app-screenshot-end]

### Praćenje uvoza

FastComments koristi sustav obrade poslova za obradu uvoza i izvoza. Nakon što sustav preuzme vaš posao, povremeno će izvještavati o statusu posla u sučelju za uvoz ili izvoz.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; title='Import Job Status' app-screenshot-end]

Imajte na umu da su statusi uvoza i izvoza vidljivi svim administratorima na računu.

Ako vaš posao ne uspije, neće se automatski ponovno pokrenuti. Uvoz će se morati pokušati ponovno. Ako bilo koji uvoz ili izvoz ne uspije, naši administratori sustava bit će automatski obaviješteni. Ako utvrdimo problem, kontaktirat ćemo vas da vidimo možemo li pomoći.

### Ponovno pokretanje uvoza

Tijekom nekih migracija potrebno je pokretati uvoz više puta. Na primjer, uobičajeno je napraviti prvu migraciju za testiranje, a zatim ponovno pokrenuti uvoz s najnovijim podacima prije prebacivanja.

Ponovni uvoz istog sadržaja **neće stvoriti duplikate**.

### Sigurnost podataka i isteklost

Datoteke za uvoz nisu na bilo koji način dostupne putem vanjskih zahtjeva, a datoteke za uvoz brišu se iz našeg sustava čim se uvoz dovrši.