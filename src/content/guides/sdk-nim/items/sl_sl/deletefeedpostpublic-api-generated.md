## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| postId | string | Ne |  |
| broadcastId | string | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vrača: [`Option[DeleteFeedPostPublicResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_feed_post_public_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer deleteFeedPostPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteFeedPostPublic(tenantId = "my-tenant-123", postId = "", broadcastId = "", sso = "")
if response.isSome:
  let deleted = response.get()
  echo "Delete successful"
else:
  echo "Delete failed"
[inline-code-end]

---