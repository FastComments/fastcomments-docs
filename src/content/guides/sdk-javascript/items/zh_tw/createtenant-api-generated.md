## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | 是 |  |
| createTenantBody | CreateTenantBody | 是 |  |

## 回應

返回: [`CreateTenantResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantResponse1.ts)

## 範例

[inline-code-attrs-start title = 'createTenant 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function example() {
  const tenantId: string = 'tenant-2024-01';
  const createTenantBody: CreateTenantBody = {
    // 必填欄位
    name: 'Acme International',
    // 可根據需要新增選用欄位，例如：
    // billingInfo: { address: '123 Main St', city: 'Metropolis' } as BillingInfo,
  };
  const response: CreateTenantResponse1 = await createTenant(tenantId, createTenantBody);
  console.log(response);
}
[inline-code-end]

---