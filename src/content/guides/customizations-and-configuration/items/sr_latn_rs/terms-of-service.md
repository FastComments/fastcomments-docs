FastComments vam omogućava da zahtevate od prvi put komentatora da prihvate vaše Uslove korišćenja pre slanja komentara.

Kada je omogućeno:
- **Anonimni korisnici** će videti polje za potvrdu Uslova korišćenja svaki put kada komentarišu
- **Prijavljeni korisnici** će videti polje za potvrdu samo na svom prvom komentaru, ili kada ažurirate svoje Uslove korišćenja

### Konfiguracija

Idite na stranicu za prilagođavanje vidžeta i uključite polje za potvrdu "Require Terms of Service acceptance". Kada je omogućeno, videćete sledeće opcije:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.tos-enabled'; selector = '#tos-options'; alt='Panel Uslova korišćenja koji prikazuje izbor režima teksta TOS-a i polje za datum poslednje izmene'; title='Opcije Uslova korišćenja' app-screenshot-end]

- **TOS Text Mode**: Podrazumevano, polje za potvrdu prikazuje "I agree to the Terms of Service and Privacy Policy" sa linkovima ka oba dokumenta. Izaberite "Customize text per locale" da biste obezbedili svoj tekst za svaki jezik.
- **TOS Last Updated Date**: Kada ažurirate svoje Uslove korišćenja, postavite ovaj datum. Korisnici koji su prihvatili pre ovog datuma biće dužni da ponovo prihvate.

### Kako funkcioniše

- Vremenska oznaka prihvatanja Uslova korišćenja čuva se po korisniku i po komentaru
- Kada korisnik prihvati Uslove korišćenja, datum se beleži na njegovom korisničkom profilu (po zakupcu)
- Ako postavite datum "Last Updated" koji je nakon datuma prihvatanja korisnika, oni će morati ponovo da prihvate
- Za anonimne korisnike koji se ne mogu pratiti, polje za potvrdu se pojavljuje pri svakom slanju komentara