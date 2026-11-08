## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | नहीं |  |
| replaceTenantUserBody | ReplaceTenantUserBody | नहीं |  |
| updateComments | string = "" | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'replaceTenantUser उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let replaceBody = ReplaceTenantUserBody()
let (response, httpResponse) = client.replaceTenantUser(
  tenantId = "my-tenant-123",
  id = "user-456",
  replaceTenantUserBody = replaceBody,
  updateComments = "")
if response.isSome:
  let empty = response.get()
[inline-code-end]