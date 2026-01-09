[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

По умолчанию FastComments не отслеживает, кто просматривал каждый комментарий, и не предоставляет статистику по этому поводу.

Однако эту функцию можно включить, и система начнет отслеживать просмотры при прокрутке пользователем до комментария.

В этом случае счетчик рядом с иконкой глаза на каждом комментарии будет увеличиваться. Счетчик обновляется в реальном времени и отображается в сокращенном виде в соответствии с локалью пользователя.

Эту функцию можно включить, установив флаг **enableViewCounts** в значение true:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Enabling Comment View Counts'; code-example-end]

Это можно настроить без кода на странице настройки виджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; title='Enabling Comment View Counts' app-screenshot-end]

Мы отслеживаем user id* который просматривал комментарий, поэтому при повторном просмотре комментарий не увеличивает счетчик. Если вы просмотрите комментарий снова
спустя два года, счетчик увеличится.

- *Примечание: или anon session id, или IP пользователя в виде хэш-значения.