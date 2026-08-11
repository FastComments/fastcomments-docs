[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

Domyślnie FastComments pozwala użytkownikom blokować innych użytkowników. Zablokowanie użytkownika spowoduje, że jego komentarze będą maskowane, zapobiegnie powiadomieniom między użytkownikami i tak dalej.

Możliwe, że zechcesz wyłączyć tę funkcjonalność. Można to zrobić w następujący sposób:

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Wyłącz blokowanie'; code-example-end]

Można to również zrobić bez kodu, co umożliwia prawidłową walidację po stronie serwera, za pomocą interfejsu UI Dostosowywania Widżetu:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; alt='Opcja wyłączenia blokowania w interfejsie UI dostosowywania widżetu, która zapobiega blokowaniu się użytkowników nawzajem'; title='Wyłącz blokowanie' app-screenshot-end]