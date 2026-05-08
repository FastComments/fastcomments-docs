---
[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Som standard viser FastComments ikke en liste over brugere på siden.

Du kan vise en liste over personer, der i øjeblikket ser siden, ved siden af kommentar-widget'en. Listen opdateres live, når brugere tilslutter sig og forlader, og viser deres navn, avatar og en online-indikator.

Der er tre layoutmuligheder:

- `1` - Øverst: en vandret række af overlappende avatarer gengivet over kommentarerne.
- `2` - Venstre: en sidepanel med navne og online-prikker gengivet til venstre for widget'en.
- `3` - Højre: samme sidepanel gengivet til højre for widget'en.

Indstil **usersListLocation**-flaget for at aktivere funktionen:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

Som standard viser listen kun brugere, der er online lige nu. For også at inkludere personer, der tidligere har kommenteret på siden (men som ikke aktuelt ser den), sæt **usersListIncludeOffline** til true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Tidligere kommentatorer vises uden den grønne online-prik, så det er tydeligt, hvem der er til stede lige nu.

Brugere med private profiler vises med en generisk avatar og en "Privat profil"-etiket, så antallet forbliver korrekt uden at afsløre identiteter.

Dette kan også konfigureres uden kode. På siden til tilpasning af widget'en, se indstillingen "Users List Location":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-location'; title='Users List Location' app-screenshot-end]

Når placeringen er sat til andet end Off, vises afkrydsningsfeltet "Include Past Commenters" nedenunder:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-include-offline'; title='Include Past Commenters' app-screenshot-end]

---