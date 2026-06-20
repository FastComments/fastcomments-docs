## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tag | string | No |  |
| tenantId | string | Yes |  |
| updateHashTagBody | UpdateHashTagBody | No |  |

## Response

Returns: [`Option[UpdateHashTagResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_hash_tag_response.nim)

## Example

[inline-code-attrs-start title = 'patchHashTag Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.patchHashTag(tag = "breaking-news", tenantId = "my-tenant-123", updateHashTagBody = UpdateHashTagBody())
if response.isSome:
  let updatedHashTag = response.get()
  echo updatedHashTag
[inline-code-end]
