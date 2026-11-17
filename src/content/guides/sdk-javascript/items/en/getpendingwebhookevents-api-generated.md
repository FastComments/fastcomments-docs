## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | No |  |
| externalId | string | No |  |
| eventType | string | No |  |
| type | string | No |  |
| domain | string | No |  |
| attemptCountGT | number | No |  |
| skip | number | No |  |

## Response

Returns: [`GetPendingWebhookEventsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEventsResponse.ts)

## Example

[inline-code-attrs-start title = 'getPendingWebhookEvents Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '5a2f4d3c8b9e1a2b3c4d5e6f';

const pendingEvents: GetPendingWebhookEventsResponse = await getPendingWebhookEvents(tenantId, {
    commentId: 'abc123def456',
    eventType: 'comment-created',
    attemptCountGT: 2,
    skip: 0
});

console.log(`Found ${pendingEvents.items.length} pending webhook events`);
[inline-code-end]
