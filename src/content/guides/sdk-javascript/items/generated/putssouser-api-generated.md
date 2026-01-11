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
const tenantId: string = 'acme-tenant-9b3f2a';
const id: string = 'sso-user-7f1c2d';
const updateData: UpdateAPISSOUserData = {
  externalId: 'auth0|6032b1a7',
  email: 'jane.doe@acme.com',
  displayName: 'Jane Doe',
  avatarUrl: 'https://cdn.acme.com/avatars/jane.jpg',
  roles: ['moderator'],
  metadata: { team: 'Platform', location: 'NYC' }
};
const updateComments: boolean = true;
const result: PutSSOUserAPIResponse = await putSSOUser(tenantId, id, updateData, updateComments);
[inline-code-end]
