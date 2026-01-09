[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

Domyślnie widżet komentarzy FastComments ustawi `gif rating` na `pg`.

Dostępne opcje to `g`, `pg`, `pg-13` i `r`.

Można to ustawić w kodzie lub za pomocą UI. W kodzie możemy to zrobić w następujący sposób:

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = 'Set Gif Rating'; code-example-end]

W UI znajdziesz to pod `Gif Picker Rating`, o ile `Disable Image Uploads?` nie jest zaznaczone.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; title='Setting The Gif Rating' app-screenshot-end]