FastComments vam omogućava da od prvog komentatora zahtevate prihvatanje vaših Uslova korišćenja pre slanja komentara.

Kada je omogućeno:
- **Anonimni korisnici** videće polje za potvrdu Uslova pri svakom komentaru
- **Autentifikovani korisnici** videće polje samo pri svom prvom komentaru, ili kada ažurirate svoje Uslove korišćenja

### Konfiguracija

Idite na stranicu za prilagođavanje vidžeta i omogućite polje „Zahtevaj prihvatanje Uslova korišćenja“. Kada je omogućeno, videćete sledeće opcije:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.tos-enabled'; selector = '#tos-options'; alt='Panel uslova korišćenja koji prikazuje selektor režima teksta TOS-a i polje datuma poslednjeg ažuriranja'; title='Opcije uslova korišćenja' app-screenshot-end]

- **TOS Text Mode**: Podrazumevano, polje prikazuje „Slažem se sa Uslovima korišćenja i Politikom privatnosti“ sa linkovima ka oba dokumenta. Izaberite „Prilagodi tekst po lokalu“ da biste obezbedili svoj tekst za svaki jezik.
- **TOS Last Updated Date**: Kada ažurirate svoje Uslove korišćenja, postavite ovaj datum. Korisnici koji su prihvatili pre ovog datuma biće ponovo upitani da prihvate.

### Kako funkcioniše

- Vremenska oznaka prihvatanja Uslova čuva se po korisniku i po komentaru
- Kada korisnik prihvati Uslove, datum se beleži na njegovom korisničkom profilu (po tenantu)
- Ako postavite datum „Poslednje ažuriranje“ koji je posle datuma prihvatanja korisnika, moraće ponovo da prihvati
- Za anonimne korisnike koji se ne mogu pratiti, polje se pojavljuje pri svakom slanju komentara