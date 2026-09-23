## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Απάντηση

Επιστρέφει: [`GetPollVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPollVoteResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getPollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchPollVote(): Promise<void> {
  const tenantId: string = "acme-corp-tenant";
  const pollId: string = "poll-2024-05";
  const result: GetPollVoteResponse = await getPollVote(tenantId, pollId);
  const status: APIStatus = result.status;
  const vote: PublicPollVote | undefined = result.vote;
}
[inline-code-end]

---