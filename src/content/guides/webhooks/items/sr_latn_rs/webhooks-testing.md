U Webhooks admin postoje `Send Test Payload` dugmad za svaki tip događaja (Create, Update, Delete). Događaji Create i Update šalju dummy objekat WebhookComment, dok će testiranje Delete poslati dummy telo zahteva sa samo ID-jem.

## Provera payload-ova

Prilikom testiranja vaše webhook integracije, proverite da dolazni zahtevi uključuju sledeće zaglavlja:

1. **`token`** - Vaš API Secret
2. **`X-FastComments-Timestamp`** - Unix timestamp (sekunde)
3. **`X-FastComments-Signature`** - HMAC-SHA256 potpis

Koristite verifikaciju HMAC potpisa da biste osigurali da su payload-ovi autentični.

## Alati za testiranje

Možete koristiti alate kao što su [webhook.site](https://webhook.site) ili [ngrok](https://ngrok.com) da pregledate dolazne webhook payload-ove tokom razvoja.

## Tipovi događaja

- **Create Event**: Pokreće se kada je kreiran novi komentar. Podrazumevana metoda: PUT
- **Update Event**: Pokreće se kada je komentar izmenjen. Podrazumevana metoda: PUT
- **Delete Event**: Pokreće se kada je komentar obrisan. Podrazumevana metoda: DELETE

Svaki događaj uključuje kompletne podatke komentara u telu zahteva (pogledajte [Strukture podataka](/guides/webhooks/webhooks-structures) za format payload-a).