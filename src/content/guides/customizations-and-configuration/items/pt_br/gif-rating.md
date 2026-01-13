[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

Por padrão, o widget de comentários FastComments definirá um `gif rating` de `pg`.

As opções disponíveis são `g`, `pg`, `pg-13` e `r`.

Isso pode ser configurado no código ou via a UI. No código, podemos fazê-lo da seguinte forma:

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = 'Set Gif Rating'; code-example-end]

Na UI, você encontrará isso em `Gif Picker Rating`, desde que `Disable Image Uploads?` não esteja marcado.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; title='Setting The Gif Rating' app-screenshot-end]