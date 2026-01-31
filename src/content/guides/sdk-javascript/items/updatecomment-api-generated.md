## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updatableCommentParams | UpdatableCommentParams | Yes |  |
| contextUserId | string | No |  |
| doSpamCheck | boolean | No |  |
| isLive | boolean | No |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'updateComment Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_corp_001';
const id: string = 'comment_832019';
const meta: FCommentMeta = { ipAddress: '198.51.100.24', userAgent: 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)' };
const updatableCommentParams: UpdatableCommentParams = { content: 'Thanks for the article — could you source the statistics in paragraph 3?', meta };
const contextUserId: string = 'moderator_jane_doe';
const doSpamCheck: boolean = true;
const isLive: boolean = true;
const result: FlagCommentPublic200Response = await updateComment(tenantId, id, updatableCommentParams, contextUserId, doSpamCheck, isLive);
[inline-code-end]
