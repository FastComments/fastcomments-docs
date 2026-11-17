Recent comments in exactly the shape webhook deliveries use, for building integrations (for example Zapier
sample data). Every event delivers the same comment object, so `event` only has to be valid.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| event | WebhookEventName | No |  |
| limit | number | No |  |

## Response

Returns: [`GetWebhookSamplePayloadsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetWebhookSamplePayloadsResponse.ts)

## Example

[inline-code-attrs-start title = 'getWebhookSamplePayloads Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
declare function getWebhookSamplePayloads(
  tenantId: string,
  event?: WebhookEventName,
  limit?: number
): Promise<GetWebhookSamplePayloadsResponse>;

const tenantId: string = '5f8d9b3e2a1b4c6d7e8f9a0b';

// Get sample payloads for a specific event type
const commentPayloads: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(
  tenantId,
  WebhookEventName.COMMENT_SUCCESS_PAGE,
  10
);

// Get all sample payloads (no optional parameters)
const allPayloads: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(tenantId);

console.log(`Comment payload status: ${commentPayloads.status}`);
console.log(`Fetched ${allPayloads.payloads.length} sample payloads`);
[inline-code-end]
