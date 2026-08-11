[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

Som standard vil FastComments-kommentarwidgetten sætte en `gif rating` til `pg`.

Tilgængelige muligheder er `g`, `pg`, `pg-13` og `r`.

Dette kan indstilles i koden eller via UI'en. I koden kan vi gøre det som følger:

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = 'Set Gif Rating'; code-example-end]

I UI'en finder du dette under `Gif Picker Rating`, så længe `Disable Image Uploads?` ikke er markeret.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; alt='Gif Picker Rating dropdown på widget-tilpasningssiden, der tilbyder g, pg, pg-13 og r'; title='Indstilling af Gif Rating' app-screenshot-end]