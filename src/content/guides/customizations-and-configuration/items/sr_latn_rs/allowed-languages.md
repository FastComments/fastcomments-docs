---
Podrazumevano, FastComments ne ograničava jezike koji se koriste za komentarisanje. 

Možda je poželjno ograničiti jezike koje zajednica koristi.

Ovo se može konfigurisati bez koda, na stranici za prilagođavanje widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.allowed-languages']; selector = '.allowed-languages'; alt='Selektor dozvoljenih jezika na stranici za prilagođavanje widgeta za ograničavanje koje jezike komentari mogu koristiti'; title='Dozvoljeni jezici' app-screenshot-end]

Sistem će analizirati njihov komentar i odrediti njegov jezik, a zatim ga uporediti sa listom dozvoljenih.

Ako je komentar napisan na jeziku koji nije dozvoljen, prikazaće se lokalizovana poruka o grešci. 

---