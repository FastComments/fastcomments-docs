[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

Prema zadanim postavkama, widget za komentare FastComments će postaviti `gif rating` na `pg`.

Dostupne opcije su `g`, `pg`, `pg-13` i `r`.

Ovo se može postaviti u kodu ili putem UI-ja. U kodu to možemo učiniti na sljedeći način:

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = 'Set Gif Rating'; code-example-end]

U UI-ju, to ćete pronaći pod `Gif Picker Rating` sve dok `Disable Image Uploads?` nije označeno.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; title='Setting The Gif Rating' app-screenshot-end]