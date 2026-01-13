## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| createCommentParams | seq[CreateCommentParams] | Non |  |
| isLive | bool | Non |  |
| doSpamCheck | bool | Non |  |
| sendEmails | bool | Non |  |
| populateNotifications | bool): (Option[seq[SaveComment_200_response]] | Non |  |
| id | string | Non |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | Non |  |
| userId | string | Non |  |
| anonUserId | string | Non |  |

## Réponse

Renvoie: [`Option[UnBlockCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_un_block_comment_public200response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de saveCommentsBulk'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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