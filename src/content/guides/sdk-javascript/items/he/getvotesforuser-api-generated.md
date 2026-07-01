## Parameters

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Response

מחזיר: [`GetVotesForUserResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotesForUserResponse1.ts)

## Example

[inline-code-attrs-start title = 'דוגמה getVotesForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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