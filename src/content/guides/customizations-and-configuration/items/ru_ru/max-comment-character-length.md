---
[related-parameter-start name = 'maxCommentCharacterLength'; type = 'number'; related-parameter-end]

Максимальное количество символов, разрешённое для ввода в поле комментария, может быть ограничено параметром **maxCommentCharacterLength**.

По умолчанию — 2000.

Такие вещи, как URL изображений, не учитываются при определении длины.

[code-example-start config = {maxCommentCharacterLength: 500}; linesToHighlight = [6]; title = 'Ограничить длину комментария'; code-example-end]

Это можно настроить без кода на странице настройки виджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; alt='Поле максимального размера комментария на странице настройки виджета, используемое для ограничения количества символов в комментарии'; title='Ограничить длину комментария' app-screenshot-end]

---