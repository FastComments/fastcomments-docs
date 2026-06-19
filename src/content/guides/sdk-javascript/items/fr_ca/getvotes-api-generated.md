## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| urlId | string | Oui |  |

## Réponse

Renvoie : [`GetVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotesResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getVotes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_8421';
const urlId: string | undefined = 'posts/2026/06/typescript-api-examples';
const votes: GetVotesResponse = await getVotes(tenantId, urlId!);
[inline-code-end]

---