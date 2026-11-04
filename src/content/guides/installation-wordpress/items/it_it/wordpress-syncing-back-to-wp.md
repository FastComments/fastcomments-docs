Per impostazione predefinita, FastComments esegue la sincronizzazione verso il tuo sito WordPress ogni giorno. Questo avviene esclusivamente a scopo di backup, così che tu continui a possedere una copia dei dati, e per i plugin
che potrebbero dipendere da essi.

Questo non avviene immediatamente a ogni commento salvato poiché, per la natura di alcuni siti in grado di gestire un elevato traffico in lettura, i loro database non sono sempre in grado di gestire l'elevato traffico in scrittura (da qui lo scaricamento di questo lavoro su FastComments).

La pianificazione della sincronizzazione verso WordPress può essere personalizzata installando un plugin. Raccomandiamo [WP Crontrol](https://wordpress.org/plugins/wp-crontrol/#description).

Passaggi:

1. Installa WP Crontrol
2. Vai su `Settings -> Cron Schedules`.
3. Vai alla scheda `Cron Events`.
4. Cerca `fastcomments_cron_hook`.
5. Modifica l'evento. Puoi configurare l'hook per essere eseguito ogni ora, due volte al giorno, ogni giorno (predefinito) o una volta a settimana.

La sincronizzazione verso WordPress può essere eseguita anche manualmente in qualsiasi momento, andando alla dashboard del plugin FastComments e selezionando `Manually Sync`. Avrai la possibilità di sincronizzare verso la tua installazione WP, oppure di ricaricare i tuoi commenti WP sui server di FastComments.