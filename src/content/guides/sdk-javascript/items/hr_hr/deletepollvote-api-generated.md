---
Povuci glas. Opcija na koju je glas bio izabran vraća svoj broj glasova.

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| id | string | Da |  |

## Odgovor

Vraća: [`DeletePollVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeletePollVoteResponse.ts)

## Primjer

[inline-code-attrs-start title = 'deletePollVote Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runDeletePollVoteExample(): Promise<void> {
  const tenantId: string = "tenant_42abc";
  const pollId: string = "poll_7f9e2d";

  const result: DeletePollVoteResponse = await deletePollVote(tenantId, pollId);

  console.log(result);
}
[inline-code-end]

---