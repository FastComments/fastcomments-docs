## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| notificationId | string | Yes |  |
| newStatus | UpdateUserNotificationStatusNewStatusEnum | Yes |  |
| sso | string | No |  |

## Response

Returns: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationStatus200Response.ts)

## Example

[inline-code-attrs-start title = 'updateUserNotificationStatus Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'acme-corp-tenant-01';
  const notificationId: string = 'f47ac10b-58cc-4372-a567-0e02b2c3d479';
  const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9';
  const result: UpdateUserNotificationStatus200Response = await updateUserNotificationStatus(tenantId, notificationId, UpdateUserNotificationStatusNewStatusEnum.Read, sso);
  console.log(result);
})();
[inline-code-end]
