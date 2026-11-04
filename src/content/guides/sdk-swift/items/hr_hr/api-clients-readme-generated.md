FastComments SDK pruža tri API klijenta:

### PublicAPI - Metode sigurne za klijenta

`PublicAPI` sadrži metode koje je sigurno pozivati iz koda na strani klijenta (iOS/macOS aplikacije). Ove metode:
- Ne zahtijevaju API ključ
- Mogu koristiti SSO tokene za autentifikaciju
- Ograničene su brzinom po korisniku/uređaju
- Pogodne su za aplikacije namijenjene krajnjim korisnicima

**Primjer upotrebe**: Dohvaćanje i stvaranje komentara u vašoj iOS aplikaciji

### DefaultAPI - Metode na strani poslužitelja

`DefaultAPI` sadrži autentificirane metode koje zahtijevaju API ključ. Ove metode:
- Zahtijevaju vaš FastComments API ključ
- Trebaju se pozivati SAMO iz koda na strani poslužitelja
- Pružaju potpuni pristup vašim FastComments podacima
- Ograničene su brzinom po tenantu

**Primjer upotrebe**: Administrativne operacije, izvoz podataka u serijama, upravljanje korisnicima

### ModerationAPI - Metode moderatorske ploče

`ModerationAPI` pruža opsežan skup API-ja za moderaciju uživo i brzu moderaciju. Svaka metoda `ModerationAPI` prihvaća `sso` parametar i može se autentificirati putem SSO-a ili FastComments.com kolačića sesije.

**Primjer upotrebe**: Izgradnja iskustva moderacije za moderatore vaše zajednice

**VAŽNO**: Nikada ne otkrivajte svoj API ključ u kodu na strani klijenta. API ključevi trebaju se koristiti samo na strani poslužitelja.