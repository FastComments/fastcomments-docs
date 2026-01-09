---
[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

По умолчанию виджет комментариев FastComments устанавливает `gif rating` в `pg`.

Доступные варианты: `g`, `pg`, `pg-13` и `r`.

Это можно задать в коде или через интерфейс. В коде это делается следующим образом:

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = 'Set Gif Rating'; code-example-end]

В интерфейсе это находится в разделе `Gif Picker Rating`, при условии что опция `Disable Image Uploads?` не отмечена.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; title='Setting The Gif Rating' app-screenshot-end]

---