## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | नहीं |  |
| updateTenantUserBody | UpdateTenantUserBody | नहीं |  |
| updateComments | string = "" | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'updateTenantUser उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let updateBody = UpdateTenantUserBody()
let (optResp, httpResp) = client.updateTenantUser(
  tenantId = "my-tenant-123",
  id = "user-456",
  updateTenantUserBody = updateBody,
  updateComments = "Changed role to moderator",
)
if optResp.isSome:
  let _ = optResp.get()
[inline-code-end]