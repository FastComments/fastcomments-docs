## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| options | GetApiCommentsOptions | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`Option[ModerationAPIGetCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_get_comments_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'getApiComments उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResp, httpResp) = client.getApiComments(tenantId = "my-tenant-123", options = GetApiCommentsOptions())
if maybeResp.isSome:
  let resp = maybeResp.get()
  # आगे की प्रोसेसिंग `resp` के साथ की जा सकती है
[inline-code-end]