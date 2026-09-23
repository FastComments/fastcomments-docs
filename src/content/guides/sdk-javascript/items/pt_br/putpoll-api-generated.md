Anexe uma enquete a um comentário, ou defina o estado completo da enquete que ele já possui.

As opções são correspondidas por id: uma opção enviada com o id de uma opção existente mantém seus votos (e recebe o novo rótulo e posição), uma opção enviada sem id é adicionada, e opções existentes que não constam na lista são removidas juntamente com os votos lançados nelas.

Manter nenhum id de opção existente em uma enquete que tem votos exclui todas elas, portanto é necessário replaceVotes=true.

## Parameters

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| commentId | string | Sim |  |
| commentPollPutInput | CommentPollPutInput | Sim |  |
| replaceVotes | boolean | Não |  |

## Response

Retorna: [`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo putPoll'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "c1a2b3d4-5678-90ab-cdef-1234567890ab";
const commentId: string = "9876543210";

const optionA: CommentPollOptionInput = { text: "Dark mode" };
const optionB: CommentPollOptionInput = { text: "Light mode" };

const pollInput: CommentPollPutInput = {
  question: "Which UI theme do you prefer?",
  options: [optionA, optionB],
};

const replaceVotes: boolean = true;

const result: SavePollResponse = await putPoll(tenantId, commentId, pollInput, replaceVotes);
[inline-code-end]