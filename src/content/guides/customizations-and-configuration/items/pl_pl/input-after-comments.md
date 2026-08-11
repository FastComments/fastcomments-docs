[related-parameter-start name = 'inputAfterComments'; type = 'boolean'; related-parameter-end]

Domyślnie obszar wprowadzania komentarza jest **przed** wątkiem komentarzy. Jednak ustawiając ten parametr konfiguracyjny na true, możemy przenieść go **po**.

[code-example-start config = {inputAfterComments: true}; linesToHighlight = [6]; title = 'Przenoszenie pola odpowiedzi na dół'; code-example-end]

Można to dostosować bez kodu, na stronie dostosowywania widgetu:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.input-after-comments'; alt='Opcja na stronie dostosowywania widgetu, która umieszcza obszar wprowadzania komentarza po wątku komentarzy zamiast przed nim'; title='Przenoszenie pola odpowiedzi na dół' app-screenshot-end]