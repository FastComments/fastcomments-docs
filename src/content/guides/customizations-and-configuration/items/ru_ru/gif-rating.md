[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

По умолчанию FastComments comment widget будет устанавливать `gif rating` значение `pg`.

Доступные варианты: `g`, `pg`, `pg-13` и `r`.

Это можно задать в коде или через UI. В коде мы можем сделать это следующим образом:

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = 'Установить рейтинг GIF'; code-example-end]

В UI вы найдете это под `Gif Picker Rating`, пока `Disable Image Uploads?` не отмечен.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; alt='Выпадающий список Gif Picker Rating на странице настройки виджета, предлагающий g, pg, pg-13 и r'; title='Настройка рейтинга GIF' app-screenshot-end]