## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| postId | string | Όχι |  |
| updateFeedPostParams | UpdateFeedPostParams | Όχι |  |
| options | UpdateFeedPostPublicOptions | Όχι |  |

## Απόκριση

Επιστρέφει: [`Option[CreateFeedPostResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_feed_post_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα updateFeedPostPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateFeedPostPublic(
  tenantId = "my-tenant-123",
  postId = "post-789",
  updateFeedPostParams = UpdateFeedPostParams(),
  options = UpdateFeedPostPublicOptions()
)

if response.isSome:
  let post = response.get()
[inline-code-end]