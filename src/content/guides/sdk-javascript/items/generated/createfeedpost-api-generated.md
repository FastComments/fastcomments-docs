## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createFeedPostParams | CreateFeedPostParams | Yes |  |
| broadcastId | string | No |  |
| isLive | boolean | No |  |
| doSpamCheck | boolean | No |  |
| skipDupCheck | boolean | No |  |

## Response

Returns: [`CreateFeedPost200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateFeedPost200Response.ts)

## Example

[inline-code-attrs-start title = 'createFeedPost Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f2b1c';

const createFeedPostParams: CreateFeedPostParams = ({
  title: 'Feature X US Launch',
  body: 'Feature X is now available to all US customers. See rollout details and known issues below.',
  authorId: 'user_8421',
  tags: ['launch', 'feature-x', 'us'],
  media: [
    {
      type: 'image',
      assets: [{ url: 'https://cdn.example.com/featurex/hero.png', mimeType: 'image/png', width: 1200, height: 628 }]
    }
  ],
  links: [{ url: 'https://status.example.com/feature-x', title: 'Live Status' }]
} as unknown) as CreateFeedPostParams;

const broadcastId: string = 'broadcast_2025_11_22';
const isLive: boolean = false;
const doSpamCheck: boolean = true;
const skipDupCheck: boolean = false;

const result: CreateFeedPost200Response = await createFeedPost(tenantId, createFeedPostParams, broadcastId, isLive, doSpamCheck, skipDupCheck);
[inline-code-end]
