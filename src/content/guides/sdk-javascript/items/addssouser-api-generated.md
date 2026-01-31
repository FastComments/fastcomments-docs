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
const tenantId: string = 'tenant_7f3c2b9e-4a1d-4d2a-b1c2-6f2e8f9a7c3b';
const createAPISSOUserData: CreateAPISSOUserData = {
  email: 'jane.doe@acme-corp.com',
  displayName: 'Jane Doe',
  externalId: 'okta|00u1abcd2EFGHIJ3',
  roles: ['moderator'], // optional role assignment
  metadata: { department: 'Product', office: 'NYC' } // optional metadata
};
const result: AddSSOUserAPIResponse = await addSSOUser(tenantId, createAPISSOUserData);
[inline-code-end]
