## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateAPISSOUserData | UpdateAPISSOUserData | Yes |  |
| updateComments | boolean | No |  |

## Response

Returns: [`PatchSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PatchSSOUserAPIResponse.ts)

## Example

[inline-code-attrs-start title = 'patchSSOUser Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_acme_01';
  const id: string = 'ssouser_0a1b2c3d';
  const updateData: UpdateAPISSOUserData = {
    email: 'jane.doe@acme-corp.com',
    name: 'Jane Doe',
    provider: 'saml',
    externalId: 'CN=Jane Doe,OU=Users,DC=acme,DC=com'
  };
  const updateComments: boolean = true;
  const result: PatchSSOUserAPIResponse = await patchSSOUser(tenantId, id, updateData, updateComments);
})();
[inline-code-end]
