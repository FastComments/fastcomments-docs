[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

По подразумевању, FastComments не приказује листу корисника на страници.

Можете приказати листу особа које тренутно гледају страницу, поред видгета за коментаре. Листа се ажурира у реалном времену док корисници улазе и излазе, и показује њихово име, аватар и индикатор онлајн статуса.

Постоје три опције распореда:

- `1` - Горе: хоризонтални ред преклапајућих аватара приказан изнад коментара.
- `2` - Лево: бочна трака са именима и онлајн тачкама приказана лево од видгета.
- `3` - Десно: иста бочна трака приказана десно од видгета.

Подесите флаг **usersListLocation** да бисте омогућили ову функцију:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

Подразумевано листа приказује само кориснике који су тренутно онлајн. Да бисте укључили и људе који су раније коментарисали на страници (али тренутно не гледају), поставите **usersListIncludeOffline** на true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Ранији коментатори се приказују без зелене онлајн тачке како би било јасно ко је тренутно присутан.

Корисници са приватним профилима се приказују са општим аватаром и ознаком "Приватни профил" тако да број остаје тачан без откривања идентитета.

Ово се такође може конфигурисати без кода. На страници за прилагођавање видгета, погледајте опцију "Users List Location":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-location'; title='Users List Location' app-screenshot-end]

Када је локација подешена на било шта друго осим Off, испод ње се приказује поље за потврду "Include Past Commenters":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-include-offline'; title='Include Past Commenters' app-screenshot-end]