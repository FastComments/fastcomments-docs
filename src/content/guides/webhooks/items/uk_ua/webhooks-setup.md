Виконайте ті самі кроки для `localhost`, що й для production. Переконайтеся, що у вас налаштовані production domains та API Secrets.

Спочатку перейдіть до [Webhooks admin](https://fastcomments.com/auth/my-account/manage-data/webhooks). Це доступно через Manage Data -> Webhooks.

The configuration page appears as follows:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Webhooks Configuration'; cacheBuster = 'v3' app-screenshot-end]

На цій сторінці ви можете вказати endpoints для кожного типу події коментаря.

Для кожного типу подій обов'язково натискайте Send Test Payload, щоб переконатися, що інтеграція налаштована правильно. Деталі див. у наступному розділі "Testing".

---