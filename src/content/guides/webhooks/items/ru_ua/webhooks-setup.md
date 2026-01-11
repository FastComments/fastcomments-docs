---
Выполните те же шаги для `localhost`, что и для `production`. Убедитесь, что у вас настроены production-домены и API Secrets.

Сначала перейдите в раздел [Webhooks admin](https://fastcomments.com/auth/my-account/manage-data/webhooks). К нему можно добраться через Manage Data -> Webhooks.

Страница конфигурации выглядит следующим образом:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Webhooks Configuration' app-screenshot-end]

На этой странице вы можете указать endpoints для каждого типа события комментария.

Для каждого типа события обязательно нажимайте Send Test Payload, чтобы убедиться, что интеграция настроена правильно. Подробнее см. в следующем разделе «Testing».

---