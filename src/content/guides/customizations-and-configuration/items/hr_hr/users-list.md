[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Prema zadanim postavkama, FastComments ne prikazuje popis korisnika na stranici.

Možete prikazati popis ljudi koji trenutno pregledavaju stranicu, uz widget za komentare. Popis se ažurira u stvarnom vremenu kako se korisnici pridružuju i napuštaju stranicu, te prikazuje njihovo ime, avatar i indikator online statusa.

Postoje tri opcije rasporeda:

- `1` - Vrh: vodoravni red preklapajućih avatara prikazan iznad komentara.
- `2` - Lijevo: bočna traka s imenima i online točkicama prikazana s lijeve strane widgeta.
- `3` - Desno: ista bočna traka prikazana s desne strane widgeta.

Postavite zastavicu **usersListLocation** da omogućite ovu značajku:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Prikaži popis korisnika s desne strane'; code-example-end]

Prema zadanim postavkama popis prikazuje samo korisnike koji su trenutno online. Da biste također uključili osobe koje su u prošlosti komentirale stranicu (ali je trenutno ne pregledavaju), postavite **usersListIncludeOffline** na true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Uključi prošle komentatore'; code-example-end]

Prošli komentatori se prikazuju bez zelenog online kruga kako bi bilo jasno tko je trenutno prisutan.

Korisnici s privatnim profilima prikazani su generičkim avatarom i oznakom "Privatni profil" kako bi broj ostao točan, a identiteti ostali neotkriveni.

Ovo se također može konfigurirati bez koda. Na stranici za prilagodbu widgeta, pogledajte opciju "Users List Location". Kada je lokacija postavljena na bilo koju vrijednost osim Off, pojavljuje se potvrdni okvir "Include past commenters" ispod nje.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; alt='Lokacija popisa korisnika postavljena na desno, s potvrdnim okvirom \'Uključi prošle komentatore\' prikazanim ispod'; title='Postavke popisa korisnika'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

Nakon 500 aktivnih korisnika, popis može biti za najviše 30 sekundi zastario.