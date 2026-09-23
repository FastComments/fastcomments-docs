Remova uma enquete do seu comentário, juntamente com todos os votos lançados nela. O próprio comentário permanece intacto.

## Parameters

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |

## Resposta

Retorna: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo deletePoll'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runDeletePoll(): Promise<void> {
  const tenantId: string = "tenant_9f8b7c6d";
  const commentId: string = "comment_a1b2c3d4";
  const result: APIEmptyResponse = await deletePoll(tenantId, commentId);
  console.log(result);
}
[inline-code-end]

---