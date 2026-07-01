## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|------------|
| tenantId | string | Ναι |  |
| urlId | string | Ναι |  |
| userId | string | Όχι |  |
| anonUserId | string | Όχι |  |

## Απάντηση

Επιστρέφει: [`GetVotesForUserResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotesForUserResponse1.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getVotesForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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