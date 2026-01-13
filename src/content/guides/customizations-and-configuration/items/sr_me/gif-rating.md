[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

Po zadatim postavkama, FastComments widget za komentare će postaviti `gif rating` na `pg`.

Dostupne opcije su `g`, `pg`, `pg-13`, i `r`.

Ovo se može podesiti u kodu ili preko UI-a. U kodu to možemo uraditi na sljedeći način:

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = 'Set Gif Rating'; code-example-end]

U UI-ju ćete ovo naći pod `Gif Picker Rating`, sve dok opcija `Disable Image Uploads?` nije označena.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; title='Setting The Gif Rating' app-screenshot-end]