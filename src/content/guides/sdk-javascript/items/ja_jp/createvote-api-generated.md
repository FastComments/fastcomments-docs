## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| commentId | string | はい |  |
| direction | CreateVoteDirectionEnum | はい |  |
| userId | string | いいえ |  |
| anonUserId | string | いいえ |  |

## レスポンス

返却: [`VoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteResponse.ts)

## 例

[inline-code-attrs-start title = 'createVote の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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