[related-parameter-start name = 'maxCommentCharacterLength'; type = 'number'; related-parameter-end]

Maksymalna liczba znaków, które można wprowadzić w polu komentarza, może być ograniczona przez parametr **maxCommentCharacterLength**.

Wartość domyślna to 2000.

Elementy takie jak adresy URL obrazów nie są uwzględniane przy określaniu długości.

[code-example-start config = {maxCommentCharacterLength: 500}; linesToHighlight = [6]; title = 'Limit Comment Length'; code-example-end]

Można to dostosować bez użycia kodu, na stronie dostosowywania widżetu:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; title='Limit Comment Length' app-screenshot-end]

---