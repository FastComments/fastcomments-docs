Stranice za kreiranje i izmenu webhook-a imaju dugme `Send Test Payload` koje šalje zahtev na URL koji je trenutno u formi, bez obzira na to da li je sačuvan ili ne. Create i Update događaji šalju lažni WebhookComment objekat, dok testiranje Delete-a šalje lažni request body sa samo ID-jem.

## Verifikacija Payload-a

Kada testirate svoju webhook integraciju, proverite da dolazni zahtevi uključuju sledeće zaglavlja:

1. **`X-FastComments-Timestamp`** - Unix vremenski pečat (sekunde)
2. **`X-FastComments-Signature`** - HMAC-SHA256 potpis

Webhook-ovi kreirani pre uvođenja šeme potpisa takođe primaju **`token`** zaglavlje koje sadrži vaš API Secret. Novi webhook-ovi to ne primaju.

Koristite HMAC verifikaciju potpisa da biste osigurali da su payload-i autentični.

## Alati za testiranje

Možete koristiti alate kao što su [webhook.site](https://webhook.site) ili [ngrok](https://ngrok.com) da biste pregledali dolazne webhook payload-e tokom razvoja.

## Tipovi događaja

- **Create Event**: Aktivira se kada se kreira novi komentar.
- **Update Event**: Aktivira se kada se komentar izmeni.
- **Delete Event**: Aktivira se kada se komentar obriše.

Svaki webhook je vezan za jedan događaj i jednu HTTP metodu (POST, PUT ili DELETE). Svaki događaj uključuje kompletne podatke o komentaru u request body-ju (vidite [Data Structures](/guide-webhooks.html#webhooks-structures) za format payload-a).