I voti individuali dietro i conteggi di un sondaggio, dal più vecchio al più recente.

Un sondaggio appartiene a un commento, quindi i voti vengono sempre letti un sondaggio alla volta - commentId è obbligatorio. Questo mantiene ogni query sugli indici che la collezione possiede già.

Rispetta la privacy del sondaggio: i voti di un sondaggio anonimo non possono essere letti (poll-anonymous), qui o tramite ID.

## Parameters

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| commentId | string | Sì |  |
| voterId | string | No |  |
| optionId | string | No |  |
| skip | number | No |  |

## Response

Returns: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPollVotesResponse.ts)

## Example

[inline-code-attrs-start title = 'Esempio getPollVotes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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