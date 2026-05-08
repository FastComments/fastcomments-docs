[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Privzeto FastComments na strani ne prikazuje seznama uporabnikov.

Lahko prikažete seznam ljudi, ki trenutno gledajo stran, poleg pripomočka za komentarje. Seznam se v živo posodablja, ko se uporabniki pridružijo ali odidejo, in prikazuje njihovo ime, avatar ter indikator prisotnosti.

Na voljo so tri postavitve:

- `1` - Zgoraj: vodoravna vrstica prekrivajočih se avatarjev, prikazana nad komentarji.
- `2` - Levo: stranski stolpec z imeni in pikami, ki označujejo prisotnost, prikazan levo od pripomočka.
- `3` - Desno: enak stranski stolpec, prikazan desno od pripomočka.

Za vklop te funkcije nastavite zastavico **usersListLocation**:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

Privzeto seznam prikazuje le uporabnike, ki so trenutno na spletu. Če želite vključiti tudi osebe, ki so v preteklosti komentirale stran (a je trenutno ne gledajo), nastavite **usersListIncludeOffline** na true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Pretekli komentatorji so prikazani brez zelene pike, ki označuje prisotnost, tako da je jasno, kdo je trenutno prisoten.

Uporabniki z zasebnimi profili so prikazani z generičnim avatarjem in napisom "Zaseben profil", tako da ostane število pravilno, ne da bi bile razkrite identitete.

To lahko konfigurirate tudi brez kode. Na strani za prilagajanje pripomočka si oglejte možnost "Users List Location". Ko je lokacija nastavljena na karkoli drugega kot Off, se pod njo prikaže potrditveno polje "Include past commenters".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]