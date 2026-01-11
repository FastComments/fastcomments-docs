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
const tenantId: string = 'acme-corp-tenant-42';
const createData: CreateAPISSOUserData = {
  externalId: 'google-oauth2|112233445566778899',
  email: 'emma.jones@acme-corp.com',
  displayName: 'Emma Jones',
  roles: ['moderator'], // optional field included
  isAdmin: false,
  avatarUrl: 'https://cdn.acme-corp.com/avatars/emma.jpg',
  metadata: { department: 'product', locale: 'en-US' } // optional metadata
} as CreateAPISSOUserData;
const result: AddSSOUserAPIResponse = await addSSOUser(tenantId, createData);
[inline-code-end]
