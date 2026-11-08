## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| createTenantUserBody | CreateTenantUserBody | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`Option[CreateTenantUserResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_tenant_user_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'createTenantUser उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let tenantId = "my-tenant-123"
let userBody = CreateTenantUserBody(
  email: "john.doe@example.com",
  name: "John Doe",
  password: "s3cr3tP@ss",
  role: "admin"
)
let (optResp, httpResp) = client.createTenantUser(
  tenantId = tenantId,
  createTenantUserBody = userBody
)
if optResp.isSome:
  let resp = optResp.get()
  echo resp.userId
[inline-code-end]