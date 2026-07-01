## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|-------------|--------------|
| tenantId | string | Sí |  |
| createTenantUserBody | CreateTenantUserBody | Sí |  |

## Respuesta

Devuelve: [`CreateTenantUserResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantUserResponse1.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo createTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "a1b2c3d4-5678-90ab-cdef-1234567890ab";

const newUser: CreateTenantUserBody = {
  email: "jane.doe@example.com",
  firstName: "Jane",
  lastName: "Doe",
  role: "admin",
  // parámetros opcionales
  phoneNumber: "+15551234567",
  isActive: true,
};

const result: CreateTenantUserResponse1 = await createTenantUser(tenantId, newUser);
[inline-code-end]