[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Privzeto FastComments na strani ne prikazuje seznama uporabnikov.

Lahko prikažete seznam oseb, ki si trenutno ogledujejo stran, poleg pripomočka za komentarje. Seznam se v živo posodablja, ko se uporabniki pridružujejo in zapuščajo, ter prikazuje njihovo ime, avatar in indikator prisotnosti.

Na voljo so tri postavitve:

- `1` - Zgoraj: vodoravna vrstica prekrivajočih se avatarjev, prikazana nad komentarji.
- `2` - Levo: stranska vrstica z imeni in indikatorji prisotnosti, prikazana levo od pripomočka.
- `3` - Desno: enaka stranska vrstica, prikazana desno od pripomočka.

Nastavite zastavico **usersListLocation**, da omogočite funkcijo:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

Privzeto seznam prikazuje samo uporabnike, ki so trenutno online. Če želite vključiti tudi ljudi, ki so v preteklosti komentirali stran (vendar je trenutno ne gledajo), nastavite **usersListIncludeOffline** na true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Pretekli komentatorji so prikazani brez zelene pike za online, tako da je jasno, kdo je zdaj prisoten.

Uporabniki z zasebnimi profili so prikazani z generičnim avatarjem in oznako "Zaseben profil", tako da je število pravilno, ne da bi razkrili identitete.

To je mogoče tudi nastaviti brez kode. Na strani za prilagajanje pripomočka za komentarje poiščite možnost "Users List Location". Ko je lokacija nastavljena na karkoli drugega kot Off, se pod njo prikaže potrditveno polje "Include past commenters".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

Pri več kot 500 hkratnih uporabnikih je seznam lahko zastarel do 30 sekund.