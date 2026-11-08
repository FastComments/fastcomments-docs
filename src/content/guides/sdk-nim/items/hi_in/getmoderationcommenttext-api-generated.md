## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| sso | string = "" | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`Option[GetCommentTextResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comment_text_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'getModerationCommentText उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.getModerationCommentText(
  tenantId = "my-tenant-123",
  commentId = "comment-456abc",
  sso = ""
)

if optResp.isSome:
  let resp = optResp.get()
  echo resp
[inline-code-end]