FastComments vam omogućava da zahtijevate od korisnika koji prvi put komentarišu da prihvate vaše Uslove korištenja prije nego što pošalju komentar.

When enabled:
- **Anonimni korisnici** će vidjeti potvrdni okvir za Uslove korištenja svaki put kada komentarišu
- **Prijavljeni korisnici** će vidjeti potvrdni okvir samo pri svom prvom komentaru, ili kada ažurirate svoje Uslove korištenja

### Konfiguracija

Idite na stranicu za prilagođavanje widgeta i uključite potvrdni okvir "Require Terms of Service acceptance". Nakon uključivanja, videćete sljedeće opcije:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.tos-enabled'; selector = '#tos-options'; title='Terms of Service Options' app-screenshot-end]

- **Način prikaza teksta Uslova (TOS Text Mode)**: Po zadanom, potvrdni okvir prikazuje "I agree to the Terms of Service and Privacy Policy" sa linkovima na oba dokumenta. Odaberite "Customize text per locale" da unesete vlastiti tekst za svaki jezik.
- **Datum posljednje izmjene Uslova (TOS Last Updated Date)**: Kada ažurirate svoje Uslove korištenja, postavite ovaj datum. Korisnici koji su prihvatili prije ovog datuma biće obavezni da ponovo prihvate.

### Kako to funkcioniše

- Vremenska oznaka prihvatanja Uslova se čuva po korisniku i po komentaru
- Kada korisnik prihvati Uslove, datum se bilježi na njihovom korisničkom profilu (per-tenant)
- Ako postavite datum "Last Updated" koji je nakon datuma prihvatanja korisnika, oni će morati ponovo prihvatiti
- Za anonimne korisnike koji se ne mogu pratiti, potvrdni okvir se pojavljuje pri svakom slanju komentara