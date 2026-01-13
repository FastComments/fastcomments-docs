По подразумевању, FastComments синхронизује назад на ваш WordPress сајт дневно. Ово је искључиво у сврху резервне копије тако да и даље поседујете копију података, и за додатке који могу зависити од ње.

Ово се не дешава одмах за сваки сачувани коментар због саме природе неких сајтова који могу да поднесу велико оптерећење читања, али њихове базе података нису увек у стању да поднесу велико оптерећење уписа (због тога се овај посао пребацује на FastComments).

Распоред синхронизације назад на WordPress може се прилагодити инсталирањем додатка. Препоручујемо [WP Crontrol](https://wordpress.org/plugins/wp-crontrol/#description).

Steps:

1. Install WP Crontrol
2. Go to `Settings -> Cron Schedules`.
3. Go to the `Cron Events` tab.
4. Search for `fastcomments_cron_hook`.
5. Edit the event. You can configure the hook to run hourly, twice a day, daily (default), or once a week.

Синхронизација назад на WordPress такође се може извршити у било ком тренутку ручно тако што ћете отићи на контролну таблу додатка FastComments и изабрати `Manually Sync`. Имаћете опцију да синхронизујете назад на вашу WP инсталацију, или да поново отпремите ваше WP коментаре на FastComments сервере.