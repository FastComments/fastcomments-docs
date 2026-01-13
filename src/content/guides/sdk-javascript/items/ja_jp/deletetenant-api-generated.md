## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| sure | string | いいえ |  |

## レスポンス

戻り値: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 例

[inline-code-attrs-start title = 'deleteTenant の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7b3f1a9e";
const id: string = "flag_9c4d2b1a";
const sure: string = "yes_confirm_delete";

const result: FlagCommentPublic200Response = await deleteTenant(tenantId, id, sure);
console.log(result);
[inline-code-end]

---