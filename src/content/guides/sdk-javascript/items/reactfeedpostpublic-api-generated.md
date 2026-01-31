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
const tenantId: string = "tenant_9f2b7e";
const postId: string = "post_000142";
const reactBodyParams: ReactBodyParams = { reactionType: "heart", actorId: "user_42", metadata: { platform: "web" } };
const isUndo: boolean = false;
const broadcastId: string = "broadcast_live_20260109";
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjoidXNlcl80MiJ9.signature123";
const result: ReactFeedPostPublic200Response = await reactFeedPostPublic(tenantId, postId, reactBodyParams, isUndo, broadcastId, sso);
[inline-code-end]
