## Parameter

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Antwort

Rückgabe: [`GetVotesForUserResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotesForUserResponse1.ts)

## Beispiel

[inline-code-attrs-start title = 'getVotesForUser Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demo() {
  const tenantId: string = "acme-corp";
  const urlId: string = "post-9f8b7c";
  const userId: string = "user-42";
  const anonUserId: string = "anon-123";

  const votesRequiredOnly: GetVotesForUserResponse1 = await getVotesForUser(tenantId, urlId);
  const votesWithUserId: GetVotesForUserResponse1 = await getVotesForUser(tenantId, urlId, userId);
  const votesWithAnonId: GetVotesForUserResponse1 = await getVotesForUser(tenantId, urlId, undefined, anonUserId);
}
demo();
[inline-code-end]

---