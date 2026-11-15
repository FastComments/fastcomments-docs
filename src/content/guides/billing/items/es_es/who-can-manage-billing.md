La facturación está restringida a dos tipos de usuarios:

- El `Account Owner`, el usuario que creó la cuenta.
- Cualquier usuario con el permiso `Billing Admin`.

Son los únicos usuarios que pueden cambiar el plan, guardar o reemplazar una tarjeta, abrir `Invoice Settings`, establecer límites de gasto Flex y añadir crédito. Cualquier otra persona que lo intente verá `Only the account owner or a billing admin can change the plan or payment method.`

Los correos electrónicos de facturación, como el recordatorio a mitad de la prueba, las advertencias de pago fallido y los avisos de desactivación, se envían a los usuarios con el permiso `Billing Admin`. Otorga el permiso a quien deba recibirlos, por ejemplo alguien de finanzas, y recibirá los correos sin necesidad de ningún otro acceso al panel. Los recordatorios enviados después de que una prueba termina sin información de facturación van a todos los usuarios del panel de la cuenta. Cada uno de estos correos tiene un enlace para darse de baja; un usuario que opte por no recibir notificaciones de la cuenta deja de recibirlos.

Las facturas en sí van al `Billing Email` en [Invoice Settings](#invoice-settings), que no tiene que pertenecer en absoluto a un usuario del panel.

La página de [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) requiere en cambio el permiso `Analytics Admin`, ya que muestra el uso en lugar de los detalles de pago.

Para otorgar o quitar el permiso, abre la página `Users` y edita el usuario. Consulta [Updating Administrator Permissions](/guide-users.html#updating-administrator-permissions). En Flex, recuerda que cada administrador adicional es un asiento facturado, consulta [Admin and Moderator Seats](#admin-and-moderator-seats).

Si has iniciado sesión y no puedes ver `Billing Info` en absoluto, o te devuelve a la página de inicio, has iniciado sesión con una cuenta de comentarista en lugar de un usuario del panel. Cierra sesión e inicia sesión con la cuenta que posee el sitio.