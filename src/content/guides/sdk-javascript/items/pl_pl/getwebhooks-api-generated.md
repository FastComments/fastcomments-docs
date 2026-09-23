Wyświetla webhooki skonfigurowane dla najemcy, zarówno wiersze zarządzane w panelu, jak i subskrypcje API.

## Parameters

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| event | WebhookEventName | Nie |  |
| domain | string | Nie |  |
| source | WebhookSource | Nie |  |
| skip | number | Nie |  |

## Response

Zwraca: [`GetWebhooksResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetWebhooksResponse.ts)

## Example

[inline-code-attrs-start title = 'Przykład getWebhooks'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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