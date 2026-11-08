## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| skip | float64 | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`Option[GetTenantUsersResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_users_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'getTenantUsers उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTenantUsers(tenantId = "my-tenant-123", skip = 0.0)
if response.isSome:
  let data = response.get()
  echo data
[inline-code-end]