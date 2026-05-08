[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Som standard viser FastComments ikke en liste over brugere på siden.

Du kan vise en liste over personer, der i øjeblikket ser siden, ved siden af kommentar-widgeten. Listen opdateres live, efterhånden som brugere kommer til og forlader siden, og viser deres navn, avatar og en online-indikator.

Der er tre layoutmuligheder:

- `1` - Top: en vandret række af overlappende avatarer, der vises over kommentarerne.
- `2` - Left: en sidebjælke med navne og online-prikker, der vises til venstre for widgeten.
- `3` - Right: samme sidebjælke, der vises til højre for widgeten.

Sæt flaget **usersListLocation** for at aktivere funktionen:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

Som standard viser listen kun brugere, der er online lige nu. For også at inkludere personer, som har kommenteret på siden tidligere (men ikke i øjeblikket ser den), sæt **usersListIncludeOffline** til true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Tidligere kommentatorer vises uden den grønne online-prik, så det er tydeligt, hvem der er til stede lige nu.

Brugere med private profiler vises med et generisk avatar og etiketten "Privat profil", så antallet forbliver korrekt uden at afsløre identiteter.

Dette kan også konfigureres uden kode. På siden til tilpasning af widgeten, se indstillingen "Placering af brugerlisten". Når placeringen er sat til noget andet end Fra, vises et afkrydsningsfelt "Inkluder tidligere kommentatorer" nedenfor.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

---