## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| postId | string | Hayır |  |
| broadcastId | string | Hayır |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`Option[DeleteFeedPostPublicResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_feed_post_public_response.nim)

## Örnek

[inline-code-attrs-start title = 'deleteFeedPostPublic Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteFeedPostPublic(tenantId = "my-tenant-123", postId = "", broadcastId = "", sso = "")
if response.isSome:
  let deleted = response.get()
  echo "Delete successful"
else:
  echo "Delete failed"
[inline-code-end]

---