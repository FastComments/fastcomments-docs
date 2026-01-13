FastComments SDK obezbeđuje dve vrste API krajnjih tačaka:

### PublicAPI - Krajnje tačke sigurne za klijentski kod

The `PublicAPI` contains endpoints that are safe to call from client-side code (iOS/macOS aplikacija). These endpoints:
- Ne zahtevaju API key
- Mogu koristiti SSO tokene za autentifikaciju
- Imaju ograničenja brzine po korisniku/uređaju
- Pogodne su za aplikacije namenjene krajnjim korisnicima

**Primer upotrebe**: Preuzimanje i kreiranje komentara u vašoj iOS aplikaciji

### DefaultAPI - Krajnje tačke za serversku stranu

The `DefaultAPI` contains authenticated endpoints that require an API key. These endpoints:
- Zahtevaju vaš FastComments API key
- Treba ih pozivati ISKLJUČIVO iz serverskog koda
- Pružaju pun pristup vašim FastComments podacima
- Imaju ograničenja brzine po tenantu

**Primer upotrebe**: Administrativne operacije, masovni izvoz podataka, alati za moderaciju

**IMPORTANT**: Nikada ne izlažite vaš API key u klijentskom kodu. API keys bi trebalo da se koriste samo na serverskoj strani.