FastComments vam omogućuje da zahtijevate od komentatora koji komentiraju prvi put da prihvate vaše Uvjete korištenja prije objave komentara.

Kada je omogućeno:
- **Anonimni korisnici** vidjet će potvrđni okvir UV-a svaki put kada komentiraju
- **Prijavljeni korisnici** vidjet će potvrđni okvir samo kod svog prvog komentara, ili kada ažurirate svoje UV

### Konfiguracija

Idite na stranicu za prilagodbu widgeta i omogućite potvrđni okvir "Require Terms of Service acceptance". Nakon što je omogućeno, vidjet ćete sljedeće opcije:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.tos-enabled'; selector = '#tos-options'; alt='Panel Uvjeta korištenja s odabirnikom načina UV teksta i poljem datuma zadnje izmjene'; title='Opcije Uvjeta korištenja' app-screenshot-end]

- **TOS Text Mode**: Prema zadanim postavkama, potvrđni okvir prikazuje "I agree to the Terms of Service and Privacy Policy" s poveznicama na oba dokumenta. Odaberite "Customize text per locale" da biste naveli vlastiti tekst za svaki jezik.
- **TOS Last Updated Date**: Kada ažurirate svoje Uvjete korištenja, postavite ovaj datum. Korisnici koji su prihvatili prije ovog datuma morat će ponovno prihvatiti.

### Kako radi

- Vremenska oznaka prihvaćanja UV-a pohranjuje se po korisniku i po komentaru
- Kada korisnik prihvati UV, datum se bilježi na njegovom korisničkom profilu (po najmoprimcu)
- Ako postavite datum "Last Updated" koji je nakon datuma prihvaćanja korisnika, morat će ponovno prihvatiti
- Za anonimne korisnike koji se ne mogu pratiti, potvrđni okvir pojavljuje se kod svake objave komentara