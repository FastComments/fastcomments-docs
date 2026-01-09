---
[related-parameter-start name = 'inputAfterComments'; type = 'boolean'; related-parameter-end]

По умолчанию область ввода комментария находится **перед** потоком комментариев. Однако, установив этот параметр конфигурации
в значение true, мы можем переместить её **после**.

[code-example-start config = {inputAfterComments: true}; linesToHighlight = [6]; title = 'Moving The Reply Box to The Bottom'; code-example-end]

Это можно настроить без кода, на странице настройки виджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.input-after-comments'; title='Moving The Reply Box to The Bottom' app-screenshot-end]

---