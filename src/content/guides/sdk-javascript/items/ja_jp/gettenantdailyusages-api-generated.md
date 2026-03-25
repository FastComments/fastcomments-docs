## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| yearNumber | number | No |  |
| monthNumber | number | No |  |
| dayNumber | number | No |  |
| skip | number | No |  |

## レスポンス

戻り値: [`GetTenantDailyUsages200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantDailyUsages200Response.ts)

## 例

[inline-code-attrs-start title = 'getTenantDailyUsages の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_5f4a3b2c-1d6e-4f9a-b9d8-123456789abc';
const yearNumber: number = 2026;
const monthNumber: number = 3;
const dayNumber: number = 24;
const skip: number = 0;

const result: GetTenantDailyUsages200Response = await getTenantDailyUsages(tenantId, yearNumber, monthNumber, dayNumber, skip);
[inline-code-end]

---