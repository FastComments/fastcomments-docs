## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createFeedPostParams | CreateFeedPostParams | Yes |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Response

Returns: [`CreateFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateFeedPostPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'createFeedPostPublic Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_corp_001';
const createFeedPostParams: CreateFeedPostParams = {
  title: 'Introducing the new inline editor',
  body: 'Today we launched an editor that supports markdown, drag-and-drop images, and live preview.',
  authorId: 'user_4521',
  isPublished: true,
  tags: ['product','launch','editor'],
  media: [
    {
      type: 'image',
      assets: [
        { url: 'https://cdn.acme.com/images/editor-preview.png', mimeType: 'image/png', width: 1200, height: 800 }
      ]
    }
  ],
  links: [{ url: 'https://acme.com/blog/editor-release', title: 'Read the full release notes' }]
};
const broadcastId: string = 'broadcast_weekly_2025_11';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.example_signature';
const response: CreateFeedPostPublic200Response = await createFeedPostPublic(tenantId, createFeedPostParams, broadcastId, sso);
[inline-code-end]
