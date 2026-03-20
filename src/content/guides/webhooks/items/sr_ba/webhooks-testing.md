U Webhooks administraciji postoje dugmad `Send Test Payload` za svaki tip događaja (Kreiranje, Ažuriranje, Brisanje). Događaji Kreiranja i Ažuriranja šalju lažni objekat WebhookComment, dok testiranje Brisanja pošalje lažno tijelo zahtjeva koje sadrži samo ID.

## Verifikacija payloada

Tokom testiranja vaše webhook integracije, provjerite da dolazni zahtjevi sadrže sljedeća zaglavlja:

1. **`token`** - Vaš API tajni ključ
2. **`X-FastComments-Timestamp`** - Unix vremenska oznaka (sekunde)
3. **`X-FastComments-Signature`** - HMAC-SHA256 potpis

Koristite verifikaciju HMAC potpisa kako biste osigurali da su payloadi autentični.

## Alati za testiranje

Možete koristiti alate kao što su [webhook.site](https://webhook.site) ili [ngrok](https://ngrok.com) za pregled dolaznih webhook payloada tokom razvoja.

## Tipovi događaja

- **Događaj kreiranja**: Pokreće se kada se kreira novi komentar. Podrazumijevana metoda: PUT
- **Događaj ažuriranja**: Pokreće se kada se komentar izmijeni. Podrazumijevana metoda: PUT
- **Događaj brisanja**: Pokreće se kada se komentar obriše. Podrazumijevana metoda: DELETE

Svaki događaj uključuje pune podatke komentara u tijelu zahtjeva (pogledajte [Strukture podataka](/guide-webhooks.html#webhooks-structures) za format payloada).