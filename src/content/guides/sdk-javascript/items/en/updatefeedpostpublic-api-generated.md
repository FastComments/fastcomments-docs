## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| postId | string | Yes |  |
| updateFeedPostParams | UpdateFeedPostParams | Yes |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Response

Returns: [`CreateFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateFeedPostPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'updateFeedPostPublic Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp';
const postId: string = 'post_9f2b4a';
const updateFeedPostParams: UpdateFeedPostParams = {
  title: 'Q2 Release Notes',
  body: 'We rolled out feature X, fixed critical bugs, and improved load times.',
  mediaItems: [
    {
      id: 'media_123',
      type: 'image',
      assets: [{ url: 'https://cdn.acme.com/images/release.png', mimeType: 'image/png' }]
    }
  ],
  links: [{ url: 'https://acme.com/releases/q2', title: 'Full release notes' }],
  isPublished: true
};
const broadcastId: string = 'broadcast_20260520';
const sso: string = 'sso_token_ab12cd34';
const result: CreateFeedPostPublic200Response = await updateFeedPostPublic(tenantId, postId, updateFeedPostParams, broadcastId, sso);
[inline-code-end]
