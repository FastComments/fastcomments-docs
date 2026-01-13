## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createAPISSOUserData | CreateAPISSOUserData | Yes |  |

## Response

Returns: [`AddSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddSSOUserAPIResponse.ts)

## Example

[inline-code-attrs-start title = 'addSSOUser Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-7f8c2d4b';
const createAPISSOUserData: CreateAPISSOUserData = {
  externalId: 'okta|00u1abcd2345EfGH',
  email: 'jane.doe@acme-corp.com',
  displayName: 'Jane Doe',
  avatarUrl: 'https://cdn.acme-corp.com/avatars/jane.jpg',
  sendWelcomeEmail: true,
  metadata: { department: 'Engineering', employeeId: 'E12345' }
};
const result: AddSSOUserAPIResponse = await addSSOUser(tenantId, createAPISSOUserData);
[inline-code-end]
