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
const tenantId: string = 'tenant_acme_001';
const id: string = 'user_7842b';
const updateAPISSOUserData: UpdateAPISSOUserData = {
  email: 'jane.doe@acmecorp.com',
  displayName: 'Jane Doe',
  externalId: 'acme|jdoe',
  roles: ['editor'],
  metadata: { team: 'Growth' }
};
const updateComments: boolean = true;
const result: PatchSSOUserAPIResponse = await patchSSOUser(tenantId, id, updateAPISSOUserData, updateComments);
[inline-code-end]
