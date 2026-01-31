## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| page | float64 | No |  |

## Response

Returns: [`Option[GetHashTags_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_hash_tags200response.nim)

## Example

[inline-code-attrs-start title = 'getHashTags Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getHashTags(tenantId = "my-tenant-123", page = 1.0)
if response.isSome:
  let hashTags = response.get()
  discard hashTags
[inline-code-end]
