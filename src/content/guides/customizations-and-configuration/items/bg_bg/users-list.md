[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

По подразбиране FastComments не показва списък с потребители на страницата.

Можете да изобразите списък с хора, които в момента преглеждат страницата, заедно с уиджета за коментари. Списъкът се актуализира в реално време, когато потребителите се присъединяват или напускат, и показва тяхното име, аватар и индикатор за онлайн статус.

Има три варианта за оформление:

- `1` - Горен: хоризонтален ред от припокриващи се аватари, изобразени над коментарите.
- `2` - Ляво: странична лента с имена и онлайн точки, изобразени отляво на уиджета.
- `3` - Дясно: същата странична лента, изобразена отдясно на уиджета.

Задайте флага **usersListLocation**, за да активирате функцията:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

По подразбиране списъкът показва само потребителите, които са онлайн в момента. За да включите и хора, които са коментирали страницата в миналото (но в момента не я преглеждат), задайте **usersListIncludeOffline** на true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Потребителите, които са коментирали в миналото, се изобразяват без зеления онлайн индикатор, за да е ясно кой е присъстващ в момента.

Потребителите с частни профили се показват с общ аватар и етикет "Private Profile", за да се запази точният брой без разкриване на идентичностите.

Това може да се конфигурира и без код. На страницата за персонализиране на уиджета, вижте опцията "Users List Location". Когато местоположението е зададено на нещо различно от Off, се появява отметка "Include past commenters" под нея.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; alt='Местоположението на списъка с потребители е зададено на Дясно, с отметка за включване на минали коментари, показана под него'; title='Настройки на списъка с потребители'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

До 500 живи потребители, списъкът е с до 30 секунди закъснение.