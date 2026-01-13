Varsayılan olarak, FastComments günlük olarak WordPress sitenizle eşitleme yapar. Bu, verinin bir kopyasına sahip olmaya devam etmeniz, ve bazı eklentiler
onlara bağımlı olabileceği için tamamen yedekleme amaçlıdır.

Bu, bazı sitelerin yoğun okuma trafiğini kaldırabilme doğası nedeniyle kaydedilen her yorumla hemen gerçekleşmez; veritabanı dağıtımları her zaman yoğun yazma trafiğini kaldıramayabilir (bu nedenle bu işi FastComments'e devrediyoruz).

WordPress'e geri eşitleme zamanlaması bir eklenti yükleyerek özelleştirilebilir. Biz [WP Crontrol](https://wordpress.org/plugins/wp-crontrol/#description) öneriyoruz.

Steps:

1. Install WP Crontrol
2. Go to `Settings -> Cron Schedules`.
3. Go to the `Cron Events` tab.
4. Search for `fastcomments_cron_hook`.
5. Edit the event. You can configure the hook to run hourly, twice a day, daily (default), or once a week.

The sync back to WordPress can also be performed any time manually by going to the FastComments plugin dashboard and selecting `Manually Sync`. You will have
the option to sync back to your WP install, or to re-upload your WP comments to FastComments servers.