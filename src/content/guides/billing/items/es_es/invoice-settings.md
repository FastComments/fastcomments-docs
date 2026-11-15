La página [Invoice Settings](https://fastcomments.com/auth/my-account/invoice-settings), a la que se accede con el botón `Invoice Settings` en Billing Info, controla lo que aparece en tus facturas y a dónde se envían. Solo el `Account Owner` o un `Billing Admin` pueden abrirla.

- `Company Name`: el nombre de tu cuenta tal como se muestra en el dashboard, en los correos electrónicos y en las invitaciones del equipo. Este no es el nombre de facturación.
- `Billing Email`: a dónde se envían las facturas y las notificaciones de facturación. Configúralo con un buzón de finanzas o contabilidad si el propietario de la cuenta no debe recibirlas. No cambia el correo de inicio de sesión de ningún usuario.
- `Invoice Currency`: la moneda en la que se emiten tus facturas. Consulta [Currencies](#currencies).
- `Billing Address`: el `Name`, `Address`, `City`, `State / Province / Region`, `Postal Code` y `Country` que se imprimen en las facturas. Los mismos campos aparecen en el formulario de Billing Info cuando guardas una tarjeta; ambos escriben en el mismo lugar.
- `VAT / Tax ID`: opcional. Se imprime en tus facturas.

Haz clic en `Save Changes`. La página lo confirma con `Invoice settings updated successfully.` y los cambios se aplican a la siguiente factura. Las facturas existentes no se modifican.

### Números de IVA e identificadores fiscales

Selecciona tu `Country` de facturación antes de introducir el identificador, porque el país determina qué tipo de identificador es: un número de IVA de la UE para cualquier estado miembro de la UE, un número de IVA del Reino Unido para el Reino Unido, un ABN para Australia, un número de GST/HST para Canadá, un EIN para los Estados Unidos, y así sucesivamente para los demás países que Stripe admite.

El identificador se valida al guardar. Si no es válido para el país seleccionado, la página te lo indica y el identificador no se guarda, mientras que el resto de tus ajustes sí. Si tu país no es uno de aquellos para los que podemos registrar un identificador fiscal, la página también te lo dirá.

Si tienes una dirección de facturación en la UE, las facturas de Flex se emiten con un 0% de IVA como transacción de inversión del sujeto pasivo (reverse charge), y tu número de IVA aparece en ellas una vez guardado, de modo que tu contable pueda gestionar el IVA por tu parte.