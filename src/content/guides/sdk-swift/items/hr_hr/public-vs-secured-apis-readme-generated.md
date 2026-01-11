FastComments SDK pruža dvije vrste API krajnjih točaka:

### PublicAPI - Sigurne krajnje točke za klijent

The `PublicAPI` sadrži krajnje točke koje je sigurno pozivati iz klijentskog koda (iOS/macOS aplikacije). Ove krajnje točke:
- Ne zahtijevaju API ključ
- Mogu koristiti SSO tokene za autentifikaciju
- Imaju ograničenje brzine po korisniku/uređaju
- Pogodne su za aplikacije namijenjene krajnjim korisnicima

**Primjer upotrebe**: Dohvaćanje i stvaranje komentara u vašoj iOS aplikaciji

### DefaultAPI - Krajnje točke za serversku stranu

The `DefaultAPI` sadrži autentificirane krajnje točke koje zahtijevaju API ključ. Ove krajnje točke:
- Zahtijevaju vaš FastComments API ključ
- TREBAJU SE POZIVATI ISKLJUČIVO iz serverskog koda
- Omogućuju potpuni pristup vašim FastComments podacima
- Imaju ograničenje brzine po tenantu

**Primjer upotrebe**: Administrativne operacije, izvoz podataka u velikom opsegu, alati za moderaciju

**VAŽNO**: Nikada ne izlažite svoj API ključ u klijentskom kodu. API ključevi trebaju se koristiti isključivo na strani poslužitelja.