## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| commentId | string | Ναι |  |
| direction | CreateVoteDirectionEnum | Ναι |  |
| userId | string | Όχι |  |
| anonUserId | string | Όχι |  |

## Απάντηση

Επιστρέφει: [`VoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα createVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "comment_67890";
const direction: CreateVoteDirectionEnum = CreateVoteDirectionEnum.UP;
const userId: string = "user_abcde";

const voteResult: VoteResponse = await createVote(tenantId, commentId, direction, userId);

const anonUserId: string = "anon_zyxwv";
const anonVoteResult: VoteResponse = await createVote(tenantId, commentId, direction, undefined, anonUserId);
[inline-code-end]

---