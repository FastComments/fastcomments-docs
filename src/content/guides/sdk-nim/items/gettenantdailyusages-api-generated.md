## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| yearNumber | float64 | No |  |
| monthNumber | float64 | No |  |
| dayNumber | float64 | No |  |
| skip | float64 | No |  |

## Response

Returns: [`Option[GetTenantDailyUsages_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_daily_usages200response.nim)

## Example

[inline-code-attrs-start title = 'getTenantDailyUsages Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTenantDailyUsages(
  tenantId = "my-tenant-123",
  yearNumber = 2026.0,
  monthNumber = 1.0,
  dayNumber = 15.0,
  skip = 0.0
)

if response.isSome:
  let usages = response.get()
  echo "Daily usages retrieved for tenant: my-tenant-123"
  echo usages
[inline-code-end]
