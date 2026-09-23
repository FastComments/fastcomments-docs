## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | No |  |
| externalId | string | No |  |
| eventType | string | No |  |
| type | string | No |  |
| domain | string | No |  |
| attemptCountGT | number | No |  |
| skip | number | No |  |

## 响应

返回：[`GetPendingWebhookEventsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEventsResponse.ts)

## 示例

[inline-code-attrs-start title = 'getPendingWebhookEvents 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "123e4567-e89b-12d3-a456-426614174000";
const commentId: string = "cmt_987654321";
const externalId: string = "ext_abc123";
const eventType: string = "comment_created";
const type: string = "outbound";
const domain: string = "myblog.com";
const attemptCountGT: number = 2;
const skip: number = 10;

const pendingEvents: GetPendingWebhookEventsResponse = await getPendingWebhookEvents(
  tenantId,
  commentId,
  externalId,
  eventType,
  type,
  domain,
  attemptCountGT,
  skip
);
[inline-code-end]