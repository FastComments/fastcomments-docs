En la administración de Webhooks hay botones `Send Test Payload` para cada tipo de evento (Create, Update, Delete). Los eventos Create y Update envían un objeto WebhookComment de prueba, mientras que al probar Delete se enviará un cuerpo de solicitud de prueba con solo un ID.

## Verificación de Payloads

Al probar la integración de tu webhook, verifica que las solicitudes entrantes incluyan los siguientes encabezados:

1. **`token`** - Tu secreto de la API
2. **`X-FastComments-Timestamp`** - Marca de tiempo Unix (segundos)
3. **`X-FastComments-Signature`** - Firma HMAC-SHA256

Usa la verificación de la firma HMAC para asegurarte de que los payloads sean auténticos.

## Herramientas de prueba

Puedes usar herramientas como [webhook.site](https://webhook.site) o [ngrok](https://ngrok.com) para inspeccionar los payloads entrantes del webhook durante el desarrollo.

## Tipos de eventos

- **Create Event**: Se desencadena cuando se crea un nuevo comentario. Método predeterminado: PUT
- **Update Event**: Se desencadena cuando se edita un comentario. Método predeterminado: PUT
- **Delete Event**: Se desencadena cuando se elimina un comentario. Método predeterminado: DELETE

Cada evento incluye los datos completos del comentario en el cuerpo de la solicitud (consulta [Estructuras de datos](/guides/webhooks/webhooks-structures) para el formato del payload).