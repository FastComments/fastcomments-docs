U administratorskom delu za Webhooks postoje `Send Test Payload` dugmad za svaki tip događaja (Create, Update, Delete). Create i Update događaji šalju probni objekat WebhookComment, dok testiranje Delete šalje testno telo zahteva sa samo ID-jem.

## Provera payload-ova

Prilikom testiranja vaše webhook integracije, proverite da dolazni zahtevi sadrže sledeća zaglavlja:

1. **`token`** - Vaš API tajni ključ
2. **`X-FastComments-Timestamp`** - Unix vremenska oznaka (sekunde)
3. **`X-FastComments-Signature`** - HMAC-SHA256 potpis

Koristite verifikaciju HMAC potpisa kako biste osigurali da su payload-ovi autentični.

## Alati za testiranje

Možete koristiti alate kao što su [webhook.site](https://webhook.site) ili [ngrok](https://ngrok.com) za pregled dolaznih webhook payload-ova tokom razvoja.

## Tipovi događaja

- **Create Event**: Okida se kada je kreiran novi komentar. Podrazumevana metoda: PUT
- **Update Event**: Okida se kada je komentar izmenjen. Podrazumevana metoda: PUT
- **Delete Event**: Okida se kada je komentar obrisan. Podrazumevana metoda: DELETE

Svaki događaj uključuje kompletne podatke o komentaru u telu zahteva (vidi [Strukture podataka](/guide-webhooks.html#webhooks-structures) za format payload-a).

---