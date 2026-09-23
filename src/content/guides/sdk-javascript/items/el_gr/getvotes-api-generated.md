## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| urlId | string | Ναι |  |

## Απάντηση

Επιστρέφει: [`GetVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotesResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getVotes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const urlId: string = "article-67890";

(async () => {
  const votesResponse: GetVotesResponse = await getVotes(tenantId, urlId);
  // Παράδειγμα πρόσβασης σε προαιρετική ιδιότητα από την απάντηση
  const firstVote: PublicVote | undefined = votesResponse.votes?.[0];
})();
[inline-code-end]

---