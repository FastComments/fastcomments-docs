## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| userId | string | いいえ |  |
| anonUserId | string | いいえ |  |

## レスポンス

戻り値: [`FlagComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagComment200Response.ts)

## 例

[inline-code-attrs-start title = 'flagComment の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_4f21c9a";
const commentId: string = "cmt_7a12b3e9";
const userId: string = "user_82bd123";
const result: FlagComment200Response = await flagComment(tenantId, commentId, userId);
[inline-code-end]

---