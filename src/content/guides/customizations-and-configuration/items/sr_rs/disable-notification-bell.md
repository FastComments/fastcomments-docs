[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

По подразумеваној поставци, FastComments ће приказати звонице за обавештења у горњем десном углу простора за коментаре.

Ово звонице ће постати црвено и приказати број обавештења које корисник има. Неки примери обавештења су:

- Корисник вам је одговорио.
- Корисник је одговорио у теми у којој сте коментарисали.
- Корисник је дао позитивну оцену вашем коментару.
- Корисник је одговорио на страницу на коју сте претплаћени.

Звонице за обавештења такође пружају механизам за претплату на целу страницу.

Међутим, можемо потпуно онемогућити звонице за обавештења:

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Onemogući zvono obaveštenja'; code-example-end]

Ово се такође може урадити без кода. На страници за прилагођавање виџета, погледајте одељак „Onemogući zvono obaveštenja“.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; alt='Страница за прилагођавање виџета са означеним пољем за онемогућавање звона обавештења'; title='Onemogući zvono obaveštenja' app-screenshot-end]