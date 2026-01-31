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
const tenantId: string = "tenant_9b2f4a1c";
const createTenantUserBody: CreateTenantUserBody = {
  email: "maria.lopez@acmecorp.com",
  givenName: "María",
  familyName: "López",
  roles: ["comment_moderator"],
  sendWelcomeEmail: true // optional parameter demonstrated
};
const result: CreateTenantUser200Response = await createTenantUser(tenantId, createTenantUserBody);
[inline-code-end]
