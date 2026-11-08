## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| broadcastId | string | नहीं |  |
| sso | string = "" | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'lockComment उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (lockResult, httpRes) = client.lockComment(
  tenantId = "my-tenant-123",
  commentId = "comment-456",
  broadcastId = "",
  sso = "")

if lockResult.isSome:
  let resp = lockResult.get()
  discard resp
[inline-code-end]