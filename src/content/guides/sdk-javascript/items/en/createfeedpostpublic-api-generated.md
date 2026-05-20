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
(async () => {
  const tenantId: string = "acme-enterprises";
  const createFeedPostParams: CreateFeedPostParams = {
    title: "April Product Launch",
    content: "We launched the new Acme Turbo widget — features, pricing, and availability inside.",
    media: [
      {
        type: "image",
        caption: "Product hero",
        asset: { url: "https://cdn.acme.com/images/turbo-hero.jpg", mimeType: "image/jpeg", size: 128000 }
      }
    ],
    links: [{ url: "https://acme.com/products/turbo", text: "Product page" }]
  };
  const broadcastId: string = "broadcast-2026-04-15";
  const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...";
  const result: CreateFeedPostPublic200Response = await createFeedPostPublic(tenantId, createFeedPostParams, broadcastId, sso);
  console.log(result);
})();
[inline-code-end]
