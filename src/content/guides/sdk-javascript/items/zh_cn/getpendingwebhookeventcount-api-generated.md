## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 否 |  |
| externalId | string | 否 |  |
| eventType | string | 否 |  |
| type | string | 否 |  |
| domain | string | 否 |  |
| attemptCountGT | number | 否 |  |

## 响应

返回: [`GetPendingWebhookEventCount200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEventCount200Response.ts)

## 示例

[inline-code-attrs-start title = 'getPendingWebhookEventCount 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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