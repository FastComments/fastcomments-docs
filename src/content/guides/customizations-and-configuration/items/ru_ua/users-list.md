[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

По умолчанию FastComments не отображает список пользователей на странице.

Вы можете отобразить список людей, которые в настоящее время просматривают страницу, рядом с виджетом комментариев. Список обновляется в реальном времени по мере присоединения и ухода пользователей и показывает их имя, аватар и индикатор онлайн.

Доступно три варианта расположения:

- `1` - Вверху: горизонтальная строка перекрывающихся аватаров, отображаемая над комментариями.
- `2` - Слева: боковая панель с именами и точками онлайн, отображаемая слева от виджета.
- `3` - Справа: та же боковая панель, отображаемая справа от виджета.

Установите флаг **usersListLocation**, чтобы включить эту функцию:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

По умолчанию список показывает только пользователей, которые в данный момент онлайн. Чтобы также включить людей, которые оставляли комментарии на странице в прошлом (но сейчас не просматривают её), установите **usersListIncludeOffline** в true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Прошлые комментаторы отображаются без зелёной онлайн-точки, чтобы было видно, кто присутствует прямо сейчас.

Пользователи с приватными профилями отображаются с общим аватаром и меткой «Приватный профиль», чтобы количество оставалось точным без раскрытия личности.

Это также можно настроить без кода. На странице настройки виджета посмотрите опцию «Положение списка пользователей». Когда расположение установлено в значение, отличное от Off, под ним появляется флажок «Включать прошлых комментаторов».

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

Если онлайн-пользователей больше 500, список может отставать до 30 секунд.

---