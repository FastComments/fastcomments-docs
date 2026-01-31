## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tag | string | No |  |
| tenantId | string | Yes |  |
| updateHashTagBody | UpdateHashTagBody | No |  |

## Response

Returns: [`Option[PatchHashTag_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_patch_hash_tag200response.nim)

## Example

[inline-code-attrs-start title = 'patchHashTag Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let updateBody = UpdateHashTagBody(
  name = "Breaking News",
  description = "Tag applied to breaking news articles",
  enabled = true,
  aliases = @["breaking", "urgent"]
)
let (response, httpResponse) = client.patchHashTag(tag = "breaking-news", tenantId = "my-tenant-123", updateHashTagBody = updateBody)
if response.isSome:
  let patched = response.get()
  echo patched
[inline-code-end]
