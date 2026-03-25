## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |

## レスポンス

戻り値: [`GetUserBadge200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadge200Response.ts)

## 例

[inline-code-attrs-start title = 'getUserBadge の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme_01";
const id: string = "badge_8c7d2f";
const response: GetUserBadge200Response = await getUserBadge(tenantId, id);
[inline-code-end]

---