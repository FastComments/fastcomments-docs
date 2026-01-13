Nuestro [complemento de WordPress](https://wordpress.org/plugins/fastcomments/) tiene un potente mecanismo de importación basado en la interfaz de usuario. Al instalar el complemento, le guiará para vincular su instalación de WordPress con FastComments y copiar sus datos de comentarios existentes.

**Esto se hace sin copiar o descargar nada manualmente.**

El proceso de migración se le indicará a través de la interfaz de usuario durante la migración. La mayoría de las migraciones tarda solo un par de minutos.

El mecanismo está diseñado para no poner una carga excesiva en su instalación de WordPress durante la migración.

### CloudFlare & Cortafuegos

Para que la configuración automática de WordPress funcione, tenemos que realizar llamadas a su instalación de WordPress. Cortafuegos como Cloudflare pueden bloquearnos y provocar que la integración falle. En tales casos, [podemos proporcionarle](https://fastcomments.com/auth/my-account/help) un conjunto de IPs para incluir en la lista blanca para la integración.

### Propiedad de los datos

En el caso de nuestra migración de WordPress, cualquier dato de comentarios nuevo o actualizado se sincroniza automáticamente de vuelta a su instalación de WordPress en segundo plano. Esto significa que, aunque los comentarios los sirve FastComments para aliviar la carga de su implementación de WordPress, **también** los guardamos en su base de datos como copia de seguridad. Esto también significa que, si desea dejar FastComments, sus datos ya están migrados y actualizados.