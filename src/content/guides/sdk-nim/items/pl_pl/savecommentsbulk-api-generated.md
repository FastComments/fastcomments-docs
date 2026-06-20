---
## Parametry

| Name | Type | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| createCommentParams | seq[CreateCommentParams] | Nie |  |
| isLive | bool | Nie |  |
| doSpamCheck | bool | Nie |  |
| sendEmails | bool | Nie |  |
| populateNotifications | bool): (Option[seq[SaveCommentsBulkResponse]] | Nie |  |
| id | string | Nie |  |
| fromName | string | Nie |  |

## Odpowiedź

Zwraca: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład saveCommentsBulk'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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

---