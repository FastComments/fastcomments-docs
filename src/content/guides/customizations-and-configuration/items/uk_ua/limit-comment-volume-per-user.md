За замовчуванням кожен користувач може надіслати до `5 comments` за ту саму хвилину.

Це відстежується за user id, anon user id, та ip address (hashed).

Це можна налаштувати без коду, на сторінці налаштувань віджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; title='Limiting Comment Volume Per User' app-screenshot-end]

Зверніть увагу, що якщо ви використовуєте comment creation API, можливо, варто передати оригінальну `ip` адресу користувача в запиті до нашого backend, щоб rate limiting застосовувався per user і не глобально до вашого акаунту.