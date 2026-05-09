[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Domyślnie FastComments nie pokazuje listy użytkowników na stronie.

Możesz wyświetlić listę osób, które aktualnie przeglądają stronę, obok widżetu komentarzy. Lista aktualizuje się na żywo w miarę dołączania i opuszczania użytkowników, i pokazuje ich nazwę, avatar oraz wskaźnik online.

Dostępne są trzy opcje rozmieszczenia:

- `1` - U góry: poziomy rząd nachodzących na siebie avatarów wyświetlany nad komentarzami.
- `2` - Po lewej: pasek boczny z nazwami i kropkami oznaczającymi online wyświetlany po lewej stronie widżetu.
- `3` - Po prawej: ten sam pasek boczny wyświetlany po prawej stronie widżetu.

Ustaw flagę **usersListLocation**, aby włączyć tę funkcję:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

Domyślnie lista pokazuje tylko użytkowników aktualnie online. Aby także uwzględnić osoby, które komentowały stronę w przeszłości (ale nie oglądają jej obecnie), ustaw **usersListIncludeOffline** na true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Poprzedni komentatorzy są wyświetlani bez zielonej kropki wskazującej, że są online, dzięki czemu widać, kto jest obecny w tej chwili.

Użytkownicy z prywatnymi profilami są pokazywani z ogólnym avatarem i etykietą "Profil prywatny", dzięki czemu liczba pozostaje dokładna bez ujawniania tożsamości.

Można to także skonfigurować bez użycia kodu. Na stronie dostosowywania widżetu sprawdź opcję "Users List Location". Gdy lokalizacja ustawiona jest na inną niż Off, poniżej pojawi się pole wyboru "Include past commenters".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

Przy ponad 500 aktywnych użytkownikach lista może być nieaktualna do 30 sekund.