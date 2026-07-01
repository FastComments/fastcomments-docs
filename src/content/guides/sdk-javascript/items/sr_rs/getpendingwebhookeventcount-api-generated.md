## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| commentId | string | Ne |  |
| externalId | string | Ne |  |
| eventType | string | Ne |  |
| type | string | Ne |  |
| domain | string | Ne |  |
| attemptCountGT | number | Ne |  |

## Odgovor

Returns: [`GetPendingWebhookEventCountResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEventCountResponse1.ts)

## Primer

[inline-code-attrs-start title = 'Primer getPendingWebhookEventCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_001";

  const responseAll: GetPendingWebhookEventCountResponse1 = await getPendingWebhookEventCount(
    tenantId,
    "comment_456",
    "ext_789",
    "comment.updated",
    "webhook",
    "mydomain.com",
    3
  );

  const responseMinimal: GetPendingWebhookEventCountResponse1 = await getPendingWebhookEventCount(tenantId);

  console.log(responseAll, responseMinimal);
})();
[inline-code-end]