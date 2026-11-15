Cada cargo, ya sea una renovación de plan, un mes Flex o un cambio de plan, genera una factura. Las facturas también sirven como recibos una vez pagadas.

### Cómo encontrar las facturas

Abra [Historial de Facturación](https://fastcomments.com/auth/my-account/billing-history), o haga clic en `View Invoices` en la página de Información de Facturación. La tabla muestra cada factura con su fecha, período de facturación, número de factura, descripción, total, estado (`Paid`, `Unpaid`, `Draft` o `Void`) y fecha de vencimiento, diez por página, las más recientes primero.

Para cada factura:

- `View` abre la factura en la página alojada de Stripe, donde también puede descargarla.
- `PDF` descarga el PDF directamente.
- `Pay Now` aparece en lugar de `View` en una factura no pagada y le lleva a la página de pago de Stripe para esa factura, donde puede pagarla directamente.

Las facturas también se envían por correo electrónico a su correo de facturación cuando se crean. Para que se envíen a un buzón de finanzas en lugar del propietario de la cuenta, configure el `Billing Email` en [Configuración de Facturas](#invoice-settings).

### Qué contiene la factura

La factura muestra el nombre y la dirección de facturación y el VAT / Tax ID de [Configuración de Facturas](#invoice-settings), los conceptos de línea, cualquier [crédito](#credits) prepagado que se haya aplicado y el importe cargado a su tarjeta. Las facturas Flex enumeran una línea por tipo de uso, consulte [Cómo se mide el uso de Flex](#how-flex-usage-is-measured). Para direcciones de facturación en la Unión Europea, las facturas Flex se emiten con un 0% de IVA y con el pie de página `Reverse Charge Transaction: Customer to pay VAT`.

Los cambios en la configuración de facturas se aplican a las facturas creadas después de guardarlos. Una factura que ya se ha emitido no puede editarse, así que configure los datos de su empresa y el VAT / Tax ID antes de la próxima fecha de facturación.

### Cuentas facturadas en otro lugar

Si su facturación se gestiona externamente, por ejemplo mediante una factura acordada con soporte, las facturas se le envían para su pago en lugar de cargarse a una tarjeta y es posible que no aparezcan en el Historial de Facturación. Las cuentas instaladas a través de Shopify o BigCommerce son facturadas por esa plataforma, y la página de Información de Facturación enlaza con la página del plan allí. Las subcuentas facturadas a través de una cuenta principal no tienen facturas propias; consulte [Cuándo se le cobra](#when-you-are-charged).