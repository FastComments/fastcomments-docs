## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| direction | CreateVoteDirectionEnum | 是 |  |
| userId | string | 否 |  |
| anonUserId | string | 否 |  |

## 响应

返回: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteComment200Response.ts)

## 示例

[inline-code-attrs-start title = 'createVote 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f8b6a';
const commentId: string = 'comment_3b7d2e';
const direction: CreateVoteDirectionEnum = CreateVoteDirectionEnum.Up;
const anonUserId: string = 'anon_4c2a1f';

const voteResult: VoteComment200Response = await createVote(tenantId, commentId, direction, undefined, anonUserId);
[inline-code-end]

---