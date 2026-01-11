U Webhooks administraciji postoje dugmad `Send Test Payload` za svaki tip događaja (Create, Update, Delete). Događaji Create i Update šalju lažni WebhookComment objekat, dok testiranje Delete šalje lažno telo zahteva koje sadrži samo ID.

Test će izvršiti dva poziva da proveri statusni kod za scenarije "uspešan" (ispravan API ključ) i "neuspešan" (neispravan API ključ).

Kada test pošalje neispravan API ključ treba da vratite statusni kod 401 da bi test potpuno prošao. Ako ne proverite ispravno vrednost tokena, dobićete grešku.

Ovo je da bi se osiguralo da pravilno autentifikujete zahtev.