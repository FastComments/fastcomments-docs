## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| id | string | Não |  |
| updateUserBadgeParams | UpdateUserBadgeParams | Não |  |

## Resposta

Retorna: [`Option[APIEmptySuccessResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_success_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de updateUserBadge'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateUserBadge(
  tenantId = "my-tenant-123",
  id = "user-456",
  updateUserBadgeParams = UpdateUserBadgeParams()
)

if response.isSome:
  let success = response.get()
  echo "Badge updated successfully"
else:
  echo "Badge update failed"
[inline-code-end]

---