[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

За замовчуванням FastComments не показує список користувачів на сторінці.

Ви можете відобразити список людей, які наразі переглядають сторінку, поруч із віджетом коментарів. Список оновлюється в реальному часі при приєднанні та виході користувачів і показує їхнє ім'я, аватар та індикатор онлайн.

Існує три варіанти розташування:

- `1` - Top: горизонтальний ряд перекриваючихся аватарів, які відображаються над коментарями.
- `2` - Left: бічна панель з іменами та індикаторами онлайн, відображається ліворуч від віджета.
- `3` - Right: та сама бічна панель, відображена праворуч від віджета.

Встановіть прапорець **usersListLocation**, щоб увімкнути цю функцію:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

За замовчуванням список показує лише користувачів, які наразі онлайн. Щоб також включити людей, які раніше коментували сторінку (але зараз її не переглядають), встановіть **usersListIncludeOffline** у true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Минулі коментатори відображаються без зеленого індикатора онлайн, щоб було видно, хто присутній саме зараз.

Користувачі з приватними профілями відображаються із загальним аватаром і позначкою "Private Profile", щоб кількість була точною, не розкриваючи особистості.

Це також можна налаштувати без коду. На сторінці налаштування віджета перегляньте опцію "Users List Location":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-location'; title='Users List Location' app-screenshot-end]

Коли розташування встановлено на будь-який варіант, відмінний від Off, під ним показується прапорець "Include past commenters":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-include-offline'; title='Include Past Commenters' app-screenshot-end]