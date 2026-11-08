## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| skip | float64 | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`Option[GetTenantPackagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_packages_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'getTenantPackages उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResp, httpResp) = client.getTenantPackages(tenantId = "my-tenant-123", skip = 0.0)
if maybeResp.isSome:
  let packages = maybeResp.get()
  echo packages
  echo httpResp.statusCode
[inline-code-end]