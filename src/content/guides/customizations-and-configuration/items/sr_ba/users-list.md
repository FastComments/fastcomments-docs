[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

По подразумевању FastComments не приказује списак корисника на страници.

Можете приказати списак особа које тренутно гледају страницу, поред виджета за коментаре. Списак се ажурира уживо док корисници долазе и одлазе, и приказује њихово име, аватар и индикатор да су на мрежи.

Постоје три опције распореда:

- `1` - Горњи: хоризонтални низ преклапајућих аватара приказан изнад коментара.
- `2` - Леви: бочна трака са именима и онлајн тачкама приказана лево од виджета.
- `3` - Десни: иста бочна трака приказана десно од виджета.

Поставите **usersListLocation** флаг да омогућите ову функцију:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

По подразумевању списак приказује само кориснике који су тренутно на мрежи. Да бисте укључили и људе који су раније коментарисали на страници (али тренутно не гледају), поставите **usersListIncludeOffline** на true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Ранији коментатори се приказују без зелене онлајн тачке тако да је јасно ко је тренутно присутан.

Корисници са приватним профилима приказани су са општим аватаром и ознаком "Private Profile" па број остаје тачан без разоткривања идентитета.

Ово се такође може подесити без кода. На страници за прилагођавање виджета погледајте опцију "Users List Location". Када је локација постављена на било шта осим Искључено, испод ње се појављује поље за потврду "Include past commenters".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]