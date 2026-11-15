En Flex, las personas que pueden administrar o moderar tu cuenta se facturan como asientos mensuales. Esta es la parte de una factura de Flex que más suele sorprender, generalmente porque los asientos se crearon sin que nadie se diera cuenta.

### Qué Cuenta Como Asiento

- **Administradores**: usuarios en la página `Users` de tu cuenta con cualquier permiso administrativo (facturación, personalización, analíticas, moderación, etc.). El primer administrador, normalmente el propietario de la cuenta, es gratuito; cada administrador adicional es un asiento.
- **Moderadores**: personas en la página `Moderators` que han aceptado su invitación. Una invitación pendiente no se factura. Un usuario cuyo único permiso es la moderación de comentarios se factura como moderador, no como administrador.
- **Administradores SSO** y **Moderadores SSO**: usuarios SSO con los que tu sitio inicia sesión con `isAccountOwner`, `isAdminAdmin`, o `isCommentModeratorAdmin` establecidos en el payload SSO. Se facturan a las mismas tarifas que los administradores y moderadores normales. Consulta [SSO User Structure](/guide-api.html#sso-user-structure).

Los asientos se deduplican por dirección de correo electrónico, por lo que una persona que sea tanto administrador como moderador, o que tenga tanto un usuario SSO como un inicio de sesión en el panel con el mismo correo electrónico, se factura una vez, a la tarifa más alta. Alguien con dos direcciones de correo electrónico diferentes son dos asientos.

En cuentas facturadas por usuarios activos mensuales, un moderador SSO que no inició sesión durante el mes no se factura ese mes.

### Asientos Que No Añadiste

Dos funciones crean asientos automáticamente:

- **Sincronización de administradores SSO.** Cuando un usuario SSO inicia sesión con un correo electrónico que coincide con uno de los usuarios de tu panel, los permisos de administrador o moderador de ese usuario se copian al usuario SSO. Con WordPress SSO esto significa que cada administrador de WordPress puede convertirse en administrador o moderador de FastComments. Desactívalo en `Customize` con la opción `Disable Automatic Admin Migration` en tu regla de personalización.
- **Correos electrónicos diferentes para la misma persona.** Si tu inicio de sesión en el panel usa un correo electrónico y tu usuario SSO otro, son dos asientos. Usar el mismo correo electrónico en ambos los fusiona.

### Verificar Y Eliminar Asientos

La página [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) muestra tus recuentos de `Moderators`, `SSO Admins`, y `SSO Moderators`, y la vista previa de la próxima factura muestra cuánto cuestan. Elimina a las personas que no necesites de las páginas `Users` y `Moderators`, o elimina las marcas de administrador de tu payload SSO. Los asientos se cuentan cuando se crea la factura al final del mes, por lo que un asiento eliminado antes de entonces no se factura ese mes.

Consulta [Adding Moderators](/guide-users.html#adding-moderators) y [Updating Administrator Permissions](/guide-users.html#updating-administrator-permissions) para saber cómo gestionar ambos.