For de fleste websteder er den nemmeste måde at tilføje kommentarer at vedhæfte `FastComments`-feltet til dine indholdstyper. Gå til `Structure > Content types > [type] > Manage fields` og tilføj feltet.

Hver entitet, der har feltet, får:

- En **status-omskifter**, så redaktører kan slå kommentarer til og fra for hver entitet.
- En valgfri **tilpasset identifikator**, så du kan bruge et stabilt ID, der ikke er bundet til Drupal-entitetsstien.

Den primære `FastComments Widget` blok kender til dette felt, og vil springe entiteter over, som allerede har det vedhæftet. På den måde kan du blande kommentarer pr. entitet med blokken uden at se widget'en to gange på samme side.