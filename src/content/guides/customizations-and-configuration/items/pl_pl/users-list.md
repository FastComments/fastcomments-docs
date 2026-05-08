[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Domyślnie FastComments nie wyświetla listy użytkowników na stronie.

Możesz wyświetlić listę osób, które aktualnie przeglądają stronę, obok widgetu komentarzy. Lista aktualizuje się na żywo w miarę dołączania i opuszczania strony przez użytkowników i pokazuje ich nazwę, awatar oraz wskaźnik obecności online.

Dostępne są trzy opcje układu:

- `1` - U góry: poziomy rząd nachodzących na siebie avatarów wyświetlany nad komentarzami.
- `2` - Po lewej: pasek boczny z nazwami i kropkami oznaczającymi obecność online wyświetlany po lewej stronie widgetu.
- `3` - Po prawej: ten sam pasek boczny wyświetlany po prawej stronie widgetu.

Ustaw flagę **usersListLocation**, aby włączyć tę funkcję:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

Domyślnie lista pokazuje tylko użytkowników aktualnie online. Aby również uwzględnić osoby, które komentowały stronę w przeszłości (ale obecnie jej nie oglądają), ustaw **usersListIncludeOffline** na true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Byli komentatorzy są wyświetlani bez zielonej kropki oznaczającej obecność, dzięki czemu widać, kto jest aktualnie obecny.

Użytkownicy z prywatnymi profilami są wyświetlani z ogólnym awatarem i etykietą "Profil prywatny", dzięki czemu liczba użytkowników pozostaje dokładna bez ujawniania tożsamości.

To można też skonfigurować bez kodu. Na stronie personalizacji widgetu zobacz opcję "Lokalizacja listy użytkowników":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-location'; title='Users List Location' app-screenshot-end]

Kiedy lokalizacja jest ustawiona na inną wartość niż Off, poniżej pojawia się pole wyboru "Uwzględnij poprzednich komentujących":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-include-offline'; title='Include Past Commenters' app-screenshot-end]