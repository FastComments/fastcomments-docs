## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| id | string | Não |  |

## Resposta

Retorna: [`Option[GetModerator_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_moderator200response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getModerator'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getModerator(tenantId = "my-tenant-123", id = "mod-98765")
if response.isSome:
  let moderator = response.get()
  discard moderator
[inline-code-end]