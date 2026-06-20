## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| createCommentParams | seq[CreateCommentParams] | Non |  |
| isLive | bool | Non |  |
| doSpamCheck | bool | Non |  |
| sendEmails | bool | Non |  |
| populateNotifications | bool): (Option[seq[SaveCommentsBulkResponse]] | Non |  |
| id | string | Non |  |
| fromName | string | Non |  |

## Réponse

Retourne : [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de saveCommentsBulk'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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