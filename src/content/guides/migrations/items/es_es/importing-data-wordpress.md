Nuestro [plugin de WordPress](https://wordpress.org/plugins/fastcomments/) cuenta con un potente mecanismo de importación basado en la interfaz de usuario. Al instalar el plugin,
este te guiará a través del proceso de vincular tu instalación de WordPress con FastComments y copiar tus datos de comentarios existentes.

**Esto se hace sin copiar ni descargar nada manualmente.**

El proceso de migración se te indicará a través de la interfaz de usuario durante la migración. La mayoría de las migraciones solo toman un par de minutos.

El mecanismo está diseñado para no poner una carga excesiva en tu instalación de WordPress durante la migración.

Si estás trasladando tu sitio fuera de WordPress, puedes importar una exportación XML o CSV de WordPress en lugar de usar el plugin. Consulta
[Moving Your Comments to a New Site](/guide-installation-wordpress.html#wordpress-moving-off-wordpress).

### CloudFlare y Firewalls

Para que la configuración automatizada de WordPress funcione, tenemos que hacer llamadas a tu instalación de WordPress.
Los firewalls como Cloudflare pueden bloquearnos y hacer que la integración falle. En tales casos, [podemos proporcionarte](https://fastcomments.com/auth/my-account/help) un conjunto de IPs para incluir en la lista blanca para la integración.

### Propiedad de los datos

En el caso de nuestra migración de WordPress, cualquier dato de comentario nuevo o actualizado se sincroniza automáticamente de vuelta con tu instalación de WordPress
en segundo plano. Esto significa que, aunque los comentarios son servidos por FastComments para reducir la carga de tu despliegue de WordPress,
también los **guardamos** en tu base de datos como respaldo. Esto también significa que, si deseas dejar de usar FastComments, tus datos ya
están migrados y actualizados.