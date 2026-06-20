FastComments SDK pruža tri API klijenta:

### PublicAPI - Metode sigurne za klijenta

`PublicAPI` sadrži metode koje je sigurno pozivati iz klijentskog koda (iOS/macOS aplikacije). Ove metode:
- Ne zahtijevaju API ključ
- Mogu koristiti SSO tokene za autentikaciju
- Imaju ograničenje brzine po korisniku/uređaju
- Pogodne su za aplikacije usmjerene prema krajnjim korisnicima

**Primjer upotrebe**: Dohvaćanje i stvaranje komentara u vašoj iOS aplikaciji

### DefaultAPI - Metode na strani poslužitelja

`DefaultAPI` sadrži autentificirane metode koje zahtijevaju API ključ. Ove metode:
- Zahtijevaju vaš FastComments API ključ
- TREBAJU se pozivati SAMO iz koda na strani poslužitelja
- Pružaju potpuni pristup vašim FastComments podacima
- Imaju ograničenje brzine po tenantu

**Primjer upotrebe**: Administrativne operacije, izvoz podataka u bulk-u, upravljanje korisnicima

### ModerationAPI - Metode nadzorne ploče moderatora

`ModerationAPI` sadrži metode koje pokreću nadzornu ploču za moderatore. Ove metode obuhvaćaju:
- **Moderiranje komentara** - listanje, brojanje, pretraživanje, dohvaćanje zapisa i izvoz komentara
- **Moderacijske akcije** - uklanjanje/obnavljanje komentara, označavanje, postavljanje statusa za pregled/spam/odobrenje, upravljanje glasovima i ponovno otvaranje/zatvaranje niti
- **Zabrane** - zabrana korisnika u komentaru, poništavanje zabrana, dohvat sažetaka prije zabrane, provjera statusa zabrane i postavki, te čitanje broja zabranjenih korisnika
- **Značke i povjerenje** - dodjela/uklanjanje znački, listanje ručnih znački, dohvat/postavljanje faktora povjerenja korisnika i pregled internog profila korisnika

Svaka `ModerationAPI` metoda prihvaća `sso` parametar kako bi se moderatori mogli autentificirati putem SSO-a.

**Primjer upotrebe**: Izgradnja iskustva moderiranja za moderatore vaše zajednice

**VAŽNO**: Nikada ne otkrivajte svoj API ključ u klijentskom kodu. API ključevi trebaju se koristiti samo na strani poslužitelja.