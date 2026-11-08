## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| broadcastId | string | नहीं |  |
| sso | string = "" | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`Option[ChangeCommentPinStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_change_comment_pin_status_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'unPinComment उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (responseOpt, httpResponse) = client.unPinComment(
  tenantId = "my-tenant-123",
  commentId = "cmt-456789",
  broadcastId = "broadcast-001",
  sso = ""
)

if responseOpt.isSome:
  let resp = responseOpt.get()
  echo resp
[inline-code-end]