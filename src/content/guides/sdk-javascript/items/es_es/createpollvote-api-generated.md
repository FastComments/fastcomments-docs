Record a vote on a poll, or move an existing one to a different option. A voter has at most one vote per
poll, so calling this again for the same voter moves their vote rather than adding one.

Esto respeta la configuración de encuestas del sitio: si la votación está configurada solo para usuarios registrados, un voto con solo un anonUserId es rechazado, y los votos anónimos están limitados por tasa por IP por encuesta.

## Parameters

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| createPollVoteBody | CreatePollVoteBody | Sí |  |

## Response

Returns: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreatePollVoteResponse.ts)

## Example

[inline-code-attrs-start title = 'Ejemplo createPollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-tenant";

  const vote: CreatePollVoteBody = {
    pollId: "poll-2024-09",
    optionId: "option-A"
    // userId es opcional y se omite aquí
  };

  const result: CreatePollVoteResponse = await createPollVote(tenantId, vote);
  console.log(result);
})();
[inline-code-end]