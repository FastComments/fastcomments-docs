## Parameters

| Ime | Tip | Obvezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| postId | string | Ne |  |
| reactBodyParams | ReactBodyParams | Ne |  |
| options | ReactFeedPostPublicOptions | Ne |  |

## Response

Vrne: [`Option[ReactFeedPostResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_react_feed_post_response.nim)

## Primer

[inline-code-attrs-start title = 'reactFeedPostPublic Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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

---