## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| id | string | Não |  |

## Resposta

Retorna: [`Option[APIGetUserBadgeProgressResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_get_user_badge_progress_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getUserBadgeProgressById'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (badgeProgressOpt, httpResp) = client.getUserBadgeProgressById(tenantId = "my-tenant-123", id = "badge-456")
if badgeProgressOpt.isSome:
  let badgeProgress = badgeProgressOpt.get()
  echo badgeProgress
[inline-code-end]