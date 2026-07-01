## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | No |  |
| externalId | string | No |  |
| eventType | string | No |  |
| type | string | No |  |
| domain | string | No |  |
| attemptCountGT | number | No |  |

## Svar

Returnerer: [`GetPendingWebhookEventCountResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEventCountResponse1.ts)

## Eksempel

[inline-code-attrs-start title = 'getPendingWebhookEventCount Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---