## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createFeedPostParams | CreateFeedPostParams | No |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Response

Returns: [`Option[CreateFeedPostPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_feed_post_public200response.nim)

## Example

[inline-code-attrs-start title = 'createFeedPostPublic Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let createParams = CreateFeedPostParams(
  title = "Breaking: Major Outage Resolved",
  content = "Our engineering team has resolved the login outage. All services restored.",
  url = "news/major-outage-resolved",
  authorId = "editor-42",
  isPublic = true,
  tags = @["outage", "status"]
)

let (response, httpResponse) = client.createFeedPostPublic(
  tenantId = "my-tenant-123",
  createFeedPostParams = createParams,
  broadcastId = "",
  sso = ""
)

if response.isSome:
  let created = response.get()
  echo "Created feed post:", created
else:
  echo "Failed to create feed post, HTTP status: ", httpResponse.status
[inline-code-end]
