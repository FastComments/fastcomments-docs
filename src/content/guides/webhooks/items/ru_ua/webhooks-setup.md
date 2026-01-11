---
Выполните те же шаги для `localhost`, что и для production. Убедитесь, что у вас настроены production-домены и API Secrets.

Сначала перейдите в [Webhooks admin](https://fastcomments.com/auth/my-account/manage-data/webhooks). К этой странице можно добраться через Manage Data -> Webhooks.

Страница конфигурации выглядит следующим образом:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Webhooks Configuration'; cacheBuster = 'v3' app-screenshot-end]

На этой странице вы можете указать конечные точки для каждого типа события комментария.

Для каждого типа события обязательно нажмите Send Test Payload, чтобы убедиться, что интеграция настроена правильно. Подробности — в следующем разделе «Тестирование».
---