## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetTenantUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUser200Response.ts)

## Example

[inline-code-attrs-start title = 'getTenantUser Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme_corp_01";
const userId: string = "u_9b2f4d";
let includeDeleted: boolean | undefined = undefined; // optional parameter example (allowed to be omitted)
const result: GetTenantUser200Response = await getTenantUser(tenantId, userId);
const user: User | undefined = (result as unknown as { user?: User }).user;
[inline-code-end]
