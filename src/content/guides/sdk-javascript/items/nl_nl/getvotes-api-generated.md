---
## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |

## Respons

Retourneert: [`GetVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotesResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getVotes Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const urlId: string = "article-67890";

(async () => {
  const votesResponse: GetVotesResponse = await getVotes(tenantId, urlId);
  // Voorbeeld van het benaderen van een optionele eigenschap uit de respons
  const firstVote: PublicVote | undefined = votesResponse.votes?.[0];
})();
[inline-code-end]

---