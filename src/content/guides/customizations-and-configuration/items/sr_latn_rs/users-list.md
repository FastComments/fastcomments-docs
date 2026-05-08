[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Podrazumevano, FastComments ne prikazuje listu korisnika na stranici.

Možete prikazati listu ljudi koji trenutno gledaju stranicu, pored widgeta za komentare. Lista se ažurira u realnom vremenu kako korisnici ulaze i izlaze, i prikazuje njihovo ime, avatar i indikator prisutnosti.

Postoje tri opcije rasporeda:

- `1` - Gore: horizontalni red preklapajućih avatara prikazan iznad komentara.
- `2` - Levo: bočna traka sa imenima i online tačkicama prikazana levo od widgeta.
- `3` - Desno: ista bočna traka prikazana desno od widgeta.

Podesite zastavicu **usersListLocation** da omogućite funkciju:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

Podrazumevano lista prikazuje samo korisnike koji su trenutno online. Da biste uključili i ljude koji su prethodno komentarisali stranicu (ali trenutno je ne gledaju), podesite **usersListIncludeOffline** na true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Prethodni komentatori se prikazuju bez zelene tačkice za online status, kako bi bilo jasno ko je trenutno prisutan.

Korisnici sa privatnim profilima prikazuju se sa generičkim avatarom i oznakom "Privatan profil" tako da broj korisnika ostane tačan bez otkrivanja identiteta.

Ovo se takođe može podesiti bez koda. Na stranici za prilagođavanje widgeta, pogledajte opciju "Lokacija liste korisnika". Kada je lokacija postavljena na bilo šta osim "Isključeno", ispod se pojavljuje polje za potvrdu "Uključi prethodne komentatore".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

Kada ima više od 500 aktivnih korisnika, lista može zaostajati do 30 sekundi.