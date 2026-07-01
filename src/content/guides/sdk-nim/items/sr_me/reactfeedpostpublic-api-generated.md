## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| postId | string | No |  |
| reactBodyParams | ReactBodyParams | No |  |
| options | ReactFeedPostPublicOptions | No |  |

## Odgovor

Vraća: [`Option[ReactFeedPostResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_react_feed_post_response.nim)

## Primjer

[inline-code-attrs-start title = 'reactFeedPostPublic Primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let reactParams = ReactBodyParams()
let (optResp, httpResp) = client.reactFeedPostPublic(
  tenantId = "my-tenant-123",
  postId = "post-456",
  reactBodyParams = reactParams,
  options = ReactFeedPostPublicOptions()
)
if optResp.isSome:
  let resp = optResp.get()
  echo resp
[inline-code-end]