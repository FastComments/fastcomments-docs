Za većinu sajtova, najlakši način da dodate komentare je da pripojite polje `FastComments` vašim tipovima sadržaja. Idite na `Structure > Content types > [type] > Manage fields` i dodajte polje.

Svaki entitet koji ima polje dobija:

- **prekidač statusa** koji omogućava urednicima da uključe ili isključe komentare za svaki entitet.
- Opcionalan **prilagođeni identifikator** tako da možete koristiti stabilan ID koji nije vezan za Drupal putanju entiteta.

Glavni blok `FastComments Widget` prepoznaje ovo polje i preskočiće entitete koji ga već imaju priloženog. Na taj način možete kombinovati komentare po entitetu sa blokom bez da vidite vidžet dva puta na istoj stranici.