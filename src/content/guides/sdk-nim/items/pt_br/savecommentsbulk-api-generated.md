## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| createCommentParams | seq[CreateCommentParams] | Não |  |
| isLive | bool | Não |  |
| doSpamCheck | bool | Não |  |
| sendEmails | bool | Não |  |
| populateNotifications | bool): (Option[seq[SaveComment_200_response]] | Não |  |
| id | string | Não |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | Não |  |
| userId | string | Não |  |
| anonUserId | string | Não |  |

## Resposta

Retorna: [`Option[UnBlockCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_un_block_comment_public200response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de saveCommentsBulk'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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