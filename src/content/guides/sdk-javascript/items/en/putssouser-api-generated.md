## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateAPISSOUserData | UpdateAPISSOUserData | Yes |  |
| updateComments | boolean | No |  |

## Response

Returns: [`PutSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PutSSOUserAPIResponse.ts)

## Example

[inline-code-attrs-start title = 'putSSOUser Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-001';
const userId: string = 'f47ac10b-58cc-4372-a567-0e02b2c3d479';
const updateData: UpdateAPISSOUserData = {
  email: 'jane.doe@acme.com',
  displayName: 'Jane Doe',
  roles: ['admin'],
  isActive: true
};
const result: PutSSOUserAPIResponse = await putSSOUser(tenantId, userId, updateData, true);
[inline-code-end]
