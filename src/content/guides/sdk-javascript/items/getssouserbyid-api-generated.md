## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetSSOUserByIdAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSSOUserByIdAPIResponse.ts)

## Example

[inline-code-attrs-start title = 'getSSOUserById Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f3b2c9a';
const id: string = 'ssoUser_42f9b6';
const response: GetSSOUserByIdAPIResponse = await getSSOUserById(tenantId, id);
const user: APISSOUser | undefined = response.user;
const primaryEmail: string | undefined = user?.email;
[inline-code-end]
