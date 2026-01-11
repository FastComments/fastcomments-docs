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
const tenantId: string = 'tenant_9b7f2c';
const email: string = 'jane.doe@acme-corp.com';
const response: GetSSOUserByEmailAPIResponse = await getSSOUserByEmail(tenantId, email);
const user: APISSOUser | undefined = (response as unknown as { user?: APISSOUser }).user;
const emailVerified: boolean | undefined = user?.emailVerified;
[inline-code-end]
