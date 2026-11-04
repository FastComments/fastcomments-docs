Registra un voto en una encuesta, o mueve uno existente a una opción diferente. Un votante tiene como máximo un voto por
encuesta, por lo que llamar a esto de nuevo para el mismo votante mueve su voto en lugar de añadir uno.

Esto respeta la configuración de encuestas del sitio: si la votación está configurada solo para usuarios conectados, un voto con solo un
anonUserId es rechazado, y los votos anónimos tienen límites de frecuencia por IP por encuesta.

## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| createPollVoteBody | CreatePollVoteBody | Sí |  |

## Respuesta

Devuelve: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreatePollVoteResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de createPollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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