## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 回應

回傳: [`GetTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantResponse.ts)

## 範例

[inline-code-attrs-start title = 'getTenant 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp';
const id: string = 'tenant-987654321';
const options: { includeBilling?: boolean } = { includeBilling: true };
const response: GetTenantResponse = await getTenant(tenantId, id);
const billingInfo: BillingInfo | undefined = undefined
[inline-code-end]

---