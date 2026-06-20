## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| createCommentParams | seq[CreateCommentParams] | No |  |
| isLive | bool | No |  |
| doSpamCheck | bool | No |  |
| sendEmails | bool | No |  |
| populateNotifications | bool): (Option[seq[SaveCommentsBulkResponse]] | No |  |
| id | string | No |  |
| fromName | string | No |  |

## Risposta

Restituisce: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di saveCommentsBulk'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.saveCommentsBulk(
  tenantId = "my-tenant-123",
  createCommentParams = @[],
  isLive = false,
  doSpamCheck = false,
  sendEmails = false,
  populateNotifications = false,
  id = "",
  fromName = ""
)

if response.isSome:
  let apiResp = response.get()
  echo "Bulk save succeeded, tenant:", " my-tenant-123"
else:
  echo "Bulk save returned no API response"
[inline-code-end]