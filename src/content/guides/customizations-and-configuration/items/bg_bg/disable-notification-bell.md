[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

По подразбиране FastComments ще покаже звънец за известия в горния десен ъгъл на областта за коментари.

Този звънец ще стане червен и ще покаже броя на известията, които потребителят има. Някои примерни известия са:

- Потребителят ви отговори.
- Потребителят отговори в нишка, в която сте коментирали.
- Потребителят гласува вашия коментар.
- Потребителят отговори на страница, към която сте се абонирали.

Звънецът за известия предоставя механизъм за абониране за цяла страница, също така.

Въпреки това можем напълно да изключим звънеца за известия:

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Disable Notification Bell'; code-example-end]

Това може да се направи и без код. В страницата за персонализиране на уиджет, вижте секцията „Disable Notification Bell“.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; alt='Страница за персонализиране на уиджет с отметка за изключване на известията включена'; title='Изключване на известията' app-screenshot-end]