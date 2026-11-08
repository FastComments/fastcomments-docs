## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| createModeratorBody | CreateModeratorBody | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`Option[CreateModeratorResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_moderator_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'createModerator उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (moderatorRes, httpResp) = client.createModerator(tenantId = "my-tenant-123", createModeratorBody = CreateModeratorBody())
if moderatorRes.isSome:
  let moderator = moderatorRes.get()
[inline-code-end]