## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenantId | string | Sim |  |
| commentId | string | Sim |  |
| broadcastId | string | Não |  |
| sso | string | Não |  |

## Resposta

Retorna: [`Option[PinComment_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_pin_comment200response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de pinComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.pinComment(tenantId = "my-tenant-123", commentId = "cmt-98765", broadcastId = "", sso = "")
if response.isSome:
  let pinned = response.get()
  echo "Pinned comment response received"
else:
  echo "No pin response"
[inline-code-end]

---