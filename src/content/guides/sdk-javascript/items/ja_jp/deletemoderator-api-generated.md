## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| sendEmail | string | いいえ |  |

## レスポンス

戻り値: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 例

[inline-code-attrs-start title = 'deleteModerator の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f3b2d4a6c';
const moderatorId: string = 'moderator_84a1b9c2';
const sendEmail: string = 'true';
const result: FlagCommentPublic200Response = await deleteModerator(tenantId, moderatorId, sendEmail);
[inline-code-end]

---