## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |

## Réponse

Retourne : [`GetVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotesResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getVotes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const urlId: string = "article-67890";

(async () => {
  const votesResponse: GetVotesResponse = await getVotes(tenantId, urlId);
  // Exemple d'accès à une propriété optionnelle de la réponse
  const firstVote: PublicVote | undefined = votesResponse.votes?.[0];
})();
[inline-code-end]

---