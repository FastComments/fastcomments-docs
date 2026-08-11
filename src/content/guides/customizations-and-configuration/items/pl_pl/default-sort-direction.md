[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

Domyślnie FastComments sortuje komentarze według kierunku sortowania „Najbardziej istotne”.

Sortowanie „Najbardziej istotne” uwzględnia czas pozostawienia komentarza oraz liczbę głosów przy sortowaniu.

Użytkownik może następnie zmienić kierunek sortowania na najstarsze lub najnowsze najpierw w interfejsie widżetu komentarzy.

Możemy jednak zmienić domyślne ustawienie na dowolne z trzech. Na przykład, jeśli chcesz wyświetlać najstarsze komentarze jako pierwsze:

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Zmiana domyślnego sortowania na najstarsze najpierw'; code-example-end]

Ustawiamy wartość **defaultSortDirection** na „OF”, aby ustawić kierunek na „OF”.

Aby ustawić kierunek sortowania najnowsze najpierw, wykonujemy następujące kroki:

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Zmiana domyślnego sortowania na najnowsze najpierw'; code-example-end]

Prawidłowe wartości dla **defaultSortDirection** to:

- MR: "Najbardziej aktualne"
- NF: "Najnowsze najpierw"
- OF: "Najstarsze najpierw"

Można to zrobić również bez kodu. Na stronie dostosowywania widżetu, zobacz sekcję „Domyślny kierunek sortowania”.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; alt='Selektor domyślnego kierunku sortowania oferujący Najbardziej istotne, Najnowsze najpierw i Najstarsze najpierw'; title='Zmiana domyślnego kierunku sortowania' app-screenshot-end]

Należy zauważyć, że komentarze na każdej stronie dla każdego kierunku sortowania są wstępnie obliczane, więc wszystkie kierunki sortowania mają taką samą wydajność.