[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

По умолчанию FastComments позволяет пользователю вводить комментарий из сколь угодно строк, в рамках установленного по умолчанию ограничения по количеству символов.

Тем не менее, может потребоваться ограничить пользователя вводом только одной строки текста. Некоторые примеры случаев использования включают онлайн-аукцион или живой чат, которые FastComments
может быть использован.

Флаг **useSingleLineCommentInput** включается следующим образом:

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Enable Single-Line Comment Input'; code-example-end]

Это также можно сделать без кода. На странице настройки виджета см. раздел «Включить однострочный ввод комментария».

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; title='Enable Single-Line Comment Input' app-screenshot-end]

Обратите внимание, что комментарии на каждой странице для каждого направления сортировки предварительно вычисляются, поэтому все направления сортировки имеют одинаковую производительность.