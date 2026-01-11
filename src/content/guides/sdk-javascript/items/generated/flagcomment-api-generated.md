## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Response

Returns: [`FlagComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagComment200Response.ts)

## Example

[inline-code-attrs-start title = 'flagComment Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-acme-01';
const commentId: string = 'cmt_5f8e9a2b';
const userId: string = 'user_1024';

const flaggedByUserResponse: FlagComment200Response = await flagComment(tenantId, commentId, userId);

const anonUserId: string = 'anon_9f2b3c';
const flaggedAnonResponse: FlagComment200Response = await flagComment(tenantId, commentId, undefined, anonUserId);
[inline-code-end]
