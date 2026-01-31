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
const tenantId: string = 'tenant_acme_42';
const id: string = 'user_0a1b2c3d';
const updateAPISSOUserData: UpdateAPISSOUserData = {
  ssoId: '0a1b2c3d',
  email: 'jane.doe@acme-corp.com',
  displayName: 'Jane Doe',
  roles: ['editor']
} as UpdateAPISSOUserData;
const updateComments: boolean = true;
const result: PutSSOUserAPIResponse = await putSSOUser(tenantId, id, updateAPISSOUserData, updateComments);
[inline-code-end]
