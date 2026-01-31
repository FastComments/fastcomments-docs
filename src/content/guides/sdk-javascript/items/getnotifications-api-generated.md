## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | No |  |
| urlId | string | No |  |
| fromCommentId | string | No |  |
| viewed | boolean | No |  |
| type | string | No |  |
| skip | number | No |  |

## Response

Returns: [`GetNotifications200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotifications200Response.ts)

## Example

[inline-code-attrs-start title = 'getNotifications Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_12345';
const userId: string | undefined = 'user_98765';
const urlId: string | undefined = 'url_3f2a1b';
const fromCommentId: string | undefined = undefined;
const viewed: boolean | undefined = false;
const type: string | undefined = 'mention';
const skip: number | undefined = 0;
const response: GetNotifications200Response = await getNotifications(tenantId, userId, urlId, fromCommentId, viewed, type, skip);
[inline-code-end]
