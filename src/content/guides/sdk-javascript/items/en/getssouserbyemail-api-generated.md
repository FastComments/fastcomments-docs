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
const tenantId: string = 'acme-enterprises-7';
const email: string = 'jane.doe@acme.com';
const response: GetSSOUserByEmailAPIResponse = await getSSOUserByEmail(tenantId, email);
const getUserIfEmailProvided: (tenant: string, mail?: string) => Promise<GetSSOUserByEmailAPIResponse | undefined> =
  async (tenant: string, mail?: string): Promise<GetSSOUserByEmailAPIResponse | undefined> =>
    mail ? await getSSOUserByEmail(tenant, mail) : undefined;
[inline-code-end]
