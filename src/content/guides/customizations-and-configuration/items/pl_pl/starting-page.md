[related-parameter-start name = 'startingPage'; type = 'number'; related-parameter-end]

Kiedy pobierane i renderowane są komentarze, widżet komentarzy musi wiedzieć, od której strony rozpocząć. Domyślnie zaczyna od pierwszej strony i renderuje tylko tę stronę.

W razie potrzeby dokładna strona do wyrenderowania może zostać przekazana do widżetu komentarzy jako ustawienie *startingPage*.

[code-example-start config = {startingPage: 1}; linesToHighlight = [6]; title = 'Specifying The Page to Render'; code-example-end]

Zwróć uwagę, że numeracja stron zaczyna się od zera, więc powyższy przykład renderuje drugą stronę.