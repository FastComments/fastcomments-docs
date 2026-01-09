## Παράμετροι

| Όνομα | Type | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| afterId | string | Όχι |  |
| limit | int | Όχι |  |
| tags | seq[string] | Όχι |  |
| sso | string | Όχι |  |
| isCrawler | bool | Όχι |  |
| includeUserInfo | bool | Όχι |  |

## Απόκριση

Επιστρέφει: [`Option[GetFeedPostsPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_feed_posts_public200response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getFeedPostsPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getFeedPostsPublic(
  tenantId = "my-tenant-123",
  afterId = "",
  limit = 0,
  tags = @[],
  sso = "",
  isCrawler = false,
  includeUserInfo = false
)

if response.isSome:
  let feed = response.get()
  echo feed
[inline-code-end]

---