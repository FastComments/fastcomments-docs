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
const tenantId: string = 'acme-corp-001';
const createData: CreateAPISSOUserData = {
  email: 'jane.doe@acme-corp.com',
  givenName: 'Jane',
  familyName: 'Doe',
  externalId: 'acme|jdoe',
  roles: ['admin'],
  phoneNumber: '+1-415-555-0123'
};
const result: AddSSOUserAPIResponse = await addSSOUser(tenantId, createData);
const createdUser: APISSOUser | undefined = result?.user;
[inline-code-end]
