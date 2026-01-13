## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## レスポンス

返却値: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 例

[inline-code-attrs-start title = 'deletePendingWebhookEvent の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f4e2b';
const pendingEventId: string = '9f7b6a8c-3b2a-4c0d-a8e5-1234567890ab';
const result: FlagCommentPublic200Response = await deletePendingWebhookEvent(tenantId, pendingEventId);
console.log(result);
[inline-code-end]

---