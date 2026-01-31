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
const tenantId: string = "tenant_9a1c3f";
const userId: string = "user-42b7";
const urlId: string = "post-2026-01-new-features";
const fromCommentId: string = "cmt-78391";
const viewed: boolean = false;
const type: string = "mention";

const notificationCount: GetNotificationCount200Response = await getNotificationCount(
  tenantId,
  userId,
  urlId,
  fromCommentId,
  viewed,
  type
);
[inline-code-end]
