## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| commentId | string | Da |  |
| direction | CreateVoteDirectionEnum | Da |  |
| userId | string | Ne |  |
| anonUserId | string | Ne |  |

## Odgovor

Vrne: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteComment200Response.ts)

## Primer

[inline-code-attrs-start title = 'Primer createVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'fastcomments-tenant-001';
const commentId: string = 'cmt_8f3b2a9d';
const direction: CreateVoteDirectionEnum = CreateVoteDirectionEnum.Up;
const userId: string = 'user_5021';
const anonUserId: string = 'anon_7a9c';

const voteResponse: VoteComment200Response = await createVote(tenantId, commentId, direction, userId, anonUserId);
[inline-code-end]

---