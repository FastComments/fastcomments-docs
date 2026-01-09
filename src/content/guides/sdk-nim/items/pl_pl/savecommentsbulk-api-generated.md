## Parametry

| Name | Type | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| createCommentParams | seq[CreateCommentParams] | Nie |  |
| isLive | bool | Nie |  |
| doSpamCheck | bool | Nie |  |
| sendEmails | bool | Nie |  |
| populateNotifications | bool): (Option[seq[SaveComment_200_response]] | Nie |  |
| id | string | Nie |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | Nie |  |
| userId | string | Nie |  |
| anonUserId | string | Nie |  |

## Odpowiedź

Zwraca: [`Option[UnBlockCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_un_block_comment_public200response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład saveCommentsBulk'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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