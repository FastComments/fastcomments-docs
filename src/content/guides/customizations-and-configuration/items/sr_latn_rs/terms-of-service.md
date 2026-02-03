FastComments vam omogućava da od korisnika koji komentarišu po prvi put zahtevate da prihvate vaše Uslove korišćenja pre slanja komentara.

Kada je omogućeno:
- **Anonimni korisnici** će videti polje za potvrdu Uslova korišćenja svaki put kada komentarišu
- **Prijavljeni korisnici** će videti polje za potvrdu samo pri svom prvom komentaru, ili kada ažurirate Uslove korišćenja

### Enabling Terms of Service

Navigate to the widget customization page and enable the "Require Terms of Service acceptance" checkbox:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-enabled'; title='Enable Terms of Service Checkbox' app-screenshot-end]

### Customizing the TOS Text

Po defaultu, polje za potvrdu prikazuje "Slažem se sa Uslovima korišćenja i Politikom privatnosti" sa linkovima ka oba dokumenta. Možete prilagoditi ovaj tekst po jeziku ako je potrebno:

1. Izaberite "Customize text per locale"
2. Izaberite jezik iz padajućeg menija i unesite svoj prilagođeni tekst

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-text-mode'; title='Customize TOS Text' app-screenshot-end]

### Updating Your Terms of Service

Kada ažurirate Uslove korišćenja, podesite datum "Last Updated". Korisnici koji su prihvatili Uslove pre ovog datuma biće u obavezi da ponovo prihvate:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-last-updated'; title='TOS Last Updated Date' app-screenshot-end]

### How It Works

- Vreme prihvatanja Uslova korišćenja se čuva po korisniku i po komentaru
- Kada korisnik prihvati Uslove korišćenja, datum se beleži u njihovom korisničkom profilu (per-tenant)
- Ako podesite datum "Last Updated" koji je posle datuma prihvatanja od strane korisnika, moraće ponovo da prihvate
- Za anonimne korisnike koje nije moguće pratiti, polje za potvrdu se pojavljuje pri svakom slanju komentara