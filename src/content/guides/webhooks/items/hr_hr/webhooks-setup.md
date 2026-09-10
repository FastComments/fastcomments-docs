---
Slijedite iste korake za `localhost` kao i za produkciju. Provjerite imate li postavljene produkcijske domene i API tajne.

Prvo, idite na [Webhooks admin](https://fastcomments.com/auth/my-account/manage-data/webhooks). Ovo je dostupno putem Manage Data -> Webhooks.

Stranica prikazuje sve webhookove na vašem računu:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; alt='Stranica administracije webhookova koja prikazuje svaki webhook s njegovim URL-om, događajem, domenom, metodom, statusom i brojem zakazanih događaja'; title='Popis webhookova'; cacheBuster = 'v4' app-screenshot-end]

Kliknite **New Webhook** da biste dodali jedan. Svaki webhook ima URL, jedan događaj komentara (kreiran, ažuriran ili izbrisan), domenu i HTTP metodu:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks/new'; selector = '.content'; alt='Obrazac za novi webhook s poljima za URL, događaj, domenu i HTTP metodu, plus gumb Send Test Payload'; title='Novi webhook'; cacheBuster = 'v4' app-screenshot-end]

Svaki webhook se isporučuje neovisno. Možete poslati isti događaj na više krajnjih točaka, a webhook ograničen na **All Domains** prima komentare sa svih domena čak i kada postoji webhook specifičan za domenu za isti događaj. Isti URL, događaj i domena ne mogu se dodati dva puta.

Prije spremanja, kliknite **Send Test Payload** kako biste provjerili prihvaća li krajnja točka potpisani zahtjev. Pogledajte sljedeći odjeljak, "Testing", za detalje.

Sa popisa možete uređivati, onemogućiti, ponovo omogućiti ili izbrisati webhook. Onemogućavanje zadržava zakazane događaje dok se webhook ne ponovo omogući; brisanje ih odbacuje.

Webhookove je također moguće kreirati putem API-ja, na primjer pomoću Zapiera. Oni se pojavljuju u istom popisu s izvorom **API**. Pogledajte Managing Webhooks via the API.

---