## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |

## レスポンス

戻り値: [`GetTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantResponse.ts)

## 例

[inline-code-attrs-start title = 'getTenant の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp';
const id: string = 'tenant-987654321';
const options: { includeBilling?: boolean } = { includeBilling: true };
const response: GetTenantResponse = await getTenant(tenantId, id);
const billingInfo: BillingInfo | undefined = undefined
[inline-code-end]