[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Po zadanim postavkama, FastComments ne prikazuje popis korisnika na stranici.

Možete prikazati popis osoba koje trenutno gledaju stranicu, uz widget za komentare. Popis se ažurira uživo dok se korisnici pridružuju i napuštaju stranicu, i prikazuje njihovo ime, avatar i indikator online statusa.

Postoje tri opcije izgleda:

- `1` - Vrh: vodoravni red preklapajućih avatara prikazan iznad komentara.
- `2` - Lijevo: bočna traka s imenima i online točkicama prikazana s lijeve strane widgeta.
- `3` - Desno: ista bočna traka prikazana s desne strane widgeta.

Postavite zastavicu **usersListLocation** da omogućite ovu značajku:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

Po zadanom popis prikazuje samo korisnike koji su trenutno online. Da biste također uključili osobe koje su u prošlosti komentirale na stranici (ali trenutno je ne gledaju), postavite **usersListIncludeOffline** na true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Korisnici koji su ranije komentirali prikazuju se bez zelene online točkice kako bi bilo jasno tko je trenutno prisutan.

Korisnici s privatnim profilima prikazuju se s generičkim avatarom i oznakom "Privatni profil" tako da broj ostane točan bez otkrivanja identiteta.

Ovo se također može konfigurirati bez koda. Na stranici za prilagodbu widgeta pogledajte opciju "Lokacija popisa korisnika":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-location'; title='Users List Location' app-screenshot-end]

Kada je lokacija postavljena na bilo što osim "Isključeno", ispod nje se prikazuje potvrdni okvir "Uključi komentatore iz prošlosti":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-include-offline'; title='Include Past Commenters' app-screenshot-end]