FastComments vam omogućava da od prvog komentatora zahtevate prihvatanje vaših Uslova korišćenja pre slanja komentara.

Kada je omogućeno:
- **Anonimni korisnici** će videti polje za TOS svaki put kada komentarišu
- **Autentifikovani korisnici** će videti polje samo na svom prvom komentaru, ili kada ažurirate vaše Uslove korišćenja

### Configuration

Idite na stranicu za prilagođavanje widgeta i omogućite polje „Require Terms of Service acceptance“. Kada je omogućeno, videćete sledeće opcije:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.tos-enabled'; selector = '#tos-options'; alt='Panel uslova korišćenja koji prikazuje selektor režima teksta TOS-a i polje datuma poslednjeg ažuriranja'; title='Opcije uslova korišćenja' app-screenshot-end]

- **TOS Text Mode**: Podrazumevano, polje prikazuje „I agree to the Terms of Service and Privacy Policy“ sa linkovima ka oba dokumenta. Izaberite „Customize text per locale“ da biste obezbedili svoj tekst za svaki jezik.
- **TOS Last Updated Date**: Kada ažurirate svoje Uslove korišćenja, postavite ovaj datum. Korisnici koji su prihvatili pre ovog datuma biće ponovo zatraženi da prihvate.

### How It Works

- Vremenska oznaka prihvatanja TOS-a se čuva po korisniku i po komentaru
- Kada korisnik prihvati TOS, datum se beleži na njegovom korisničkom profilu (po tenantu)
- Ako postavite datum „Last Updated“ koji je posle datuma prihvatanja korisnika, moraće ponovo da prihvati
- Za anonimne korisnike koji se ne mogu pratiti, polje se pojavljuje na svakom slanju komentara