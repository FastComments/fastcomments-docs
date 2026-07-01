## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |

## Odgovor

Vrne: [`Option[GetVotesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_votes_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer getVotes'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optVotes, httpResp) = client.getVotes(tenantId = "my-tenant-123", urlId = "news/article-42")
if optVotes.isSome:
  let votes = optVotes.get()
  discard votes
[inline-code-end]