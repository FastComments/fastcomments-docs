## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| options | PostRemoveCommentOptions | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`Option[PostRemoveCommentApiResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_post_remove_comment_api_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'postRemoveComment उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiResponseOpt, httpResp) = client.postRemoveComment(
  tenantId = "my-tenant-123",
  commentId = "comment-987654",
  options = PostRemoveCommentOptions()
)

if apiResponseOpt.isSome:
  let apiResponse = apiResponseOpt.get()
[inline-code-end]