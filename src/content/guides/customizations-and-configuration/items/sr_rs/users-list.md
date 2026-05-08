[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

По подразумеваној вредности, FastComments не приказује листу корисника на страници.

Можете приказати листу људи који тренутно прегледају страницу, поред видгета за коментаре. Листа се ажурира уживо док корисници улазе и напуштају страницу, и приказује њихово име, аватар и индикатор онлајн статуса.

Постоје три опције распореда:

- `1` - Горе: хоризонтални ред преклапајућих се аватара који се приказују изнад коментара.
- `2` - Лево: бочни панел са именима и тачкама онлајн статуса који се приказује лево од видгета.
- `3` - Десно: исти бочни панел који се приказује десно од видгета.

Подесите флаг **usersListLocation** да бисте омогућили функцију:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

По подразумеваној вредности листа приказује само кориснике који су тренутно на мрежи. Да бисте укључили и особе које су раније коментарисале на страници (али их тренутно нема међу прегледачима), подесите **usersListIncludeOffline** на true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Ранији коментатори се приказују без зелене онлајн тачке, тако да је јасно ко је тренутно присутан.

Корисници са приватним профилима приказују се са општим аватаром и ознаком „Приватни профил“ тако да број остаје тачан без откривања идентитета.

Ово се може конфигурисати и без кода. На страници за прилагођавање видгета погледајте опцију „Локација листе корисника“. Када је локација подешена на било шта осим Искључено, испод ње се појављује поље за потврду „Укључи претходне коментаторе“.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]