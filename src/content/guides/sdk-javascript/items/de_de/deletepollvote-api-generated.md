Ziehen Sie eine Stimme zurück. Die Option, auf die sie abgegeben wurde, gibt ihre Stimmenzahl zurück.

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|-----|--------------|--------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Antwort

Rückgabe: [`DeletePollVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeletePollVoteResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'deletePollVote Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runDeletePollVoteExample(): Promise<void> {
  const tenantId: string = "tenant_42abc";
  const pollId: string = "poll_7f9e2d";

  const result: DeletePollVoteResponse = await deletePollVote(tenantId, pollId);

  console.log(result);
}
[inline-code-end]

---