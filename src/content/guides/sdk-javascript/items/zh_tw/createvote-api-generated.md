## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| direction | CreateVoteDirectionEnum | 是 |  |
| userId | string | 否 |  |
| anonUserId | string | 否 |  |

## 回應

回傳：[`VoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteResponse.ts)

## 範例

[inline-code-attrs-start title = 'createVote 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_82a1c4b7';
const commentId: string = 'cmt_5f4d3a2b1c';
const direction: CreateVoteDirectionEnum = CreateVoteDirectionEnum.UP;
const anonUserId: string = 'anon_9f8e7d6c';
const voteResponse: VoteResponse = await createVote(tenantId, commentId, direction, undefined, anonUserId);
[inline-code-end]

---