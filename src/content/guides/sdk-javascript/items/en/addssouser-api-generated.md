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
const tenantId: string = 'acme-organization-92';
const createAPISSOUserData: CreateAPISSOUserData = {
  email: 'sarah.connor@skynetpartners.com',
  firstName: 'Sarah',
  lastName: 'Connor',
  externalId: 'okta|678901234',
  roles: ['admin', 'developer'], // optional field demonstrated
  displayName: 'Sarah Connor'    // optional field demonstrated
};
const addUserResult: AddSSOUserAPIResponse = await addSSOUser(tenantId, createAPISSOUserData);
[inline-code-end]
