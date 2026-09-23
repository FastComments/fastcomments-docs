## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|--------|------|--------------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |

## Respuesta

Devuelve: [`GetPollVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPollVoteResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo getPollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchPollVote(): Promise<void> {
  const tenantId: string = "acme-corp-tenant";
  const pollId: string = "poll-2024-05";
  const result: GetPollVoteResponse = await getPollVote(tenantId, pollId);
  const status: APIStatus = result.status;
  const vote: PublicPollVote | undefined = result.vote;
}
[inline-code-end]