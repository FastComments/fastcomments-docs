req
tenantId
afterId

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| afterId | string | Nee |  |
| limit | int | Nee |  |
| tags | seq[string] | Nee |  |

## Antwoord

Retourneert: [`Option[GetFeedPostsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_feed_posts_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'getFeedPosts Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getFeedPosts(
  tenantId = "my-tenant-123",
  afterId = "",
  limit = 0,
  tags = @[]
)
if response.isSome:
  let feed = response.get()
  echo "Feed retrieved for tenant my-tenant-123"
[inline-code-end]

---