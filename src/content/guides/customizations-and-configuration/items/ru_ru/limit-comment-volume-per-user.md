По умолчанию каждый пользователь может отправить не более `5 comments` за одну минуту.

Это отслеживается по user id, anon user id и ip address (hashed).

Это можно настроить без кода на странице настройки виджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; title='Limiting Comment Volume Per User' app-screenshot-end]

Обратите внимание, что если вы используете comment creation API, возможно, стоит передать исходный `ip` пользователя в запросе на наш бэкенд, чтобы ограничение скорости применялось к каждому пользователю, а не глобально к вашей учетной записи.