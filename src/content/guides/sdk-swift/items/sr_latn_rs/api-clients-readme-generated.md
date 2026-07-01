The FastComments SDK pruža tri API klijenta:

### PublicAPI - Metode sigurne za klijent

`PublicAPI` sadrži metode koje su sigurne za pozivanje iz klijentskog koda (iOS/macOS aplikacije). Ove metode:
- Ne zahtevaju API ključ
- Mogu koristiti SSO tokene za autentifikaciju
- Imaju ograničenje brzine po korisniku/uređaju
- Su pogodne za aplikacije usmerene ka krajnjim korisnicima

**Primer upotrebe**: Preuzimanje i kreiranje komentara u vašoj iOS aplikaciji

### DefaultAPI - Metode na serverskoj strani

`DefaultAPI` sadrži autentifikovane metode koje zahtevaju API ključ. Ove metode:
- Zahtevaju vaš FastComments API ključ
- Treba ih POUZE pozivati iz serverskog koda
- Omogućavaju pun pristup vašim FastComments podacima
- Imaju ograničenje brzine po tenantu

**Primer upotrebe**: Administrativne operacije, grupni izvoz podataka, upravljanje korisnicima

### ModerationAPI - Metode kontrolne table moderatora

`ModerationAPI` pruža opsežan skup API‑ja za brzu i uživo moderaciju. Svaka `ModerationAPI` metoda prihvata parametar `sso` i može se autentifikovati putem SSO‑a ili FastComments.com sesijske kolačiće.

**Primer upotrebe**: Izgradnja iskustva moderacije za moderatore vaše zajednice

**VAŽNO**: Nikada nemojte izlagati vaš API ključ u kodu na klijentskoj strani. API ključevi treba da se koriste samo na serverskoj strani.