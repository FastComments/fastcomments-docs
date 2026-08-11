A veces FastComments tiene que enviar correos electrónicos a sus usuarios, especialmente si no está utilizando Secure SSO.

Ejemplos de esto incluyen verificar su cuenta o actividad al comentar por primera vez. FastComments también les enviará notificaciones de respuestas a sus comentarios.

Cuando FastComments envía correos electrónicos a sus usuarios, utilizaremos un Nombre y Correo Electrónico predeterminados de `FastComments Robot` y `noreply@fastcomments.com`.

También utilizaremos nuestro propio logotipo en el pie de estos correos.

Si tiene FastComments Flex o Pro, todo esto se puede personalizar por dominio a través de la página "My Domains":

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; alt='Formulario de configuración de correo electrónico por dominio con los campos Nombre del Remitente, Correo del Remitente y carga de logotipo'; title='Personalizando Nombre del Remitente, Correo y Logotipo' app-screenshot-end]

Al personalizar el logotipo que se muestra en los correos electrónicos, asegúrese de que el tamaño que está cargando sea el mismo que desea mostrar en el pie del correo.

### Al Personalizar el `From Domain`

Si personaliza el `From Domain`, los proveedores de correo electrónico y los clientes deben saber que FastComments está autorizado a enviar correos en su nombre. De lo contrario, definir el `From Domain` y no seguir los pasos a continuación probablemente hará que los correos terminen en spam.

#### 1. Configurar SPF

Para permitir que FastComments envíe correos de forma segura como su dominio, asegúrese de agregar un registro SPF que nos lo permita.

Asegúrese de que existan registros SPF que permitan a `mail.fastcomments.com` y `sib.fastcomments.com` enviar correos como su dominio.

Más información sobre cómo hacerlo está aquí: https://mailtrap.io/blog/multiple-spf-records/

#### 2. Configurar DKIM

Además de SPF, debe configurar DKIM. Una vez que su configuración DNS esté lista, puede hacer clic en "Show Advanced" en la página de configuraciones de dominio para mostrar la configuración DKIM por dominio.

También puede [invocar la API](/guide-api.html#domain-config-structure) para establecer la configuración DKIM.

### Enlaces para Darse de Baja

Al usar SSO, las funciones de darse de baja utilizadas en correos y notificaciones pueden personalizarse [a través de la API DomainConfigs](/guide-api.html#domain-config-structure).

### Ofuscación de Enlaces de Correo

Si la reputación del dominio de su sitio está provocando que los correos de notificación terminen en spam, puede enrutar los botones "ver comentario" a través de `fastcomments.com` en lugar de enlazar directamente a su página. Los proveedores de buzones califican cada enlace en el cuerpo del correo según la reputación del destino, por lo que cuando su dominio está marcado, los enlaces directos contribuyen a la puntuación de spam sin importar cuán limpia sea su configuración de envío.

Active esto bajo "Show Advanced" en la página My Domains, en la sección "Email Link Obfuscation". La configuración es por dominio.

Cuando está habilitado, los enlaces en correos de mención, respuesta, nuevo comentario, página suscrita, comentario de perfil y digest se reescriben a tokens cortos que redirigen a la página original al hacer clic. El destino está vinculado a su inquilino: la redirección solo se envía a URLs cuyo host coincida con uno de sus dominios configurados, y los tokens expiran automáticamente después de 30 días.

La experiencia al hacer clic no cambia. Los lectores siguen llegando a su página con el comentario desplazado a la vista.

---