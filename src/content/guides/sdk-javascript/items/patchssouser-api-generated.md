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
const tenantId: string = 'tenant-acme-42';
const id: string = 'sso_user_9f8e7d6c';
const updateAPISSOUserData: UpdateAPISSOUserData = {
  email: 'jane.doe@acme-corp.com',
  displayName: 'Jane Doe',
  externalId: 'auth0|1234567890',
  avatarUrl: 'https://cdn.acme-corp.com/avatars/jane.jpg',
  roles: ['moderator']
};
const updateComments: boolean = true;
const result: PatchSSOUserAPIResponse = await patchSSOUser(tenantId, id, updateAPISSOUserData, updateComments);
[inline-code-end]
