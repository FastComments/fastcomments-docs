[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

За замовчуванням FastComments не показує список користувачів на сторінці.

Ви можете відобразити список людей, які наразі переглядають сторінку, поряд із віджетом коментарів. Список оновлюється в реальному часі, коли користувачі приєднуються та виходять, і показує їхнє ім'я, аватар і індикатор онлайн.

Існує три варіанти розташування:

- `1` - Зверху: горизонтальний ряд перекриваючихся аватарів, відображених над коментарями.
- `2` - Зліва: бічна панель з іменами та індикаторами онлайн, відображена ліворуч від віджета.
- `3` - Справа: та сама бічна панель, відображена праворуч від віджета.

Установіть прапорець **usersListLocation**, щоб увімкнути цю функцію:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

За замовчуванням список показує лише користувачів, які наразі онлайн. Щоб також включити людей, які коментували сторінку в минулому (але нині її не переглядають), встановіть **usersListIncludeOffline** у true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Колишні коментатори відображаються без зеленої точки онлайн, щоб було зрозуміло, хто присутній саме зараз.

Користувачі з приватними профілями показуються з загальним аватаром і позначкою «Приватний профіль», щоб кількість залишалася точною без розкриття особистостей.

Це також можна налаштувати без коду. На сторінці налаштування віджета див. опцію «Розташування списку користувачів». Коли розташування встановлено на будь-яке значення, крім Off, під ним з'явиться прапорець «Включити минулих коментаторів».

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

---