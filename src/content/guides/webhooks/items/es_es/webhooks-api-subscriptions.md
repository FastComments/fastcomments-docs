Webhooks también pueden gestionarse a través de la API REST. Así es como integraciones como Zapier se suscriben a eventos de comentarios sin tocar el panel, y sigue el patrón REST Hooks: suscribirse, recibir eventos, cancelar la suscripción.

Las suscripciones API conviven con los webhooks configurados en el panel. Un evento de comentario se entrega al webhook del panel para su dominio y a cada suscripción API que coincida, cada una como su propia entrega. No hay límite de un suscriptor por evento.

## Autenticación

Cada solicitud necesita su clave API en el encabezado `x-api-key` (o el parámetro de consulta `API_KEY`) y su ID de inquilino en el parámetro de consulta `tenantId`. Ambos se muestran en la página de Secretos de API en el panel.

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
|-------|----------|-------------|
| `url` | Sí | Una URL http o https absoluta. |
| `event` | Sí | `comment-created`, `comment-updated` o `comment-deleted`. |
| `domain` | No | Un dominio de la configuración de su cuenta. Por defecto `*`, que recibe eventos para todos los dominios. |
| `method` | No | `POST` (por defecto), `PUT` o `DELETE`. |

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

## Listar

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

Devuelve todos los webhooks del inquilino, incluidos los gestionados en el panel (`\"source\": \"dashboard\"`). Filtre con `event`, `domain` o `source`.

## Cancelar suscripción

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

Eliminar una suscripción también descarta cualquier evento que aún esté en cola para ella. Solo las suscripciones creadas a través de la API pueden eliminarse de esta manera. Los webhooks del panel se editan en la página de Webhooks.

## Cargas útiles y firma

Las entregas utilizan la misma carga útil que los webhooks del panel (ver Estructuras de datos) y están firmadas con el mismo esquema HMAC (ver Seguridad y Tokens API). Las suscripciones API nunca reciben el encabezado `token` heredado, por lo que debe verificar el encabezado `X-FastComments-Signature` en su lugar.

## Responder con 410 Gone

Si el endpoint de una suscripción API responde con HTTP `410 Gone`, FastComments lo interpreta como una cancelación de suscripción: la suscripción se elimina junto con sus eventos en cola, y no se intentan más entregas. Los webhooks configurados en el panel nunca se eliminan automáticamente; para ellos un 410 es un fallo ordinario. Cualquier otro estado de error se reintenta y eventualmente desactiva el webhook, como se describe en Cómo funciona y Manejo de reintentos.

## Panel

Las suscripciones API se enumeran en la página de Webhooks bajo el dominio para el que fueron creadas, donde un administrador puede desactivarlas, volver a activarlas o eliminarlas.