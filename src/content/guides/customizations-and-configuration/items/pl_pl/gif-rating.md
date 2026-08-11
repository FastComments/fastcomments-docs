[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

Domyślnie widżet komentarzy FastComments ustawi `gif rating` na `pg`.

Dostępne opcje to `g`, `pg`, `pg-13` i `r`.

Można to ustawić w kodzie lub za pomocą interfejsu UI. W kodzie możemy to zrobić w następujący sposób:

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = 'Ustawienie oceny GIF'; code-example-end]

W interfejsie UI znajdziesz to pod `Gif Picker Rating`, o ile opcja `Disable Image Uploads?` nie jest zaznaczona.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; alt='Lista rozwijana Gif Picker Rating na stronie dostosowywania widżetu, oferująca g, pg, pg-13 i r'; title='Ustawianie oceny GIF' app-screenshot-end]