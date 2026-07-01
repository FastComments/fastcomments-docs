## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|--------|------|-------------|-------------|
| tenantId | string | Sí |  |
| urlId | string | Sí |  |

## Respuesta

Devuelve: [`GetVotesResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotesResponse1.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo getVotes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchVotes(): Promise<void> {
  const tenantId: string = "acme-corp-01";
  const urlId: string = "article-2024-05-15";

  const response: GetVotesResponse1 = await getVotes(tenantId, urlId);

  // Ejemplo de acceso a un campo opcional en la respuesta
  const firstVoteId: string | undefined = response?.votes?.[0]?.id;
}
[inline-code-end]

---