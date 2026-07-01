## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| createModeratorBody | CreateModeratorBody | Não |  |

## Resposta

Retorna: [`Option[CreateModeratorResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_moderator_response.nim)

## Exemplo

[inline-code-attrs-start title = 'createModerator Exemplo'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (moderatorRes, httpResp) = client.createModerator(tenantId = "my-tenant-123", createModeratorBody = CreateModeratorBody())
if moderatorRes.isSome:
  let moderator = moderatorRes.get()
[inline-code-end]