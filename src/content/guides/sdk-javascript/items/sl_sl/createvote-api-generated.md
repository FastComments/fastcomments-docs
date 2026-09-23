## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| direction | CreateVoteDirectionEnum | Yes |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Odgovor

Vrne: [`VoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer createVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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