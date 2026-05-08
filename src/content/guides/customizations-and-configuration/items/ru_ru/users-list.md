[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

По умолчанию FastComments не показывает список пользователей на странице.

Вы можете отображать список людей, которые в данный момент просматривают страницу, рядом с виджетом комментариев. Список обновляется в реальном времени по мере прихода и ухода пользователей и показывает их имя, аватар и индикатор онлайн.

Есть три варианта расположения:

- `1` - Top: горизонтальная строка перекрывающихся аватаров, отображаемая над комментариями.
- `2` - Left: боковая панель с именами и индикаторами онлайн, отображаемая слева от виджета.
- `3` - Right: та же боковая панель, отображаемая справа от виджета.

Установите флаг **usersListLocation**, чтобы включить эту функцию:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

По умолчанию список показывает только пользователей, которые сейчас онлайн. Чтобы также включить людей, которые ранее оставляли комментарии на странице (но сейчас не просматривают её), установите **usersListIncludeOffline** в true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Пользователи, оставлявшие комментарии ранее, отображаются без зелёной точки онлайн, чтобы было ясно, кто присутствует в данный момент.

Пользователи с приватными профилями отображаются с общим аватаром и пометкой "Private Profile", чтобы количество было точным без раскрытия личностей.

Это также можно настроить без кода. На странице настройки виджета смотрите опцию "Users List Location". Когда расположение установлено в любое значение, кроме Off, под ним появляется флажок "Include past commenters".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]