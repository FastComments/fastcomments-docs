Pratite iste korake za `localhost` kao i za produkciju. Proverite da imate podešene produkcijske domene i API Secrets.

Prvo, idite na [Webhooks admin](https://fastcomments.com/auth/my-account/manage-data/webhooks). Ovo je dostupno preko Manage Data -> Webhooks.

Stranica prikazuje svaki webhook na vašem nalogu:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; alt='Administratorska stranica webhooks koja prikazuje svaki webhook sa njegovim URL-om, događajem, domenom, metodom, statusom i brojem događaja u redu čekanja'; title='Lista Webhooks'; cacheBuster = 'v4' app-screenshot-end]

Kliknite na **New Webhook** da biste dodali novi. Svaki webhook ima URL, jedan događaj komentara (kreiran, ažuriran ili obrisan), domen i HTTP metodu:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks/new'; selector = '.content'; alt='Forma za novi webhook sa poljima URL, događaj, domen i HTTP metoda plus Send Test Payload'; title='Novi Webhook'; cacheBuster = 'v4' app-screenshot-end]

Svaki webhook se dostavlja nezavisno. Isti događaj možete poslati na više krajnjih tačaka, a webhook opsežen na **All Domains** prima komentare sa svakog domena čak i kada postoji webhook specifičan za domen za isti događaj.
Isti URL, događaj i domen ne mogu se dodati dva puta.

Pre čuvanja, kliknite na **Send Test Payload** da proverite da krajnja tačka prihvata potpisan zahtev. Pogledajte sledeću sekciju, "Testiranje", za detalje.

Iz liste možete izmeniti, onemogućiti, ponovo omogućiti ili obrisati webhook. Onemogućavanje zadržava događaje u redu čekanja dok se webhook ponovo ne omogući; brisanje ih odbacuje.

Webhooks se mogu kreirati i putem API-ja, na primer putem Zapier-a. Oni se pojavljuju u istoj listi sa izvorom **API**. Pogledajte Upravljanje webhooks-ima putem API-ja.