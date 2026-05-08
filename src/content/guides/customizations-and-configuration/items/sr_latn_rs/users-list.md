[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Podrazumevano, FastComments ne prikazuje listu korisnika na stranici.

Možete prikazati listu osoba koje trenutno gledaju stranicu, pored komentarskog widgeta. Lista se ažurira uživo kako korisnici ulaze i napuštaju stranicu i prikazuje njihovo ime, avatar i indikator da su online.

Postoje tri opcije rasporeda:

- `1` - Top: horizontalni red preklapajućih avatara prikazan iznad komentara.
- `2` - Left: bočna traka sa imenima i tačkicama koje označavaju online status prikazana levo od widgeta.
- `3` - Right: ista bočna traka prikazana desno od widgeta.

Podesite **usersListLocation** zastavicu da omogućite ovu funkciju:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Prikaži listu korisnika na desnoj strani'; code-example-end]

Podrazumevano lista prikazuje samo korisnike koji su trenutno online. Da biste uključili i ljude koji su ranije komentarisali stranicu (ali trenutno je ne gledaju), podesite **usersListIncludeOffline** na true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Uključi prethodne komentatore'; code-example-end]

Prethodni komentatori se prikazuju bez zelene tačkice koja označava da su online, tako da je jasno ko je sada prisutan.

Korisnici sa privatnim profilima prikazuju se sa generičkim avatarom i oznakom "Privatan profil", tako da broj ostaje tačan bez otkrivanja identiteta.

Ovo se takođe može podesiti bez koda. Na stranici za prilagođavanje widgeta pogledajte opciju "Users List Location". Kada je lokacija postavljena na bilo šta drugo osim Off, ispod nje se pojavljuje čekboks "Include past commenters".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Podešavanja liste korisnika'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]