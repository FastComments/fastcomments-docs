## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| createFeedPostParams | CreateFeedPostParams | No |  |
| options | CreateFeedPostOptions | No |  |

## Risposta

Restituisce: [`Option[CreateFeedPostsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_feed_posts_response.nim)

## Esempio

[inline-code-attrs-start title = 'createFeedPost Esempio'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (respOpt, httpResp) = client.createFeedPost(
  tenantId = "my-tenant-123",
  createFeedPostParams = CreateFeedPostParams(),
  options = CreateFeedPostOptions()
)

if respOpt.isSome:
  let feedPost = respOpt.get()
[inline-code-end]