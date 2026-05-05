A veces FastComments tiene que enviar correos electrónicos a tus usuarios, especialmente si no estás utilizando Secure SSO.

Ejemplos de esto incluyen verificar su cuenta o actividad cuando comentan por primera vez. FastComments
también les enviará notificaciones por respuestas a sus comentarios.

Cuando FastComments envía correos a tus usuarios, usaremos un Nombre del Remitente y un Correo electrónico predeterminados de `FastComments Robot` y `noreply@fastcomments.com`.

También usaremos nuestro propio logotipo en el pie de página de estos correos.

Si tienes FastComments Flex o Pro, todo esto se puede personalizar en una base por dominio vía la "My Domains page":

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; title='Customizing From Name, Email, and Logo' app-screenshot-end]

Al personalizar el logotipo que se muestra en los correos, asegúrate de que el tamaño que estás subiendo sea el mismo tamaño que quieres mostrar en el pie del correo.

### When Customizing The `From Domain`

Si personalizas el `From Domain`, los proveedores y clientes de correo necesitan saber que FastComments está autorizado para enviar correos en tu nombre. De lo contrario,
definir el `From Domain` sin seguir los pasos que se indican a continuación probablemente hará que los correos vayan a la carpeta de spam.

#### 1. Setup SPF

Para permitir que FastComments envíe correos de forma segura como tu dominio, asegúrate de añadir un registro SPF que nos lo permita.

Asegúrate de que existan registros SPF que permitan que `mail.fastcomments.com` y `sib.fastcomments.com` envíen correos como tu dominio.

Más información sobre cómo hacer esto está aquí: https://mailtrap.io/blog/multiple-spf-records/

#### 2. Setup DKIM

Además del SPF, deberías configurar DKIM. Una vez que tu configuración DNS esté lista, puedes hacer clic en "Mostrar avanzado" en la página de configuración de dominios
para ver la configuración DKIM por dominio.

También puedes [invocar la API](/guide-api.html#domain-config-structure) para establecer la configuración DKIM.

### Unsubscribe Links

Al usar SSO, las funciones de cancelación de suscripción utilizadas en correos y notificaciones se pueden personalizar [a través de la API DomainConfigs](/guide-api.html#domain-config-structure).

### Email Link Obfuscation

Si la reputación del dominio de tu sitio está causando que los correos de notificación lleguen a spam, puedes enrutar los botones "view comment" a través de `fastcomments.com` en lugar de enlazar directamente a tu página. Los proveedores de correo puntúan cada enlace en el cuerpo del correo en función de la reputación del destino, por lo que cuando tu dominio está siendo señalado, los enlaces directos contribuyen a la puntuación de spam independientemente de lo correcta que esté tu configuración de envío.

Activa esto en "Mostrar avanzado" en la My Domains page, en la sección "Email Link Obfuscation". La configuración es por dominio.

Cuando está habilitado, los enlaces en los correos de mention, reply, new-comment, subscribed-page, profile-comment y digest se reescriben a tokens cortos que redirigen a la página original al hacer clic. El destino está vinculado a tu tenant: la redirección solo reenvía a URLs cuyo host coincide con uno de tus dominios configurados, y los tokens caducan automáticamente después de 30 días.

La experiencia al hacer clic no cambia. Los lectores siguen llegando a tu página con el comentario desplazado a la vista.