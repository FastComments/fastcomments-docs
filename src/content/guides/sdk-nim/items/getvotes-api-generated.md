## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |

## Response

Returns: [`Option[GetVotes_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_votes200response.nim)

## Example

[inline-code-attrs-start title = 'getVotes Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getVotes(tenantId = "my-tenant-123", urlId = "news/article-2026-01-09")
if response.isSome:
  let votes = response.get()
  echo "Received votes:", $votes
else:
  echo "No votes returned"
[inline-code-end]
