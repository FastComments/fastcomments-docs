## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| createFeedPostParams | CreateFeedPostParams | Όχι |  |
| broadcastId | string | Όχι |  |
| sso | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`Option[CreateFeedPostPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_feed_post_public200response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα createFeedPostPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createFeedPostPublic(
  tenantId = "my-tenant-123",
  createFeedPostParams = CreateFeedPostParams(
    title = "Product Launch Announcement",
    content = "We just launched a new commenting feature to improve engagement.",
    authorId = "team-product",
    url = "news/product-launch",
    tags = @["launch", "comments"],
    isFeatured = false
  ),
  broadcastId = "broadcast-009",
  sso = ""
)
if response.isSome:
  let created = response.get()
  discard created
[inline-code-end]

---