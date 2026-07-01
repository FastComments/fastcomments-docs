Informazioni di massa sugli utenti per un tenant. Dati gli userIds, restituisce le informazioni di visualizzazione da User / SSOUser.  
Utilizzato dal widget dei commenti per arricchire gli utenti che sono appena comparsi tramite un evento di presenza.  
Nessun contesto di pagina: la privacy è applicata uniformemente (i profili privati sono mascherati).

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| ids | string | No |  |

## Response

Restituisce: [`Option[PageUsersInfoResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_info_response.nim)

## Example

[inline-code-attrs-start title = 'Esempio getUsersInfo'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (usersInfoOpt, httpResp) = client.getUsersInfo(tenantId = "my-tenant-123", ids = "user42")
if usersInfoOpt.isSome:
  let usersInfo = usersInfoOpt.get()
[inline-code-end]