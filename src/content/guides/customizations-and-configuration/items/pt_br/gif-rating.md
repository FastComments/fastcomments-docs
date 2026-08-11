[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

Por padrão, o widget de comentários FastComments definirá uma `gif rating` de `pg`.

As opções disponíveis são `g`, `pg`, `pg-13` e `r`.

Isso pode ser definido no código ou via UI. No código, podemos fazer da seguinte forma:

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = 'Set Gif Rating'; code-example-end]

Na UI, você encontrará isso em `Gif Picker Rating` desde que `Disable Image Uploads?` não esteja marcado.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; alt='Menu suspenso de classificação de GIF no painel de personalização do widget oferecendo g, pg, pg-13 e r'; title='Configurando a classificação do GIF' app-screenshot-end]