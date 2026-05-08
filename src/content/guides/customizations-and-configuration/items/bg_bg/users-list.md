[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

По подразбиране FastComments не показва списък с потребители на страницата.

Можете да показвате списък с хора, които в момента разглеждат страницата, до коментарния уиджет. Списъкът се обновява в реално време, когато потребители влизат и излизат, и показва тяхното име, аватар и индикатор за онлайн статут.

Има три опции за оформление:

- `1` - Top: хоризонтален ред от припокриващи се аватари, показани над коментарите.
- `2` - Left: странична лента с имена и онлайн точки, показана вляво от уиджета.
- `3` - Right: същата странична лента, показана вдясно от уиджета.

Задайте флага **usersListLocation**, за да активирате функцията:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

По подразбиране списъкът показва само потребителите, които в момента са онлайн. За да включите и хора, които са коментирали страницата в миналото (но в момента не я разглеждат), задайте **usersListIncludeOffline** на true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Потребителите, които са коментирали в миналото, се показват без зелена онлайн точка, за да е ясно кои са присъстващите в момента.

Потребителите с частни профили се показват с общ аватар и етикет „Частен профил“, така че броят да остава точен без разкриване на самоличности.

Това може да се конфигурира и без код. В страницата за персонализиране на уиджета вижте опцията „Местоположение на списъка с потребители“. Когато местоположението е зададено на нещо различно от „Изключено“, под него се появява квадратче за отметка „Включи предишни коментатори“.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

Ако има над 500 потребителя онлайн, списъкът може да е до 30 секунди остарял.