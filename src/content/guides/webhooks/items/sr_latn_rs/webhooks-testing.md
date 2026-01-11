U administratorskom delu Webhooks postoje dugmad `Send Test Payload` za svaki tip događaja (Create, Update, Delete). Događaji Create i Update šalju dummy objekat WebhookComment, dok će testiranje Delete poslati dummy telo zahteva sa samo ID-jem.

Test će napraviti dva poziva da proveri statusni kod za scenarije "happy" (ispravan API ključ) i "sad" (nevažeći API ključ).

Kada test pošalje nevažeći API ključ, treba da vratite statusni kod 401 da bi test u potpunosti prošao. Ako ne proverite ispravno vrednost tokena, videćete grešku.

Ovo je da bi se osiguralo da pravilno autentifikujete zahtev.