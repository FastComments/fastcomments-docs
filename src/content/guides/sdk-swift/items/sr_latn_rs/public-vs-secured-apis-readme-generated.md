FastComments SDK pruža dve vrste API endpointa:

### PublicAPI - Endpointi bezbedni za klijenta

`PublicAPI` sadrži endpointe koji su bezbedni za pozivanje iz klijentskog koda (iOS/macOS aplikacije). Ovi endpointi:
- Ne zahtevaju API ključ
- Mogu koristiti SSO tokene za autentifikaciju
- Imaju ograničenje broja zahteva po korisniku/uređaju
- Su pogodni za aplikacije namenjene krajnjim korisnicima

**Primer upotrebe**: Dohvatanje i kreiranje komentara u vašoj iOS aplikaciji

### DefaultAPI - Endpointi na serverskoj strani

`DefaultAPI` sadrži autentifikovane endpointe koji zahtevaju API ključ. Ovi endpointi:
- Zahtevaju vaš FastComments API ključ
- Treba da se pozivaju SAMO iz serverskog koda
- Pružaju pun pristup vašim FastComments podacima
- Imaju ograničenje broja zahteva po tenantu

**Primer upotrebe**: Administrativne operacije, masovni izvoz podataka, alati za moderaciju

**VAŽNO**: Nikada ne otkrivajte svoj API ključ u klijentskom kodu. API ključevi treba da se koriste samo na serverskoj strani.