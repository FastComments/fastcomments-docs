Per impostazione predefinita, FastComments esegue una sincronizzazione giornaliera verso il tuo sito WordPress. Questo serve esclusivamente a scopi di backup, così continui a possedere una copia dei dati, e per i plugin che potrebbero dipendere da essa.

Questo non avviene immediatamente ad ogni salvataggio di commento perché, dato che alcuni siti riescono a gestire un elevato traffico di lettura, i loro deployment di database non sono sempre in grado di gestire l'elevato traffico di scrittura (da qui l'idea di delegare questo lavoro a FastComments).

La pianificazione della sincronizzazione verso WordPress può essere personalizzata installando un plugin. Raccomandiamo [WP Crontrol](https://wordpress.org/plugins/wp-crontrol/#description).

Passaggi:

1. Installa WP Crontrol
2. Go to `Settings -> Cron Schedules`.
3. Go to the `Cron Events` tab.
4. Search for `fastcomments_cron_hook`.
5. Edit the event. You can configure the hook to run hourly, twice a day, daily (default), or once a week.

La sincronizzazione verso WordPress può anche essere eseguita manualmente in qualsiasi momento accedendo alla dashboard del plugin FastComments e selezionando `Manually Sync`. Avrai l'opzione di sincronizzare nuovamente la tua installazione WP, o di caricare nuovamente i commenti WP sui server di FastComments.