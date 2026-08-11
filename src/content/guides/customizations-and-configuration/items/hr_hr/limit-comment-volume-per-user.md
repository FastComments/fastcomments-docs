---
Prema zadanim postavkama, svaki korisnik može poslati najviše `5 komentara` u istoj minuti.

Ovo se prati prema ID-u korisnika, anonimnom ID-u korisnika i IP adresi (hashirana).

Ovo se može prilagoditi bez koda, na stranici za prilagodbu widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; alt='Polje maksimalnog broja komentara po minuti na stranici za prilagodbu widgeta, zadano na 5.'; title='Ograničavanje volumena komentara po korisniku' app-screenshot-end]

Napomena da ako koristite API za stvaranje komentara, možda ćete htjeti proslijediti originalnu `ip` adresu korisnika u zahtjevu našem backendu kako bi se ograničavanje brzine primijenilo
po korisniku i ne globalno na vaš račun.

---