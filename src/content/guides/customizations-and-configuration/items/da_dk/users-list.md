[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Som standard viser FastComments ikke en liste over brugere på siden.

Du kan vise en liste over personer, der i øjeblikket ser siden, ved siden af kommentarfunktionen. Listen opdateres live, efterhånden som brugere kommer og går, og viser deres navn, avatar og en online‑indikator.

Der er tre layoutmuligheder:

- `1` - Top: en vandret række af overlappende avatarer vist over kommentarerne.
- `2` - Left: en sidebjælke med navne og online‑punkter vist til venstre for widget'en.
- `3` - Right: den samme sidebjælke vist til højre for widget'en.

Sæt flaget **usersListLocation** for at aktivere funktionen:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Vis brugerliste til højre'; code-example-end]

Som standard viser listen kun brugere, der er online i øjeblikket. For også at inkludere personer, der har kommenteret på siden tidligere (men som ikke ser den i øjeblikket), sæt **usersListIncludeOffline** til true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Inkluder tidligere kommentatorer'; code-example-end]

Tidligere kommentatorer vises uden den grønne online‑punkt, så det er tydeligt, hvem der er til stede lige nu.

Brugere med private profiler vises med en generisk avatar og en "Privat profil"-etiket, så antallet forbliver nøjagtigt uden at afsløre identiteter.

Dette kan også konfigureres uden kode. På widget‑tilpasningssiden, se indstillingen "Users List Location". Når placeringen er sat til noget andet end Off, vises en "Include past commenters"-afkrydsningsboks nedenunder.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; alt='Placeringen for brugerliste sat til Højre, med afkrydsningsboksen Inkluder tidligere kommentatorer vist nedenunder'; title='Indstillinger for brugerliste'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

Op til 500 live‑brugere, listen kan være op til 30 sekunder forældet.