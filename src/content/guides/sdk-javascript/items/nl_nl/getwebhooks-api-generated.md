Lijst de webhooks die zijn geconfigureerd voor de tenant, zowel dashboard‑beheerde rijen als API‑abonnementen.

## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| event | WebhookEventName | Nee |  |
| domain | string | Nee |  |
| source | WebhookSource | Nee |  |
| skip | number | Nee |  |

## Respons

Retourneert: [`GetWebhooksResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetWebhooksResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getWebhooks Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_42';
  const event: WebhookEventName = 'comment.created';
  const domain: string = 'forum.example.org';
  const source: WebhookSource = 'admin';
  const skip: number = 0;

  const fullResponse: GetWebhooksResponse = await getWebhooks(tenantId, event, domain, source, skip);
  const minimalResponse: GetWebhooksResponse = await getWebhooks(tenantId);
})();
[inline-code-end]