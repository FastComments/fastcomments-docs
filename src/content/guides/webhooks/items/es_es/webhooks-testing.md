Las nuevas páginas de webhook y de edición tienen un botón `Send Test Payload` que envía una solicitud a la URL que está actualmente en el formulario, haya sido guardada o no. Los eventos **Create** y **Update** envían un objeto **WebhookComment** de prueba, mientras que al probar **Delete** se enviará un cuerpo de solicitud de prueba con solo un ID.

## Verificando Cargas Útiles

Al probar su integración de webhook, verifique que las solicitudes entrantes incluyan los siguientes encabezados:

1. **`X-FastComments-Timestamp`** - Marca de tiempo Unix (segundos)  
2. **`X-FastComments-Signature`** - Firma HMAC‑SHA256  

Los webhooks creados antes de que se introdujera el esquema de firma también reciben un encabezado **`token`** que contiene su **API Secret**. Los webhooks nuevos no lo hacen.

Utilice la verificación de firma HMAC para garantizar que las cargas sean auténticas.

## Herramientas de Prueba

Puede usar herramientas como [webhook.site](https://webhook.site) o [ngrok](https://ngrok.com) para inspeccionar las cargas de webhook entrantes durante el desarrollo.

## Tipos de Eventos

- **Create Event**: Se dispara cuando se crea un nuevo comentario.  
- **Update Event**: Se dispara cuando se edita un comentario.  
- **Delete Event**: Se dispara cuando se elimina un comentario.  

Cada webhook está asociado a un evento y a un método HTTP (POST, PUT o DELETE). Cada evento incluye los datos completos del comentario en el cuerpo de la solicitud (vea [Data Structures](/guide-webhooks.html#webhooks-structures) para el formato de la carga).

---