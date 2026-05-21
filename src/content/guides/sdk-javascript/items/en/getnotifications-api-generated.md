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
const tenantId: string = "tenant_4f7a9c";
const userId: string | undefined = "user_83a2";
const urlId: string | undefined = "https://app.example.com/posts/123";
const fromCommentId: string | undefined = "cmt_1024";
const viewed: boolean | undefined = false;
const type: string | undefined = "mention";
const skip: number | undefined = 25;
const notifications: GetNotifications200Response = await getNotifications(tenantId, userId, urlId, fromCommentId, viewed, type, skip);
[inline-code-end]
