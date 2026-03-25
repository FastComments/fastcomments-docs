## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| commentId | string | Не |  |
| externalId | string | Не |  |
| eventType | string | Не |  |
| type | string | Не |  |
| domain | string | Не |  |
| attemptCountGT | number | Не |  |

## Одговор

Враћа: [`GetPendingWebhookEventCount200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEventCount200Response.ts)

## Примјер

[inline-code-attrs-start title = 'getPendingWebhookEventCount Примјер'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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