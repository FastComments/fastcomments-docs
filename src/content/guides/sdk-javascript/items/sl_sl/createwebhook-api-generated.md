Naroči URL na dogodek komentarja (REST hook subscribe). Ponovno naročanje istega URL-ja na isti dogodek in domeno vrne obstoječo naročnino. Dostave so podpisane z HMAC, glejte vodnik za spletne kuke; starejša glava `token` se nikoli ne pošlje naročninam API.

## Parameters

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| createWebhookParams | CreateWebhookParams | Da |  |

## Response

Vrne: [`CreateWebhookResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateWebhookResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer createWebhook'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c6d";

const webhookParams: CreateWebhookParams = {
  event: "comment.created" as WebhookEventName,
  method: "POST" as WebhookHTTPMethod,
  url: "https://myapp.example.com/webhook",
  source: "comment" as WebhookSource,
  // neobvezno skrivnost za preverjanje vsebine
  secret: "s3cr3tK3y123",
};

const result: CreateWebhookResponse = await createWebhook(tenantId, webhookParams);

const webhookId: string = result.webhook.id;
[inline-code-end]