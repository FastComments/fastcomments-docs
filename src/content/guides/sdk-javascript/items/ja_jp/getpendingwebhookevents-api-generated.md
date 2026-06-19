## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| commentId | string | いいえ |  |
| externalId | string | いいえ |  |
| eventType | string | いいえ |  |
| type | string | いいえ |  |
| domain | string | いいえ |  |
| attemptCountGT | number | いいえ |  |
| skip | number | いいえ |  |

## レスポンス

戻り値: [`GetPendingWebhookEventsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEventsResponse.ts)

## 例

[inline-code-attrs-start title = 'getPendingWebhookEvents の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_a1b2c3';
const commentId: string = 'cmt_9f8e7d';
const eventType: string = 'comment.created';
const domain: string = 'comments.acme-corp.com';
const attemptCountGT: number = 2;
const skip: number = 5;

const result: GetPendingWebhookEventsResponse = await getPendingWebhookEvents(
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