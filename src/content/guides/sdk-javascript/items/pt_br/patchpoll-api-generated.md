Edite uma enquete no local, mantendo suas contagens: altere a pergunta, renomeie uma opção, feche ou reabra-a,
ou altere quem pode ver os votantes. As opções são referenciadas por id – para adicionar, remover ou reordenar, faça PUT da lista completa.

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| commentId | string | Sim |  |
| commentPollPatch | CommentPollPatch | Sim |  |

## Resposta

Retorna: [`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo patchPoll'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const commentId: string = "cmt_9876543210";

  const pollPatch: CommentPollPatch = {
    question: "Which new feature would you like to see?",
    options: [
      { id: "opt-1", label: "Dark Mode" },
      { id: "opt-2", label: "Multi-language Support" }
    ]
  };

  const response: SavePollResponse = await patchPoll(tenantId, commentId, pollPatch);
  console.log(response);
})();
[inline-code-end]