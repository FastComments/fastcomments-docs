Po podrazumevanoj postavci, svaki korisnik može da pošalje do `5 comments` u istoj minuti.

Ovo se prati po user id, anon user id i ip address (hashed).

Ovo se može prilagoditi bez koda, na stranici za prilagođavanje widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; title='Limiting Comment Volume Per User' app-screenshot-end]

Imajte na umu da, ako koristite comment creation API, možda ćete želeti da prosledite originalnu `ip` adresu korisnika u zahtevu našem backendu tako da ograničenje brzine bude primenjeno
po korisniku, a ne globalno na vaš nalog.