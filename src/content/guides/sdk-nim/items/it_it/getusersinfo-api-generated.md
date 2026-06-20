Informazioni utente in blocco per un tenant. Dato userIds, restituisce le informazioni di visualizzazione da User / SSOUser.
Utilizzato dal widget dei commenti per arricchire gli utenti che sono appena apparsi tramite un evento di presenza.
Nessun contesto di pagina: la privacy è applicata in modo uniforme (i profili privati sono mascherati).

## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| ids | string | No |  |

## Risposta

Restituisce: [`Option[PageUsersInfoResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_info_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di getUsersInfo'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUsersInfo(tenantId = "my-tenant-123", ids = "user-42,user-87")
if response.isSome:
  let usersInfo = response.get()
  echo "Retrieved users info:", usersInfo
else:
  echo "No users info returned. HTTP status:", httpResponse.status
[inline-code-end]

---