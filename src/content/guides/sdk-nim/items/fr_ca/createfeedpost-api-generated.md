## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| createFeedPostParams | CreateFeedPostParams | Non |  |
| broadcastId | string | Non |  |
| isLive | bool | Non |  |
| doSpamCheck | bool | Non |  |
| skipDupCheck | bool | Non |  |

## Réponse

Retourne: [`Option[CreateFeedPost_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_feed_post200response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de createFeedPost'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let createParams = CreateFeedPostParams(
  content = "We're rolling out realtime comments to all users!",
  title = "Realtime Comments Rollout",
  url = "news/realtime-comments-rollout-2025",
  authorId = "prod-team",
  tags = @["release", "comments"]
)

let (response, httpResponse) = client.createFeedPost(
  tenantId = "my-tenant-123",
  createFeedPostParams = createParams,
  broadcastId = "broadcast-2025-11",
  isLive = true,
  doSpamCheck = true,
  skipDupCheck = false
)

if response.isSome:
  let created = response.get()
  echo "Feed post created, id: ", $created.id
else:
  echo "Failed to create feed post, HTTP status: ", $httpResponse.statusCode
[inline-code-end]

---