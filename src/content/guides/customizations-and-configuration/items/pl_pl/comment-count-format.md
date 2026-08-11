[related-parameter-start name = 'commentCountFormat'; type = 'string'; related-parameter-end]

Liczba komentarzy wyświetlana u góry widżetu komentarzy może być dostosowana.

Można to zastąpić dowolnym ciągiem znaków, a wartość **[count]** zostanie zastąpiona liczbą komentarzy, zlokalizowaną dla użytkownika.

[code-example-start config = {commentCountFormat: "There are [count] comments."}; linesToHighlight = [6]; title = 'Dostosowywanie tekstu liczby komentarzy'; code-example-end]

Można to dostosować bez kodu, na stronie dostosowywania widżetu:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.comment-count'; alt='Pole tekstowe liczby komentarzy na stronie dostosowywania widżetu, gdzie [count] jest zastępowane bieżącą sumą'; title='Dostosowywanie tekstu liczby komentarzy' app-screenshot-end]