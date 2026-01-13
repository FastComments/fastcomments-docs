---
[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

Podrazumevano, komentarski vidžet FastComments postaviće `gif rating` na `pg`.

Dostupne opcije su `g`, `pg`, `pg-13`, i `r`.

Ovo se može postaviti u kodu ili putem UI-a. U kodu to možemo uraditi na sledeći način:

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = 'Set Gif Rating'; code-example-end]

U UI-u ćete ovo pronaći pod `Gif Picker Rating` sve dok opcija `Disable Image Uploads?` nije označena.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; title='Setting The Gif Rating' app-screenshot-end]

---