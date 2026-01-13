По умолчанию каждый пользователь может отправлять до `5 comments` в течение одной минуты.

Это отслеживается по user id, anon user id и ip адресу (в хэшированном виде).

Это можно настроить без кода на странице настройки виджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; title='Limiting Comment Volume Per User' app-screenshot-end]

Обратите внимание, что если вы используете comment creation API, возможно, стоит передать исходный `ip` адрес пользователя в запросе к нашему бэкенду, чтобы rate limiting применялся
к пользователю, а не глобально ко всему вашему аккаунту.