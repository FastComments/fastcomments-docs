[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

За замовчуванням FastComments не показує список користувачів на сторінці.

Ви можете відобразити список осіб, які зараз переглядають сторінку, поруч із віджетом коментарів. Список оновлюється в реальному часі під час приєднання та виходу користувачів, і показує їх ім'я, аватар і індикатор онлайн.

Існує три варіанти розміщення:

- `1` - Вгорі: горизонтальний ряд перекриваючихся аватарів, відображуваних над коментарями.
- `2` - Ліворуч: бічна панель з іменами та індикаторами онлайн, що відображається зліва від віджета.
- `3` - Праворуч: та ж бічна панель, що відображається праворуч від віджета.

Встановіть прапорець **usersListLocation** для увімкнення цієї функції:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

За замовчуванням список показує тільки користувачів, які зараз онлайн. Щоб також включити людей, які залишали коментарі на сторінці раніше (але наразі її не переглядають), встановіть **usersListIncludeOffline** в true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Колишні коментатори відображаються без зеленого індикатора онлайн, щоб було зрозуміло, хто присутній прямо зараз.

Користувачі з приватними профілями показуються з загальним аватаром і позначкою «Приватний профіль», щоб кількість залишалася точною без розкриття особистостей.

Це також можна налаштувати без коду. На сторінці налаштування віджета див. опцію «Розташування списку користувачів». Коли місце розташування встановлено не в положення Вимкнено, під ним з'являється прапорець «Включити тих, хто коментував раніше».

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

За межами перших 500 активних користувачів, список може відставати до 30 секунд.