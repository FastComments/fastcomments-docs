[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

По подразумеваној поставци, FastComments ће приказати икону звона за обавештења у горњем десном углу области за коментаре.

То звонце ће постати црвено и показати број обавештења које корисник има. Неки примери обавештења су:

- Корисник вам је одговорио.
- Корисник је одговорио у нити у којој сте коментарисали.
- Корисник је лајковао ваш коментар.
- Корисник је одговорио на страницу на коју сте се претплатили.

Икона звона за обавештења такође пружа механизам за претплату на целу страницу.

Међутим, можемо у потпуности онемогућити икону звона за обавештења:

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Disable Notification Bell'; code-example-end]

Ово се такође може урадити и без кода. На страници за прилагођавање видгета, погледајте одељак „Онемогући икону звона за обавештења“.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; title='Disable Notification Bell' app-screenshot-end]