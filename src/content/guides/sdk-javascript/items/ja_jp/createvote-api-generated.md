## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| commentId | string | はい |  |
| direction | CreateVoteDirectionEnum | はい |  |
| userId | string | いいえ |  |
| anonUserId | string | いいえ |  |

## レスポンス

戻り値: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteComment200Response.ts)

## 例

[inline-code-attrs-start title = 'createVote の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f8b6a';
const commentId: string = 'comment_3b7d2e';
const direction: CreateVoteDirectionEnum = CreateVoteDirectionEnum.Up;
const anonUserId: string = 'anon_4c2a1f';

const voteResult: VoteComment200Response = await createVote(tenantId, commentId, direction, undefined, anonUserId);
[inline-code-end]

---