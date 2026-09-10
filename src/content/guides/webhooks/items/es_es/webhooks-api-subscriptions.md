Webhooks también pueden gestionarse a través de la API REST. Así es como integraciones como Zapier se suscriben a eventos de comentarios sin tocar el panel de control, y sigue el patrón REST Hooks: suscribirse, recibir eventos, darse de baja.

Las suscripciones API conviven con los webhooks configurados en el panel de control. Un evento de comentario se entrega a cada webhook que coincida con su dominio, cada uno como una entrega independiente, sin importar cómo se haya creado el webhook.

## Autenticación

Cada solicitud necesita su clave API en el encabezado `x-api-key` (o el parámetro de consulta `API_KEY`) y su ID de inquilino en el parámetro de consulta `tenantId`. Ambos se muestran en la página de Secretos de API en el panel de control.

## Suscribirse

```
POST https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
x-api-key: YOUR_API_KEY
Content-Type: application/json

{
    "url": "https://hooks.zapier.com/hooks/catch/123/abc",
    "event": "comment-created"
}
```

| Campo | Requerido | Descripción |
|-------|-----------|-------------|
| `url` | Sí | Una URL http o https absoluta. |
| `event` | Sí | `comment-created`, `comment-updated` o `comment-deleted`. |
| `domain` | No | Un dominio de la configuración de su cuenta. Por defecto `*`, que recibe eventos para todos los dominios. |
| `method` | No | `POST` (default), `PUT` o `DELETE`. |

La respuesta contiene la suscripción:

```json
{
    "status": "success",
    "webhook": {
        "id": "66f1c4c1e7a2b3d4f5a6b7c8",
        "url": "https://hooks.zapier.com/hooks/catch/123/abc",
        "event": "comment-created",
        "domain": "*",
        "method": "POST",
        "source": "api",
        "enabled": true,
        "createdAt": "2026-09-08T12:00:00.000Z"
    }
}
```

Suscribiendo la misma URL al mismo evento y dominio nuevamente devuelve la suscripción existente en lugar de crear un duplicado, por lo que un cliente puede reintentar de forma segura. Cada inquilino puede tener hasta 50 suscripciones API.

## Listado

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

Devuelve todos los webhooks del inquilino, incluidos los gestionados en el panel de control (`\"source\": \"dashboard\"`). Filtre con `event`, `domain` o `source`.

## Cancelar suscripción

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

Eliminar una suscripción también descarta cualquier evento que aún esté en cola para ella. Solo las suscripciones creadas a través de la API pueden eliminarse de esta manera. Los webhooks del panel de control se editan en la página de Webhooks.

## Cargas útiles y firma

Las entregas utilizan la misma carga útil que los webhooks del panel de control (ver Estructuras de datos) y están firmadas con el mismo esquema HMAC (ver Seguridad y Tokens API). Las suscripciones API nunca reciben el encabezado `token` heredado, por lo que debe verificar el encabezado `X-FastComments-Signature` en su lugar.

## Cargas útiles de ejemplo

```
GET https://fastcomments.com/api/v1/webhooks/sample-payloads?tenantId=YOUR_TENANT_ID&event=comment-created&limit=3
```

Devuelve los comentarios más recientes de la cuenta con exactamente la forma que lleva una entrega, de modo que una integración pueda mostrar datos de ejemplo reales antes de que llegue el primer evento. `event` es opcional y solo se valida, ya que cada evento entrega el mismo objeto de comentario. `limit` por defecto es 3 y acepta de 1 a 10. Cuesta 2 créditos API.

```json
{
    "status": "success",
    "payloads": [
        {
            "id": "66f1c4c1e7a2b3d4f5a6b7c8",
            "urlId": "https://example.com/blog/hello-world",
            "commenterName": "Jane Reader",
            "comment": "Great article!",
            "date": "2026-09-08T12:00:00.000Z",
            "approved": true
        }
    ]
}
```

## Responder con 410 Gone

Si el endpoint de una suscripción API responde con HTTP `410 Gone`, FastComments lo trata como una cancelación de suscripción: la suscripción se elimina junto con sus eventos en cola, y no se intentan más entregas. Los webhooks configurados en el panel de control nunca se eliminan automáticamente; para ellos un 410 es un error ordinario. Cualquier otro estado de error se reintenta y eventualmente desactiva el webhook, como se describe en Cómo funciona y Manejo de reintentos.

## Panel de control

Las suscripciones API aparecen en la lista de Webhooks con la fuente **API**, donde un administrador puede editarlas, desactivarlas, volver a activarlas o eliminarlas.

---