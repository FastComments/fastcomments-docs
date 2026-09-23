Iscrive un URL a un evento di commento (sottoscrizione REST hook). Sottoscrivendo lo stesso URL allo stesso
evento e dominio di nuovo restituisce la sottoscrizione esistente. Le consegne sono firmate con HMAC, vedere la
guida ai webhook; l'intestazione legacy `token` non viene mai inviata alle sottoscrizioni API.

## Parameters

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| createWebhookParams | CreateWebhookParams | Sì |  |

## Response

Restituisce: [`CreateWebhookResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateWebhookResponse.ts)

## Example

[inline-code-attrs-start title = 'Esempio createWebhook'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c6d";

const webhookParams: CreateWebhookParams = {
  event: "comment.created" as WebhookEventName,
  method: "POST" as WebhookHTTPMethod,
  url: "https://myapp.example.com/webhook",
  source: "comment" as WebhookSource,
  // segreto opzionale per la verifica del payload
  secret: "s3cr3tK3y123",
};

const result: CreateWebhookResponse = await createWebhook(tenantId, webhookParams);

const webhookId: string = result.webhook.id;
[inline-code-end]