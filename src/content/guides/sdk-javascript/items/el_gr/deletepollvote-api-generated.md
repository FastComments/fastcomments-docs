Ανακαλέστε μια ψήφο. Η επιλογή στην οποία καταχωρήθηκε επιστρέφει το σύνολό της.

## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| id | string | Ναι |  |

## Απάντηση

Returns: [`DeletePollVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeletePollVoteResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'deletePollVote Παράδειγμα'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runDeletePollVoteExample(): Promise<void> {
  const tenantId: string = "tenant_42abc";
  const pollId: string = "poll_7f9e2d";

  const result: DeletePollVoteResponse = await deletePollVote(tenantId, pollId);

  console.log(result);
}
[inline-code-end]

---