Bulk-Benutzerinformationen für einen Mandanten. Bei gegebenen userIds werden Anzeigendaten aus User / SSOUser zurückgegeben.  
Wird vom Kommentar‑Widget verwendet, um Benutzer, die gerade über ein Präsenz‑Ereignis erschienen sind, anzureichern.  
Kein Seitenkontext: Die Privatsphäre wird einheitlich durchgesetzt (private Profile werden maskiert).

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| ids | string | Nein |  |

## Response

Rückgabe: [`Option[PageUsersInfoResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_info_response.nim)

## Example

[inline-code-attrs-start title = 'getUsersInfo Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (usersInfoOpt, httpResp) = client.getUsersInfo(tenantId = "my-tenant-123", ids = "user42")
if usersInfoOpt.isSome:
  let usersInfo = usersInfoOpt.get()
[inline-code-end]