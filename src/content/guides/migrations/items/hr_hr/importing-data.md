Iako podrška FastComments može pomoći kod migracija, većinu je moguće izvesti i nadzirati lako bez intervencije osoblja za podršku.

Nativno podržavamo uvoz izvoznih datoteka sljedećih pružatelja usluga:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- WordPress (putem dodatka)

Navigiranjem [ovdje](https://fastcomments.com/auth/my-account/manage-data/import) možemo učitati datoteku koja sadrži podatke za migraciju.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; title='The Import Page Form' app-screenshot-end]

### Praćenje uvoza

FastComments koristi sustav obrade zadataka za obradu uvoza i izvoza. Nakon što sustav preuzme vaš zadatak, povremeno će prijavljivati status zadatka u sučelju za uvoz ili izvoz.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; title='Import Job Status' app-screenshot-end]

Imajte na umu da status uvoza i izvoza mogu vidjeti svi administratori na računu.

Ako vaš zadatak ne uspije, neće se automatski ponovno pokrenuti. Uvoz će se morati pokušati ponovno. Ako bilo koji uvoz ili izvoz zakaže, administratori našeg sustava automatski se obavještavaju. Ako identificiramo problem, kontaktirat ćemo vas kako bismo vidjeli možemo li pomoći.

### Ponovno pokretanje uvoza

Tijekom nekih migracija potrebno je pokretati uvoz više puta. Na primjer, uobičajeno je napraviti prvu probnu migraciju za testiranje, a potom ponovno pokrenuti uvoz s najnovijim podacima prije nego što prebacite proizvodnju.

Ponovni uvoz istog sadržaja **neće stvoriti duplikate**.

### Sigurnost podataka i brisanje

Datoteke za uvoz nisu dostupne putem vanjskih zahtjeva ni na koji način, a datoteke za uvoz brišu se iz našeg sustava čim uvoz završi.

---