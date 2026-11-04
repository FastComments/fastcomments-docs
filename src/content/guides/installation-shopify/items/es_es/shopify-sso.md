El bloque de **FastComments** admite el inicio de sesión único (SSO) para que tus clientes de Shopify puedan comentar como ellos mismos sin necesidad de crear una cuenta de FastComments separada.

### Cómo funciona

Cuando un visitante que ha iniciado sesión en tu tienda abre una página con el bloque de **FastComments**:

1. El bloque detecta el objeto `customer` de Shopify.
2. Envía el nombre y el correo electrónico del cliente a FastComments a través de una solicitud firmada de app proxy.
3. FastComments crea o encuentra un usuario identificado por `shopify-{customerId}`, de modo que el mismo cliente siempre se corresponde con el mismo usuario de FastComments entre sesiones y reinstalaciones.
4. El nombre del visitante aparece en sus comentarios. No se le solicita iniciar sesión de nuevo.

Si el visitante no ha iniciado sesión en la tienda, el bloque recurre a los comentarios anónimos (o al flujo de inicio de sesión de FastComments, dependiendo de la configuración de tu widget).

### Desactivar SSO

SSO está activado por defecto en cada bloque de **FastComments**. Para desactivarlo en un bloque específico:

1. Abre el editor de temas de Shopify.
2. Abre la plantilla que contiene el bloque y haz clic en el bloque para seleccionarlo.
3. Desmarca **SSO**.
4. Haz clic en **Guardar**.

Desactiva SSO si quieres que los comentaristas elijan una identidad separada para la conversación. Por ejemplo, una página de comunidad interna donde el personal comenta con un nombre para mostrar diferente.

### Qué recibe FastComments

La carga de SSO enviada para cada cliente contiene:

- Un ID de usuario derivado del ID del cliente de Shopify (`shopify-{customerId}`).
- El correo electrónico del cliente (usado para identificar al usuario; no se muestra públicamente).
- El nombre para mostrar del cliente (usado como su nombre de autor de comentario).

No se envía ningún historial de pedidos, datos de pago ni de dirección. La carga está firmada en el servidor; el navegador del cliente nunca ve una credencial.

### Enlaces de inicio y cierre de sesión

Cuando SSO está activado, los enlaces de inicio y cierre de sesión del widget de comentarios apuntan a `/account/login` y `/account/logout`, las rutas estándar de cuentas de cliente de Shopify. No hay nada que configurar. Los enlaces funcionan en cualquier tienda con cuentas de cliente habilitadas.