## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | नहीं |  |
| updateModeratorBody | UpdateModeratorBody | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'updateModerator उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let body = UpdateModeratorBody(name = "John Doe", email = "john@example.com", isActive = true)
let (apiResult, httpResp) = client.updateModerator(tenantId = "my-tenant-123", id = "mod-456", updateModeratorBody = body)
if apiResult.isSome:
  let result = apiResult.get()
[inline-code-end]