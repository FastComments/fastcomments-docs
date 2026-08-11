[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Domyślnie FastComments nie wyświetla listy użytkowników na stronie.

Możesz wyświetlić listę osób, które aktualnie przeglądają stronę, obok widżetu komentarzy. Lista aktualizuje się na żywo, gdy użytkownicy dołączają i odchodzą, oraz pokazuje ich imię, awatar i wskaźnik online.

Istnieją trzy opcje układu:

- `1` - Góra: poziomy rząd nakładających się awatarów wyświetlanych nad komentarzami.
- `2` - Lewo: pasek boczny z nazwiskami i kropkami online wyświetlany po lewej stronie widżetu.
- `3` - Prawo: ten sam pasek boczny wyświetlany po prawej stronie widżetu.

Ustaw flagę **usersListLocation**, aby włączyć tę funkcję:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Pokaż listę użytkowników po prawej'; code-example-end]

Domyślnie lista wyświetla tylko użytkowników aktualnie online. Aby również uwzględnić osoby, które komentowały stronę w przeszłości (ale nie przeglądają jej w tej chwili), ustaw **usersListIncludeOffline** na true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Uwzględnij wcześniejszych komentujących'; code-example-end]

Poprzedni komentujący są wyświetlani bez zielonej kropki online, aby było jasne, kto jest obecny w tej chwili.

Użytkownicy z prywatnymi profilami są wyświetlani z ogólnym awatarem i etykietą „Prywatny profil”, aby liczba pozostała dokładna bez ujawniania tożsamości.

Można to również skonfigurować bez kodu. Na stronie dostosowywania widżetu zobacz opcję „Lokalizacja listy użytkowników”. Gdy lokalizacja jest ustawiona na cokolwiek oprócz Wyłącz, pojawia się pole wyboru „Uwzględnij wcześniejszych komentujących” pod nią.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; alt='Lokalizacja listy użytkowników ustawiona na Prawo, z polem wyboru „Uwzględnij wcześniejszych komentujących” wyświetlonym pod nią'; title='Ustawienia listy użytkowników'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

Do 500 aktywnych użytkowników, lista może być opóźniona o do 30 sekund.

---