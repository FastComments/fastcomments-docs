## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| id | string | Não |  |
| updateModeratorBody | UpdateModeratorBody | Não |  |

## Resposta

Retorna: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de updateModerator'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let updateBody = UpdateModeratorBody(
  name: "Alicia Gomez",
  email: "alicia.gomez@dailynews.com",
  active: true,
  roles: @["moderator"]
)

let (response, httpResponse) = client.updateModerator(tenantId = "my-tenant-123", id = "moderator-789", updateModeratorBody = updateBody)

if response.isSome:
  let updated = response.get()
  echo "Moderator updated:", updated
[inline-code-end]

---