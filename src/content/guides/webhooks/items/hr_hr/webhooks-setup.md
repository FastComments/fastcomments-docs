---
Slijedite iste korake za `localhost` kao i za `production`. Provjerite imate li postavljene produkcijske domene i API Secrets.

Prvo, otvorite [Administracija Webhookova](https://fastcomments.com/auth/my-account/manage-data/webhooks). Dostupno je putem izbornika Upravljanje podacima -> Webhooks.

Stranica za konfiguraciju izgleda ovako:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Webhooks Configuration' app-screenshot-end]

Na ovoj stranici možete navesti endpointe za svaku vrstu događaja vezanog uz komentare.

Za svaku vrstu događaja obavezno kliknite Pošalji testni payload kako biste provjerili je li vaša integracija ispravno postavljena. Za detalje pogledajte sljedeći odjeljak, "Testiranje".
---