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
const tenantId: string = "fastcomments-enterprise-42";
const id: string = "sso_user_9f3b1c2d";
const result: GetSSOUserByIdAPIResponse = await getSSOUserById(tenantId, id);
const ssoUser: APISSOUser | undefined = result.user;
const email: string | undefined = ssoUser?.email;
console.log(email);
[inline-code-end]
