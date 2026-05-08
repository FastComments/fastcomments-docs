[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Domyślnie FastComments nie wyświetla listy użytkowników na stronie.

Możesz wyrenderować listę osób, które aktualnie oglądają stronę, obok widżetu komentarzy. Lista aktualizuje się na żywo w miarę dołączania i opuszczania użytkowników oraz pokazuje ich imię, avatar i wskaźnik online.

Istnieją trzy opcje układu:

- `1` - Top: poziomy rząd nakładających się avatarów wyświetlany nad komentarzami.
- `2` - Left: pasek boczny z nazwami i wskaźnikami online wyświetlany po lewej stronie widżetu.
- `3` - Right: ten sam pasek boczny wyświetlany po prawej stronie widżetu.

Ustaw flagę **usersListLocation**, aby włączyć funkcję:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

Domyślnie lista pokazuje tylko użytkowników aktualnie online. Aby również uwzględnić osoby, które komentowały stronę w przeszłości (ale nie oglądają jej obecnie), ustaw **usersListIncludeOffline** na true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Osoby, które komentowały wcześniej, są renderowane bez zielonej kropki online, aby było jasne, kto jest obecny teraz.

Użytkownicy z prywatnymi profilami są pokazywani z ogólnym awatarem i etykietą "Prywatny profil", dzięki czemu liczba pozostaje dokładna bez ujawniania tożsamości.

Można to również skonfigurować bez użycia kodu. Na stronie dostosowywania widżetu zobacz opcję "Lokalizacja listy użytkowników". Gdy lokalizacja jest ustawiona na wartość inną niż Wyłączone, poniżej pojawia się pole wyboru "Uwzględnij poprzednich komentatorów".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]