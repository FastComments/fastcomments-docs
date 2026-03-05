## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createFeedPostParams | CreateFeedPostParams | No |  |
| broadcastId | string | No |  |
| isLive | bool | No |  |
| doSpamCheck | bool | No |  |
| skipDupCheck | bool | No |  |

## Response

Returns: [`Option[CreateFeedPost_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_feed_post200response.nim)

## Example

[inline-code-attrs-start title = 'createFeedPost Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createFeedPost(
  tenantId = "my-tenant-123",
  createFeedPostParams = CreateFeedPostParams(
    title = "Breaking: Major Storm Hits City",
    content = "A severe storm caused widespread outages and travel delays across the metro area.",
    authorId = "author-789",
    url = "news/major-storm",
    tags = @["weather", "breaking"],
    attachments = @[]
  ),
  broadcastId = "",
  isLive = false,
  doSpamCheck = false,
  skipDupCheck = false
)

if response.isSome:
  let created = response.get()
  echo "Feed post created:", created
[inline-code-end]
