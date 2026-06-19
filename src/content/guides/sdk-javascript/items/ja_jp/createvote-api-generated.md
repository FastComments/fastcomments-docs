## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| commentId | string | はい |  |
| direction | CreateVoteDirectionEnum | はい |  |
| userId | string | いいえ |  |
| anonUserId | string | いいえ |  |

## レスポンス

戻り値: [`VoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteResponse.ts)

## 例

[inline-code-attrs-start title = 'createVote の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_82a1c4b7';
const commentId: string = 'cmt_5f4d3a2b1c';
const direction: CreateVoteDirectionEnum = CreateVoteDirectionEnum.UP;
const anonUserId: string = 'anon_9f8e7d6c';
const voteResponse: VoteResponse = await createVote(tenantId, commentId, direction, undefined, anonUserId);
[inline-code-end]

---