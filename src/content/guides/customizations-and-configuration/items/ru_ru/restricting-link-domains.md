---
По умолчанию FastComments позволяет добавлять ссылки на любые внешние сайты.

Это можно ограничить до нужного списка сайтов или доменов. Попытка опубликовать ссылку на сайт или домен,
не включённый в заданный список, приведёт к тому, что пользователю будет показана ошибка.

Эта проверка применяется только к виджету комментариев и API. Импорты не затрагиваются.

Это делается без кода, на странице настройки виджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.restricted-link-domains-list'; selector = '.external-link-settings'; title='Restrict External Link Domains' app-screenshot-end]

---