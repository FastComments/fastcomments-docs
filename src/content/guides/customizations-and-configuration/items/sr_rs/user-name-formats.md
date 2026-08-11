По подразумеваној поставци, FastComments ће приказати име корисника онако како је унето, или како је прослеђено преко SSO.

Међутим, можда је потребно маскирати или приказати име корисника на другачији начин. На пример, ако је име корисника Аллен Рекс, можда желите да прикажете само „Аллен Р.“.

Ово се може урадити без кода у UI-ју за прилагођавање виџета, под подешавањем названим `Commenter Name Format`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; alt='Коментатор Формат Име падајући мени отворен са изборима као што су Capitalize, Last Initial и All Initials'; title='Промени Формат Име' app-screenshot-end]

Доступни формати су:

- Capitalize (display example user as Example User)
- Last Initial (display Example User as Example U.)
- All Initials (display Example User as E. U.)
- Show "Anonymous"

Ефекат промене је одмах. Корисници ће и даље видети своје пуно корисничко име на врху простора за коментаре за себе, али њихови коментари ће приказивати измењено корисничко име.

Корисничка имена се маскирају на серверу ради заштите корисника.