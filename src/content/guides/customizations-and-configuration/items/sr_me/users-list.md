[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Подразумевано, FastComments не приказује листу корисника на страници.

Можете приказати листу особа које тренутно прегледају страницу, заједно са видгетом за коментаре. Листа се ажурира уживо док корисници долазе и одлазе, и приказује њихово име, аватар и индикатор да су онлајн.

Постоје три опције распореда:

- `1` - На врху: хоризонтални ред преклапајућих се аватара приказан изнад коментара.
- `2` - Лево: бочна трака са именима и онлајн тачкицама приказана лево од видгета.
- `3` - Десно: иста бочна трака приказана десно од видгета.

Поставите заставицу **usersListLocation** да омогућите функцију:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

Подразумевано листа приказује само кориснике који су тренутно онлајн. Да бисте укључили и људе који су раније коментарисали на страници (али тренутно нису присутни), поставите **usersListIncludeOffline** на true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Ранији коментатори се приказују без зелене онлајн тачкице како би било јасно ко је тренутно присутан.

Корисници са приватним профилима се приказују са општим аватаром и ознаком "Private Profile" тако да број остаје тачан без откривања идентитета.

Ово се може подесити и без кода. На страници за прилагођавање видгета, погледајте опцију "Локација листе корисника":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-location'; title='Users List Location' app-screenshot-end]

Када је локација постављена на било шта осим Искључено, испод ње се приказује поље за потврду "Укључи претходне коментаторе":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-include-offline'; title='Include Past Commenters' app-screenshot-end]