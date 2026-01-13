A veces FastComments tiene que enviar correos electrónicos a tus usuarios, especialmente si no estás usando Secure SSO.

Ejemplos de esto incluyen verificar su cuenta o actividad cuando comentan por primera vez. FastComments
también les enviará notificaciones por respuestas a sus comentarios.

Cuando FastComments envía correos a tus usuarios, usaremos un Nombre y Correo From predeterminados de `FastComments Robot` y `noreply@fastcomments.com`.

También usaremos nuestro propio logo en el pie de estos correos.

Si tienes FastComments Flex o Pro, todo esto se puede personalizar por dominio a través de la "página Mis dominios":

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; title='Customizing From Name, Email, and Logo' app-screenshot-end]

Al personalizar el logo que se muestra en los correos, asegúrate de que el tamaño que estás subiendo sea el mismo tamaño que quieres mostrar en el pie del correo.

### When Customizing The `From Domain`

Si personalizas el `From Domain`, los proveedores y clientes de correo necesitan saber que FastComments está autorizado para enviar correos en tu nombre. De lo contrario,
definir el `From Domain` y no seguir los pasos a continuación probablemente hará que los correos vayan a spam.

#### 1. Setup SPF

Para permitir que FastComments envíe correo de forma segura como tu dominio, asegúrate de añadir un registro SPF que nos lo permita.

Asegúrate de que haya registros SPF que permitan que `mail.fastcomments.com` y `sib.fastcomments.com` envíen correo como tu dominio.

Más información sobre cómo hacerlo está aquí: https://mailtrap.io/blog/multiple-spf-records/

#### 2. Setup DKIM

Además del SPF, deberías configurar DKIM. Una vez que tu configuración DNS esté lista, puedes hacer clic en "Show Advanced" en la página de configuraciones de dominio
para mostrar los ajustes DKIM por dominio.

También puedes [invocar la API](/guide-api.html#domain-config-structure) para establecer la configuración DKIM.

### Unsubscribe Links

Al usar SSO, las funciones de cancelación de suscripción utilizadas en correos y notificaciones se pueden personalizar [a través de la API DomainConfigs](/guide-api.html#domain-config-structure).