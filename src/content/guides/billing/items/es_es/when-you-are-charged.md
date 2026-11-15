Lo que se cobra a tu tarjeta, y cuándo, depende del tipo de plan que tengas.

### Planes Fijos

Cuando inicias una suscripción `Creators` o `Pro`, el primer mes o año se cobra de inmediato, en el navegador, al hacer clic en `Pay Now & Start Subscription`. Si tu banco te pide confirmar el pago, completa la verificación en la ventana que se abre. La suscripción solo comienza una vez que el pago se haya procesado.

Después de eso, la suscripción se renueva el mismo día de cada mes, o de cada año si elegiste `Pay Annually`, contado desde el día en que te suscribiste. La renovación se cobra automáticamente a la tarjeta registrada y la factura se envía por correo electrónico a tu correo de facturación. No hay ninguna acción que realizar.

La facturación anual es un 15% más barata que la mensual. Flex no puede facturarse anualmente.

### Flex

Flex se factura a posteriori: usas el servicio durante un mes calendario, y la factura de ese mes se crea el último día del mes (UTC) y se cobra a tu tarjeta de inmediato. La fecha de facturación es siempre el último día del mes y no se puede cambiar.

La factura cubre solo ese mes calendario. El uso del mes calendario en el que te registraste nunca se factura, por lo que tu primera factura será, como muy pronto, al final del mes siguiente al registro.

Si tu uso es inferior al mínimo de Flex, una línea que dice `FastComments Flex. (minimum ... /mo)` completa la factura hasta el monto mínimo. El mínimo actual se muestra en tu página de [Billing Info](https://fastcomments.com/auth/my-account/billing-info) y en la [página de precios](https://fastcomments.com/traffic-pricing).

Puedes ver cuál será la próxima factura en cualquier momento. El `Next Invoice Amount` está en la página de [Billing Info](https://fastcomments.com/auth/my-account/billing-info), y el desglose completo línea por línea está en la página de [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing). Lo que se contabiliza se explica en [Cómo se mide el uso de Flex](#how-flex-usage-is-measured).

### Guardar una Tarjeta No Genera Cargos

Una vez configurada tu facturación, guardar o actualizar una tarjeta sin cambiar de plan nunca genera un cargo, a menos que tengas una factura pendiente de pago. El formulario dice `You won't be charged now unless you change plans.` y la tarjeta simplemente se almacena con Stripe para la próxima factura. Solo cuando aún no tienes una facturación válida el formulario dice `Your card will be charged now.`, e incluso entonces una cuenta Flex no se cobra hasta el final del mes. Cualquier [crédito](#credits) prepagado en tu cuenta se aplica a cada factura antes de cobrar la tarjeta.

### Subcuentas Gestionadas

Si tu cuenta gestiona subcuentas (por ejemplo, una por sitio o entorno, o inquilinos con marca blanca), solo se factura la cuenta principal. El uso de cada subcuenta aparece como sus propias líneas en la factura Flex de la cuenta principal, precedidas por el nombre de la subcuenta, y las subcuentas comparten el método de pago y el estado de facturación de la cuenta principal.