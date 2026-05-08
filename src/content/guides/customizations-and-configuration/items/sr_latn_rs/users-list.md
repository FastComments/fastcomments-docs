[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Podrazumevano, FastComments ne prikazuje listu korisnika na stranici.

Možete prikazati listu osoba koje trenutno gledaju stranicu, pored widgeta za komentare. Lista se ažurira uživo dok se korisnici pridružuju i napuštaju, i prikazuje njihovo ime, avatar i indikator online statusa.

Postoje tri opcije izgleda:

- `1` - Gore: horizontalni red preklapajućih avatara prikazan iznad komentara.
- `2` - Levo: bočna traka sa imenima i indikatorima online prikazana levo od widgeta.
- `3` - Desno: ista bočna traka prikazana desno od widgeta.

Postavite zastavicu **usersListLocation** da omogućite funkciju:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Prikaži listu korisnika sa desne strane'; code-example-end]

Podrazumevano lista prikazuje samo korisnike koji su trenutno online. Da biste uključili i osobe koje su komentarisale na stranici u prošlosti (ali trenutno ne gledaju stranicu), postavite **usersListIncludeOffline** na true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Uključi prethodne komentatore'; code-example-end]

Prethodni komentatori se prikazuju bez zelene tačke koja označava online status, kako bi bilo jasno ko je trenutno prisutan.

Korisnici sa privatnim profilima se prikazuju sa generičkim avatarom i oznakom 'Privatni profil' tako da broj ostane tačan bez otkrivanja identiteta.

Ovo se može konfigurisati i bez koda. Na stranici za prilagođavanje widgeta, pogledajte opciju 'Lokacija liste korisnika':

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-location'; title='Lokacija liste korisnika' app-screenshot-end]

Kada je lokacija postavljena na bilo šta osim Isključeno, ispod se prikazuje checkbox 'Uključi prethodne komentatore':

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-include-offline'; title='Uključi prethodne komentatore' app-screenshot-end]

---