---
За замовчуванням FastComments дозволяє користувачам видаляти свої коментарі.

Однак це можна заборонити.

На сторінці налаштувань віджета знайдіть опцію «Вимкнути видалення».

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-delete'; selector = '.disable-commenter-comment-delete'; title='Disable Comment Deleting' app-screenshot-end]

- Це впливає лише на звичайних коментаторів, а не на модераторів або адміністраторів, які все ще зможуть видаляти.
- Це також впливатиме на інтеграції через API у випадку, коли передається `contextUserId`. 

---