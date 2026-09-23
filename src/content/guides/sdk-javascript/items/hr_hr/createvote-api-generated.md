## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| commentId | string | Da |  |
| direction | CreateVoteDirectionEnum | Da |  |
| userId | string | Ne |  |
| anonUserId | string | Ne |  |

## Odgovor

Vraća: [`VoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteResponse.ts)

## Primjer

[inline-code-attrs-start title = 'createVote Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "comment_67890";
const direction: CreateVoteDirectionEnum = CreateVoteDirectionEnum.UP;
const userId: string = "user_abcde";

const voteResult: VoteResponse = await createVote(tenantId, commentId, direction, userId);

const anonUserId: string = "anon_zyxwv";
const anonVoteResult: VoteResponse = await createVote(tenantId, commentId, direction, undefined, anonUserId);
[inline-code-end]