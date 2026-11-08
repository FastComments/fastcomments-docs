## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| page | float64 | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`Option[GetHashTagsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_hash_tags_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'getHashTags उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (hashTagsOpt, httpResp) = client.getHashTags(tenantId = "my-tenant-123", page = 0.0)
if hashTagsOpt.isSome:
  let hashTags = hashTagsOpt.get()
  echo hashTags
else:
  echo "No hashtags found"
[inline-code-end]