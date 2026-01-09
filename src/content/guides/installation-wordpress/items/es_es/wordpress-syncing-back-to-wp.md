---
Por defecto, FastComments sincroniza de vuelta con su sitio de WordPress diariamente. Esto es puramente para fines de copia de seguridad para que usted continúe poseyendo una copia de los datos, y para plugins
que puedan depender de ello.

Esto no ocurre inmediatamente con cada comentario guardado debido a la naturaleza de que algunos sitios, aunque pueden manejar un alto tráfico de lectura, sus despliegues de base de datos no siempre pueden manejar el alto tráfico de escritura (por eso se descarga este trabajo a FastComments).

El horario de sincronización de regreso a WordPress se puede personalizar instalando un plugin. Recomendamos [WP Crontrol](https://wordpress.org/plugins/wp-crontrol/#description).

Steps:

1. Install WP Crontrol
2. Go to `Settings -> Cron Schedules`.
3. Go to the `Cron Events` tab.
4. Search for `fastcomments_cron_hook`.
5. Edit the event. You can configure the hook to run hourly, twice a day, daily (default), or once a week.

The sync back to WordPress can also be performed any time manually by going to the FastComments plugin dashboard and selecting `Manually Sync`. You will have
the option to sync back to your WP install, or to re-upload your WP comments to FastComments servers.

---