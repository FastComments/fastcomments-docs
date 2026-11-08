## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentsByIdsParams | CommentsByIdsParams | नहीं |  |
| sso | string = "" | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`Option[ModerationAPIChildCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_child_comments_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'postCommentsByIds उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let tenantId = "my-tenant-123"
let params = CommentsByIdsParams(commentIds = @["cmt-001", "cmt-002"])
let (maybeResp, httpResp) = client.postCommentsByIds(tenantId = tenantId, commentsByIdsParams = params, sso = "")
if maybeResp.isSome:
  let resp = maybeResp.get()
  # resp का आवश्यकतानुसार उपयोग करें
[inline-code-end]