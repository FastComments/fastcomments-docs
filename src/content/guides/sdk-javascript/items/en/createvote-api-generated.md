## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| direction | CreateVoteDirectionEnum | Yes |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Response

Returns: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteComment200Response.ts)

## Example

[inline-code-attrs-start title = 'createVote Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_01';
const commentId: string = 'comment_6f3b2';
const direction: CreateVoteDirectionEnum = CreateVoteDirectionEnum.UP;
const userId: string = 'user_9342';
const voteResult: VoteComment200Response = await createVote(tenantId, commentId, direction, userId);
[inline-code-end]
