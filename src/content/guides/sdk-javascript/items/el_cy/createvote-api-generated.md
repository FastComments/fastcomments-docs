## Παράμετροι

| Name | Type | Απαραίτητο | Περιγραφή |
|------|------|------------|-----------|
| tenantId | string | Ναι |  |
| commentId | string | Ναι |  |
| direction | CreateVoteDirectionEnum | Ναι |  |
| userId | string | Όχι |  |
| anonUserId | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`VoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα createVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_82a1c4b7';
const commentId: string = 'cmt_5f4d3a2b1c';
const direction: CreateVoteDirectionEnum = CreateVoteDirectionEnum.UP;
const anonUserId: string = 'anon_9f8e7d6c';
const voteResponse: VoteResponse = await createVote(tenantId, commentId, direction, undefined, anonUserId);
[inline-code-end]

---