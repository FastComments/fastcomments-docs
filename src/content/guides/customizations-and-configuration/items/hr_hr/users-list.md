[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Po zadanim postavkama, FastComments ne prikazuje popis korisnika na stranici.

Možete prikazati popis osoba koje trenutno pregledavaju stranicu, uz widget za komentare. Popis se ažurira uživo kako korisnici dolaze i odlaze te prikazuje njihovo ime, avatar i indikator online statusa.

Postoje tri opcije rasporeda:

- `1` - Gore: horizontalni red preklapajućih avatara prikazan iznad komentara.
- `2` - Lijevo: bočna traka s imenima i online točkicama prikazana s lijeve strane widgeta.
- `3` - Desno: ista bočna traka prikazana s desne strane widgeta.

Postavite zastavicu **usersListLocation** da omogućite značajku:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

Po zadanim postavkama popis prikazuje samo korisnike koji su trenutno online. Da biste također uključili osobe koje su u prošlosti komentirale na stranici (ali trenutno je ne gledaju), postavite **usersListIncludeOffline** na true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Prošli komentatori se prikazuju bez zelene online točkice kako bi bilo jasno tko je trenutno prisutan.

Korisnici s privatnim profilima prikazuju se s generičkim avatarom i oznakom "Privatni profil" tako da broj ostaje točan bez otkrivanja identiteta.

Ovo se također može konfigurirati bez koda. Na stranici za prilagodbu widgeta pogledajte opciju "Users List Location". Kada je lokacija postavljena na nešto drugo osim Off, ispod će se pojaviti potvrdni okvir "Include past commenters".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]