## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 否 |  |
| externalId | string | 否 |  |
| eventType | string | 否 |  |
| type | string | 否 |  |
| domain | string | 否 |  |
| attemptCountGT | number | 否 |  |
| skip | number | 否 |  |

## 回傳

回傳: [`GetPendingWebhookEvents200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEvents200Response.ts)

## 範例

[inline-code-attrs-start title = 'getPendingWebhookEvents 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f3b2a';
const commentId: string = 'cmt_8a7d1';
const eventType: string = 'comment.created';
const domain: string = 'reviews.myshop.com';
const attemptCountGT: number = 1;
const skip: number = 0;

const result: GetPendingWebhookEvents200Response = await getPendingWebhookEvents(
  tenantId,
  commentId,
  undefined,
  eventType,
  undefined,
  domain,
  attemptCountGT,
  skip
);
[inline-code-end]

---