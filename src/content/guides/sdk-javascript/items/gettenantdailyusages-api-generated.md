## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| yearNumber | number | No |  |
| monthNumber | number | No |  |
| dayNumber | number | No |  |
| skip | number | No |  |

## Response

Returns: [`GetTenantDailyUsages200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantDailyUsages200Response.ts)

## Example

[inline-code-attrs-start title = 'getTenantDailyUsages Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_4b7d2e9f';
  const summaryResult: GetTenantDailyUsages200Response = await getTenantDailyUsages(tenantId);
  const yearNumber: number = 2025;
  const monthNumber: number = 12;
  const dayNumber: number = 31;
  const skip: number = 0;
  const detailedResult: GetTenantDailyUsages200Response = await getTenantDailyUsages(tenantId, yearNumber, monthNumber, dayNumber, skip);
  console.log(summaryResult, detailedResult);
})();
[inline-code-end]
