## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| postId | string | Yes |  |
| reactBodyParams | ReactBodyParams | Yes |  |
| isUndo | boolean | No |  |
| broadcastId | string | No |  |
| urlId | string | No |  |
| sso | string | No |  |

## Response

Returns: [`ReactFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ReactFeedPostPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'reactFeedPostPublic Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_company42';
const postId: string = 'post_2026-05-20_0012';
const reactBodyParams: ReactBodyParams = { reaction: 'like', emoji: '👍' } as ReactBodyParams;
const isUndo: boolean = false;
const broadcastId: string = 'broadcast_84';
const urlId: string = 'url_9f8b';
const sso: string = 'sso_jwt_eyJhbGciOiJIUzI1Ni';
const result: ReactFeedPostPublic200Response = await reactFeedPostPublic(
  tenantId,
  postId,
  reactBodyParams,
  isUndo,
  broadcastId,
  urlId,
  sso
);
[inline-code-end]
