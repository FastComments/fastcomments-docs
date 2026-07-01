## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|---------------|
| tenantId | string | Ja |  |
| commentId | string | Nein |  |
| externalId | string | Nein |  |
| eventType | string | Nein |  |
| type | string | Nein |  |
| domain | string | Nein |  |
| attemptCountGT | number | Nein |  |

## Antwort

Rückgabe: [`GetPendingWebhookEventCountResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEventCountResponse1.ts)

## Beispiel

[inline-code-attrs-start title = 'getPendingWebhookEventCount Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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