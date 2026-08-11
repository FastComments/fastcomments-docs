By default, each user can submit up to `5 comments` in the same minute.

This is tracked by user id, anon user id, and ip address (hashed).

This can be customized without code, on the widget customization page:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; alt='Поле «Максимальна кількість коментарів за хвилину» на сторінці налаштування віджету, встановлене за замовчуванням на 5'; title='Limiting Comment Volume Per User' app-screenshot-end]

Note that if you're using the comment creation API may want to pass the user's original `ip` address in the request to our backend so rate limiting is applied
до користувача і не глобально до вашого облікового запису.