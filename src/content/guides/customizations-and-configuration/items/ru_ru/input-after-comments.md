[related-parameter-start name = 'inputAfterComments'; type = 'boolean'; related-parameter-end]

По умолчанию область ввода комментариев находится **before** веткой комментариев. Однако, установив этот параметр конфигурации в true, мы можем переместить её **after**.

[code-example-start config = {inputAfterComments: true}; linesToHighlight = [6]; title = 'Перемещение поля ответа в нижнюю часть'; code-example-end]

Это можно настроить без кода на странице настройки виджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.input-after-comments'; alt='Опция страницы настройки виджета, которая размещает область ввода комментариев после ветки комментариев вместо перед ней'; title='Перемещение поля ответа в нижнюю часть' app-screenshot-end]