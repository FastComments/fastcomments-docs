U Webhooks admin panelu postoje dugmad `Send Test Payload` za svaki tip događaja (Create, Update, Delete). Događaji Create i Update šalju lažni objekat WebhookComment, dok testiranje Delete šalje lažno telo zahteva koje sadrži samo ID.

Test će napraviti dva poziva da verifikuje statusni kod za scenarije 'happy' (ispravan API Key) i 'sad' (neispravan API Key).

Kada test pošalje neispravan API Key, treba da vratite statusni kod 401 da bi test u potpunosti prošao. Ako ne proverite ispravno vrednost tokena, videćete grešku.

Ovo je da bi se osiguralo da pravilno autentifikujete zahtev.