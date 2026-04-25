Za većinu sajtova, najlakši način za dodavanje komentara je da prikačite polje `FastComments` na vaše tipove sadržaja. Idite na `Structure > Content types > [type] > Manage fields` i dodajte polje.

Svaki entitet koji ima ovo polje dobija:

- **prekidač statusa** koji omogućava urednicima da uključuju ili isključuju komentarisanje po entitetu.
- Opcionalni **prilagođeni identifikator** tako da možete koristiti stabilan ID koji nije vezan za Drupal entitetski put.

Glavni blok `FastComments Widget` zna za ovo polje, i preskočiće entitete koji ga već imaju prikačenog. Na taj način možete kombinovati komentare po entitetu sa blokom bez da vidite widget dvaput na istoj stranici.