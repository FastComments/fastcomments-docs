Por defecto, FastComments sincroniza de vuelta con tu sitio de WordPress a diario. Esto es puramente con fines de copia de seguridad, para que sigas siendo dueño de una copia de los datos, y para los plugins
que puedan depender de ello.

Esto no ocurre inmediatamente con cada comentario guardado debido a que, por la naturaleza de algunos sitios capaces de manejar un gran tráfico de lectura, sus despliegues de base de datos no siempre pueden manejar el gran tráfico de escritura (de ahí que se descargue este trabajo en FastComments).

El programa de sincronización de vuelta a WordPress se puede personalizar instalando un plugin. Recomendamos [WP Crontrol](https://wordpress.org/plugins/wp-crontrol/#description).

Pasos:

1. Instala WP Crontrol
2. Ve a `Settings -> Cron Schedules`.
3. Ve a la pestaña `Cron Events`.
4. Busca `fastcomments_cron_hook`.
5. Edita el evento. Puedes configurar el hook para que se ejecute cada hora, dos veces al día, a diario (por defecto) o una vez por semana.

La sincronización de vuelta a WordPress también se puede realizar manualmente en cualquier momento yendo al panel del plugin de FastComments y seleccionando `Manually Sync`. Tendrás la
opción de sincronizar de vuelta con tu instalación de WP, o de volver a subir tus comentarios de WP a los servidores de FastComments.