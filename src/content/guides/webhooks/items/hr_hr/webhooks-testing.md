Stranice za stvaranje i uređivanje webhooks imaju gumb `Send Test Payload` koji šalje zahtjev na URL trenutno unesen u obrascu, bez obzira je li spremljen ili ne. Događaji Create i Update šalju lažni WebhookComment objekt, dok testiranje Delete šalje lažno tijelo zahtjeva samo s ID-om.

## Provjera Payloada

Prilikom testiranja vaše webhook integracije, provjerite uključuju li dolazni zahtjevi sljedeća zaglavlja:

1. **`X-FastComments-Timestamp`** - Unix vremenski pečat (sekunde)
2. **`X-FastComments-Signature`** - HMAC-SHA256 potpis

Webhooks stvoreni prije uvođenja sheme potpisa također primaju zaglavlje **`token`** koje sadrži vaš API Secret. Novi webhooks to ne primaju.

Koristite HMAC provjeru potpisa kako biste osigurali da su payloadi autentični.

## Alati za testiranje

Možete koristiti alate poput [webhook.site](https://webhook.site) ili [ngrok](https://ngrok.com) za pregled dolaznih webhook payloada tijekom razvoja.

## Vrste događaja

- **Create Event**: Pokreće se kada se stvori novi komentar.
- **Update Event**: Pokreće se kada se komentar uredi.
- **Delete Event**: Pokreće se kada se komentar izbriše.

Svaki webhook vezan je uz jedan događaj i jednu HTTP metodu (POST, PUT ili DELETE). Svaki događaj uključuje potpune podatke o komentaru u tijelu zahtjeva (vidite [Data Structures](/guide-webhooks.html#webhooks-structures) za format payloada).