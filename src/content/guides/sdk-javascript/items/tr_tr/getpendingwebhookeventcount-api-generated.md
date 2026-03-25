## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| commentId | string | Hayır |  |
| externalId | string | Hayır |  |
| eventType | string | Hayır |  |
| type | string | Hayır |  |
| domain | string | Hayır |  |
| attemptCountGT | number | Hayır |  |

## Yanıt

Döndürür: [`GetPendingWebhookEventCount200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEventCount200Response.ts)

## Örnek

[inline-code-attrs-start title = 'getPendingWebhookEventCount Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_8d3b7a2f";
const commentId: string | undefined = "comment_79a2b";
const eventType: string | undefined = "comment.created";
const domain: string | undefined = "forum.acme-corp.com";
const attemptCountGT: number | undefined = 1;
const result: GetPendingWebhookEventCount200Response = await getPendingWebhookEventCount(
  tenantId,
  commentId,
  undefined,
  eventType,
  undefined,
  domain,
  attemptCountGT
);
[inline-code-end]

---