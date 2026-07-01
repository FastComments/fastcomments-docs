## Parametreler

| İsim | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| postId | string | Hayır |  |
| updateFeedPostParams | UpdateFeedPostParams | Hayır |  |
| options | UpdateFeedPostPublicOptions | Hayır |  |

## Yanıt

Döndürür: [`Option[CreateFeedPostResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_feed_post_response.nim)

## Örnek

[inline-code-attrs-start title = 'updateFeedPostPublic Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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

---