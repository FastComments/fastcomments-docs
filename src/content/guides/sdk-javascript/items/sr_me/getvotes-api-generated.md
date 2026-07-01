## Parameters

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| urlId | string | Da |  |

## Response

Vraća: [`GetVotesResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotesResponse1.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer getVotes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchVotes(): Promise<void> {
  const tenantId: string = "acme-corp-01";
  const urlId: string = "article-2024-05-15";

  const response: GetVotesResponse1 = await getVotes(tenantId, urlId);

  // Primjer pristupa opcionom polju u odgovoru
  const firstVoteId: string | undefined = response?.votes?.[0]?.id;
}
[inline-code-end]