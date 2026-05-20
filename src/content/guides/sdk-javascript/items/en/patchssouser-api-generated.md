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
const tenantId: string = "tenant_8f3a4b2c";
const id: string = "sso_user_72b9a1";
const updateAPISSOUserData: UpdateAPISSOUserData = {
  email: "jane.doe@acme-corp.com",
  displayName: "Jane Doe",
  department: "Product",
  roles: ["user", "editor"]
};
const updateComments: boolean = true;
const result: PatchSSOUserAPIResponse = await patchSSOUser(tenantId, id, updateAPISSOUserData, updateComments)
[inline-code-end]
