---
## Parametri

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | No |  |
| externalId | string | No |  |
| eventType | string | No |  |
| type | string | No |  |
| domain | string | No |  |
| attemptCountGT | number | No |  |

## Odgovor

Vrne: [`GetPendingWebhookEventCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEventCountResponse.ts)

## Primer

[inline-code-attrs-start title = 'getPendingWebhookEventCount Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = "tenant_42";
  const commentId: string = "comment_1001";
  const eventType: string = "comment.deleted";
  const domain: string = "myblog.com";

  const result: GetPendingWebhookEventCountResponse = await getPendingWebhookEventCount(
    tenantId,
    commentId,
    undefined,
    eventType,
    undefined,
    domain
  );

  console.log(result);
}

runExample();
[inline-code-end]

---