---
Našteje spletne kuke, konfigurirane za najemnika, tako vrstice, ki jih upravlja nadzorna plošča, kot tudi naročnine na API.

## Parameters

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| event | WebhookEventName | Ne |  |
| domain | string | Ne |  |
| source | WebhookSource | Ne |  |
| skip | number | Ne |  |

## Response

Vrne: [`GetWebhooksResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetWebhooksResponse.ts)

## Example

[inline-code-attrs-start title = 'Primer getWebhooks'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---