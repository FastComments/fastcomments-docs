[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

Domyślnie FastComments sortuje komentarze według kierunku sortowania „Most Relevant”.

Sortowanie „Most Relevant” bierze pod uwagę czas dodania komentarza oraz liczbę głosów przy ustalaniu kolejności.

Użytkownik może następnie zmienić kierunek sortowania na Najstarsze najpierw lub Najnowsze najpierw w interfejsie widżetu komentarzy.

Możemy jednak ustawić domyślny kierunek na dowolny z tych trzech. Na przykład, jeśli chcesz pokazywać najpierw najstarsze komentarze:

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Oldest First'; code-example-end]

Ustawiamy wartość **defaultSortDirection** na "OF", aby ustawić kierunek na "OF".

Aby ustawić kierunek sortowania na najnowsze najpierw, zrobimy następująco:

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Newest First'; code-example-end]

Prawidłowe wartości dla **defaultSortDirection** to:

- MR: "Most Recent"
- NF: "Newest First"
- OF: "Oldest First"

Można to także zrobić bez użycia kodu. Na stronie dostosowywania widżetu zobacz sekcję „Domyślny kierunek sortowania”.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; title='Changing The Default Sort Direction' app-screenshot-end]

Zauważ, że komentarze na każdej stronie dla każdego kierunku sortowania są wstępnie obliczane, więc wszystkie kierunki sortowania mają taką samą wydajność.