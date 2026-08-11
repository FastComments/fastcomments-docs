Podrazumevano, svaki korisnik može poslati do `5 comments` u istoj minuti.

Ovo se prati po ID‑u korisnika, anonimnom ID‑u korisnika i IP adresi (hashovana).

Ovo se može prilagoditi bez koda, na stranici za prilagođavanje widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; alt='Polje maksimalnog broja komentara po minuti na stranici za prilagođavanje widgeta, podrazumevano postavljeno na 5'; title='Ograničavanje broja komentara po korisniku' app-screenshot-end]

Napomena: ako koristite API za kreiranje komentara, možda ćete želeti da prosledite originalnu `ip` adresu korisnika u zahtevu našem backendu kako bi se ograničavanje brzine primenilo po korisniku, a ne globalno na vaš nalog.