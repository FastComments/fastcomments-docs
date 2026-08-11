---
[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

Комментирование можно заблокировать, чтобы новые комментарии или голоса не могли быть оставлены, установив флаг readonly в значение true.

Комментарии также нельзя будет редактировать или удалять.

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = 'Making The Comment Thread Readonly'; code-example-end]

Это можно настроить без кода на странице настройки виджета для всего домена или отдельной страницы:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; alt='Настройка предотвращения новых ответов на странице настройки виджета, которая блокирует ветку для домена или страницы'; title='Making The Comment Thread Readonly' app-screenshot-end]

## Update!

Начиная с ноября 2022 года, ветки можно блокировать или разблокировать **в реальном времени** администраторами и модераторами через меню с тремя точками над областью ответа.

Это предотвратит появление новых комментариев, при этом оставит возможность голосования и позволит пользователям удалять свои комментарии при желании, тогда как `readonly` не позволяет этого. 

Это соответствует полю `isClosed` в API `Page`.

---