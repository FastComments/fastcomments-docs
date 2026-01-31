## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createHashTagBody | CreateHashTagBody | No |  |

## Response

Returns: [`Option[AddHashTag_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_add_hash_tag200response.nim)

## Example

[inline-code-attrs-start title = 'addHashTag Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let createHashTagBody = CreateHashTagBody(
  name = "breaking-news",
  displayName = "Breaking News",
  color = "#FF0000",
  aliases = @["breaking", "top-story"],
  isPublic = true
)

let (response, httpResponse) = client.addHashTag(tenantId = "my-tenant-123", createHashTagBody = createHashTagBody)

if response.isSome:
  let added = response.get()
  echo "Added hashtag: ", $added
else:
  echo "Failed to add hashtag, status: ", $httpResponse.status
[inline-code-end]
