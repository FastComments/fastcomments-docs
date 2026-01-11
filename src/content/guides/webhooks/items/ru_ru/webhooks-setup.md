Выполните те же шаги для `localhost`, что и для production. Убедитесь, что у вас настроены production domains и API Secrets.

Сначала перейдите в [Webhooks admin](https://fastcomments.com/auth/my-account/manage-data/webhooks). Это доступно через Manage Data -> Webhooks.

Страница конфигурации выглядит следующим образом:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Webhooks Configuration' app-screenshot-end]

На этой странице вы можете указать endpoints для каждого типа события комментария.

Для каждого типа события обязательно нажимайте Send Test Payload, чтобы убедиться, что интеграция настроена правильно. См. следующий раздел «Testing» для подробностей.