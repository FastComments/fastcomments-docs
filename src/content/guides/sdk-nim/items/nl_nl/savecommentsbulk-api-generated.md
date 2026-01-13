## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenantId | string | Ja |  |
| createCommentParams | seq[CreateCommentParams] | Nee |  |
| isLive | bool | Nee |  |
| doSpamCheck | bool | Nee |  |
| sendEmails | bool | Nee |  |
| populateNotifications | bool): (Option[seq[SaveComment_200_response]] | Nee |  |
| id | string | Nee |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | Nee |  |
| userId | string | Nee |  |
| anonUserId | string | Nee |  |

## Antwoord

Geeft terug: [`Option[UnBlockCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_un_block_comment_public200response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'saveCommentsBulk Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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