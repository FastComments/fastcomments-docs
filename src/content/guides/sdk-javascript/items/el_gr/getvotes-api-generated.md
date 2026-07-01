## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |

## Απόκριση

Επιστρέφει: [`GetVotesResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotesResponse1.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'getVotes Παράδειγμα'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchVotes(): Promise<void> {
  const tenantId: string = "acme-corp-01";
  const urlId: string = "article-2024-05-15";

  const response: GetVotesResponse1 = await getVotes(tenantId, urlId);

  // Παράδειγμα πρόσβασης σε προαιρετικό πεδίο στην απόκριση
  const firstVoteId: string | undefined = response?.votes?.[0]?.id;
}
[inline-code-end]

---