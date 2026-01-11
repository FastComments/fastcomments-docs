U Webhooks administraciji postoje gumbi `Send Test Payload` za svaki tip događaja (Create, Update, Delete). Događaji Create i Update šalju lažni WebhookComment objekt, dok će kod testiranja Delete poslati lažno tijelo zahtjeva s samo ID-jem.

Test će izvršiti dva poziva kako bi provjerio statusni kod za scenarije "happy" (ispravan API ključ) i "sad" (neispravan API ključ).

Kada test pošalje neispravan API ključ trebali biste vratiti statusni kod 401 da bi test potpuno prošao. Ako ne provjerite ispravno vrijednost tokena, vidjet ćete grešku.

Time se osigurava da pravilno autentificirate zahtjev.