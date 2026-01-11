Виконайте ті ж кроки для `localhost`, що й для production. Переконайтеся, що у вас налаштовані production domains і API Secrets.

Спочатку перейдіть до [Webhooks admin](https://fastcomments.com/auth/my-account/manage-data/webhooks). Це доступно через Manage Data -> Webhooks.

The configuration page appears as follows:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Webhooks Configuration' app-screenshot-end]

На цій сторінці ви можете вказати endpoints для кожного типу події коментаря.

Для кожного типу події обов'язково натисніть Send Test Payload, щоб переконатися, що інтеграція налаштована правильно. Див. наступний розділ, "Testing", для деталей.