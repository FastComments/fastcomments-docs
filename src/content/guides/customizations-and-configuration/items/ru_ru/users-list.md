[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

По умолчанию FastComments не отображает список пользователей на странице.

Вы можете отобразить список людей, которые в данный момент просматривают страницу, рядом с виджетом комментариев. Список обновляется в реальном времени по мере присоединения и ухода пользователей и показывает их имя, аватар и индикатор онлайн.

Доступны три варианта расположения:

- `1` - Верх: горизонтальная строка перекрывающихся аватаров, отображаемая над комментариями.
- `2` - Слева: боковая панель с именами и точками онлайн, отображаемая слева от виджета.
- `3` - Справа: та же боковая панель, отображаемая справа от виджета.

Set the **usersListLocation** flag to enable the feature:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

По умолчанию список показывает только пользователей, которые в настоящее время в сети. Чтобы также включить людей, которые оставляли комментарии на странице в прошлом (но сейчас не просматривают её), установите **usersListIncludeOffline** в true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Бывшие комментаторы отображаются без зелёной точки «онлайн», чтобы было ясно, кто присутствует прямо сейчас.

Пользователи с приватным профилем отображаются с общим аватаром и меткой «Приватный профиль», чтобы счёт оставался точным без раскрытия личностей.

Это также можно настроить без кода. На странице настройки виджета смотрите опцию «Расположение списка пользователей». Когда местоположение установлено в любое значение, отличное от «Отключено», под ним появляется флажок «Включить прошлых комментаторов».

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

Если одновременно онлайн более 500 пользователей, список может отставать до 30 секунд.