[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

По умолчанию FastComments позволяет пользователю вводить комментарий из любого количества строк в пределах установленного по умолчанию лимита символов.

Однако может возникнуть необходимость ограничить пользователя вводом только одной строки текста. Примеры использования включают онлайн-торги или живой чат, для которых FastComments
можно использовать.

Включаем флаг **useSingleLineCommentInput** следующим образом:

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Enable Single-Line Comment Input'; code-example-end]

Это также можно сделать без кода. На странице настройки виджета смотрите раздел "Включить ввод комментария в одну строку".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; title='Enable Single-Line Comment Input' app-screenshot-end]

Обратите внимание, что комментарии на каждой странице для каждого направления сортировки предварительно вычислены, поэтому все направления сортировки имеют одинаковую производительность.