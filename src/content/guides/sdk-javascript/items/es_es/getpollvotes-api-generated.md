Los votos individuales detrás de los recuentos de una encuesta, del más antiguo al más reciente.

Una encuesta pertenece a un comentario, por lo que los votos siempre se leen una encuesta a la vez - commentId es obligatorio. Eso mantiene cada consulta en los índices que ya tiene la colección.

Respeta la privacidad de la encuesta: los votos de una encuesta anónima no pueden leerse (poll-anonymous), aquí o por id.

## Parameters

| Nombre | Tipo | Obligatorio | Descripción |
|--------|------|-------------|-------------|
| tenantId | string | Sí |  |
| commentId | string | Sí |  |
| voterId | string | No |  |
| optionId | string | No |  |
| skip | number | No |  |

## Response

Devuelve: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPollVotesResponse.ts)

## Example

[inline-code-attrs-start title = 'Ejemplo getPollVotes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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