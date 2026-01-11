FastComments SDK pruža dvije vrste API krajnjih tačaka:

### `PublicAPI` - Krajnje tačke sigurne za klijentski kod

The `PublicAPI` contains endpoints that are safe to call from client-side code (iOS/macOS apps). These endpoints:
- Ne zahtijevaju API ključ
- Mogu koristiti SSO tokene za autentifikaciju
- Podliježu ograničenju broja zahtjeva po korisniku/uređaju
- Pogodne su za aplikacije usmjerene krajnjim korisnicima

**Primjer upotrebe**: Dohvatanje i kreiranje komentara u vašoj iOS aplikaciji

### `DefaultAPI` - Krajnje tačke za serversku stranu

The `DefaultAPI` contains authenticated endpoints that require an API key. These endpoints:
- Zahtijevaju vaš FastComments API ključ
- Treba da se pozivaju SAMO iz serverskog koda
- Pružaju potpuni pristup vašim FastComments podacima
- Podliježu ograničenju broja zahtjeva po tenantu

**Primjer upotrebe**: Administrativne operacije, izvoz podataka u velikom obimu, alati za moderaciju

**VAŽNO**: Nikada ne otkrivajte svoj API ključ u kodu na strani klijenta. API ključevi trebaju se koristiti samo na serverskoj strani.