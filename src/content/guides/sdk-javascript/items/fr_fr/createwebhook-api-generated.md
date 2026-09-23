Abonne une URL à un événement de commentaire (abonnement REST hook). S'abonner la même URL au même  
événement et domaine à nouveau renvoie l'abonnement existant. Les livraisons sont signées HMAC, voir le  
guide des webhooks ; l'en‑tête `token` hérité n'est jamais envoyé aux abonnements API.

## Parameters

| Nom | Type | Obligatoire | Description |
|------|------|------------|-------------|
| tenantId | string | Oui |  |
| createWebhookParams | CreateWebhookParams | Oui |  |

## Response

Returns: [`CreateWebhookResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateWebhookResponse.ts)

## Example

[inline-code-attrs-start title = 'Exemple createWebhook'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c6d";

const webhookParams: CreateWebhookParams = {
  event: "comment.created" as WebhookEventName,
  method: "POST" as WebhookHTTPMethod,
  url: "https://myapp.example.com/webhook",
  source: "comment" as WebhookSource,
  // secret optionnel pour la vérification de la charge utile
  secret: "s3cr3tK3y123",
};

const result: CreateWebhookResponse = await createWebhook(tenantId, webhookParams);

const webhookId: string = result.webhook.id;
[inline-code-end]