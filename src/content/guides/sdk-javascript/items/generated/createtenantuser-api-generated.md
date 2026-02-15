## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createTenantUserBody | CreateTenantUserBody | Yes |  |

## Response

Returns: [`CreateTenantUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantUser200Response.ts)

## Example

[inline-code-attrs-start title = 'createTenantUser Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9a8c7e4b";
const createTenantUserBody: CreateTenantUserBody = {
  email: "julia.smith@acme-corp.com",
  displayName: "Julia Smith",
  sendInviteEmail: true, // optional parameter demonstrated
  locale: "en-US",
  metadata: { department: "Customer Success" }
};
const result: CreateTenantUser200Response = await createTenantUser(tenantId, createTenantUserBody);
[inline-code-end]
