FastComments omogućuje da od prvog komentatora zahtijevate prihvaćanje vaših Terms of Service prije slanja komentara.

When enabled:
- **Anonymous users** će vidjeti TOS potvrdni okvir svaki put kad komentiraju
- **Authenticated users** će vidjeti potvrdni okvir samo na svom prvom komentaru, ili kada ažurirate svoje Terms of Service

### Configuration

Idite na stranicu prilagodbe widgeta i omogućite potvrdni okvir "Require Terms of Service acceptance". Nakon što je omogućeno, vidjet ćete sljedeće opcije:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.tos-enabled'; selector = '#tos-options'; alt='Panel Uvjeta pružanja usluge koji prikazuje odabir načina teksta TOS-a i polje datuma posljednjeg ažuriranja'; title='Opcije Uvjeta pružanja usluge' app-screenshot-end]

- **TOS Text Mode**: Po zadanom, potvrdni okvir prikazuje "I agree to the Terms of Service and Privacy Policy" s poveznicama na oba dokumenta. Odaberite "Customize text per locale" kako biste pružili vlastiti tekst za svaki jezik.
- **TOS Last Updated Date**: Kada ažurirate svoje Terms of Service, postavite ovaj datum. Korisnici koji su prihvatili prije tog datuma morat će ponovno prihvatiti.

### How It Works

- Vremenska oznaka prihvaćanja TOS-a pohranjuje se po korisniku i po komentaru
- Kada korisnik prihvati TOS, datum se bilježi u njegovom korisničkom profilu (per-tenant)
- Ako postavite datum "Last Updated" koji je nakon datuma prihvaćanja korisnika, morat će ponovno prihvatiti
- Za anonimne korisnike koji se ne mogu pratiti, potvrdni okvir se pojavljuje pri svakom slanju komentara