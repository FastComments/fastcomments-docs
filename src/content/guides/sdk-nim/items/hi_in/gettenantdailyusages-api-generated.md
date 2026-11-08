## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| options | GetTenantDailyUsagesOptions | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`Option[GetTenantDailyUsagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_daily_usages_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'getTenantDailyUsages उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (respOpt, httpResp) = client.getTenantDailyUsages(
  tenantId = "my-tenant-123",
  options = default(GetTenantDailyUsagesOptions),
)
if respOpt.isSome:
  let usage = respOpt.get()
  echo usage
  echo httpResp.statusCode
[inline-code-end]