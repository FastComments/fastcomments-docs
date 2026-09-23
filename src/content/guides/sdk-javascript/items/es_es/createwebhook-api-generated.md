Suscribe una URL a un evento de comentario (suscripción de webhook REST). Suscribir la misma URL al mismo evento y dominio nuevamente devuelve la suscripción existente. Las entregas están firmadas con HMAC, consulte la guía de webhooks; el encabezado `token` heredado nunca se envía a suscripciones de API.

## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|--------|------|-------------|-------------|
| tenantId | string | Sí |  |
| createWebhookParams | CreateWebhookParams | Sí |  |

## Respuesta

Devuelve: [`CreateWebhookResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateWebhookResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo createWebhook'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c6d";

const webhookParams: CreateWebhookParams = {
  event: "comment.created" as WebhookEventName,
  method: "POST" as WebhookHTTPMethod,
  url: "https://myapp.example.com/webhook",
  source: "comment" as WebhookSource,
  // secreto opcional para la verificación de la carga útil
  secret: "s3cr3tK3y123",
};

const result: CreateWebhookResponse = await createWebhook(tenantId, webhookParams);

const webhookId: string = result.webhook.id;
[inline-code-end]