U upravljačkom sučelju za Webhookse postoje gumbi `Send Test Payload` za svaku vrstu događaja (Create, Update, Delete). Događaji Create i Update šalju lažni WebhookComment objekt, dok testiranje Delete šalje lažno tijelo zahtjeva s samo ID-om.

## Provjera payloadova

Prilikom testiranja integracije webhooka, provjerite da dolazni zahtjevi uključuju sljedeća zaglavlja:

1. **`token`** - Vaš API tajni ključ
2. **`X-FastComments-Timestamp`** - Unix vremenska oznaka (sekunde)
3. **`X-FastComments-Signature`** - HMAC-SHA256 potpis

Koristite verifikaciju HMAC potpisa kako biste osigurali autentičnost payloadova.

## Alati za testiranje

Možete koristiti alate poput [webhook.site](https://webhook.site) ili [ngrok](https://ngrok.com) za pregled dolaznih webhook payloadova tijekom razvoja.

## Vrste događaja

- **Create Event**: Pokreće se kada se stvori novi komentar. Zadana metoda: PUT
- **Update Event**: Pokreće se kada se komentar uređuje. Zadana metoda: PUT
- **Delete Event**: Pokreće se kada se komentar obriše. Zadana metoda: DELETE

Svaki događaj uključuje potpune podatke komentara u tijelu zahtjeva (pogledajte [Strukture podataka](/guides/webhooks/webhooks-structures) za format payloada).