---
За замовчуванням FastComments дозволяє користувачам видаляти їхні коментарі.

Проте, це можна запобігти.

На сторінці налаштування віджету, перегляньте параметр "Disable Deleting".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-delete'; selector = '.disable-commenter-comment-delete'; alt='Параметр Disable Deleting на сторінці налаштування віджету, який запобігає видаленню коментарів користувачами'; title='Вимкнути видалення коментарів' app-screenshot-end]

- Це стосується лише звичайних коментаторів і не стосується модераторів або адміністраторів, які все ще зможуть видаляти.
- Це також вплине на інтеграції API, коли передається `contextUserId`.

---