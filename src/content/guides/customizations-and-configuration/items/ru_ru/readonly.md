[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

Возможность комментирования можно заблокировать, чтобы нельзя было оставлять новые комментарии или голосовать, установив флаг readonly в true.

Кроме того, комментарии нельзя будет редактировать или удалять.

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = 'Making The Comment Thread Readonly'; code-example-end]

Это можно настроить без кода на странице настройки виджета для всего домена или отдельной страницы:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; title='Making The Comment Thread Readonly' app-screenshot-end]

## Обновление!

Начиная с ноября 2022 года, потоки могут быть заблокированы или разблокированы **в реальном времени** администраторами и модераторами через меню с тремя точками над областью ответа.

Это предотвратит появление новых комментариев, при этом по‑прежнему будет доступно голосование и пользователи смогут удалять свои комментарии при желании, тогда как `readonly` этого не позволяет. 

Это соответствует полю `isClosed` в API `Page`.

---