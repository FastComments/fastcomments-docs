## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| options | GetVotesForUserOptions | No |  |

## Yanıt

Döndürür: [`Option[GetVotesForUserResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_votes_for_user_response.nim)

## Örnek

[inline-code-attrs-start title = 'getVotesForUser Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.getVotesForUser(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  options = GetVotesForUserOptions()
)

if optResp.isSome:
  let resp = optResp.get()
  echo resp
[inline-code-end]