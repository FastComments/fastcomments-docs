## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| direction | CreateVoteDirectionEnum | 是 |  |
| userId | string | 否 |  |
| anonUserId | string | 否 |  |

## 响应

返回：[`VoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteResponse.ts)

## 示例

[inline-code-attrs-start title = 'createVote 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_82a1c4b7';
const commentId: string = 'cmt_5f4d3a2b1c';
const direction: CreateVoteDirectionEnum = CreateVoteDirectionEnum.UP;
const anonUserId: string = 'anon_9f8e7d6c';
const voteResponse: VoteResponse = await createVote(tenantId, commentId, direction, undefined, anonUserId);
[inline-code-end]

---