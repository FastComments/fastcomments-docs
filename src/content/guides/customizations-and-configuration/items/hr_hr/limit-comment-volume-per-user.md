---
Po zadanim postavkama, svaki korisnik može poslati do `5 comments` u istoj minuti.

Ovo se prati po ID-u korisnika, anonimnom ID-u korisnika i IP adresi (heširano).

Ovo se može prilagoditi bez koda, na stranici za prilagodbu widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; title='Limiting Comment Volume Per User' app-screenshot-end]

Imajte na umu da, ako koristite comment creation API, možda ćete htjeti proslijediti korisnikovu izvornu `ip` adresu u zahtjevu našem backendu kako bi se ograničavanje stope primjenjivalo po korisniku, a ne globalno na vaš račun.

---