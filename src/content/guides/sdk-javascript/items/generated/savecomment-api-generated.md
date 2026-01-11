## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createCommentParams | CreateCommentParams | Yes |  |
| isLive | boolean | No |  |
| doSpamCheck | boolean | No |  |
| sendEmails | boolean | No |  |
| populateNotifications | boolean | No |  |

## Response

Returns: [`SaveComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SaveComment200Response.ts)

## Example

[inline-code-attrs-start title = 'saveComment Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_6f9a2b4c';
const createCommentParams: CreateCommentParams = {
  threadId: 'article_2025-11-22_001',
  content: 'Great analysis — I learned a lot from this breakdown.',
  userId: 'user_00421',
  userDisplayName: 'Aisha Patel',
  userEmail: 'aisha.patel@example.org',
  metadata: { device: 'iPhone', locale: 'en-US' }
};
const isLive: boolean = true;
const doSpamCheck: boolean = true;
const sendEmails: boolean = false;
const populateNotifications: boolean = true;
const result: SaveComment200Response = await saveComment(tenantId, createCommentParams, isLive, doSpamCheck, sendEmails, populateNotifications);
[inline-code-end]
