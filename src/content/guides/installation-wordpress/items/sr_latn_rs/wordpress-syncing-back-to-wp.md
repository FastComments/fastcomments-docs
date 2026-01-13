Podrazumevano, FastComments svakodnevno sinhronizuje podatke nazad na vaš WordPress sajt. Ovo je isključivo u svrhe rezervne kopije tako da i dalje posedujete kopiju podataka, i za pluginove
koji od toga mogu zavisiti.

Ovo se ne dešava odmah za svaki sačuvan komentar zbog toga što, iako neki sajtovi mogu da podnesu veliki saobraćaj za čitanje, njihove baze podataka nisu uvek u mogućnosti da podnesu intenzivan saobraćaj za pisanje (zbog toga se ovaj posao prebacuje na FastComments).

Raspored sinhronizacije nazad na WordPress može se prilagoditi instaliranjem plugina. Preporučujemo [WP Crontrol](https://wordpress.org/plugins/wp-crontrol/#description).

Steps:

1. Install WP Crontrol
2. Go to `Settings -> Cron Schedules`.
3. Go to the `Cron Events` tab.
4. Search for `fastcomments_cron_hook`.
5. Edit the event. You can configure the hook to run hourly, twice a day, daily (default), or once a week.

The sync back to WordPress can also be performed any time manually by going to the FastComments plugin dashboard and selecting `Manually Sync`. You will have
the option to sync back to your WP install, or to re-upload your WP comments to FastComments servers.