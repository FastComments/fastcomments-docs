За замовчуванням FastComments дозволяє користувачам редагувати свої коментарі.

Проте це можна заборонити.

На сторінці налаштування віджета знайдіть опцію "Вимкнути редагування".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-edit'; selector = '.disable-commenter-comment-edit'; title='Disable Comment Editing' app-screenshot-end]

- Це впливає лише на звичайних коментаторів і не зачіпає модераторів або адміністраторів, які все одно зможуть редагувати.
- Це також вплине на інтеграції через API, коли передається `contextUserId`.