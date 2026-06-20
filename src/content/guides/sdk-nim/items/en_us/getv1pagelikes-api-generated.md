## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |

## Response

Returns: [`Option[GetV1PageLikes]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_v1_page_likes.nim)

## Example

[inline-code-attrs-start title = 'getV1PageLikes Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getV1PageLikes(tenantId = "my-tenant-123", urlId = "news/how-to-train-your-dragon")
if response.isSome:
  let pageLikes = response.get()
  echo "Fetched page likes for url:", "news/how-to-train-your-dragon"
else:
  echo "No likes returned for url:", "news/how-to-train-your-dragon"
[inline-code-end]
