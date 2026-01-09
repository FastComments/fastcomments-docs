## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| createCommentParams | seq[CreateCommentParams] | Nein |  |
| isLive | bool | Nein |  |
| doSpamCheck | bool | Nein |  |
| sendEmails | bool | Nein |  |
| populateNotifications | bool): (Option[seq[SaveComment_200_response]] | Nein |  |
| id | string | Nein |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | Nein |  |
| userId | string | Nein |  |
| anonUserId | string | Nein |  |

## Antwort

Gibt zurück: [`Option[UnBlockCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_un_block_comment_public200response.nim)

## Beispiel

[inline-code-attrs-start title = 'saveCommentsBulk Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.saveCommentsBulk(
  tenantId = "my-tenant-123",
  createCommentParams = @[],
  isLive = true,
  doSpamCheck = true,
  sendEmails = false,
  populateNotifications = true,
  id = "batch-20251122",
  unBlockFromCommentParams = UnBlockFromCommentParams(),
  userId = "user-456",
  anonUserId = "anon-789"
)
if response.isSome:
  let unblocked = response.get()
  echo "Unblocked response received: ", unblocked
else:
  echo "No unblocked response, httpResponse: ", $httpResponse
[inline-code-end]

---