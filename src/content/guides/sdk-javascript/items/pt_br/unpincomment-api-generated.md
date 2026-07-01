## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| commentId | string | Sim |  |
| broadcastId | string | Sim |  |
| sso | string | Não |  |

## Resposta

Retorna: [`UnPinCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnPinCommentResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo unPinComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant-001"
  const commentId: string = "comment-123"
  const broadcastId: string = "broadcast-456"
  const sso: string = "sso-token-xyz"

  const resultWithSso: UnPinCommentResponse = await unPinComment(tenantId, commentId, broadcastId, sso)
  const resultWithoutSso: UnPinCommentResponse = await unPinComment(tenantId, commentId, broadcastId)

  console.log(resultWithSso, resultWithoutSso)
})()
[inline-code-end]

---