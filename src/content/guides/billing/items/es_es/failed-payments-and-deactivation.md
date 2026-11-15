Un pago puede fallar porque la tarjeta expiró, fue reemplazada, no tiene fondos o el banco bloqueó el cargo. FastComments te da tiempo para corregirlo antes de que algo deje de funcionar, y avisa a las personas que pueden solucionarlo.

### Qué Ocurre

1. **El cargo falla.** La factura permanece `Unpaid` en [Billing History](https://fastcomments.com/auth/my-account/billing-history) con un botón `Pay Now`. En un plan fijo, Stripe puede reintentar el cargo. En Flex, FastComments reintenta cada noche cualquier factura impaga con más antigüedad que el período de gracia.
2. **Se envía una advertencia.** Cada noche FastComments revisa cada cuenta. Si una factura Flex sigue impaga después del período de gracia y el reintento falla, o una suscripción de plan fijo ha terminado porque no se pudo cobrar, cada usuario con el permiso `Billing Admin` recibe el correo electrónico `Your FastComments Tenant Will Be Deactivated Soon!` con la fecha, y la cuenta se programa para desactivación un período de gracia después. Mientras tanto, el widget sigue funcionando.
3. **Desactivación.** Si nada ha cambiado para la fecha programada, la cuenta se desactiva: los comentarios se ocultan en tu sitio y los visitantes ven `This FastComments account's billing information is more than sixty days out of date. Comments hidden until the issue is resolved.` Se envía otro correo a los administradores de facturación. Tus datos se conservan.

El período de gracia es de una semana para la mayoría de las cuentas y de dos semanas para las cuentas con más de un año. En Flex se aplica dos veces, una antes de la advertencia y otra entre la advertencia y la desactivación, por lo que hay al menos dos semanas entre un cargo fallido y la ocultación de los comentarios.

### Cómo Solucionarlo

Guarda una tarjeta válida en la página [Billing Info](https://fastcomments.com/auth/my-account/billing-info). Eso solo reactiva la cuenta de inmediato y cancela cualquier desactivación programada; la factura impaga se cobra con la nueva tarjeta, inmediatamente en un plan fijo y durante la verificación nocturna en Flex. También puedes pagar la factura de inmediato con `Pay Now` en Billing History.

Si la tarjeta registrada está bien y el fallo fue temporal, no tienes que hacer nada: la verificación nocturna detecta la factura pagada o la suscripción activa, cancela la desactivación y envía un correo a los administradores de facturación para confirmarlo. Una cuenta ya desactivada se reactiva de la misma manera.

### Varias Facturas a la Vez

Si tu tarjeta fue inválida durante unos meses y luego se corrigió, cada mes impago se cobra por separado, por lo que puedes ver varios cargos el mismo día. Revisa el período de facturación de cada factura en Billing History: corresponden a meses diferentes, no son duplicados.

### ¿Sigues Viendo una Advertencia de Facturación Después de Pagar?

La advertencia pertenece a una cuenta específica. Si pagaste pero un sitio sigue mostrando `billing information is out of date`, lo más probable es que el widget de ese sitio esté usando una cuenta diferente a la que pagaste. Esto sucede cuando un sitio se configuró dos veces, o cuando hay una cuenta en `fastcomments.com` y otra en el despliegue de la UE en `eu.fastcomments.com`, que se facturan por separado. Compara el `tenantId` en el código de tu página con el que aparece en `Settings -> API/SSO`, o [contacta al soporte](https://fastcomments.com/auth/my-account/help) y lo resolveremos.