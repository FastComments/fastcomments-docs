## Παράμετροι

| Name | Type | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| createCommentParams | seq[CreateCommentParams] | Όχι |  |
| isLive | bool | Όχι |  |
| doSpamCheck | bool | Όχι |  |
| sendEmails | bool | Όχι |  |
| populateNotifications | bool): (Option[seq[SaveCommentsBulkResponse]] | Όχι |  |
| id | string | Όχι |  |
| fromName | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα saveCommentsBulk'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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