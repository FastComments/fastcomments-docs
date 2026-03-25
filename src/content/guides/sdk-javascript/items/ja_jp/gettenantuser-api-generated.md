---
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |

## レスポンス

戻り値: [`GetTenantUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUser200Response.ts)

## 例

[inline-code-attrs-start title = 'getTenantUser の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_fc5a9b2c';
const userId: string = 'user_0a12b3';
const result: GetTenantUser200Response = await getTenantUser(tenantId, userId);
const user: User | undefined = (result as any).user; // ペイロードにアクセス
const userEmail: string | undefined = user?.email;
console.log('Fetched user email:', userEmail);
[inline-code-end]

---