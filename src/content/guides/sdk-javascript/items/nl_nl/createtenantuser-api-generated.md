## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createTenantUserBody | CreateTenantUserBody | Yes |  |

## Respons

Retourneert: [`CreateTenantUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantUser200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'createTenantUser Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f4a2b";
const createTenantUserBody: CreateTenantUserBody = {
  email: "jane.doe@example.com",
  firstName: "Jane",
  lastName: "Doe",
  role: "commenter",
  approved: true,
  displayName: "Jane D." // optioneel: een vriendelijke naam opgeven
};
const result: CreateTenantUser200Response = await createTenantUser(tenantId, createTenantUserBody);
console.log(result);
[inline-code-end]