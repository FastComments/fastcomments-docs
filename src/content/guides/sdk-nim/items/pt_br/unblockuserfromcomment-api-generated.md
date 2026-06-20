## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| id | string | Não |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | Não |  |
| userId | string | Não |  |
| anonUserId | string | Não |  |

## Resposta

Retorna: [`Option[UnblockSuccess]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_unblock_success.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de unBlockUserFromComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.unBlockUserFromComment(
  tenantId = "my-tenant-123",
  id = "comment-9f3b2a",
  unBlockFromCommentParams = UnBlockFromCommentParams(),
  userId = "user-1024",
  anonUserId = "anon-77b"
)

if response.isSome:
  let unblockResult = response.get()
  echo unblockResult
else:
  echo "Unblock failed"
[inline-code-end]

---