[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

Po defaultu, FastComments widget za komentare će postaviti `gif rating` na `pg`.

Raspoložive opcije su `g`, `pg`, `pg-13` i `r`.

Ovo se može postaviti u kodu ili putem UI‑a. U kodu to možemo uraditi na sledeći način:

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = 'Postavi Gif Rejting'; code-example-end]

U UI‑u ćete ovo naći pod `Gif Picker Rating` sve dok `Disable Image Uploads?` nije označena.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; alt='Padajući meni Gif Picker Rating na stranici prilagođavanja widgeta koji nudi g, pg, pg-13 i r'; title='Postavljanje Gif Rejtinga' app-screenshot-end]