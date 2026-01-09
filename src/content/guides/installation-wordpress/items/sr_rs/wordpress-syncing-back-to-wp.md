По подразумеваној поставци, FastComments синхронизује податке назад на ваш WordPress сајт дневно. Ово је искључиво у сврху резервне копије како бисте и даље поседовали копију података, и за додатке
који могу зависити од ње.

Ово се не дешава одмах при сваком сачуваном коментару због природе неких сајтова који могу да поднесу велики читачки саобраћај, њихове базе података нису увек у стању да издрже велики уписни саобраћај (због тога се тај посао преусмерава на FastComments).

Распоред синхронизације назад на WordPress може се прилагодити инсталацијом додатка. Препоручујемо [WP Crontrol](https://wordpress.org/plugins/wp-crontrol/#description).

Steps:

1. Install WP Crontrol
2. Go to `Settings -> Cron Schedules`.
3. Go to the `Cron Events` tab.
4. Search for `fastcomments_cron_hook`.
5. Edit the event. You can configure the hook to run hourly, twice a day, daily (default), or once a week.

Синхронизација назад на WordPress се такође може обавити у било ком тренутку ручно тако што ћете отићи на контролну таблу FastComments додатка и изабрати `Manually Sync`. You will have
the option to sync back to your WP install, or to re-upload your WP comments to FastComments servers.