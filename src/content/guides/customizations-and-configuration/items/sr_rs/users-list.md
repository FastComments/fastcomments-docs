[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Podrazumevano, FastComments ne prikazuje listu korisnika na stranici.

Možete prikazati listu ljudi koji trenutno gledaju stranicu, uz widget za komentare. Lista se ažurira u realnom vremenu kako se korisnici pridružuju i napuštaju, i prikazuje njihovo ime, avatar i indikator online statusa.

Postoje tri opcije rasporeda:

- `1` - Gornji: horizontalni red preklapajućih avatara prikazan iznad komentara.
- `2` - Levi: bočna traka sa imenima i online tačkama prikazana levo od widgeta.
- `3` - Desni: ista bočna traka prikazana desno od widgeta.

Postavite **usersListLocation** zastavicu da omogućite ovu funkciju:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Prikaži listu korisnika desno'; code-example-end]

Podrazumevano, lista prikazuje samo korisnike koji su trenutno online. Da biste takođe uključili ljude koji su u prošlosti komentarisali stranicu (ali je trenutno ne gledaju), postavite **usersListIncludeOffline** na true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Uključi prethodne komentatore'; code-example-end]

Prethodni komentatori se prikazuju bez zelene online tačke kako bi bilo jasno ko je trenutno prisutan.

Korisnici sa privatnim profilima se prikazuju sa generičkim avatarom i oznakom „Privatan profil“ kako bi broj ostao tačan bez otkrivanja identiteta.

Ovo se takođe može konfigurisati bez koda. Na stranici za prilagođavanje widgeta, pogledajte opciju „Lokacija liste korisnika“. Kada je lokacija postavljena na bilo šta osim „Isključeno“, pojavljuje se potvrdno polje „Uključi prethodne komentatore“ ispod nje.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; alt='Lokacija liste korisnika postavljena na desno, sa potvrdnim poljem „Uključi prethodne komentatore“ prikazanim ispod.'; title='Podešavanja liste korisnika'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

Posle 500 aktivnih korisnika, lista može biti zastarela do 30 sekundi.