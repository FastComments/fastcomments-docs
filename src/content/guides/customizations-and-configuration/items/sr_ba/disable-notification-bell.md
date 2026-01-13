---
[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

По подразумеваној поставци, FastComments ће приказати звонац за обавештења у горњем десном углу подручја за коментаре.

Ово звоно ће постати црвено и показати број обавештења која корисник има. Нека примјерна обавештења су:

- Корисник вам је одговорио.
- Корисник је одговорио у ниту у којој сте коментарисали.
- Корисник је позитивно оценио ваш коментар.
- Корисник је одговорио на страницу на коју сте претплаћени.

Звоно за обавештења такође пружа механизам за претплату на целу страницу.

Међутим, можемо потпуно онемогућити звоно за обавештења:

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Disable Notification Bell'; code-example-end]

Ово се такође може урадити без кода. На страници за прилагођавање виџета погледајте одељак "Онемогући звоно за обавештења".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; title='Disable Notification Bell' app-screenshot-end]

---