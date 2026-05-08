[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Po defaultu, FastComments ne prikazuje listu korisnika na stranici.

Možete prikazati listu ljudi koji trenutno gledaju stranicu, pored widgeta za komentare. Lista se ažurira uživo dok se korisnici priključuju i odlaze, i prikazuje njihovo ime, avatar i indikator prisutnosti.

Postoje tri opcije rasporeda:

- `1` - Gore: horizontalni red preklapajućih avatara prikazan iznad komentara.
- `2` - Levo: bočna traka sa imenima i tačkicama koje označavaju prisutnost prikazana sa leve strane widgeta.
- `3` - Desno: ista bočna traka prikazana sa desne strane widgeta.

Postavite zastavicu **usersListLocation** da omogućite ovu funkciju:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Prikaži listu korisnika s desne strane'; code-example-end]

Po defaultu lista prikazuje samo korisnike koji su trenutno online. Da biste uključili i ljude koji su ranije komentarisali na stranici (ali trenutno je ne gledaju), postavite **usersListIncludeOffline** na true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Uključi prethodne komentatore'; code-example-end]

Prethodni komentatori se prikazuju bez zelene tačkice koja označava prisutnost, tako da je jasno ko je trenutno prisutan.

Korisnici sa privatnim profilima prikazuju se sa generičkim avatarom i oznakom "Privatni profil", tako da broj ostaje tačan bez otkrivanja identiteta.

Ovo se takođe može podesiti bez koda. Na stranici za prilagođavanje widgeta pogledajte opciju "Lokacija liste korisnika". Kada je lokacija postavljena na bilo šta osim Isključeno, ispod se pojavljuje potvrdni okvir "Uključi prethodne komentatore".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Podešavanja liste korisnika'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]