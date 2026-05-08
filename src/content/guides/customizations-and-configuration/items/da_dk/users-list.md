[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Som standard viser FastComments ikke en liste over brugere på siden.

Du kan gengive en liste over personer, som i øjeblikket besøger siden, sammen med kommentar-widget'en. Listen opdateres live, når brugere kommer til og forlader siden, og viser deres navn, avatar og en onlineindikator.

Der er tre layoutmuligheder:

- `1` - Øverst: en vandret række af overlappende avatarer gengivet over kommentarerne.
- `2` - Venstre: en sidebjælke med navne og online-prikker gengivet til venstre for widget'en.
- `3` - Højre: samme sidebjælke gengivet til højre for widget'en.

Sæt flaget **usersListLocation** for at aktivere funktionen:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

Som standard viser listen kun brugere, der er online lige nu. For også at inkludere personer, som tidligere har kommenteret på siden (men ikke i øjeblikket ser den), sæt **usersListIncludeOffline** til true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Tidligere kommentatorer vises uden den grønne online-prik, så det er tydeligt, hvem der er til stede lige nu.

Brugere med private profiler vises med en generisk avatar og en etiket "Privat profil", så antallet forbliver korrekt uden at afsløre identiteter.

Dette kan også konfigureres uden kode. På siden til tilpasning af widget, se indstillingen "Placering af brugerliste". Når placeringen er indstillet til andet end Fra, vises der et afkrydsningsfelt "Inkluder tidligere kommentatorer" nedenunder.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

Når der er mere end 500 live-brugere, kan listen være op til 30 sekunder forsinket.