## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|--------------|-------------|
| tenantId | string | Oui |  |
| urlId | string | Oui |  |

## Réponse

Retourne : [`GetVotesResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotesResponse1.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getVotes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchVotes(): Promise<void> {
  const tenantId: string = "acme-corp-01";
  const urlId: string = "article-2024-05-15";

  const response: GetVotesResponse1 = await getVotes(tenantId, urlId);

  // Exemple d'accès à un champ optionnel dans la réponse
  const firstVoteId: string | undefined = response?.votes?.[0]?.id;
}
[inline-code-end]