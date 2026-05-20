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
const tenantId: string = 'acme-enterprises';
const id: string = 'user-0a1b2c3d';
const updateAPISSOUserData: UpdateAPISSOUserData = {
  email: 'jane.doe@acme.com',
  displayName: 'Jane Doe',
  jobTitle: 'Engineering Manager',
  roles: ['admin', 'sso_user'],
  isActive: true
};
const updateComments: boolean = true;
const result: PatchSSOUserAPIResponse = await patchSSOUser(tenantId, id, updateAPISSOUserData, updateComments);
[inline-code-end]
