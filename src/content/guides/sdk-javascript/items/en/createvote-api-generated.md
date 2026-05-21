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
const tenantId: string = 'tenant_6a1b2c';
const commentId: string = 'cmt_d4e5f6';
const direction: CreateVoteDirectionEnum = 'UP' as CreateVoteDirectionEnum;
const userId: string = 'user_93b7a1';
const result: VoteComment200Response = await createVote(tenantId, commentId, direction, userId);
[inline-code-end]
