## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| postIds | seq[string] | No |  |
| sso | string | No |  |

## Response

Returns: [`Option[UserReactsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_user_reacts_response.nim)

## Example

[inline-code-attrs-start title = 'getUserReactsPublic Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserReactsPublic(
  tenantId = "my-tenant-123",
  postIds = @["news/article-2026", "blog/opinion-987"],
  sso = ""
)
if response.isSome:
  let reacts = response.get()
  echo "Received user reacts for tenant: ", "my-tenant-123"
[inline-code-end]
