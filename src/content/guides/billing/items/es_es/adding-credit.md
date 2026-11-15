Los recargas de saldo se cargan al método de pago guardado en tu cuenta. Si aún no has guardado una tarjeta, la página de Créditos muestra `Add a payment method first` con un botón `Go to billing`. Guarda una tarjeta en la página [Billing Info](https://fastcomments.com/auth/my-account/billing-info) y luego vuelve.

Para añadir saldo:

1. Abre [My Account -> Billing Info](https://fastcomments.com/auth/my-account/billing-info) y haz clic en `Manage Credits`.
2. En `Add credit`, elige una de las cantidades predefinidas, o selecciona `Other amount` e introduce la tuya.
3. Haz clic en el botón `Add`. Muestra la cantidad que elegiste, por ejemplo `Add $25.00`.
4. Si tu banco te pide confirmar el pago, completa la verificación en la ventana que se abre.

Cuando el pago se procesa, la página muestra `Credit added` con tu nuevo saldo, y la recarga aparece en `Past top-ups` como `Credited`. Haz clic en `Add more` para recargar de nuevo.

### Cantidades y límites

Las cantidades predefinidas son `10`, `25`, `50` y `100` en tu moneda de facturación. Una cantidad personalizada puede ser cualquier valor desde `5` hasta `10,000` por recarga, y los límites exactos para tu moneda se muestran debajo del formulario. Para monedas sin decimales, como el yen japonés, las cantidades predefinidas y los límites son cien veces mayores para representar aproximadamente el mismo importe.

Las recargas siempre se realizan en la moneda de facturación de tu cuenta, la misma que la de tus facturas. La página te indica qué moneda se cargará.

### Confirmación bancaria (3D Secure)

Algunos bancos requieren que confirmes un pago en línea, normalmente a través de tu aplicación bancaria o un código de un solo uso. Si el tuyo lo hace, se abre una ventana de confirmación después de hacer clic en `Add`. Tu tarjeta solo se carga, y tu saldo solo se actualiza, después de que el banco lo apruebe.

Si cierras la página antes de terminar la confirmación, la recarga se registra como `Pending`. La próxima vez que abras la página de Créditos, comprobamos el estado del pago con Stripe y lo marcamos como `Credited` o `Failed` según corresponda. Si el banco no aprueba el pago, la página muestra `Your bank did not approve the payment. Try again, or use a different card.` y no se carga nada.

### Tarjetas rechazadas

Si tu tarjeta es rechazada, la página muestra `Your card was declined. Check your card on the billing page, or try another card.` No se carga nada, tu saldo no cambia y el intento aparece en `Past top-ups` como `Failed`. Actualiza la tarjeta en la página [Billing Info](https://fastcomments.com/auth/my-account/billing-info) e inténtalo de nuevo.

Para proteger tu tarjeta, una cuenta puede realizar un número limitado de intentos de recarga por hora. Si ves `Too many top-up attempts. Wait a while and try again.`, espera y vuelve a intentarlo más tarde.

Los pagos son procesados por Stripe. FastComments nunca almacena los datos de tu tarjeta.