## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | नहीं |  |
| sso | string = "" | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`Option[BlockSuccess]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_block_success.nim)

## उदाहरण

[inline-code-attrs-start title = 'blockFromCommentPublic उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (blockOpt, httpResp) = client.blockFromCommentPublic(
  tenantId = "my-tenant-123",
  commentId = "cmt-456789",
  publicBlockFromCommentParams = PublicBlockFromCommentParams(),
  sso = ""
)

if blockOpt.isSome:
  let block = blockOpt.get()
[inline-code-end]