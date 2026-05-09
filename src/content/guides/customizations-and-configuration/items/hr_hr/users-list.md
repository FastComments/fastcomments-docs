[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Po zadanim postavkama, FastComments ne prikazuje listu korisnika na stranici.

Možete prikazati listu osoba koje trenutno gledaju stranicu, uz widget za komentare. Lista se ažurira uživo kako korisnici dolaze i odlaze, i prikazuje njihovo ime, avatar i indikator online statusa.

Postoje tri opcije izgleda:

- `1` - Vrh: horizontalni red preklapajućih avatara prikazan iznad komentara.
- `2` - Lijevo: bočna traka s imenima i točkama za online status prikazana lijevo od widgeta.
- `3` - Desno: ista bočna traka prikazana desno od widgeta.

Postavite zastavicu **usersListLocation** za omogućavanje značajke:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

Po zadanim postavkama lista prikazuje samo korisnike koji su trenutno online. Da biste uključili i osobe koje su komentirale na stranici u prošlosti (ali trenutno je ne gledaju), postavite **usersListIncludeOffline** na true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Prošli komentatori se prikazuju bez zelene točkice online statusa kako bi bilo jasno tko je trenutno prisutan.

Korisnici s privatnim profilima prikazuju se s generičkim avatarom i oznakom "Private Profile" kako bi broj ostao točan bez otkrivanja identiteta.

Ovo se također može konfigurirati bez koda. Na stranici za prilagodbu widgeta, pogledajte opciju "Users List Location". Kada je lokacija postavljena na bilo što osim Off, ispod se pojavljuje potvrdni okvir "Include past commenters".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

Ako je više od 500 korisnika uživo, lista može biti ažurirana s zakašnjenjem do 30 sekundi.