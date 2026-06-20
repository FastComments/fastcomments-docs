## Parametri

| Ime | Type | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| createCommentParams | seq[CreateCommentParams] | Ne |  |
| isLive | bool | Ne |  |
| doSpamCheck | bool | Ne |  |
| sendEmails | bool | Ne |  |
| populateNotifications | bool): (Option[seq[SaveCommentsBulkResponse]] | Ne |  |
| id | string | Ne |  |
| fromName | string | Ne |  |

## Odgovor

Vrne: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer saveCommentsBulk'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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