FastComments vam omogućava da zahtevate od korisnika koji komentarišu prvi put da prihvate vaše Uslove korišćenja pre nego što pošalju komentar.

Kada je omogućeno:
- **Anonimni korisnici** će videti polje za potvrdu Uslova korišćenja svaki put kada komentarišu
- **Autentifikovani korisnici** će videti polje za potvrdu samo pri njihovom prvom komentaru, ili kada ažurirate svoje Uslove korišćenja

### Konfiguracija

Idite na stranicu za prilagođavanje widgeta i omogućite opciju "Zahtevaj prihvatanje Uslova korišćenja". Kada je omogućeno, videćete sledeće opcije:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.tos-enabled'; selector = '#tos-options'; title='Terms of Service Options' app-screenshot-end]

- **Način prikaza TOS teksta**: Po podrazumevanju, polje za potvrdu prikazuje "Slažem se sa Uslovima korišćenja i Politikom privatnosti" sa linkovima ka oba dokumenta. Izaberite "Prilagodi tekst po jeziku" da obezbedite sopstveni tekst za svaki jezik.
- **Datum poslednjeg ažuriranja TOS-a**: Kada ažurirate svoje Uslove korišćenja, postavite ovaj datum. Korisnici koji su prihvatili pre ovog datuma biće u obavezi da ponovo prihvate.

### Kako funkcioniše

- Vreme prihvatanja TOS-a se beleži po korisniku i po komentaru
- Kada korisnik prihvati TOS, datum se evidentira u njegovom korisničkom profilu (per-tenant)
- Ako postavite datum "Last Updated" koji je posle datuma prihvatanja korisnika, oni će morati ponovo da prihvate
- Za anonimne korisnike koje nije moguće pratiti, polje za potvrdu se pojavljuje pri svakom slanju komentara