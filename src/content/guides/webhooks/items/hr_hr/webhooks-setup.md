Slijedite iste korake za `localhost` kao i za produkciju. Provjerite imate li postavljene produkcijske domene i API Secrets.

Prvo, idite na [Administracija Webhookova](https://fastcomments.com/auth/my-account/manage-data/webhooks). Do toga se dolazi putem Upravljanje podacima -> Webhookovi.

Stranica za konfiguraciju izgleda ovako:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Webhooks Configuration'; cacheBuster = 'v3' app-screenshot-end]

Na ovoj stranici možete navesti krajnje točke za svaku vrstu događaja komentara.

Za svaku vrstu događaja obavezno kliknite Pošalji testni payload kako biste osigurali da ste pravilno postavili svoju integraciju. Pogledajte sljedeći odjeljak, "Testiranje", za detalje.