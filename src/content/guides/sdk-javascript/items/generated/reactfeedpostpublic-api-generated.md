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
const tenantId: string = 'acme-corp-tenant-47';
const postId: string = 'post_8f3a9b2c';
const reactBodyParams: ReactBodyParams = { reaction: 'like', userId: 'editor_42', metadata: { locale: 'en-US' } };
const isUndo: boolean = false;
const broadcastId: string = 'broadcast_20251122_01';
const sso: string = 'sso_eyJ1c2VyIjoiZWRpdG9yXzQyIn0';
const result: ReactFeedPostPublic200Response = await reactFeedPostPublic(tenantId, postId, reactBodyParams, isUndo, broadcastId, sso);
[inline-code-end]
