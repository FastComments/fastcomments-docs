## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| email | string | Yes |  |

## Response

Returns: [`GetSSOUserByEmailAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSSOUserByEmailAPIResponse.ts)

## Example

[inline-code-attrs-start title = 'getSSOUserByEmail Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_fc_7a9d3b";
const email: string = "laura.powell@dailynews.com";
const response: GetSSOUserByEmailAPIResponse = await getSSOUserByEmail(tenantId, email);
const ssoUser: APISSOUser | undefined = response.user;
const ssoUserEmail: string | undefined = ssoUser?.email;
[inline-code-end]
