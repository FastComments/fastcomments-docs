[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

По подразбиране FastComments не показва списък с потребители на страницата.

Можете да визуализирате списък с хора, които в момента разглеждат страницата, до уиджета за коментари. Списъкът се обновява в реално време, когато потребителите влизат и излизат, и показва техните имена, аватари и индикатор за онлайн статус.

Има три опции за оформление:

- `1` - Горе: хоризонтален ред с припокриващи се аватари, изобразен над коментарите.
- `2` - Вляво: странична лента с имена и точки за онлайн статус, изобразена вляво от уиджета.
- `3` - Вдясно: същата странична лента, изобразена вдясно от уиджета.

Задайте флага **usersListLocation**, за да активирате тази функция:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

По подразбиране списъкът показва само потребителите, които са в момента онлайн. За да включите също хора, които са коментирали страницата в миналото (но в момента не я разглеждат), задайте **usersListIncludeOffline** на true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Потребителите, които са коментирали в миналото, се показват без зелената точка за онлайн статус, за да е ясно кои са присъстващите в момента.

Потребители с частни профили се показват с общ аватар и етикет "Частен профил", така че броят да остане точен, без да се разкриват самоличности.

Това може да се конфигурира и без код. В страницата за персонализиране на уиджета вижте опцията "Позиция на списъка с потребители". Когато местоположението е зададено на нещо различно от Изключено, под нея се появява квадратче "Включи минали коментатори".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

---