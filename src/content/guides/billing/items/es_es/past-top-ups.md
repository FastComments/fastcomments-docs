La tabla `Past top-ups` en la parte inferior de la página de Credits muestra tus recargas más recientes, las más nuevas primero, con la `Date`, el `Amount` y el `Status` de cada una. Antes de tu primera recarga muestra `No top-ups yet.`

Cada recarga tiene uno de tres estados:

- `Credited`: el pago se realizó correctamente y el importe se añadió a tu saldo.
- `Pending`: el pago se inició pero no se ha completado, normalmente porque la confirmación bancaria no se finalizó. Las recargas pendientes se verifican con Stripe la próxima vez que se abra la página de Credits y pasan a `Credited` o `Failed`.
- `Failed`: la tarjeta fue rechazada o el banco no aprobó el pago. No se cobró nada y tu saldo no cambió.

La tabla registra las recargas realizadas desde esta página. Las facturas pagadas con crédito se muestran junto con las propias facturas, en `View Invoices` en la página de [Billing Info](https://fastcomments.com/auth/my-account/billing-info).