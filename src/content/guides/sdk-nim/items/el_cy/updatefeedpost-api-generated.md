## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenantId | string | Ναι |  |
| id | string | Όχι |  |
| feedPost | FeedPost | Όχι |  |

## Απάντηση

Επιστρέφει: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα updateFeedPost'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let feedPost = FeedPost(
  title: "Breaking News",
  content: "Updated story content",
  tags: @["news", "update"]
)

let (apiRes, httpRes) = client.updateFeedPost(
  tenantId = "my-tenant-123",
  id = "post-456",
  feedPost = feedPost
)

if apiRes.isSome:
  let res = apiRes.get()
[inline-code-end]