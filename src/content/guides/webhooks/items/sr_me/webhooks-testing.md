U Webhooks administraciji postoje dugmad `Send Test Payload` za svaki tip događaja (Kreiranje, Ažuriranje, Brisanje). Događaji Kreiranja i Ažuriranja šalju lažni objekat `WebhookComment`, dok testiranje Brisanja šalje lažno tijelo zahtjeva sa samo jednim ID-jem.

## Provjera payload-ova

Prilikom testiranja vaše webhook integracije, provjerite da dolazni zahtjevi sadrže sljedeća zaglavlja:

1. **`token`** - Vaš API tajni ključ
2. **`X-FastComments-Timestamp`** - Unix vremenska oznaka (sekunde)
3. **`X-FastComments-Signature`** - HMAC-SHA256 potpis

Koristite verifikaciju HMAC potpisa da biste osigurali da su payload-ovi autentični.

## Alati za testiranje

Možete koristiti alate kao što su [webhook.site](https://webhook.site) ili [ngrok](https://ngrok.com) da pregledate dolazne webhook payload-ove tokom razvoja.

## Tipovi događaja

- **Događaj kreiranja**: Okida se kada se kreira novi komentar. Zadana metoda: PUT
- **Događaj ažuriranja**: Okida se kada se komentar izmijeni. Zadana metoda: PUT
- **Događaj brisanja**: Okida se kada se komentar obriše. Zadana metoda: DELETE

Svaki događaj uključuje kompletne podatke komentara u tijelu zahtjeva (pogledajte [Strukture podataka](/guides/webhooks/webhooks-structures) za format payload-a).