## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| createCommentParams | seq[CreateCommentParams] | Nee |  |
| isLive | bool | Nee |  |
| doSpamCheck | bool | Nee |  |
| sendEmails | bool | Nee |  |
| populateNotifications | bool): (Option[seq[SaveCommentsBulkResponse]] | Nee |  |
| id | string | Nee |  |
| fromName | string | Nee |  |

## Antwoord

Retourneert: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'saveCommentsBulk Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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