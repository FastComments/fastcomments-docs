No hay una pantalla separada de "edición de tarjeta". Para reemplazar la tarjeta registrada, abra [Billing Info](https://fastcomments.com/auth/my-account/billing-info), deje su plan como está, introduzca los datos de la nueva tarjeta en `Payment method` y haga clic en `Save Info`.

La nueva tarjeta se convierte en la predeterminada para todas las facturas futuras. No se realiza ningún cargo al guardarla, a menos que tenga una factura pendiente de pago, en cuyo caso esa factura se cobra con la nueva tarjeta: de inmediato en un plan fijo, y durante la noche en Flex. Mientras su facturación esté al día, el formulario indica `You won't be charged now unless you change plans.` Si su suscripción de plan fijo ha finalizado, guardar una tarjeta inicia una nueva y se cobra el primer período, como en el registro inicial.

Si su banco requiere que confirme la tarjeta, se abre una ventana de confirmación después de hacer clic en guardar. La tarjeta solo se almacena una vez que el banco la aprueba.

### Quién Puede Hacer Esto

Solo el `Account Owner` o un usuario con el permiso `Billing Admin` puede guardar una tarjeta o cambiar el plan. Los demás usuarios ven `Only the account owner or a billing admin can change the plan or payment method.` Si el menú `Settings` está en gris o la página de facturación le redirige a la página de inicio, ha iniciado sesión como comentarista o como un usuario sin acceso a la facturación, en lugar de como el propietario de la cuenta. Inicie sesión con la cuenta que creó el sitio, o pida al propietario que le otorgue el permiso; consulte [Who Can Manage Billing](#who-can-manage-billing).

### Tarjetas Rechazadas O Bloqueadas

Si la tarjeta es rechazada, la página muestra `Your card was declined. Check that the entered information is correct.` y no se guarda nada. Verifique el número, la fecha de caducidad, el CVC y la dirección de facturación, y vuelva a intentarlo. Si el banco rechazó una confirmación, verá `Your bank did not approve the payment. Try again, or use a different card.`

Algunos bancos bloquean los pagos en línea recurrentes, o bloquean una tarjeta después de un intento rechazado. En ese caso, pruebe con otra tarjeta, o pague una factura abierta con `Pay Now` en [Billing History](https://fastcomments.com/auth/my-account/billing-history). Si su banco no permite cargos automáticos en absoluto, [contacte con soporte](https://fastcomments.com/auth/my-account/help) y podemos enviar un enlace de pago para la factura en su lugar.

Solo se aceptan tarjetas. La lista `Country` del formulario contiene los países que Stripe puede facturar; si el suyo no aparece, contacte con soporte.