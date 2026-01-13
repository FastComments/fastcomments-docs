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
let params = CreateFeedPostParams(
  title: "Local Elections Update",
  content: "Final results will be posted after polls close at 8pm.",
  authorId: "reporter-789",
  tags: @["politics", "local"],
  isPublic: true
)
let (response, httpResponse) = client.createFeedPostPublic(
  tenantId = "my-tenant-123",
  createFeedPostParams = params,
  broadcastId = "bcast-456",
  sso = "sso-token-abc"
)
if response.isSome:
  let created = response.get()
  echo "created post id: ", created.id
else:
  echo "request failed, status: ", httpResponse.status
[inline-code-end]
