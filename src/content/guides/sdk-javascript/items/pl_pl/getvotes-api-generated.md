## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| urlId | string | Tak |  |

## Odpowiedź

Zwraca: [`GetVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotesResponse.ts)

## Przykład

[inline-code-attrs-start title = 'getVotes Przykład'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const urlId: string = "article-67890";

(async () => {
  const votesResponse: GetVotesResponse = await getVotes(tenantId, urlId);
  // Przykład uzyskiwania dostępu do opcjonalnej właściwości z odpowiedzi
  const firstVote: PublicVote | undefined = votesResponse.votes?.[0];
})();
[inline-code-end]

---