[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Privzeto FastComments ne prikazuje seznama uporabnikov na strani.

Lahko prikažete seznam ljudi, ki si trenutno ogledujejo stran, poleg pripomočka za komentarje. Seznam se v živo posodablja, ko se uporabniki pridružijo ali odidejo, in prikazuje njihovo ime, avatar ter indikator prisotnosti.

Na voljo so tri postavitve:

- `1` - Zgoraj: vodoravna vrstica prekrivajočih se avatarjev, prikazana nad komentarji.
- `2` - Levo: stranski stolpec z imeni in indikatorji prisotnosti, prikazan na levi strani vtičnika.
- `3` - Desno: enak stranski stolpec, prikazan na desni strani vtičnika.

Za omogočanje te funkcije nastavite zastavico **usersListLocation**:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

Privzeto seznam prikazuje le uporabnike, ki so trenutno na spletu. Če želite vključiti tudi ljudi, ki so v preteklosti komentirali stran (ampak trenutno niso na njej), nastavite **usersListIncludeOffline** na true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Prejšnji komentatorji se prikažejo brez zelene pike, tako da je jasno, kdo je trenutno prisoten.

Uporabniki z zasebnimi profili so prikazani z generičnim avatarjem in oznako "Private Profile", tako da je število uporabnikov natančno, ne da bi se razkrile identitete.

To je mogoče nastaviti tudi brez kode. Na strani za prilagoditev vtičnika si oglejte možnost "Users List Location":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-location'; title='Users List Location' app-screenshot-end]

Ko je lokacija nastavljena na karkoli drugega kot Off, se spodaj prikaže potrditveno polje "Include past commenters":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-include-offline'; title='Include Past Commenters' app-screenshot-end]

---