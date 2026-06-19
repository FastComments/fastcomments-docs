---
## パラメータ

| 名前 | Type | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| deleteComments | string | いいえ |  |
| commentDeleteMode | string | いいえ |  |

## レスポンス

戻り値: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 例

[inline-code-attrs-start title = 'deleteTenantUser の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_4b7a9f2c";
const id: string = "user_9d3f1b6a";
const deleteComments: string = "true";
const commentDeleteMode: string = "permanent";
const result: APIEmptyResponse = await deleteTenantUser(tenantId, id, deleteComments, commentDeleteMode);
[inline-code-end]

---