---
## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| urlId | string | Sì |  |

## Risposta

Restituisce: [`Option[GetVotesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_votes_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio getVotes'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getVotes(tenantId = "my-tenant-123", urlId = "news/breaking-article-456")
if response.isSome:
  let votesResp = response.get()
  echo "Received votes response:", $votesResp
else:
  echo "No votes returned, HTTP response:", $httpResponse
[inline-code-end]

---