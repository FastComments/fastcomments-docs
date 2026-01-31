## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Response

Returns: [`Option[GetVotesForUser_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_votes_for_user200response.nim)

## Example

[inline-code-attrs-start title = 'getVotesForUser Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getVotesForUser(
  tenantId = "my-tenant-123",
  urlId = "news/election-coverage-2026",
  userId = "user-789",
  anonUserId = ""
)

if response.isSome:
  let votes = response.get()
  echo "Received votes: ", $votes
else:
  echo "No votes returned for tenant my-tenant-123 / news/election-coverage-2026"
[inline-code-end]
