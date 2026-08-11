---
[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Privzeto FastComments ne prikazuje seznama uporabnikov na strani.

Lahko prikažete seznam ljudi, ki trenutno gledajo stran, poleg pripomočka za komentarje. Seznam se v živo posodablja, ko se uporabniki pridružijo ali odidejo, in prikazuje njihovo ime, avatar ter indikator, da so online.

Obstajajo tri možnosti postavitve:

- `1` - Zgoraj: vodoravna vrstica prekrivajočih se avatarjev, prikazana nad komentarji.
- `2` - Levo: stranska vrstica z imeni in točkami, ki označujejo online stanje, prikazana levo od pripomočka.
- `3` - Desno: ista stranska vrstica, prikazana desno od pripomočka.

Nastavite zastavico **usersListLocation**, da omogočite funkcijo:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Prikaži seznam uporabnikov na desni'; code-example-end]

Privzeto seznam prikazuje le uporabnike, ki so trenutno online. Če želite vključiti tudi ljudi, ki so v preteklosti komentirali stran (a je trenutno ne gledajo), nastavite **usersListIncludeOffline** na true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Vključi pretekle komentatorje'; code-example-end]

Pretekli komentatorji so prikazani brez zelenega online pika, da je jasno, kdo je trenutno prisoten.

Uporabniki z zasebnimi profili so prikazani z generičnim avatarjem in oznako "Zasebni profil", tako da je število natančno, ne da bi razkrili identitete.

To lahko nastavite tudi brez kode. Na strani za prilagajanje pripomočka poiščite možnost "Lokacija seznama uporabnikov". Ko je lokacija nastavljena na karkoli drugega kot Izklopljeno, se pod njo prikaže potrditveno polje "Vključi pretekle komentatorje".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; alt='Lokacija seznama uporabnikov nastavljena na desno, s potrditvenim poljem za vključitev preteklih komentatorjev, prikazanim pod njo'; title='Nastavitve seznama uporabnikov'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

Pri več kot 500 živih uporabnikih je seznam lahko zakasnel do 30 sekund.