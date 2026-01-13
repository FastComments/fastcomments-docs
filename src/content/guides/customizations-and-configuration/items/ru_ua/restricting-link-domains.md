По умолчанию FastComments позволяет добавлять ссылки на любые внешние сайты.

Это можно ограничить до списка желаемых сайтов или доменов. Попытка отправить ссылку на сайт или домен,
который не входит в определённый список, приведёт к отображению ошибки для пользователя.

Эта проверка применяется только к виджету комментариев и API. Импорт не затрагивается.

Это делается без кода, на странице настройки виджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.restricted-link-domains-list'; selector = '.external-link-settings'; title='Restrict External Link Domains' app-screenshot-end]