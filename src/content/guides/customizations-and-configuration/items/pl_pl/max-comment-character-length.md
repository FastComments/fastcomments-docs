---
[related-parameter-start name = 'maxCommentCharacterLength'; type = 'number'; related-parameter-end]

Maksymalna liczba znaków, które można wprowadzić w polu wprowadzania komentarza, może być ograniczona przy użyciu parametru **maxCommentCharacterLength**.

Domyślnie wynosi 2000.

Elementy takie jak adresy URL obrazów nie są uwzględniane przy określaniu długości.

[code-example-start config = {maxCommentCharacterLength: 500}; linesToHighlight = [6]; title = 'Ogranicz długość komentarza'; code-example-end]

Można to dostosować bez kodu, na stronie dostosowywania widgetu:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; alt='Pole maksymalnego rozmiaru komentarza na stronie dostosowywania widgetu, używane do ograniczenia liczby znaków, które może zawierać komentarz'; title='Ogranicz długość komentarza' app-screenshot-end]

---