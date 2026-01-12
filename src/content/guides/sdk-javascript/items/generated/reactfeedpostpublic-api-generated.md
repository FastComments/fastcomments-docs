## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| postId | string | Yes |  |
| reactBodyParams | ReactBodyParams | Yes |  |
| isUndo | boolean | No |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Response

Returns: [`ReactFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ReactFeedPostPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'reactFeedPostPublic Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9b1f4c";
const postId: string = "post_c3f1a7";
const reactBody: ReactBodyParams = { reactionType: "like", emoji: "👍", userId: "user_42", timestamp: new Date().toISOString() };
const isUndo: boolean = false;
const broadcastId: string = "broadcast_20260112_001";
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyXzQyIn0.signature";
const result: ReactFeedPostPublic200Response = await reactFeedPostPublic(tenantId, postId, reactBody, isUndo, broadcastId, sso);
[inline-code-end]
