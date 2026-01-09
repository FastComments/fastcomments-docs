[related-parameter-start name = 'newCommentsToBottom'; type = 'boolean'; related-parameter-end]

По умолчанию новые комментарии в реальном времени появляются в верхней части списка комментариев по мере их публикации.

Когда эта опция включена, новые комментарии в реальном времени будут добавляться в нижнюю часть списка. Это влияет на то, как комментарии появляются при их публикации в режиме реального времени, пока пользователи просматривают ветку комментариев.

[code-example-start config = {newCommentsToBottom: true}; linesToHighlight = [6]; title = 'New Live Comments to Bottom'; code-example-end]

With this setting enabled:
- Новые комментарии в реальном времени, опубликованные другими пользователями, будут появляться в нижней части списка комментариев
- Пользователи будут видеть, как новые комментарии появляются ниже уже существующих в режиме реального времени
- Это влияет только на обновления комментариев в реальном времени - не на первоначальную загрузку страницы
- Это может помочь поддерживать поток чтения, когда пользователи следят за обсуждением

Note that this setting only affects where new live comments are placed as they arrive in real-time. It does not affect the initial sort order when the page loads.