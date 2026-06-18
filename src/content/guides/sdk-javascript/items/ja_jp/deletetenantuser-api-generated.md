## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| deleteComments | string | いいえ |  |
| commentDeleteMode | string | いいえ |  |

## レスポンス

返却値: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 例

[inline-code-attrs-start title = 'deleteTenantUser の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_8f3a2b1c4d";
const id: string = "user_62a4f9e0b7";
const deleteComments: string = "true";
const commentDeleteMode: string = "permanent";
const result: FlagCommentPublic200Response = await deleteTenantUser(tenantId, id, deleteComments, commentDeleteMode);
[inline-code-end]

---