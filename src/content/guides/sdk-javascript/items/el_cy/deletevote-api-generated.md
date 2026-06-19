## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| id | string | Ναι |  |
| editKey | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteDeleteResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα deleteVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f7c2b1a';
const id: string = 'vote_4b6e9a23';
const editKey: string = 'editkey_02a8f3';

const deleteResultWithoutKey: VoteDeleteResponse = await deleteVote(tenantId, id);
const deleteResultWithKey: VoteDeleteResponse = await deleteVote(tenantId, id, editKey);
[inline-code-end]