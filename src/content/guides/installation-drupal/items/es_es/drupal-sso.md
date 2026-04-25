FastComments se integra con el sistema de usuarios de Drupal mediante SSO, o inicio de sesión único. Tus usuarios inician sesión en tu sitio Drupal, y el módulo pasa su identidad a FastComments automáticamente. No hay cuentas adicionales que crear, ni sincronización inicial que ejecutar.

El módulo admite tres modos de SSO, configurables en `Administration > Configuration > Content > FastComments`.

### Ninguno

Sin SSO. Los usuarios comentan como invitados o crean una cuenta de FastComments. Úsalo si tu sitio es público y no necesitas vincular los comentarios a los usuarios de Drupal.

### Simple

Pasa el nombre, correo electrónico y avatar del usuario de Drupal a FastComments sin verificación del lado del servidor. No se necesita API Secret. Adecuado para sitios internos o de bajo riesgo.

### Seguro (recomendado)

Utiliza [HMAC-SHA256](https://en.wikipedia.org/wiki/HMAC) para verificar cada identidad de usuario con FastComments. Este es el modo que debes usar cuando tienes un API Secret configurado, y es el único modo que evita que un visitante suplante a otro usuario.

La identidad del usuario se envía a FastComments cada vez que un usuario visualiza un hilo de comentarios. No hay ninguna sincronización inicial ni continua que deba ejecutarse.

<sup>(Opcional)</sup> Añade a tus administradores en [Usuarios y Administradores](https://fastcomments.com/auth/my-account/users) y a los moderadores en [Moderadores de comentarios](https://fastcomments.com/auth/my-account/moderate-comments/moderators) para mejorar su experiencia y habilitar el seguimiento de estadísticas para moderadores.

Para entender más a fondo cómo funciona SSO, consulta la [sección SSO](/guide-customizations-and-configuration.html#sso) de la documentación de personalización.

---