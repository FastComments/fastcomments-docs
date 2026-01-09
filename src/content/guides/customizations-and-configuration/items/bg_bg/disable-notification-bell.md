[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

По подразбиране FastComments ще покаже звънец за уведомления в горния десен ъгъл на зоната за коментари.

Този звънец ще стане червен и ще покаже брой на уведомленията, които потребителят има. Някои примерни уведомления са:

- Потребителят ви отговори.
- Потребителят отговори в нишка, в която сте коментирали.
- Потребителят гласува положително за вашия коментар.
- Потребителят отговори на страница, за която сте се абонирали.

Звънецът за уведомления предоставя и механизъм за абониране за цяла страница.

Въпреки това, можем да деактивираме звънеца за уведомления напълно:

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Disable Notification Bell'; code-example-end]

Това може да се направи и без код. В страницата за персонализиране на джаджата вижте раздела "Disable Notification Bell".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; title='Disable Notification Bell' app-screenshot-end]