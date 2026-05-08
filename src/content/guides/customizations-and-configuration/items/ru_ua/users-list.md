[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

По умолчанию FastComments не показывает список пользователей на странице.

Вы можете отобразить список людей, которые в данный момент просматривают страницу, рядом с виджетом комментариев. Список обновляется в реальном времени по мере присоединения и ухода пользователей и показывает их имя, аватар и индикатор онлайн.

Есть три варианта расположения:

- `1` - Верх: горизонтальная строка перекрывающихся аватаров, отображаемая над комментариями.
- `2` - Слева: боковая панель с именами и индикаторами онлайн, отображаемая слева от виджета.
- `3` - Справа: та же боковая панель, отображаемая справа от виджета.

Установите флаг **usersListLocation**, чтобы включить эту функцию:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

По умолчанию список показывает только пользователей, которые сейчас в сети. Чтобы также включать людей, которые комментировали страницу в прошлом (но сейчас её не просматривают), установите **usersListIncludeOffline** в true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Бывшие комментаторы отображаются без зелёной точки онлайн, чтобы было ясно, кто присутствует прямо сейчас.

Пользователи с приватными профилями отображаются с универсальным аватаром и меткой "Приватный профиль", так что количество остаётся точным, не раскрывая личности.

Это также можно настроить без кода. На странице настройки виджета посмотрите опцию "Расположение списка пользователей". Когда расположение установлено не в значение "Выключено", под ним появляется флажок "Включить прошлых комментаторов".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]