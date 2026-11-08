## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`Option[GetTenantUserResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_user_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'getTenantUser उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeUser, httpResp) = client.getTenantUser(tenantId = "my-tenant-123", id = "user-456")
if maybeUser.isSome:
  let user = maybeUser.get()
  echo user
[inline-code-end]