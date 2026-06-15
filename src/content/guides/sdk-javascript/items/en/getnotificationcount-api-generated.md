## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | No |  |
| urlId | string | No |  |
| fromCommentId | string | No |  |
| viewed | boolean | No |  |
| type | string | No |  |

## Response

Returns: [`GetNotificationCount200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotificationCount200Response.ts)

## Example

[inline-code-attrs-start title = 'getNotificationCount Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_82a3b9f';
const userId: string = 'user_43721';
const urlId: string = 'https://news.example.com/articles/2026/06/15/coverage-123';
const fromCommentId: string = 'comment_98765';
const viewed: boolean = false;
const notificationType: string = 'mention';

const result: GetNotificationCount200Response = await getNotificationCount(tenantId, userId, urlId, fromCommentId, viewed, notificationType);
[inline-code-end]
