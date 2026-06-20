## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| search | string | No |  |
| locale | string | No |  |
| rating | string | No |  |
| page | float64 | No |  |

## Response

Returns: [`Option[GetGifsSearchResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_gifs_search_response.nim)

## Example

[inline-code-attrs-start title = 'getGifsSearch Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getGifsSearch(
  tenantId = "my-tenant-123",
  search = "funny cat",
  locale = "en-US",
  rating = "PG",
  page = 1.0
)

if response.isSome:
  let gifs = response.get()
  echo "Fetched GIFs response:", gifs
[inline-code-end]
