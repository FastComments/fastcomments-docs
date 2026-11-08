## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| options | GetCountOptions | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`Option[ModerationAPICountCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_count_comments_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'getCount उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (countOpt, httpResponse) = client.getCount(tenantId = "my-tenant-123", options = GetCountOptions())
if countOpt.isSome:
  let count = countOpt.get()
[inline-code-end]