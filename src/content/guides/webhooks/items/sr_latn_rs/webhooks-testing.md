The new and edit webhook pages have a `Send Test Payload` button that sends a request to the URL currently in the form, whether or not it has been saved. The Create and Update events send a dummy WebhookComment object, while testing Delete will send a dummy request body with just an ID.

## Verifikacija payload‑ova

Kada testirate vašu webhook integraciju, proverite da dolazni zahtevi sadrže sledeća zaglavlja:

1. **`X-FastComments-Timestamp`** – Unix vremenski pečat (sekunde)  
2. **`X-FastComments-Signature`** – HMAC‑SHA256 potpis  

Webhook‑ovi kreirani pre uvođenja šeme potpisa takođe primaju **`token`** zaglavlje koje sadrži vaš API Secret. Novi webhook‑ovi to ne rade.

Koristite verifikaciju HMAC potpisa da biste osigurali da su payload‑ovi autentični.

## Alati za testiranje

Možete koristiti alate poput [webhook.site](https://webhook.site) ili [ngrok](https://ngrok.com) da pregledate dolazne webhook payload‑ove tokom razvoja.

## Tipovi događaja

- **Create Event**: Pokreće se kada se kreira novi komentar.  
- **Update Event**: Pokreće se kada se komentar izmeni.  
- **Delete Event**: Pokreće se kada se komentar obriše.  

Svaki webhook je vezan za jedan događaj i jednu HTTP metodu (POST, PUT ili DELETE). Svaki događaj uključuje kompletne podatke o komentaru u telu zahteva (pogledajte [Data Structures](/guide-webhooks.html#webhooks-structures) za format payload‑a).