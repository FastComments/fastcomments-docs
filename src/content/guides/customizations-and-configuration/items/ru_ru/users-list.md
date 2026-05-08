[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

По умолчанию FastComments не показывает список пользователей на странице.

Вы можете отображать список людей, которые в данный момент просматривают страницу, рядом с виджетом комментариев. Список обновляется в реальном времени по мере того, как пользователи заходят и уходят, и показывает их имя, аватар и индикатор онлайн.

Доступны три варианта расположения:

- `1` - Вверху: горизонтальная строка перекрывающихся аватаров, отображаемая над комментариями.
- `2` - Слева: боковая панель с именами и индикаторами онлайн, отображаемая слева от виджета.
- `3` - Справа: та же боковая панель, отображаемая справа от виджета.

Установите флаг **usersListLocation**, чтобы включить эту функцию:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

По умолчанию список показывает только пользователей, которые находятся в сети в данный момент. Чтобы также включить людей, которые в прошлом комментировали страницу (но сейчас её не просматривают), установите **usersListIncludeOffline** в true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Прошлые комментаторы отображаются без зелёной точки онлайн, чтобы было видно, кто присутствует прямо сейчас.

Пользователи с приватными профилями отображаются с универсальным аватаром и меткой «Приватный профиль», чтобы число оставалось точным без раскрытия личности.

Это также можно настроить без кода. На странице настройки виджета воспользуйтесь опцией «Расположение списка пользователей»:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-location'; title='Users List Location' app-screenshot-end]

Когда расположение установлено в значение, отличное от Off, ниже отображается флажок «Include past commenters»:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-include-offline'; title='Include Past Commenters' app-screenshot-end]

---