[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

Podrazumevano, FastComments widget za komentare postavlja `gif rating` na `pg`.

Dostupne opcije su `g`, `pg`, `pg-13` i `r`.

Ovo se može postaviti u kodu ili putem UI-a. U kodu to možemo uraditi na sledeći način:

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = 'Postavi ocenu GIF-a'; code-example-end]

U UI‑u, ovo ćete naći pod `Gif Picker Rating` sve dok opcija `Disable Image Uploads?` nije označena.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; alt='Padajući meni ocene GIF birača na stranici prilagođavanja widgeta koji nudi g, pg, pg-13 i r'; title='Podešavanje ocene GIF-a' app-screenshot-end]