[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

За замовчуванням FastComments не показує список користувачів на сторінці.

Ви можете відобразити список людей, які наразі переглядають сторінку, поруч із віджетом коментарів. Список оновлюється в реальному часі, коли користувачі заходять і виходять, і показує їхнє ім'я, аватар та індикатор онлайн.

Існує три варіанти розташування:

- `1` - Вгорі: горизонтальний ряд накладених аватарів, що відображається над коментарями.
- `2` - Зліва: бокова панель з іменами та індикаторами онлайн, що відображається зліва від віджета.
- `3` - Справа: та сама бокова панель, що відображається справа від віджета.

Встановіть параметр **usersListLocation**, щоб увімкнути цю функцію:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

За замовчуванням список показує лише користувачів, які наразі онлайн. Щоб також включити людей, які раніше залишали коментарі на сторінці (але зараз її не переглядають), встановіть **usersListIncludeOffline** в true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Користувачі, які раніше коментували, відображаються без зеленої точки онлайн, щоб було зрозуміло, хто присутній саме зараз.

Користувачі з приватними профілями показуються з універсальною аватаркою та міткою "Приватний профіль", щоб загальна кількість залишалася точною без розкриття їхніх ідентичностей.

Це також можна налаштувати без коду. На сторінці налаштування віджета див. опцію "Users List Location":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-location'; title='Users List Location' app-screenshot-end]

Коли розташування встановлено не на Off, під ним відображається чекбокс "Include Past Commenters":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-include-offline'; title='Include Past Commenters' app-screenshot-end]