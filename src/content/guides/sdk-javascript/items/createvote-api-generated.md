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
const tenantId: string = "tenant_acme_1001";
const commentId: string = "cmt_20260109_452";
const direction: CreateVoteDirectionEnum = CreateVoteDirectionEnum.UP;
const anonUserId: string = "anon_5f3b9c";

const voteResponse: VoteComment200Response = await createVote(tenantId, commentId, direction, undefined, anonUserId);
[inline-code-end]
