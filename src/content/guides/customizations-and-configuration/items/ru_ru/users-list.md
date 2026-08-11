[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

По умолчанию FastComments не отображает список пользователей на странице.

Вы можете отобразить список людей, которые в данный момент просматривают страницу, рядом с виджетом комментариев. Список обновляется в реальном времени, когда пользователи присоединяются и уходят, и показывает их имя, аватар и индикатор онлайн.

Существует три варианта расположения:

- `1` — Верх: горизонтальный ряд перекрывающихся аватаров, отображаемый над комментариями.
- `2` — Слева: боковая панель с именами и точками онлайн, отображаемая слева от виджета.
- `3` — Справа: такая же боковая панель, отображаемая справа от виджета.

Установите флаг **usersListLocation**, чтобы включить эту функцию:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

По умолчанию список показывает только пользователей, находящихся онлайн. Чтобы также включить людей, которые комментировали страницу в прошлом (но сейчас её не просматривают), установите **usersListIncludeOffline** в значение true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Прошлые комментаторы отображаются без зелёной онлайн-точки, чтобы было ясно, кто присутствует прямо сейчас.

Пользователи с приватными профилями отображаются с общим аватаром и меткой «Private Profile», чтобы счёт оставался точным без раскрытия личностей.

Это также можно настроить без кода. На странице настройки виджета смотрите параметр «Users List Location». Когда расположение установлено не в положение Off, появляется флажок «Include past commenters» под ним.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; alt='Расположение списка пользователей установлено в Право, с отображённым ниже флажком «Include past commenters»'; title='Настройки списка пользователей'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

Для последних 500 живых пользователей список может отставать до 30 секунд.