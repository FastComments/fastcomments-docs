[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

За замовчуванням FastComments показує іконку дзвінка сповіщень у верхньому правому куті області коментарів.

Ця іконка почервоніє і покаже кількість сповіщень у користувача. Приклади таких сповіщень:

- Користувач відповів вам.
- Користувач відповів у гілці, в якій ви коментували.
- Користувач проголосував за ваш коментар.
- Користувач відповів на сторінці, на яку ви підписані.

Іконка дзвінка сповіщень також надає можливість підписатися на всю сторінку.

Однак можна повністю вимкнути іконку дзвінка сповіщень:

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Disable Notification Bell'; code-example-end]

Це також можна зробити без коду. На сторінці налаштування віджета див. розділ "Disable Notification Bell".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; title='Disable Notification Bell' app-screenshot-end]