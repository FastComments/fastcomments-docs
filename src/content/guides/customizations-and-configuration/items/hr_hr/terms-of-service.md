---
FastComments vam omogućuje zahtijevati od komentatora koji komentiraju po prvi put da prihvate vaše Uvjete korištenja prije nego što pošalju komentar.

When enabled:
- **Anonimni korisnici** vidjet će potvrdni okvir za Uvjete korištenja svaki put kada komentiraju
- **Prijavljeni korisnici** vidjet će potvrdni okvir samo pri svom prvom komentaru, ili kada ažurirate svoje Uvjete korištenja

### Konfiguracija

Idite na stranicu za prilagodbu widgeta i omogućite potvrdni okvir "Require Terms of Service acceptance". Nakon omogućavanja, vidjet ćete sljedeće opcije:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.tos-enabled'; selector = '#tos-options'; title='Terms of Service Options' app-screenshot-end]

- **Način prikaza teksta Uvjeta korištenja**: Po zadanom potvrdni okvir prikazuje "I agree to the Terms of Service and Privacy Policy" s poveznicama na oba dokumenta. Odaberite "Customize text per locale" da biste unijeli vlastiti tekst za svaki jezik.
- **Datum posljednje izmjene Uvjeta korištenja**: Kada ažurirate svoje Uvjete korištenja, postavite ovaj datum. Korisnici koji su prihvatili prije tog datuma bit će obvezni ponovno prihvatiti.

### Kako to radi

- Vremenski pečat prihvaćanja Uvjeta pohranjuje se po korisniku i po komentaru
- Kada korisnik prihvati Uvjete, datum se bilježi u njihovom korisničkom profilu (per-tenant)
- Ako postavite datum "Posljednja izmjena" koji je nakon datuma prihvaćanja korisnika, morat će ponovno prihvatiti
- Za anonimne korisnike koje nije moguće pratiti, potvrdni okvir pojavljuje se pri svakom slanju komentara

---