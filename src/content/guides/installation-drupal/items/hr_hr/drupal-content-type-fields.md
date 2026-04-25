Za većinu stranica, najlakši način za dodavanje komentara je pridruživanje polja `FastComments` vašim tipovima sadržaja. Idite na `Structure > Content types > [type] > Manage fields` i dodajte polje.

Svaki entitet kojem je polje pridruženo dobiva:

- A **status toggle** koji urednicima omogućuje uključivanje ili isključivanje komentiranja za pojedini entitet.
- An optional **custom identifier** koji vam omogućuje upotrebu stabilnog ID-a koji nije vezan uz Drupal put entiteta.

Glavni blok `FastComments Widget` prepoznaje ovo polje i preskočit će entitete kojima je polje već pridruženo. Na taj način možete kombinirati komentare po entitetu s blokom bez prikazivanja widgeta dvaput na istoj stranici.