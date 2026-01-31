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
const tenantId: string = "tenant_9b3f2";
const createFeedPostParams: CreateFeedPostParams = {
  title: "Product update: Moderation tools",
  body: "We launched improved moderation controls and analytics for community managers.",
  media: [
    {
      type: "image",
      caption: "Dashboard preview",
      assets: [{ url: "https://assets.fastcomments.com/dash-preview.png", width: 1280, height: 720 }]
    }
  ],
  links: [{ url: "https://fastcomments.com/blog/mod-tools", title: "Read more" }],
  visibility: "public"
};
const broadcastId: string = "broadcast_20260109";
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.fakeSignature";
const result: CreateFeedPostPublic200Response = await createFeedPostPublic(tenantId, createFeedPostParams, broadcastId, sso);
[inline-code-end]
