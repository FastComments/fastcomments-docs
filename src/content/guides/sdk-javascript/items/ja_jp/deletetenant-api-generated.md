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
const tenantId: string = "tenant_7f3d2c";
const id: string = "flag_8392b1a7";
const sure: string = "confirmed";

const responseWithoutSure: FlagCommentPublic200Response = await deleteTenant(tenantId, id);
const responseWithSure: FlagCommentPublic200Response = await deleteTenant(tenantId, id, sure);
[inline-code-end]

---