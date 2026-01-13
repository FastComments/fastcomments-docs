## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenantId | string | Sim |  |
| id | string | Não |  |

## Resposta

Retorna: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de deletePendingWebhookEvent'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deletePendingWebhookEvent(tenantId = "my-tenant-123", id = "wh_evt_6f1e3b2a")
if response.isSome:
  let flagResp = response.get()
  discard flagResp
[inline-code-end]

---