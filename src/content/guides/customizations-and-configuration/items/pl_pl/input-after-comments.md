[related-parameter-start name = 'inputAfterComments'; type = 'boolean'; related-parameter-end]

Domyślnie pole wpisywania komentarza znajduje się **przed** wątkiem komentarzy. Jednak ustawiając ten parametr konfiguracji
na true możemy przenieść je **po**.

[code-example-start config = {inputAfterComments: true}; linesToHighlight = [6]; title = 'Moving The Reply Box to The Bottom'; code-example-end]

Można to dostosować bez kodu, na stronie dostosowywania widżetu:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.input-after-comments'; title='Moving The Reply Box to The Bottom' app-screenshot-end]