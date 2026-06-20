The FastComments SDK pruža tri API klijenta:

### `PublicAPI` - Metode sigurne za klijentski kod

The `PublicAPI` sadrži metode koje je bezbedno pozivati iz klijentskog koda (iOS/macOS aplikacije). Ove metode:
- Ne zahtevaju API ključ
- Mogu koristiti SSO tokene za autentifikaciju
- Imaju ograničenje broja zahteva po korisniku/uređaju
- Pogodne su za aplikacije namenjene krajnjim korisnicima

**Primer upotrebe**: Dohvatanje i kreiranje komentara u vašoj iOS aplikaciji

### `DefaultAPI` - Metode za serversku stranu

The `DefaultAPI` sadrži autentifikovane metode koje zahtevaju API ključ. Ove metode:
- Zahtevaju vaš FastComments API ključ
- TREBAJU se pozivati ISKLJUČIVO iz serverskog koda
- Pružaju potpuni pristup vašim FastComments podacima
- Imaju ograničenje broja zahteva po tenantu

**Primer upotrebe**: Administrativne operacije, masovni izvoz podataka, upravljanje korisnicima

### `ModerationAPI` - Metode za moderatorski panel

The `ModerationAPI` sadrži metode koje pokreću moderatorski panel. Ove metode obuhvataju:
- **Moderacija komentara** - listanje, brojanje, pretraga, preuzimanje logova i izvoz komentara
- **Moderacione akcije** - ukloni/vrati komentare, označi/prijavi, postavi status za pregled/spam/odobrenje, upravljaj glasovima, i ponovo otvori/zatvori teme
- **Zabrane** - zabrani korisnika u vezi komentara, poništi zabrane, dohvati sažetke pre zabrane, proveri status zabrane i podešavanja, i pročitaj broj zabranjenih korisnika
- **Bedževi & poverenje** - dodeli/ukloni bedževe, listaj ručno dodeljene bedževe, dohvati/postavi faktor poverenja korisnika, i pročitaj korisnikov interni profil

Svaka `ModerationAPI` metoda prihvata `sso` parametar tako da moderatori mogu biti autentifikovani putem SSO.

**Primer upotrebe**: Izgradnja moderatorskog iskustva za moderatore vaše zajednice

**VAŽNO**: Nikada ne izlažite svoj API ključ u klijentskom kodu. API ključevi treba da se koriste samo na serverskoj strani.