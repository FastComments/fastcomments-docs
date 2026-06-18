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

戻り値: [`GetPendingWebhookEvents200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEvents200Response.ts)

## 例

[inline-code-attrs-start title = 'getPendingWebhookEvents の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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