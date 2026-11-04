Commentatori passati sulla pagina che NON sono attualmente online. Ordinati per displayName.
Usare questo dopo aver esaurito /users/online per renderizzare una sezione "Members".
Paginazione con cursore su commenterName: il server percorre l'indice parziale {tenantId, urlId, commenterName}
a partire da afterName in avanti tramite $gt, senza costo di $skip.

## Parametri

| Nome | Tipo | Richiesto | Descrizione |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| urlId | string | Sì |  |
| options | GetOfflineUsersOptions | No |  |

## Risposta

Restituisce: [`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di getOfflineUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (offlineResp, httpResponse) = client.getOfflineUsers(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  options = GetOfflineUsersOptions()
)
if offlineResp.isSome:
  let offline = offlineResp.get()
  echo offline)
[inline-code-end]