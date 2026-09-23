Os votos individuais por trás da contagem de uma enquete, do mais antigo ao mais recente.

Uma enquete pertence a um comentário, portanto os votos são sempre lidos uma enquete por vez – commentId é obrigatório. Isso mantém cada consulta nos índices que a coleção já possui.

Obedece à privacidade da enquete: os votos de uma enquete anônima não podem ser lidos (poll-anonymous), aqui ou por ID.

## Parameters

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| commentId | string | Sim |  |
| voterId | string | Não |  |
| optionId | string | Não |  |
| skip | number | Não |  |

## Response

Retorna: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPollVotesResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getPollVotes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "cmt_98765";
const voterId: string = "user_abc";
const optionId: string = "opt_1";
const skip: number = 20;

const pollResult: GetPollVotesResponse = await getPollVotes(
  tenantId,
  commentId,
  voterId,
  optionId,
  skip
);

console.log(pollResult);
[inline-code-end]

---