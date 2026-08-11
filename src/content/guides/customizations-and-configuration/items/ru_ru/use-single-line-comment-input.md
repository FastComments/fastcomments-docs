[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

По умолчанию FastComments позволяет пользователю вводить комментарий в любом количестве строк, вплоть до предела по количеству символов.

Однако может потребоваться ограничить ввод пользователем только одной строкой текста. Примеры таких сценариев включают онлайн‑аукционы или живой чат, для которых можно использовать FastComments.

Мы включаем флаг **useSingleLineCommentInput** следующим образом:

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Enable Single-Line Comment Input'; code-example-end]

Это также можно сделать без кода. На странице настройки виджета см. раздел «Enable Single-Line Comment Input».

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; alt='Флажок ввода однострочного комментария включён на странице настройки виджета, ограничивая ввод одной строкой'; title='Включить ввод однострочного комментария' app-screenshot-end]

Обратите внимание, что комментарии на каждой странице для каждого направления сортировки предварительно вычисляются, поэтому все направления сортировки имеют одинаковую производительность.