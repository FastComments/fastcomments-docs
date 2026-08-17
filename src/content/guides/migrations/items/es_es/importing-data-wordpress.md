Nuestro [WordPress Plugin](https://wordpress.org/plugins/fastcomments/) tiene un potente mecanismo de importación basado en UI. Al instalar el plugin, te guiará para vincular tu instalación de WordPress con FastComments y copiar tus datos de comentarios existentes.

**Esto se hace sin copiar o descargar nada manualmente.**

El proceso de migración se indicará a través de la UI durante la migración. La mayoría de las migraciones solo toman un par de minutos.

El mecanismo está diseñado para no imponer una carga excesiva a tu instalación de WordPress durante la migración.

### CloudFlare y Firewalls

Para que la configuración automática de WordPress funcione, debemos hacer llamadas a tu instalación de WordPress.  
Los firewalls como Cloudflare pueden bloquearnos y causar que la integración falle. En esos casos, [podemos proporcionarte](https://fastcomments.com/auth/my-account/help) un conjunto de IPs para incluir en la lista blanca para la integración.

### Propiedad de los datos

En el caso de nuestra migración de WordPress, cualquier dato de comentario nuevo o actualizado se sincroniza automáticamente de vuelta a tu instalación de WordPress detrás de escena. Esto significa que, mientras los comentarios son servidos por FastComments mismo para aliviar la carga de tu despliegue de WordPress, **también** los guardamos en tu base de datos como respaldo. Esto también significa que si deseas cambiarte de FastComments, tus datos ya están migrados y actualizados.