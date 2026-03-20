---
U administratorskom sučelju za Webhooks nalaze se gumbi `Send Test Payload` za svaku vrstu događaja (Create, Update, Delete). Create i Update događaji šalju lažni `WebhookComment` objekt, dok testiranje Delete šalje lažno tijelo zahtjeva s samo ID-jem.

## Provjera payloada

Prilikom testiranja vaše webhook integracije, provjerite da dolazni zahtjevi sadrže sljedeća zaglavlja:

1. **`token`** - Vaš API Secret
2. **`X-FastComments-Timestamp`** - Unix timestamp (sekunde)
3. **`X-FastComments-Signature`** - HMAC-SHA256 potpis

Koristite provjeru HMAC potpisa kako biste osigurali da su payloadi autentični.

## Alati za testiranje

Možete koristiti alate poput [webhook.site](https://webhook.site) ili [ngrok](https://ngrok.com) za pregled dolaznih webhook payloadova tijekom razvoja.

## Event Types

- **Create Event**: Pokreće se kada je kreiran novi komentar. Zadana metoda: PUT
- **Update Event**: Pokreće se kada je komentar uređivan. Zadana metoda: PUT
- **Delete Event**: Pokreće se kada se komentar izbriše. Zadana metoda: DELETE

Svaki događaj uključuje potpune podatke komentara u tijelu zahtjeva (pogledajte [Data Structures](/guide-webhooks.html#webhooks-structures) za format payloada).

---