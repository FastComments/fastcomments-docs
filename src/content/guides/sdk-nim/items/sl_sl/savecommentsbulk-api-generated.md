## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| createCommentParams | seq[CreateCommentParams] | Ne |  |
| isLive | bool | Ne |  |
| doSpamCheck | bool | Ne |  |
| sendEmails | bool | Ne |  |
| populateNotifications | bool): (Option[seq[SaveComment_200_response]] | Ne |  |
| id | string | Ne |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | Ne |  |
| userId | string | Ne |  |
| anonUserId | string | Ne |  |

## Odgovor

Vrne: [`Option[UnBlockCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_un_block_comment_public200response.nim)

## Primer

[inline-code-attrs-start title = 'Primer saveCommentsBulk'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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