Privzeto FastComments sinhronizira nazaj na vašo WordPress stran dnevno. To je izključno za potrebe varnostne kopije, da še vedno posedujete kopijo podatkov, in za vtičnike, ki bi morda odvisni od tega.

To se ne zgodi takoj ob vsakem shranjenem komentarju zaradi narave nekaterih strani, ki lahko obvladujejo veliko bralnega prometa, njihovi podatkovni sklopi pa niso vedno sposobni obvladovati velikega pisalnega prometa (zaradi tega to delo preusmerimo na FastComments).

Urnik sinhronizacije nazaj v WordPress je mogoče prilagoditi z namestitvijo vtičnika. Priporočamo [WP Crontrol](https://wordpress.org/plugins/wp-crontrol/#description).

Steps:

1. Install WP Crontrol
2. Go to `Settings -> Cron Schedules`.
3. Go to the `Cron Events` tab.
4. Search for `fastcomments_cron_hook`.
5. Edit the event. You can configure the hook to run hourly, twice a day, daily (default), or once a week.

Sinhronizacijo nazaj v WordPress lahko izvedete tudi ročno kadarkoli, tako da odprete nadzorno ploščo vtičnika FastComments in izberete `Manually Sync`. Imeli boste možnost sinhronizirati nazaj na vašo WP namestitev ali ponovno naložiti svoje WP komentarje na strežnike FastComments.