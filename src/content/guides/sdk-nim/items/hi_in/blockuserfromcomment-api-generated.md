## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | नहीं |  |
| blockFromCommentParams | BlockFromCommentParams | नहीं |  |
| options | BlockUserFromCommentOptions | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`Option[BlockSuccess]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_block_success.nim)

## उदाहरण

[inline-code-attrs-start title = 'blockUserFromComment उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params = BlockFromCommentParams()
let opts = BlockUserFromCommentOptions()
let (blockResult, httpResp) = client.blockUserFromComment(
  tenantId = "my-tenant-123",
  id = "comment-456",
  blockFromCommentParams = params,
  options = opts
)
if blockResult.isSome:
  let success = blockResult.get()
  discard success
[inline-code-end]