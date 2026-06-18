## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| createTenantUserBody | CreateTenantUserBody | Tak |  |

## Odpowiedź

Zwraca: [`CreateTenantUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantUser200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład createTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f4a2b";
const createTenantUserBody: CreateTenantUserBody = {
  email: "jane.doe@example.com",
  firstName: "Jane",
  lastName: "Doe",
  role: "commenter",
  approved: true,
  displayName: "Jane D." // opcjonalne: podanie przyjaznej nazwy
};
const result: CreateTenantUser200Response = await createTenantUser(tenantId, createTenantUserBody);
console.log(result);
[inline-code-end]

---