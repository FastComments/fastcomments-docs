[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

За замовчуванням FastComments не показує список користувачів на сторінці.

Ви можете відобразити список людей, які зараз переглядають сторінку, поряд з віджетом коментарів. Список оновлюється в реальному часі, коли користувачі приєднуються та виходять, і показує їх ім'я, аватар та індикатор онлайн.

Існує три варіанти розташування:

- `1` - Верх: горизонтальний ряд накладаються аватарок, розташований над коментарями.
- `2` - Ліво: бічна панель з іменами та онлайн‑точками, розташована ліворуч від віджета.
- `3` - Право: та сама бічна панель, розташована праворуч від віджета.

Встановіть прапорець **usersListLocation**, щоб увімкнути функцію:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

За замовчуванням список показує лише користувачів, які зараз онлайн. Щоб також включити людей, які раніше коментували сторінку (але зараз її не переглядають), встановіть **usersListIncludeOffline** у true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Колишні коментатори відображаються без зеленого онлайн‑крапки, щоб було зрозуміло, хто присутній зараз.

Користувачі з приватними профілями відображаються загальним аватаром і міткою "Приватний профіль", щоб підрахунок залишався точним без розкриття особистих даних.

Це також можна налаштувати без коду. На сторінці налаштування віджета перегляньте параметр "Users List Location". Коли розташування встановлено не в "Вимкнено", під ним з'являється прапорець "Include past commenters".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; alt='Розташування списку користувачів встановлено праворуч, з прапорцем "Include past commenters", що показаний під ним'; title='Налаштування списку користувачів'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

За останні 500 живих користувачів список може відставати до 30 секунд.