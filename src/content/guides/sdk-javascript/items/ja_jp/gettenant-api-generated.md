## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |

## レスポンス

返却: [`GetTenantResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantResponse1.ts)

## 例

[inline-code-attrs-start title = 'getTenant の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchTenant(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const id: string = "user_9876";
  const tenantInfo: GetTenantResponse1 = await getTenant(tenantId, id);
  console.log(tenantInfo);
}

fetchTenant();
[inline-code-end]