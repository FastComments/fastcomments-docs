---
FastComments SDK pruža tri API klijenta:

### PublicAPI - Metode sigurne za klijenta

`PublicAPI` sadrži metode koje je sigurno pozivati iz klijentskog koda (iOS/macOS aplikacije). Ove metode:
- Ne zahtijevaju API ključ
- Mogu koristiti SSO tokena za autentifikaciju
- Imaju ograničenje brzine po korisniku/uređaju
- Su pogodne za aplikacije usmjerene prema krajnjem korisniku

**Example use case**: Dohvaćanje i kreiranje komentara u vašoj iOS aplikaciji

### DefaultAPI - Metode na strani servera

`DefaultAPI` sadrži autentifikovane metode koje zahtijevaju API ključ. Ove metode:
- Zahtijevaju vaš FastComments API ključ
- Trebaju BITI pozivane SAMO iz server‑side koda
- Omogućavaju potpuni pristup vašim FastComments podacima
- Imaju ograničenje brzine po tenantu

**Example use case**: Administrativne operacije, izvoz podataka u velikim količinama, upravljanje korisnicima

### ModerationAPI - Metode moderacijskog kontrolnog panela

`ModerationAPI` pruža opsežan skup live i brzih moderacijskih API‑ja. Svaka metoda `ModerationAPI` prihvata parametar `sso` i može se autentifikovati putem SSO‑a ili putem FastComments.com sesijske kolačića.

**Example use case**: Izgradnja moderacijskog iskustva za moderatore vaše zajednice

**IMPORTANT**: Nikada ne izlažite svoj API ključ u klijentskom kodu. API ključevi trebaju biti korišćeni samo na serveru.
---