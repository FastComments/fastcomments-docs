## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| commentId | string | Nee |  |
| externalId | string | Nee |  |
| eventType | string | Nee |  |
| type | string | Nee |  |
| domain | string | Nee |  |
| attemptCountGT | number | Nee |  |

## Respons

Retourneert: [`GetPendingWebhookEventCountResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEventCountResponse1.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getPendingWebhookEventCount Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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