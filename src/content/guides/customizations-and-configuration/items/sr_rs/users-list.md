[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

По подразумевaњу, FastComments не приказује листу корисника на страници.

Можете приказати листу људи који тренутно прегледају страницу, поред виџета за коментаре. Листа се ажурира у реалном времену како корисници улазе и излазе, и приказује њихово име, аватар и индикатор онлајн статуса.

Постоје три опције распореда:

- `1` - Top: a horizontal row of overlapping avatars rendered above the comments.
- `2` - Left: a sidebar with names and online dots rendered to the left of the widget.
- `3` - Right: the same sidebar rendered to the right of the widget.

Поставите флаг **usersListLocation** да омогућите ову функцију:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

По подразумевању, листа приказује само кориснике који су тренутно онлајн. Да бисте такође укључили људе који су раније коментарисали на страници (али тренутно је не прегледају), поставите **usersListIncludeOffline** на true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Ранији коментатори се приказују без зелене тачке онлајн статуса како би било јасно ко је тренутно присутан.

Корисници са приватним профилима се приказују са општим аватаром и ознаком "Приватни профил" тако да бројка остаје тачна без откривања идентитета.

Ово се такође може подесити без кода. На страници за прилагођавање виџета, погледајте опцију "Users List Location". Када је локација постављена на било шта друго осим Off, појављује се поље за потврду "Include past commenters" испод ње.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

За преко 500 активних корисника, листа може каснити до 30 секунди.