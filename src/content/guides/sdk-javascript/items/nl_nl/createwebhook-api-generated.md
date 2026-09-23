Abonneert een URL op een commentaar‑event (REST‑hook‑abonnement). Het opnieuw abonneren van dezelfde URL op hetzelfde event en domein retourneert de bestaande abonnement. Leveringen zijn HMAC‑ondertekend, zie de webhooks‑gids; de legacy `token`‑header wordt nooit verzonden naar API‑abonnementen.

## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| createWebhookParams | CreateWebhookParams | Ja |  |

## Respons

Returns: [`CreateWebhookResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateWebhookResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'createWebhook Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c6d";

const webhookParams: CreateWebhookParams = {
  event: "comment.created" as WebhookEventName,
  method: "POST" as WebhookHTTPMethod,
  url: "https://myapp.example.com/webhook",
  source: "comment" as WebhookSource,
  // optionele secret voor payloadverificatie
  secret: "s3cr3tK3y123",
};

const result: CreateWebhookResponse = await createWebhook(tenantId, webhookParams);

const webhookId: string = result.webhook.id;
[inline-code-end]